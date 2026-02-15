//! Main CodeEditor component

use std::collections::HashMap;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

use crate::annotation::{Annotation, AnnotationType};
use crate::diff::{Diff, DiffType};
use crate::styles::CodeEditorStyles;
use crate::type_hint::TypeHint;

/// Properties for the CodeEditor component
#[derive(Properties, PartialEq, Clone)]
pub struct CodeEditorProps {
    /// Initial code content
    #[prop_or_default]
    pub code: String,

    /// Alias for code (for compatibility)
    #[prop_or_default]
    pub value: String,

    /// Show line numbers
    #[prop_or(true)]
    pub show_line_numbers: bool,

    /// Language for syntax highlighting
    #[prop_or_else(|| "rust".to_string())]
    pub language: String,

    /// Theme name (light or dark)
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,

    /// Font size in pixels (or as string like "14px")
    #[prop_or(14)]
    pub font_size: u8,

    /// Font size as string (for compatibility)
    #[prop_or_default]
    pub font_size_str: Option<String>,

    /// Font family
    #[prop_or_else(|| "JetBrains Mono, monospace".to_string())]
    pub font_family: String,

    /// Line height
    #[prop_or(1.5)]
    pub line_height: f32,

    /// Line height as string (for compatibility)
    #[prop_or_default]
    pub line_height_str: Option<String>,

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
    pub readonly: bool,

    /// Alias for readonly (for compatibility)
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

    /// Show line numbers (alias)
    #[prop_or_default]
    pub line_numbers: Option<bool>,
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
        let props = ctx.props();
        let code = if !props.code.is_empty() {
            props.code.clone()
        } else {
            props.value.clone()
        };

        Self {
            code,
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
                let is_readonly = ctx.props().readonly || ctx.props().read_only;
                if !is_readonly {
                    self.handle_keydown(event, ctx);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let show_line_numbers = props.line_numbers.unwrap_or(props.show_line_numbers);
        let is_readonly = props.readonly || props.read_only;

        // Parse font_size - support both numeric and string formats
        let font_size: u8 = if let Some(ref fs_str) = props.font_size_str {
            fs_str
                .trim_end_matches("px")
                .parse()
                .unwrap_or(props.font_size)
        } else {
            props.font_size
        };

        // Parse line_height - support both numeric and string formats
        let line_height: f32 = if let Some(ref lh_str) = props.line_height_str {
            lh_str.parse().unwrap_or(props.line_height)
        } else {
            props.line_height
        };

        let container_style = format!(
            "font-family: {}; font-size: {}px; line-height: {};{}",
            props.font_family,
            font_size,
            line_height,
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
                <CodeEditorStyles />
                <div
                    class={classes!(
                        props.class.clone(),
                        "relative", "border", "border-gray-300", "dark:border-gray-700",
                        "bg-white", "dark:bg-gray-900", "rounded-sm", "overflow-hidden"
                    )}
                    style={format!("{} {}", container_style, max_height_style)}
                >
                    <div class="flex">
                        // Line numbers with diff indicators
                        if show_line_numbers {
                            <div class="flex-none bg-gray-100 dark:bg-gray-800 px-3 py-2 text-gray-500 dark:text-gray-400 text-right select-none border-r border-gray-300 dark:border-gray-700">
                                {
                                    if props.diff_view {
                                        self.render_diff_line_numbers(ctx, font_size, line_height)
                                    } else {
                                        self.render_line_numbers_with_diffs(ctx, font_size, line_height)
                                    }
                                }
                            </div>
                        }

                        // Editor area
                        <div class="flex-1 relative">
                            // Diff backgrounds layer (behind everything)
                            <div class="absolute inset-0 pointer-events-none">
                                { self.render_diff_backgrounds(ctx, font_size, line_height) }
                            </div>

                            // Syntax highlighting overlay
                            <div
                                class="absolute inset-0 p-2 m-0 pointer-events-none overflow-hidden whitespace-pre-wrap break-words text-gray-900 dark:text-gray-100"
                                style={format!("{} font-family: inherit; font-size: inherit; line-height: inherit;", editor_style)}
                            >
                                { self.render_highlighted_code(ctx) }
                            </div>

                            // Annotations and type hints overlay
                            <div class="absolute inset-0 pointer-events-none">
                                { self.render_annotations_and_hints(ctx, font_size, line_height) }
                            </div>

                            // Inline diff controls overlay
                            if props.diff_view {
                                <div class="absolute inset-0 pointer-events-none">
                                    { self.render_inline_diff_controls(ctx, font_size, line_height) }
                                </div>
                            }

                            // Actual textarea
                            <textarea
                                ref={self.textarea_ref.clone()}
                                class="absolute inset-0 p-2 m-0 resize-none bg-transparent outline-none border-none whitespace-pre-wrap break-words"
                                style={format!("{} color: transparent; caret-color: #3b82f6; font-family: inherit; font-size: inherit; line-height: inherit;", editor_style)}
                                value={self.code.clone()}
                                readonly={is_readonly}
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
        let new_code = if !props.code.is_empty() {
            &props.code
        } else {
            &props.value
        };

        if new_code != &self.code {
            self.code = new_code.clone();
            if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea.set_value(&self.code);
            }
            return true;
        }

        old_props.language != props.language || old_props.theme != props.theme
    }
}

impl CodeEditor {
    fn setup_listeners(&mut self, ctx: &Context<Self>) {
        if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            // Input event
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
            if key == "Tab" {
                event.prevent_default();
                let tab_spaces = " ".repeat(ctx.props().tab_size as usize);

                let start = textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
                let end = textarea.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;
                let value = textarea.value();

                let new_value = format!("{}{}{}", &value[0..start], tab_spaces, &value[end..]);

                textarea.set_value(&new_value);
                let new_pos = start + tab_spaces.len();
                textarea.set_selection_start(Some(new_pos as u32)).ok();
                textarea.set_selection_end(Some(new_pos as u32)).ok();

                ctx.link()
                    .send_message(CodeEditorMsg::CodeChanged(new_value));
            }
        }
    }

    fn render_line_numbers_with_diffs(
        &self,
        ctx: &Context<Self>,
        _font_size: u8,
        _line_height: f32,
    ) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();

        let mut diff_map = std::collections::HashMap::new();
        for diff in &props.diffs {
            diff_map.insert(diff.line_number, diff);
        }

        lines
            .iter()
            .enumerate()
            .map(|(i, _)| {
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
                            <span class="w-3 h-3 rounded-full bg-current shadow-md animate-pulse"></span>
                        }
                    </div>
                }
            })
            .collect::<Html>()
    }

    fn render_diff_line_numbers(
        &self,
        ctx: &Context<Self>,
        _font_size: u8,
        _line_height: f32,
    ) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();

        let mut diff_map = std::collections::HashMap::new();
        for diff in &props.diffs {
            diff_map.insert(diff.line_number, diff);
        }

        let mut line_counter = 0;

        lines
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let display_line_num = i + 1;
                let has_diff = diff_map.get(&display_line_num);

                let (diff_classes, symbol, show_line_number) = if let Some(diff) = has_diff {
                    match diff.diff_type {
                        DiffType::Added => {
                            line_counter += 1;
                            (
                                "text-emerald-500 dark:text-emerald-400 font-bold",
                                "+",
                                true,
                            )
                        }
                        DiffType::Removed => (
                            "text-rose-500 dark:text-rose-400 font-bold line-through",
                            "−",
                            false,
                        ),
                        DiffType::Modified => {
                            line_counter += 1;
                            ("text-amber-500 dark:text-amber-400 font-bold", "~", true)
                        }
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
                            <span class="invisible">{ "---" }</span>
                        }
                        if has_diff.is_some() {
                            <span class="w-3 h-3 flex items-center justify-center text-xs font-bold">{ symbol }</span>
                        }
                    </div>
                }
            })
            .collect::<Html>()
    }

    fn render_diff_backgrounds(
        &self,
        ctx: &Context<Self>,
        font_size: u8,
        line_height: f32,
    ) -> Html {
        let props = ctx.props();
        let lines = self.code.lines().collect::<Vec<_>>();

        props
            .diffs
            .iter()
            .map(|diff| {
                if diff.line_number == 0 || diff.line_number > lines.len() {
                    return html! {};
                }

                let line_index = diff.line_number - 1;
                let bg_class = match diff.diff_type {
                    DiffType::Added => "bg-emerald-200 dark:bg-emerald-800/40 shadow-sm",
                    DiffType::Removed => "bg-rose-200 dark:bg-rose-800/40 shadow-sm",
                    DiffType::Modified => "bg-amber-200 dark:bg-amber-800/40 shadow-sm",
                };

                html! {
                    <div
                        class={format!("absolute left-0 right-0 {}", bg_class)}
                        style={format!(
                            "top: {}px; height: {}px; margin-left: 8px; margin-right: 8px;",
                            line_index as f32 * font_size as f32 * line_height + 8.0,
                            font_size as f32 * line_height
                        )}
                    />
                }
            })
            .collect::<Html>()
    }

    fn render_annotations_and_hints(
        &self,
        ctx: &Context<Self>,
        font_size: u8,
        line_height: f32,
    ) -> Html {
        let props = ctx.props();
        let mut elements = Vec::new();

        // Render annotations
        for annotation in &props.annotations {
            if annotation.line_number == 0 {
                continue;
            }

            let line_index = annotation.line_number - 1;
            let line_top = line_index as f32 * font_size as f32 * line_height + 8.0;

            if annotation.inline {
                let column_pos = annotation.column_range.map(|(start, _)| start).unwrap_or(0);
                let annotation_class = match annotation.annotation_type {
                    AnnotationType::Error => "text-rose-700 dark:text-rose-300 bg-rose-100 dark:bg-rose-900 border border-rose-300 dark:border-rose-700 shadow-md font-semibold",
                    AnnotationType::Warning => "text-amber-700 dark:text-amber-300 bg-amber-100 dark:bg-amber-900 border border-amber-300 dark:border-amber-700 shadow-md font-semibold",
                    AnnotationType::Info => "text-sky-700 dark:text-sky-300 bg-sky-100 dark:bg-sky-900 border border-sky-300 dark:border-sky-700 shadow-md font-semibold",
                    AnnotationType::Success => "text-emerald-700 dark:text-emerald-300 bg-emerald-100 dark:bg-emerald-900 border border-emerald-300 dark:border-emerald-700 shadow-md font-semibold",
                };

                elements.push(html! {
                    <div
                        class={format!("absolute px-2 py-1 rounded-sm text-xs pointer-events-auto {}", annotation_class)}
                        style={format!(
                            "top: {}px; left: {}ch; z-index: 10;",
                            line_top + font_size as f32 * line_height + 4.0,
                            column_pos
                        )}
                    >
                        { &annotation.message }
                    </div>
                });
            } else {
                // Tooltip annotation
                let annotation_color = match annotation.annotation_type {
                    AnnotationType::Error => {
                        "from-rose-400 to-red-500 shadow-rose-200 dark:shadow-rose-800"
                    }
                    AnnotationType::Warning => {
                        "from-amber-400 to-yellow-500 shadow-amber-200 dark:shadow-amber-800"
                    }
                    AnnotationType::Info => {
                        "from-sky-400 to-blue-500 shadow-sky-200 dark:shadow-sky-800"
                    }
                    AnnotationType::Success => {
                        "from-emerald-400 to-green-500 shadow-emerald-200 dark:shadow-emerald-800"
                    }
                };

                elements.push(html! {
                    <div
                        class="absolute group pointer-events-auto"
                        style={format!("top: {}px; left: 0; z-index: 10;", line_top + 2.0)}
                    >
                        <div class={format!("w-3 h-3 rounded-full bg-gradient-to-br {} shadow-md cursor-help hover:scale-125 transition-all", annotation_color)}></div>
                        <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 p-2 rounded-sm shadow-md z-50 left-4 top-0 whitespace-nowrap max-w-md">
                            <div class="text-sm text-gray-800 dark:text-gray-200">{ &annotation.message }</div>
                        </div>
                    </div>
                });
            }
        }

        // Render type hints
        for hint in &props.type_hints {
            if hint.line_number == 0 {
                continue;
            }

            let line_index = hint.line_number - 1;
            let line_top = line_index as f32 * font_size as f32 * line_height + 8.0;
            let column_pos = hint.column.unwrap_or(0);

            if hint.inline {
                elements.push(html! {
                    <div
                        class={classes!("absolute", "text-xs", "text-gray-500", "dark:text-gray-400", "italic", "pointer-events-auto", hint.class.clone())}
                        style={format!("top: {}px; left: {}ch; z-index: 10;", line_top, column_pos)}
                    >
                        { &hint.hint }
                    </div>
                });
            } else {
                elements.push(html! {
                    <div
                        class={classes!("absolute", "group", "pointer-events-auto", hint.class.clone())}
                        style={format!("top: {}px; left: {}ch; z-index: 10;", line_top + 2.0, column_pos)}
                    >
                        <div class="w-3 h-3 rounded-full bg-gradient-to-br from-violet-400 to-purple-500 shadow-md shadow-violet-200 dark:shadow-violet-800 cursor-help hover:scale-125 transition-all"></div>
                        <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 p-2 rounded-sm shadow-md z-50 left-4 top-0 whitespace-nowrap">
                            <div class="text-sm text-gray-800 dark:text-gray-200">{ &hint.hint }</div>
                        </div>
                    </div>
                });
            }
        }

        html! { <>{ for elements }</> }
    }

    fn render_inline_diff_controls(
        &self,
        ctx: &Context<Self>,
        font_size: u8,
        line_height: f32,
    ) -> Html {
        let props = ctx.props();

        // Group consecutive diffs
        let mut diff_blocks = Vec::new();
        let mut current_block = Vec::new();

        for (i, diff) in props.diffs.iter().enumerate() {
            if current_block.is_empty() {
                current_block.push((i, diff));
            } else if let Some((_, last_diff)) = current_block.last() {
                if diff.line_number == last_diff.line_number + 1 {
                    current_block.push((i, diff));
                } else {
                    diff_blocks.push(current_block.clone());
                    current_block.clear();
                    current_block.push((i, diff));
                }
            }
        }
        if !current_block.is_empty() {
            diff_blocks.push(current_block);
        }

        diff_blocks
            .into_iter()
            .map(|block| {
                if block.is_empty() {
                    return html! {};
                }

                let (_, last_diff) = block.last().unwrap();
                let line_index = last_diff.line_number - 1;
                let control_top = line_index as f32 * font_size as f32 * line_height + 8.0;

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
            })
            .collect::<Html>()
    }

    fn render_highlighted_code(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let lines = self.code.lines();

        let highlighted_lines = lines
            .map(|line| self.highlight_line(line, &props.language))
            .collect::<Vec<_>>();

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
        match language {
            "rust" => self.highlight_rust_line(line),
            "javascript" | "typescript" | "js" | "ts" => self.highlight_js_line(line),
            _ => html! { { line } },
        }
    }

    fn highlight_rust_line(&self, line: &str) -> Html {
        let keywords = [
            "fn", "let", "mut", "pub", "struct", "impl", "enum", "trait", "use", "mod", "match",
            "if", "else", "for", "while", "loop", "return", "self", "Self", "true", "false",
            "const", "static", "async", "await", "move", "ref",
        ];

        // Check for comment first
        if let Some(comment_start) = line.find("//") {
            let before_comment = &line[..comment_start];
            let comment_part = &line[comment_start..];

            let mut result = Vec::new();

            if !before_comment.is_empty() {
                result.push(self.highlight_rust_tokens(before_comment, &keywords));
            }

            result.push(html! { <span class="text-slate-500 dark:text-slate-400 italic font-medium opacity-75">{ comment_part }</span> });

            return html! { <>{ for result }</> };
        }

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
                ' ' | '\t' | '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' | '='
                | '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '!' => {
                    if !current_word.is_empty() {
                        self.push_word(&mut result, &current_word, keywords);
                        current_word.clear();
                    }
                    if ch != ' ' && ch != '\t' {
                        result.push(
                            html! { <span class="text-slate-600 dark:text-slate-400 font-semibold">{ ch.to_string() }</span> },
                        );
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
        // Simplified for now
        html! { { line } }
    }

    fn push_word(&self, result: &mut Vec<Html>, word: &str, keywords: &[&str]) {
        if keywords.contains(&word) {
            result.push(
                html! { <span class="text-indigo-600 dark:text-indigo-400 font-bold">{ word.to_string() }</span> },
            );
        } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
            result.push(
                html! { <span class="text-fuchsia-600 dark:text-fuchsia-400 font-semibold">{ word.to_string() }</span> },
            );
        } else {
            result.push(
                html! { <span class="text-gray-800 dark:text-gray-200">{ word.to_string() }</span> },
            );
        }
    }
}
