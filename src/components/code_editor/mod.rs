use std::collections::HashMap;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

// Internal modules
pub mod annotation;
pub mod diff;
pub mod diff_types;
pub mod diffview;
pub mod styles;
pub mod syntax_highlighter;
pub mod type_hint;

#[cfg(test)]
mod diffview_test;
#[cfg(test)]
mod syntax_test;

pub use annotation::{Annotation, AnnotationType};
pub use diff::{Diff, DiffType};
pub use diff_types::{DiffViewMode, DiffSide, ChangeType};
pub use diffview::DiffView;
pub use type_hint::TypeHint;

/// A simple, fast code editor component
#[derive(Properties, PartialEq, Clone)]
pub struct CodeEditorProps {
    /// Initial code content
    #[prop_or_default]
    pub code: String,

    /// Show line numbers
    #[prop_or(true)]
    pub show_line_numbers: bool,

    /// Language for syntax highlighting
    #[prop_or_else(|| "rust".to_string())]
    pub language: String,

    /// Theme name (light or dark)
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,

    /// Font size in pixels
    #[prop_or(14)]
    pub font_size: u8,

    /// Font family
    #[prop_or_else(|| "JetBrains Mono, monospace".to_string())]
    pub font_family: String,

    /// Line height
    #[prop_or(1.5)]
    pub line_height: f32,

    /// Custom CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Inline code diffs
    #[prop_or_default]
    pub diffs: Vec<Diff>,

    /// Code annotations
    #[prop_or_default]
    pub annotations: Vec<Annotation>,

    /// Type hints
    #[prop_or_default]
    pub type_hints: Vec<TypeHint>,

    /// Optional callback when code changes
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,

    /// Optional callback for code focus
    #[prop_or_default]
    pub on_focus: Option<Callback<FocusEvent>>,

    /// Optional callback for code blur
    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent>>,

    /// Tab size
    #[prop_or(4)]
    pub tab_size: u8,

    /// Whether the editor is in read-only mode
    #[prop_or(false)]
    pub read_only: bool,

    /// Maximum height of the editor in pixels (scrolls after this)
    #[prop_or(0)]
    pub max_height: u32,

    /// Enable line wrapping
    #[prop_or(true)]
    pub line_wrap: bool,

    /// Enable multi-cursor support
    #[prop_or(false)]
    pub enable_multi_cursor: bool,

    /// Enable custom keymap
    #[prop_or(false)]
    pub enable_keymap: bool,

    /// Custom keymap definitions
    #[prop_or_default]
    pub keymap: Option<HashMap<String, Callback<KeyboardEvent>>>,

    /// Custom inline style
    #[prop_or_default]
    pub style: String,

    /// Enable diff view mode with original line numbers
    #[prop_or(false)]
    pub diff_view: bool,

    /// Original line numbers for diff view (maps display line to original line)
    #[prop_or_default]
    pub original_line_numbers: Vec<Option<usize>>,
}

pub enum CodeEditorMsg {
    CodeChanged(String),
    Focus(FocusEvent),
    Blur(FocusEvent),
    Keydown(KeyboardEvent),
}

pub struct CodeEditor {
    code: String,
    textarea_ref: NodeRef,
    _listeners: Vec<EventListener>,
}

impl Component for CodeEditor {
    type Message = CodeEditorMsg;
    type Properties = CodeEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            code: ctx.props().code.clone(),
            textarea_ref: NodeRef::default(),
            _listeners: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CodeEditorMsg::CodeChanged(code) => {
                self.code = code.clone();
                if let Some(callback) = &ctx.props().on_change {
                    callback.emit(code);
                }
                true
            }
            CodeEditorMsg::Focus(event) => {
                if let Some(callback) = &ctx.props().on_focus {
                    callback.emit(event);
                }
                false
            }
            CodeEditorMsg::Blur(event) => {
                if let Some(callback) = &ctx.props().on_blur {
                    callback.emit(event);
                }
                false
            }
            CodeEditorMsg::Keydown(event) => {
                if !ctx.props().read_only {
                    self.handle_keydown(event, ctx);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let _lines = self.code.lines().collect::<Vec<_>>();

        let container_style = format!(
            "font-family: {}; font-size: {}px; line-height: {};{}",
            props.font_family,
            props.font_size,
            props.line_height,
            if !props.style.is_empty() {
                format!(" {}", props.style)
            } else {
                String::new()
            }
        );

        let editor_style = format!(
            "tab-size: {}; -moz-tab-size: {};",
            props.tab_size, props.tab_size
        );

        let max_height_style = if props.max_height > 0 {
            format!("max-height: {}px; overflow-y: auto;", props.max_height)
        } else {
            String::new()
        };

        html! {
            <>
                <styles::CodeEditorStyles />
                <div
                    class={classes!(
                        props.class.clone(),
                        "relative", "border", "border-gray-300", "dark:border-gray-700",
                        "bg-white", "dark:bg-gray-900", "rounded", "overflow-hidden"
                    )}
                    style={format!("{} {}", container_style, max_height_style)}
                >
                    <div class="flex">
                        // Line numbers with diff indicators
                        if props.show_line_numbers {
                            <div class="flex-none bg-gray-100 dark:bg-gray-800 px-3 py-2 text-gray-500 dark:text-gray-400 text-right select-none border-r border-gray-300 dark:border-gray-700">
                                { 
                                    if props.diff_view {
                                        self.render_diff_line_numbers(ctx) 
                                    } else {
                                        self.render_line_numbers_with_diffs(ctx)
                                    }
                                }
                            </div>
                        }

                        // Editor area
                        <div class="flex-1 relative">
                            // Diff backgrounds layer (behind everything)
                            <div class="absolute inset-0 pointer-events-none">
                                { self.render_diff_backgrounds(ctx) }
                            </div>

                            // Syntax highlighting overlay - exactly matches textarea positioning
                            <div 
                                class="absolute inset-0 p-2 m-0 pointer-events-none overflow-hidden whitespace-pre-wrap break-words text-gray-900 dark:text-gray-100"
                                style={format!("{} font-family: inherit; font-size: inherit; line-height: inherit;", editor_style)}
                            >
                                { self.render_highlighted_code_with_annotations(ctx) }
                            </div>

                            // Annotations and type hints overlay (above highlighting but below textarea)
                            <div class="absolute inset-0 pointer-events-none">
                                { self.render_annotations_and_hints(ctx) }
                            </div>

                            // Inline diff controls overlay (for accept/reject buttons)
                            if props.diff_view {
                                <div class="absolute inset-0 pointer-events-none">
                                    { self.render_inline_diff_controls(ctx) }
                                </div>
                            }

                            // Actual textarea - transparent text, visible cursor
                            <textarea
                                ref={self.textarea_ref.clone()}
                                class="absolute inset-0 p-2 m-0 resize-none bg-transparent outline-none border-none whitespace-pre-wrap break-words"
                                style={format!("{} color: transparent; caret-color: #3b82f6; font-family: inherit; font-size: inherit; line-height: inherit;", editor_style)}
                                value={self.code.clone()}
                                readonly={props.read_only}
                                spellcheck="false"
                                autocomplete="off"
                                autocorrect="off"
                                autocapitalize="off"
                                wrap="off"
                            />
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.setup_listeners(ctx);
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        
        // Update code if it changed from props
        if props.code != self.code {
            self.code = props.code.clone();
            if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea.set_value(&self.code);
            }
            return true;
        }

        // Re-render if language or theme changed
        old_props.language != props.language || old_props.theme != props.theme
    }
}

impl CodeEditor {
    fn setup_listeners(&mut self, ctx: &Context<Self>) {
        if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            // Input event for real-time updates
            {
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "input", move |event| {
                    if let Some(target) = event.target() {
                        if let Some(textarea) = target.dyn_ref::<HtmlTextAreaElement>() {
                            let value = textarea.value();
                            link.send_message(CodeEditorMsg::CodeChanged(value));
                        }
                    }
                });
                self._listeners.push(listener);
            }

            // Focus event
            {
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "focus", move |event| {
                    link.send_message(CodeEditorMsg::Focus(
                        event.clone().dyn_into::<FocusEvent>().unwrap(),
                    ));
                });
                self._listeners.push(listener);
            }

            // Blur event
            {
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "blur", move |event| {
                    link.send_message(CodeEditorMsg::Blur(
                        event.clone().dyn_into::<FocusEvent>().unwrap(),
                    ));
                });
                self._listeners.push(listener);
            }

            // Keydown event
            {
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "keydown", move |event| {
                    link.send_message(CodeEditorMsg::Keydown(
                        event.clone().dyn_into::<KeyboardEvent>().unwrap(),
                    ));
                });
                self._listeners.push(listener);
            }
        }
    }

    fn handle_keydown(&mut self, event: KeyboardEvent, ctx: &Context<Self>) {
        let key = event.key();
        
        if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            match key.as_str() {
                "Tab" => {
                    event.prevent_default();
                    let tab_spaces = " ".repeat(ctx.props().tab_size as usize);
                    
                    let start = textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
                    let end = textarea.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;
                    let value = textarea.value();
                    
                    let new_value = format!(
                        "{}{}{}",
                        &value[0..start],
                        tab_spaces,
                        &value[end..]
                    );
                    
                    textarea.set_value(&new_value);
                    let new_pos = start + tab_spaces.len();
                    textarea.set_selection_start(Some(new_pos as u32)).ok();
                    textarea.set_selection_end(Some(new_pos as u32)).ok();
                    
                    ctx.link().send_message(CodeEditorMsg::CodeChanged(new_value));
                }
                _ => {
                    // Let default behavior handle other keys
                }
            }
        }
    }

    fn render_line_numbers_with_diffs(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();
        
        // Create a map of line numbers to their diffs for quick lookup
        let mut diff_map = std::collections::HashMap::new();
        for diff in &props.diffs {
            diff_map.insert(diff.line_number, diff);
        }

        lines.iter().enumerate().map(|(i, _)| {
            let line_num = i + 1;
            let has_diff = diff_map.get(&line_num);
            
            let diff_classes = if let Some(diff) = has_diff {
                match diff.diff_type {
                    DiffType::Added => "text-emerald-500 dark:text-emerald-400 font-bold",
                    DiffType::Removed => "text-rose-500 dark:text-rose-400 font-bold", 
                    DiffType::Modified => "text-amber-500 dark:text-amber-400 font-bold",
                }
            } else {
                ""
            };

            html! {
                <div key={i} class={format!("leading-[inherit] flex items-center justify-end gap-1 {}", diff_classes)}>
                    <span>{ line_num }</span>
                    if has_diff.is_some() {
                        <span class="w-3 h-3 rounded-full bg-current shadow-lg animate-pulse"></span>
                    }
                </div>
            }
        }).collect::<Html>()
    }

    fn render_diff_line_numbers(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();
        
        // Create a map of line numbers to their diffs for quick lookup
        let mut diff_map = std::collections::HashMap::new();
        for diff in &props.diffs {
            diff_map.insert(diff.line_number, diff);
        }

        let mut line_counter = 0; // Track actual line numbers (skip removed lines)

        lines.iter().enumerate().map(|(i, _)| {
            let display_line_num = i + 1;
            let has_diff = diff_map.get(&display_line_num);
            
            let (diff_classes, symbol, show_line_number) = if let Some(diff) = has_diff {
                match diff.diff_type {
                    DiffType::Added => {
                        line_counter += 1;
                        ("text-emerald-500 dark:text-emerald-400 font-bold", "+", true)
                    },
                    DiffType::Removed => {
                        // Don't increment counter for removed lines and don't show line number
                        ("text-rose-500 dark:text-rose-400 font-bold line-through", "−", false)
                    }, 
                    DiffType::Modified => {
                        line_counter += 1;
                        ("text-amber-500 dark:text-amber-400 font-bold", "~", true)
                    },
                }
            } else {
                line_counter += 1;
                ("", "", true)
            };

            html! {
                <div key={i} class={format!("leading-[inherit] flex items-center justify-end gap-1 {}", diff_classes)}>
                    if show_line_number {
                        <span>{ line_counter }</span>
                    } else {
                        // Empty space for removed lines (no line number shown)
                        <span class="invisible">{ "---" }</span>
                    }
                    if has_diff.is_some() {
                        <span class="w-3 h-3 flex items-center justify-center text-xs font-bold">{ symbol }</span>
                    }
                </div>
            }
        }).collect::<Html>()
    }

    fn render_diff_backgrounds(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();
        
        props.diffs.iter().map(|diff| {
            if diff.line_number == 0 || diff.line_number > lines.len() {
                return html! {};
            }

            let line_index = diff.line_number - 1;
            let bg_class = match diff.diff_type {
                DiffType::Added => "bg-emerald-200 dark:bg-emerald-800 bg-opacity-60 dark:bg-opacity-40 shadow-sm",
                DiffType::Removed => "bg-rose-200 dark:bg-rose-800 bg-opacity-60 dark:bg-opacity-40 shadow-sm",
                DiffType::Modified => "bg-amber-200 dark:bg-amber-800 bg-opacity-60 dark:bg-opacity-40 shadow-sm",
            };

            html! {
                <div 
                    class={format!("absolute left-0 right-0 {}", bg_class)}
                    style={format!(
                        "top: {}px; height: {}px; margin-left: 8px; margin-right: 8px;", 
                        line_index as f32 * props.font_size as f32 * props.line_height + 8.0, // +8 for padding
                        props.font_size as f32 * props.line_height
                    )}
                />
            }
        }).collect::<Html>()
    }

    fn render_annotations_and_hints(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut elements = Vec::new();

        // Render annotations
        for annotation in &props.annotations {
            if annotation.line_number == 0 {
                continue;
            }

            let line_index = annotation.line_number - 1;
            let line_top = line_index as f32 * props.font_size as f32 * props.line_height + 8.0; // +8 for padding

            if annotation.inline {
                // Inline annotation - show message directly in the editor
                let column_pos = annotation.column_range.map(|(start, _)| start).unwrap_or(0);
                let annotation_class = match annotation.annotation_type {
                    AnnotationType::Error => "text-rose-700 dark:text-rose-300 bg-rose-100 dark:bg-rose-900 border border-rose-300 dark:border-rose-700 shadow-lg font-semibold",
                    AnnotationType::Warning => "text-amber-700 dark:text-amber-300 bg-amber-100 dark:bg-amber-900 border border-amber-300 dark:border-amber-700 shadow-lg font-semibold",
                    AnnotationType::Info => "text-sky-700 dark:text-sky-300 bg-sky-100 dark:bg-sky-900 border border-sky-300 dark:border-sky-700 shadow-lg font-semibold",
                    AnnotationType::Success => "text-emerald-700 dark:text-emerald-300 bg-emerald-100 dark:bg-emerald-900 border border-emerald-300 dark:border-emerald-700 shadow-lg font-semibold",
                };

                elements.push(html! {
                    <div 
                        class={format!("absolute px-2 py-1 rounded text-xs pointer-events-auto {}", annotation_class)}
                        style={format!(
                            "top: {}px; left: {}ch; z-index: 10;",
                            line_top + props.font_size as f32 * props.line_height,
                            column_pos
                        )}
                        title={annotation.message.clone()}
                    >
                        { &annotation.message }
                    </div>
                });
            } else {
                // Gutter annotation - show indicator with tooltip
                let annotation_class = match annotation.annotation_type {
                    AnnotationType::Error => "text-rose-500 border-rose-500 shadow-lg shadow-rose-200 dark:shadow-rose-800",
                    AnnotationType::Warning => "text-amber-500 border-amber-500 shadow-lg shadow-amber-200 dark:shadow-amber-800", 
                    AnnotationType::Info => "text-sky-500 border-sky-500 shadow-lg shadow-sky-200 dark:shadow-sky-800",
                    AnnotationType::Success => "text-emerald-500 border-emerald-500 shadow-lg shadow-emerald-200 dark:shadow-emerald-800",
                };

                elements.push(html! {
                    <div 
                        class={format!("absolute group pointer-events-auto {}", annotation_class)}
                        style={format!(
                            "top: {}px; left: 4px; z-index: 10;",
                            line_top + 2.0
                        )}
                    >
                        <div class="w-4 h-4 rounded-full border-2 bg-white dark:bg-gray-800 cursor-help hover:scale-110 transition-transform"></div>
                        <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 p-2 rounded shadow-lg z-50 left-6 top-0 whitespace-nowrap">
                            <div class="text-sm text-gray-800 dark:text-gray-200">{ &annotation.message }</div>
                        </div>
                    </div>
                });

                // Add underline for column range
                if let Some((start, end)) = annotation.column_range {
                    let underline_class = match annotation.annotation_type {
                        AnnotationType::Error => "border-b-4 border-rose-500 border-double shadow-sm animate-pulse",
                        AnnotationType::Warning => "border-b-4 border-amber-500 border-double shadow-sm animate-pulse",
                        AnnotationType::Info => "border-b-4 border-sky-500 border-double shadow-sm animate-pulse", 
                        AnnotationType::Success => "border-b-4 border-emerald-500 border-double shadow-sm animate-pulse",
                    };

                    elements.push(html! {
                        <div 
                            class={format!("absolute {}", underline_class)}
                            style={format!(
                                "top: {}px; left: {}ch; width: {}ch; height: 2px; margin-left: 8px;",
                                line_top + props.font_size as f32 * props.line_height - 2.0,
                                start,
                                end.saturating_sub(start)
                            )}
                        />
                    });
                }
            }
        }

        // Render type hints
        for hint in &props.type_hints {
            if hint.line_number == 0 {
                continue;
            }

            let line_index = hint.line_number - 1;
            let line_top = line_index as f32 * props.font_size as f32 * props.line_height + 8.0; // +8 for padding
            let column_pos = hint.column.unwrap_or(0);

            if hint.inline {
                // Inline type hint
                elements.push(html! {
                    <div 
                        class={classes!("absolute", "text-xs", "text-gray-500", "dark:text-gray-400", "italic", "pointer-events-auto", hint.class.clone())}
                        style={format!(
                            "top: {}px; left: {}ch; z-index: 10;",
                            line_top,
                            column_pos
                        )}
                    >
                        { &hint.hint }
                    </div>
                });
            } else {
                // Tooltip type hint
                elements.push(html! {
                    <div 
                        class={classes!("absolute", "group", "pointer-events-auto", hint.class.clone())}
                        style={format!(
                            "top: {}px; left: {}ch; z-index: 10;",
                            line_top + 2.0,
                            column_pos
                        )}
                    >
                        <div class="w-3 h-3 rounded-full bg-gradient-to-br from-violet-400 to-purple-500 shadow-lg shadow-violet-200 dark:shadow-violet-800 cursor-help hover:scale-125 transition-all"></div>
                        <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 p-2 rounded shadow-lg z-50 left-4 top-0 whitespace-nowrap">
                            <div class="text-sm text-gray-800 dark:text-gray-200">{ &hint.hint }</div>
                        </div>
                    </div>
                });
            }
        }

        html! { <>{ for elements }</> }
    }

    fn render_inline_diff_controls(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();
        
        // Group consecutive diffs into blocks
        let mut diff_blocks = Vec::new();
        let mut current_block = Vec::new();
        
        for (i, diff) in props.diffs.iter().enumerate() {
            if current_block.is_empty() {
                current_block.push((i, diff));
            } else if let Some((_, last_diff)) = current_block.last() {
                // If this diff is consecutive to the last one, add to current block
                if diff.line_number == last_diff.line_number + 1 {
                    current_block.push((i, diff));
                } else {
                    // Start a new block
                    diff_blocks.push(current_block.clone());
                    current_block.clear();
                    current_block.push((i, diff));
                }
            }
        }
        if !current_block.is_empty() {
            diff_blocks.push(current_block);
        }

        diff_blocks.into_iter().map(|block| {
            if block.is_empty() {
                return html! {};
            }
            
            let (_, first_diff) = &block[0];
            let (_, last_diff) = block.last().unwrap();
            
            // Position controls at the end of the diff block
            let line_index = last_diff.line_number - 1;
            let control_top = line_index as f32 * props.font_size as f32 * props.line_height + 8.0; // +8 for padding

            html! {
                <div 
                    class="absolute right-2 flex gap-1 pointer-events-auto z-20"
                    style={format!("top: {}px;", control_top)}
                >
                    <button class="px-2 py-1 bg-emerald-500 hover:bg-emerald-600 text-white rounded-sm text-xs font-medium transition-colors shadow-sm opacity-80 hover:opacity-100">
                        {"✓"}
                    </button>
                    <button class="px-2 py-1 bg-rose-500 hover:bg-rose-600 text-white rounded-sm text-xs font-medium transition-colors shadow-sm opacity-80 hover:opacity-100">
                        {"✗"}
                    </button>
                </div>
            }
        }).collect::<Html>()
    }

    fn render_highlighted_code_with_annotations(&self, ctx: &Context<Self>) -> Html {
        // Same as the old render_highlighted_code method
        self.render_highlighted_code(ctx)
    }

    fn render_highlighted_code(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        // Simple syntax highlighting using CSS classes
        let lines = self.code.lines();
        let highlighted_lines = lines.map(|line| {
            self.highlight_line(line, &props.language)
        }).collect::<Vec<_>>();

        html! {
            <>
                { for highlighted_lines.into_iter().enumerate().map(|(i, line_html)| {
                    html! { 
                        <span key={i}>
                            { line_html }
                            { if i < self.code.lines().count() - 1 || self.code.ends_with('\n') { "\n" } else { "" } }
                        </span>
                    }
                }) }
            </>
        }
    }

    fn highlight_line(&self, line: &str, language: &str) -> Html {
        // Basic syntax highlighting - can be extended with proper highlighting library later
        match language {
            "rust" => self.highlight_rust_line(line),
            "javascript" | "typescript" => self.highlight_js_line(line),
            _ => html! { { line } },
        }
    }

    fn highlight_rust_line(&self, line: &str) -> Html {
        let keywords = [
            "fn", "let", "mut", "pub", "struct", "impl", "enum", "trait", "use", "mod",
            "match", "if", "else", "for", "while", "loop", "return", "self", "Self",
            "true", "false", "const", "static", "async", "await", "move", "ref",
        ];

        // Check for comment first - simplest approach
        if let Some(comment_start) = line.find("//") {
            let before_comment = &line[..comment_start];
            let comment_part = &line[comment_start..];
            
            let mut result = Vec::new();
            
            // Highlight the part before the comment
            if !before_comment.is_empty() {
                result.push(self.highlight_rust_tokens(before_comment, &keywords));
            }
            
            // Add the comment  
            result.push(html! { <span class="text-slate-500 dark:text-slate-400 italic font-medium opacity-75">{ comment_part }</span> });
            
            return html! { <>{ for result }</> };
        }

        // No comment, highlight normally
        self.highlight_rust_tokens(line, &keywords)
    }

    fn highlight_rust_tokens(&self, text: &str, keywords: &[&str]) -> Html {
        let mut result = Vec::new();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_char = '"';

        for ch in text.chars() {
            if in_string {
                current_word.push(ch);
                if ch == string_char {
                    // Simple string end detection (not handling escapes for simplicity)
                    result.push(html! { <span class="text-emerald-600 dark:text-emerald-400 font-medium">{ current_word.clone() }</span> });
                    current_word.clear();
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' => {
                    if !current_word.is_empty() {
                        self.push_word(&mut result, &current_word, keywords);
                        current_word.clear();
                    }
                    current_word.push(ch);
                    in_string = true;
                    string_char = ch;
                }
                ' ' | '\t' | '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' | '=' | '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '!' => {
                    if !current_word.is_empty() {
                        self.push_word(&mut result, &current_word, keywords);
                        current_word.clear();
                    }
                    if ch != ' ' && ch != '\t' {
                        result.push(html! { <span class="text-slate-600 dark:text-slate-400 font-semibold">{ ch.to_string() }</span> });
                    } else {
                        result.push(html! { { ch.to_string() } });
                    }
                }
                _ => {
                    current_word.push(ch);
                }
            }
        }

        if !current_word.is_empty() {
            if in_string {
                result.push(html! { <span class="text-emerald-600 dark:text-emerald-400 font-medium">{ current_word }</span> });
            } else {
                self.push_word(&mut result, &current_word, keywords);
            }
        }

        html! { <>{ for result }</> }
    }

    fn highlight_js_line(&self, line: &str) -> Html {
        let _keywords = [
            "function", "const", "let", "var", "class", "extends", "import", "export",
            "from", "return", "if", "else", "for", "while", "switch", "case", "break",
            "continue", "true", "false", "null", "undefined", "async", "await",
        ];

        // Similar implementation to Rust but with JS keywords
        // Simplified for now
        html! { { line } }
    }

    fn push_word(&self, result: &mut Vec<Html>, word: &str, keywords: &[&str]) {
        if keywords.contains(&word) {
            result.push(html! { <span class="text-indigo-600 dark:text-indigo-400 font-bold">{ word.to_string() }</span> });
        } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
            result.push(html! { <span class="text-fuchsia-600 dark:text-fuchsia-400 font-semibold">{ word.to_string() }</span> });
        } else {
            result.push(html! { <span class="text-gray-800 dark:text-gray-200">{ word.to_string() }</span> });
        }
    }
}