use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::js_sys;

use gloo::events::EventListener;
use gloo_utils::document;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Element, FocusEvent, HtmlElement, HtmlTextAreaElement, KeyboardEvent, Window};
use yew::prelude::*;

// Internal modules
pub mod annotation;
pub mod diff;
pub mod styles;
pub mod type_hint;

pub use annotation::{Annotation, AnnotationType};
pub use diff::{Diff, DiffType};
pub use type_hint::TypeHint;

/// A performant code editor component based on syntect
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
}

pub enum CodeEditorMsg {
    CodeChanged(String),
    HighlightCode,
    Keydown(KeyboardEvent),
    Focus(FocusEvent),
    Blur(FocusEvent),
    ScrollSync(f64, f64),
    CursorPositionChanged((usize, usize)),
    SelectionChanged(usize, usize),
    AddCursor((usize, usize)),
    RemoveCursor(usize),
    ClearCursors,
}

pub struct CodeEditor {
    code: String,
    lines: Vec<String>,
    highlighted_html: Html,
    textarea_ref: NodeRef,
    display_ref: NodeRef,
    gutter_ref: NodeRef,
    cursor_position: (usize, usize), // (line, column)
    selection_start: usize,
    selection_end: usize,
    has_selection: bool,
    scroll_top: f64,
    scroll_left: f64,
    _listeners: Vec<EventListener>,
    cursors: Vec<(usize, usize)>, // Multiple cursors [(line, column)]
    keymap: Option<HashMap<String, Callback<KeyboardEvent>>>,
    keymap_enabled: bool,
}

impl CodeEditor {
    fn render_line_numbers(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut line_numbers = Vec::new();

        // Create a map of which lines have diffs for easy lookup
        let mut diff_map: HashMap<usize, Vec<&diff::Diff>> = HashMap::new();
        for diff in &props.diffs {
            let line_idx = diff.line_number;
            diff_map.entry(line_idx).or_default().push(diff);
        }

        for i in 0..self.lines.len() {
            let line_num = i + 1;

            // Check if this line has a diff
            let diff_class = if let Some(diffs) = diff_map.get(&line_num) {
                match diffs[0].diff_type {
                    diff::DiffType::Added => "line-number-added",
                    diff::DiffType::Removed => "line-number-removed",
                    diff::DiffType::Modified => "line-number-modified",
                }
            } else {
                ""
            };

            // Convert diff_class to Tailwind classes
            let tailwind_diff_class = match diff_class {
                "line-number-added" => "text-green-600 dark:text-green-400",
                "line-number-removed" => "text-red-600 dark:text-red-400",
                "line-number-modified" => "text-yellow-600 dark:text-yellow-400",
                _ => "",
            };

            line_numbers.push(html! {
                <div class={classes!("py-0", "px-2", "text-right", "text-xs", "text-gray-500", "dark:text-gray-400", tailwind_diff_class)}>
                    <span>{ line_num }</span>
                    if !diff_class.is_empty() {
                        <span class="ml-1 inline-block h-2 w-2 rounded-full bg-current"></span>
                    }
                </div>
            });
        }

        html! {
            <>
                { for line_numbers.into_iter() }
            </>
        }
    }

    fn render_annotations_and_hints(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut annotations_html = Vec::new();
        let mut type_hints_html = Vec::new();
        let mut column_diffs_html = Vec::new();

        // Render annotations
        for annotation in &props.annotations {
            let line_idx = annotation.line_number.saturating_sub(1);
            let annotation_class = format!("annotation-{}", annotation.annotation_type.to_string());

            if annotation.inline {
                // Render inline annotations
                annotations_html.push(html! {
                    <div
                        class={classes!("absolute", "flex", "items-center", "gap-1", &annotation_class)}
                        style={format!("top: {}em;", line_idx as f32 * props.line_height)}
                    >
                        <span class="text-xs">{ "●" }</span>
                        <span class="text-sm">{ &annotation.message }</span>
                    </div>
                });
            } else {
                // Render gutter annotations with tooltip
                let column_pos = annotation.column_range.map(|(start, _)| start).unwrap_or(0);
                annotations_html.push(html! {
                    <div
                        class={classes!("absolute", "group", &annotation_class)}
                        style={format!("top: {}em; left: {}ch;",
                               line_idx as f32 * props.line_height,
                               column_pos)}
                    >
                        <span class="text-xs cursor-help">{ "●" }</span>
                        <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 p-2 rounded shadow-lg z-50 mt-1 ml-4 max-w-xs">
                            <span class="text-sm">{ &annotation.message }</span>
                        </div>
                    </div>
                });
            }

            // If the annotation has a column range, add squiggly underline
            if let Some((start, end)) = annotation.column_range {
                annotations_html.push(html! {
                    <div
                        class={classes!("absolute", annotation_class)}
                        style={format!("top: {}em; left: {}ch; width: {}ch; height: 2px; border-bottom-width: 2px; border-bottom-style: wavy;",
                               (line_idx as f32 * props.line_height) + props.line_height - 0.2,
                               start, end - start)}
                    />
                });
            }
        }

        // Render type hints
        for hint in &props.type_hints {
            let line_idx = hint.line_number.saturating_sub(1);
            let column_pos = hint.column.unwrap_or(0);

            let max_width_style = if hint.max_width > 0 {
                format!("max-width: {}px;", hint.max_width)
            } else {
                String::new()
            };

            if hint.inline {
                type_hints_html.push(html! {
                    <div
                        class={classes!("absolute", "inline-block", "text-gray-500", "dark:text-gray-400", "text-xs", "italic", hint.class.clone())}
                        style={format!("top: {}em; left: {}ch; {}",
                               line_idx as f32 * props.line_height,
                               column_pos, max_width_style)}
                    >
                        <span>{ &hint.hint }</span>
                    </div>
                });
            } else {
                type_hints_html.push(html! {
                    <div
                        class={classes!("absolute", "group", hint.class.clone())}
                        style={format!("top: {}em; left: {}ch;",
                               line_idx as f32 * props.line_height,
                               column_pos)}
                    >
                        <div class="h-2 w-2 bg-gray-400 dark:bg-gray-600 rounded-full cursor-help"></div>
                        <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 p-2 rounded shadow-lg z-50 mt-1 ml-2" style={max_width_style}>
                            <span class="text-sm text-gray-700 dark:text-gray-300">{ &hint.hint }</span>
                        </div>
                    </div>
                });
            }
        }

        // Render column-specific diffs (partial line diffs)
        for diff in &props.diffs {
            if let Some((start, end)) = diff.column_range {
                let line_idx = diff.line_number.saturating_sub(1);
                let diff_class = match diff.diff_type {
                    diff::DiffType::Added => "diff-added",
                    diff::DiffType::Removed => "diff-removed",
                    diff::DiffType::Modified => "diff-modified",
                };

                column_diffs_html.push(html! {
                    <div
                        class={classes!("absolute", "group", diff_class)}
                        style={format!("top: {}em; left: {}ch; width: {}ch; height: {}em; opacity: 0.3;",
                               line_idx as f32 * props.line_height,
                               start, end - start, props.line_height)}
                    >
                        if let Some(ref msg) = diff.message {
                            <div class="absolute hidden group-hover:block bg-white dark:bg-gray-800 p-2 rounded shadow-lg z-50 mt-6 ml-2 text-sm">
                                { msg }
                            </div>
                        }
                    </div>
                });
            }
        }

        html! {
            <>
                <div class="pointer-events-none">
                    { for column_diffs_html.into_iter() }
                </div>
                <div class="pointer-events-auto">
                    { for annotations_html.into_iter() }
                </div>
                <div class="pointer-events-auto">
                    { for type_hints_html.into_iter() }
                </div>
            </>
        }
    }

    fn tokenize_line(&self, line: &str, language: &str) -> Vec<(String, String)> {
        let mut tokens = Vec::new();

        // Very basic tokenization for common programming languages
        match language {
            "rust" => {
                // Simple Rust tokenization
                let keywords = vec![
                    "fn", "let", "mut", "pub", "struct", "impl", "enum", "trait", "use", "mod",
                    "match", "if", "else", "for", "while", "loop", "return", "self", "Self",
                    "true", "false",
                ];

                let mut current_token = String::new();
                let mut in_string = false;
                let mut in_comment = false;

                for c in line.chars() {
                    if in_comment {
                        current_token.push(c);
                        continue;
                    }

                    if in_string {
                        current_token.push(c);
                        if c == '"' && !current_token.ends_with("\\\"") {
                            tokens.push(("string".to_string(), current_token));
                            current_token = String::new();
                            in_string = false;
                        }
                        continue;
                    }

                    match c {
                        ' ' | '\t' => {
                            if !current_token.is_empty() {
                                let token_type = if keywords.contains(&current_token.as_str()) {
                                    "keyword"
                                } else if current_token
                                    .chars()
                                    .all(|c| c.is_digit(10) || c == '.' || c == 'f' || c == '_')
                                {
                                    "number"
                                } else {
                                    "ident"
                                };
                                tokens.push((token_type.to_string(), current_token));
                                current_token = String::new();
                            }
                            tokens.push(("whitespace".to_string(), c.to_string()));
                        }
                        '"' => {
                            if !current_token.is_empty() {
                                tokens.push(("ident".to_string(), current_token));
                                current_token = String::new();
                            }
                            current_token.push(c);
                            in_string = true;
                        }
                        '/' => {
                            if !current_token.is_empty() && current_token.ends_with('/') {
                                current_token.push(c);
                                in_comment = true;
                            } else {
                                if !current_token.is_empty() {
                                    tokens.push(("ident".to_string(), current_token));
                                    current_token = String::new();
                                }
                                current_token.push(c);
                            }
                        }
                        '{' | '}' | '(' | ')' | '[' | ']' | ':' | ';' | ',' | '.' => {
                            if !current_token.is_empty() {
                                let token_type = if keywords.contains(&current_token.as_str()) {
                                    "keyword"
                                } else if current_token
                                    .chars()
                                    .all(|c| c.is_digit(10) || c == '.' || c == 'f' || c == '_')
                                {
                                    "number"
                                } else {
                                    "ident"
                                };
                                tokens.push((token_type.to_string(), current_token));
                                current_token = String::new();
                            }
                            tokens.push(("punctuation".to_string(), c.to_string()));
                        }
                        _ => {
                            current_token.push(c);
                        }
                    }
                }

                // Don't forget the last token
                if !current_token.is_empty() {
                    let token_type = if in_comment {
                        "comment"
                    } else if keywords.contains(&current_token.as_str()) {
                        "keyword"
                    } else if current_token
                        .chars()
                        .all(|c| c.is_digit(10) || c == '.' || c == 'f' || c == '_')
                    {
                        "number"
                    } else {
                        "ident"
                    };
                    tokens.push((token_type.to_string(), current_token));
                }
            }
            "javascript" | "typescript" => {
                // Simple JS/TS tokenization (very similar to Rust tokenization)
                let keywords = vec![
                    "function",
                    "const",
                    "let",
                    "var",
                    "class",
                    "extends",
                    "implements",
                    "import",
                    "export",
                    "from",
                    "return",
                    "if",
                    "else",
                    "for",
                    "while",
                    "switch",
                    "case",
                    "default",
                    "break",
                    "continue",
                    "true",
                    "false",
                    "null",
                    "undefined",
                ];

                let mut current_token = String::new();
                let mut in_string = false;
                let mut string_delimiter = '"';
                let mut in_comment = false;

                for c in line.chars() {
                    if in_comment {
                        current_token.push(c);
                        continue;
                    }

                    if in_string {
                        current_token.push(c);
                        if c == string_delimiter && !current_token.ends_with('\\') {
                            tokens.push(("string".to_string(), current_token));
                            current_token = String::new();
                            in_string = false;
                        }
                        continue;
                    }

                    match c {
                        ' ' | '\t' => {
                            if !current_token.is_empty() {
                                let token_type = if keywords.contains(&current_token.as_str()) {
                                    "keyword"
                                } else if current_token
                                    .chars()
                                    .all(|c| c.is_digit(10) || c == '.' || c == 'e' || c == '_')
                                {
                                    "number"
                                } else {
                                    "ident"
                                };
                                tokens.push((token_type.to_string(), current_token));
                                current_token = String::new();
                            }
                            tokens.push(("whitespace".to_string(), c.to_string()));
                        }
                        '"' | '\'' | '`' => {
                            if !current_token.is_empty() {
                                tokens.push(("ident".to_string(), current_token));
                                current_token = String::new();
                            }
                            current_token.push(c);
                            in_string = true;
                            string_delimiter = c;
                        }
                        '/' => {
                            if !current_token.is_empty() && current_token.ends_with('/') {
                                current_token.push(c);
                                in_comment = true;
                            } else {
                                if !current_token.is_empty() {
                                    tokens.push(("ident".to_string(), current_token));
                                    current_token = String::new();
                                }
                                current_token.push(c);
                            }
                        }
                        '{' | '}' | '(' | ')' | '[' | ']' | ':' | ';' | ',' | '.' => {
                            if !current_token.is_empty() {
                                let token_type = if keywords.contains(&current_token.as_str()) {
                                    "keyword"
                                } else if current_token
                                    .chars()
                                    .all(|c| c.is_digit(10) || c == '.' || c == 'e' || c == '_')
                                {
                                    "number"
                                } else {
                                    "ident"
                                };
                                tokens.push((token_type.to_string(), current_token));
                                current_token = String::new();
                            }
                            tokens.push(("punctuation".to_string(), c.to_string()));
                        }
                        _ => {
                            current_token.push(c);
                        }
                    }
                }

                // Don't forget the last token
                if !current_token.is_empty() {
                    let token_type = if in_comment {
                        "comment"
                    } else if keywords.contains(&current_token.as_str()) {
                        "keyword"
                    } else if current_token
                        .chars()
                        .all(|c| c.is_digit(10) || c == '.' || c == 'e' || c == '_')
                    {
                        "number"
                    } else {
                        "ident"
                    };
                    tokens.push((token_type.to_string(), current_token));
                }
            }
            _ => {
                // For other languages, just return plain text
                tokens.push(("text".to_string(), line.to_string()));
            }
        }

        tokens
    }
}

impl Component for CodeEditor {
    type Message = CodeEditorMsg;
    type Properties = CodeEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let code = props.code.clone();
        let lines = Self::split_into_lines(&code);

        Self {
            code,
            lines,
            highlighted_html: Html::default(),
            textarea_ref: NodeRef::default(),
            display_ref: NodeRef::default(),
            gutter_ref: NodeRef::default(),
            cursor_position: (0, 0),
            selection_start: 0,
            selection_end: 0,
            has_selection: false,
            scroll_top: 0.0,
            scroll_left: 0.0,
            _listeners: Vec::new(),
            cursors: Vec::new(),
            keymap: props.keymap.clone(),
            keymap_enabled: props.enable_keymap,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CodeEditorMsg::CodeChanged(code) => {
                let changed = self.code != code;
                if changed {
                    self.code = code.clone();
                    self.lines = Self::split_into_lines(&code);

                    if let Some(callback) = &ctx.props().on_change {
                        callback.emit(code);
                    }

                    // Highlight code with a small delay to avoid performance issues
                    ctx.link().send_future(async move {
                        gloo::timers::future::TimeoutFuture::new(10).await;
                        CodeEditorMsg::HighlightCode
                    });
                }
                false // Avoid re-rendering until highlighting is done
            }
            CodeEditorMsg::HighlightCode => {
                self.highlight_code(ctx);
                true
            }
            CodeEditorMsg::Keydown(event) => {
                if !ctx.props().read_only {
                    // Check if the keymap has a handler for this key combo
                    if self.keymap_enabled {
                        if let Some(keymap) = &self.keymap {
                            let key = event.key();
                            let modifier_key = format!(
                                "{}{}{}{}",
                                if event.ctrl_key() { "Ctrl+" } else { "" },
                                if event.alt_key() { "Alt+" } else { "" },
                                if event.shift_key() { "Shift+" } else { "" },
                                if event.meta_key() { "Meta+" } else { "" }
                            );
                            let key_combo = format!("{}{}", modifier_key, key);

                            if let Some(handler) = keymap.get(&key_combo) {
                                handler.emit(event.clone());
                                return false;
                            }
                        }
                    }

                    self.handle_keydown(event, ctx);
                }
                false
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
            CodeEditorMsg::ScrollSync(top, left) => {
                // Only process if values are different (avoid loop)
                if self.scroll_top != top || self.scroll_left != left {
                    self.scroll_top = top;
                    self.scroll_left = left;

                    // Sync scrolling between all components
                    // This ensures synchronized scrolling across the entire editor
                    if let Some(display) = self.display_ref.cast::<HtmlElement>() {
                        // Only update if values are different to avoid scroll jank
                        if display.scroll_top() as f64 != top {
                            display.set_scroll_top(top as i32);
                        }
                        if display.scroll_left() as f64 != left {
                            display.set_scroll_left(left as i32);
                        }
                    }

                    if let Some(gutter) = self.gutter_ref.cast::<HtmlElement>() {
                        // Only update if values are different
                        if gutter.scroll_top() as f64 != top {
                            gutter.set_scroll_top(top as i32);
                        }
                    }
                }

                // Never trigger a re-render from scroll sync
                false
            }
            CodeEditorMsg::CursorPositionChanged(pos) => {
                if self.cursor_position != pos {
                    self.cursor_position = pos;
                    true
                } else {
                    false
                }
            }
            CodeEditorMsg::SelectionChanged(start, end) => {
                let changed = self.selection_start != start || self.selection_end != end;
                if changed {
                    self.selection_start = start;
                    self.selection_end = end;
                    self.has_selection = start != end;

                    // Don't re-render if we're still processing cursor operations
                    let currently_selecting = start != end;
                    currently_selecting
                } else {
                    false
                }
            }
            CodeEditorMsg::AddCursor(pos) => {
                if !self.cursors.contains(&pos) {
                    self.cursors.push(pos);
                    true
                } else {
                    false
                }
            }
            CodeEditorMsg::RemoveCursor(index) => {
                if index < self.cursors.len() {
                    self.cursors.remove(index);
                    true
                } else {
                    false
                }
            }
            CodeEditorMsg::ClearCursors => {
                if !self.cursors.is_empty() {
                    self.cursors.clear();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

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

        let white_space = if props.line_wrap { "pre-wrap" } else { "pre" };
        let editor_style = format!(
            "tab-size: {}; -moz-tab-size: {}; white-space: {};",
            props.tab_size, props.tab_size, white_space
        );

        let max_height_style = if props.max_height > 0 {
            format!("max-height: {}px;", props.max_height)
        } else {
            String::new()
        };

        let theme_class = if props.theme == "dark" {
            "dark"
        } else {
            "light"
        };

        // Create cursor element based on the current cursor position
        let cursor_style = format!(
            "top: {}em; left: {}ch;",
            self.cursor_position.0 as f32 * props.line_height,
            self.cursor_position.1
        );

        // Get a map of which lines have diffs for easy lookup
        let mut diff_map: HashMap<usize, Vec<&diff::Diff>> = HashMap::new();
        for diff in &props.diffs {
            let line_idx = diff.line_number;
            diff_map.entry(line_idx).or_default().push(diff);
        }

        html! {
            <>
                <styles::CodeEditorStyles />
                <div
                    class={classes!(
                        props.class.clone(),
                        "flex","relative","rounded","overflow-hidden",
                        "border","border-gray-300","dark:border-gray-700",
                        "bg-white","dark:bg-gray-900",
                        "text-gray-900","dark:text-gray-100",
                        theme_class
                    )}
                    style={format!("{} {}", container_style, max_height_style)}
                >
                    // The editor layout is now a flex container with synchronized scroll
                    <div class="flex w-full h-full relative overflow-hidden">
                        // Line numbers column (conditionally rendered)
                        if props.show_line_numbers {
                            <div
                                ref={self.gutter_ref.clone()}
                                class="flex-none w-12 text-right pr-2 pt-2 pb-2 bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400 select-none border-r border-gray-300 dark:border-gray-700 overflow-y-hidden"
                                style={format!("position: sticky; left: 0; z-index: 30;")}
                            >
                                { self.render_line_numbers(ctx) }
                            </div>
                        }

                        // Main editor area with scrollable content - this will control scrolling
                        <div class="flex-grow relative overflow-hidden">
                            // Diffs background layer
                            <div class="absolute inset-0 pointer-events-none">
                                {{
                                    // Generate line-level diff highlights
                                    let mut diff_elements = Vec::<Html>::new();

                                    for (i, _) in self.lines.iter().enumerate() {
                                        let line_num = i + 1;

                                        if let Some(diffs) = diff_map.get(&line_num) {
                                            if !diffs.is_empty() {
                                                let diff = &diffs[0]; // Use the first diff for line highlighting
                                                let diff_class = match diff.diff_type {
                                                    diff::DiffType::Added => "diff-added",
                                                    diff::DiffType::Removed => "diff-removed",
                                                    diff::DiffType::Modified => "diff-modified",
                                                };

                                                diff_elements.push(html! {
                                                    <div
                                                        class={classes!("absolute", "left-0", "right-0", diff_class)}
                                                        style={format!("top: {}em; height: {}em;",
                                                            i as f32 * props.line_height, props.line_height)}
                                                    />
                                                });
                                            }
                                        }
                                    }

                                    html! { <>{for diff_elements}</> }
                                }}
                            </div>

                            // Main content container with the code display and textarea
                            <div
                                class="relative min-w-full"
                                style="display: inline-block; min-height: 100%;"
                            >
                                // Syntax highlighted code display (non-scrollable - controlled by textarea scroll)
                                <div
                                    ref={self.display_ref.clone()}
                                    class="p-2 whitespace-pre absolute top-0 left-0 right-0 w-full"
                                    style={editor_style.clone()}
                                >
                                    { self.highlighted_html.clone() }
                                </div>

                                // Main cursor
                                <div
                                    class="absolute w-[2px] bg-blue-500 animate-blink pointer-events-none z-15"
                                    style={format!("{}height: {}em;", cursor_style, props.line_height)}
                                />

                                // Render additional cursors if multi-cursor is enabled
                                if props.enable_multi_cursor {
                                    {
                                        self.cursors.iter().map(|(line, col)| {
                                            let cursor_pos_style = format!(
                                                "top: {}em; left: {}ch; height: {}em;",
                                                *line as f32 * props.line_height,
                                                col,
                                                props.line_height
                                            );
                                            html! {
                                                <div
                                                    class="absolute w-[2px] bg-blue-500 animate-blink pointer-events-none z-15"
                                                    style={cursor_pos_style}
                                                />
                                            }
                                        }).collect::<Html>()
                                    }
                                }

                                // Selection highlights
                                // Always render selection if it exists
                                { self.render_selection_highlights(ctx) }

                                // Annotations and type hints overlay
                                <div
                                    class="absolute inset-0 pointer-events-none z-10"
                                >
                                    { self.render_annotations_and_hints(ctx) }
                                </div>

                                // Actual editable textarea (transparent, handles input and controls scrolling)
                                <textarea
                                    ref={self.textarea_ref.clone()}
                                    class="absolute inset-0 p-2 bg-transparent text-transparent resize-none z-20 overflow-auto"
                                    style={editor_style}
                                    value={self.code.clone()}
                                    readonly={props.read_only}
                                    spellcheck="false"
                                    autocomplete="off"
                                    autocorrect="off"
                                    autocapitalize="off"
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.highlight_code(ctx);
            self.setup_listeners(ctx);

            // Focus the textarea to ensure the cursor is visible
            if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
                // Set cursor to the beginning
                textarea.focus().ok();
                textarea.set_selection_range(0, 0).ok();

                // Force cursor position update after initial render
                let cursor_pos = Self::get_cursor_position(&textarea);
                ctx.link()
                    .send_message(CodeEditorMsg::CursorPositionChanged(cursor_pos));

                // Initialize selection
                if let (Some(start), Some(end)) = (
                    textarea.selection_start().ok().flatten(),
                    textarea.selection_end().ok().flatten(),
                ) {
                    ctx.link().send_message(CodeEditorMsg::SelectionChanged(
                        start as usize,
                        end as usize,
                    ));
                }

                // Set initial scroll position
                if let Some(display) = self.display_ref.cast::<HtmlElement>() {
                    display.set_scroll_top(0);
                    display.set_scroll_left(0);
                }

                if let Some(gutter) = self.gutter_ref.cast::<HtmlElement>() {
                    gutter.set_scroll_top(0);
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();

        // Check if code changed from props
        if props.code != self.code {
            self.code = props.code.clone();
            self.lines = Self::split_into_lines(&self.code);
            self.highlight_code(ctx);
        }

        // If language or theme changed, we need to rehighlight
        if old_props.language != props.language || old_props.theme != props.theme {
            self.highlight_code(ctx);
        }

        // Update keymap when it changes
        if old_props.keymap != props.keymap || old_props.enable_keymap != props.enable_keymap {
            self.keymap = props.keymap.clone();
            self.keymap_enabled = props.enable_keymap;
        }

        true
    }
}

impl CodeEditor {
    fn split_into_lines(code: &str) -> Vec<String> {
        code.lines().map(String::from).collect()
    }

    fn setup_listeners(&mut self, ctx: &Context<Self>) {
        // Clear any existing listeners to prevent memory leaks
        self._listeners.clear();

        if let Some(textarea) = self.textarea_ref.cast::<HtmlElement>() {
            // Create a new scope for each event listener to avoid moved value errors
            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();

                // Use a RefCell to track last scroll time for throttling
                let last_scroll_time = Rc::new(RefCell::new(0.0));

                // Attach scroll event listener to the textarea for scrolling synchronization
                let listener = EventListener::new(&textarea, "scroll", move |_| {
                    // Get current time to throttle scroll events (max once per 16ms - ~60fps)
                    let now = js_sys::Date::now();
                    let mut last_time = last_scroll_time.borrow_mut();

                    // Only process scroll events if enough time has passed (16ms = ~60fps)
                    if now - *last_time > 16.0 {
                        *last_time = now;
                        let scroll_top = textarea_clone.scroll_top() as f64;
                        let scroll_left = textarea_clone.scroll_left() as f64;
                        link.send_message(CodeEditorMsg::ScrollSync(scroll_top, scroll_left));
                    }
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "keydown", move |event| {
                    let keyboard_event = event.clone().dyn_into::<KeyboardEvent>().unwrap();
                    link.send_message(CodeEditorMsg::Keydown(keyboard_event));
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "keyup", move |_| {
                    if let Some(input) = textarea_clone.dyn_ref::<HtmlTextAreaElement>() {
                        let cursor_pos = Self::get_cursor_position(input);
                        link.send_message(CodeEditorMsg::CursorPositionChanged(cursor_pos));

                        // Update selection
                        if let (Some(start), Some(end)) = (
                            input.selection_start().ok().flatten(),
                            input.selection_end().ok().flatten(),
                        ) {
                            link.send_message(CodeEditorMsg::SelectionChanged(
                                start as usize,
                                end as usize,
                            ));
                        }
                    }
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "input", move |_| {
                    if let Some(input) = textarea_clone.dyn_ref::<HtmlTextAreaElement>() {
                        let code = input.value();
                        link.send_message(CodeEditorMsg::CodeChanged(code));
                    }
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let props = ctx.props().clone();
                let listener = EventListener::new(&textarea, "click", move |event| {
                    if let Some(input) = textarea_clone.dyn_ref::<HtmlTextAreaElement>() {
                        let cursor_pos = Self::get_cursor_position(input);
                        link.send_message(CodeEditorMsg::CursorPositionChanged(cursor_pos));

                        // Multi-cursor support with Alt/Option key
                        if props.enable_multi_cursor {
                            let mouse_event = event.clone().dyn_into::<MouseEvent>().unwrap();
                            if mouse_event.alt_key() {
                                // Add secondary cursor when Alt/Option is pressed during click
                                link.send_message(CodeEditorMsg::AddCursor(cursor_pos));
                                // Prevent default to avoid text selection
                                mouse_event.prevent_default();
                                return;
                            } else if mouse_event.ctrl_key() && mouse_event.alt_key() {
                                // Clear all cursors with Ctrl+Alt+Click
                                link.send_message(CodeEditorMsg::ClearCursors);
                                mouse_event.prevent_default();
                                return;
                            }
                        }

                        // Update selection
                        if let (Some(start), Some(end)) = (
                            input.selection_start().ok().flatten(),
                            input.selection_end().ok().flatten(),
                        ) {
                            link.send_message(CodeEditorMsg::SelectionChanged(
                                start as usize,
                                end as usize,
                            ));
                        }
                    }
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "mouseup", move |_| {
                    if let Some(input) = textarea_clone.dyn_ref::<HtmlTextAreaElement>() {
                        let cursor_pos = Self::get_cursor_position(input);
                        link.send_message(CodeEditorMsg::CursorPositionChanged(cursor_pos));

                        // Update selection
                        if let (Some(start), Some(end)) = (
                            input.selection_start().ok().flatten(),
                            input.selection_end().ok().flatten(),
                        ) {
                            link.send_message(CodeEditorMsg::SelectionChanged(
                                start as usize,
                                end as usize,
                            ));
                        }
                    }
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "focus", move |event| {
                    link.send_message(CodeEditorMsg::Focus(
                        event.clone().dyn_into::<FocusEvent>().unwrap(),
                    ));
                });
                self._listeners.push(listener);
            }

            {
                let textarea_clone = textarea.clone();
                let link = ctx.link().clone();
                let listener = EventListener::new(&textarea, "blur", move |event| {
                    link.send_message(CodeEditorMsg::Blur(
                        event.clone().dyn_into::<FocusEvent>().unwrap(),
                    ));
                });
                self._listeners.push(listener);
            }
        }
    }

    fn get_cursor_position(textarea: &HtmlTextAreaElement) -> (usize, usize) {
        let value = textarea.value();
        let selection_start = textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

        // Count lines and columns up to the cursor position
        let text_up_to_cursor = value.chars().take(selection_start).collect::<String>();
        let lines = text_up_to_cursor.split('\n').collect::<Vec<_>>();
        let line_index = lines.len().saturating_sub(1);
        let column_index = lines.last().map_or(0, |line| line.chars().count());

        (line_index, column_index)
    }

    fn get_position_from_index(&self, index: usize) -> (usize, usize) {
        let text_up_to_index = self.code.chars().take(index).collect::<String>();
        let lines = text_up_to_index.split('\n').collect::<Vec<_>>();
        let line_index = lines.len().saturating_sub(1);
        let column_index = lines.last().map_or(0, |line| line.chars().count());

        (line_index, column_index)
    }

    fn render_selection_highlights(&self, ctx: &Context<Self>) -> Html {
        // Early return if there's no selection (start and end are the same)
        if !self.has_selection || self.selection_start == self.selection_end {
            return html! {};
        }

        let props = ctx.props();
        let start_pos = self.get_position_from_index(self.selection_start);
        let end_pos = self.get_position_from_index(self.selection_end);

        let mut highlights = Vec::<Html>::new();

        if start_pos.0 == end_pos.0 {
            // Selection is on a single line
            let line_idx = start_pos.0;
            let highlight_style = format!(
                "top: {}em; left: {}ch; width: {}ch; height: {}em;",
                line_idx as f32 * props.line_height,
                start_pos.1,
                end_pos.1 - start_pos.1,
                props.line_height
            );

            highlights.push(html! {
                <div
                    class="absolute bg-blue-200 dark:bg-blue-800 bg-opacity-30 dark:bg-opacity-30 z-5"
                    style={highlight_style}
                ></div>
            });
        } else {
            // Multi-line selection
            // First line (partial)
            let first_line_length = if start_pos.0 < self.lines.len() {
                self.lines[start_pos.0].len().saturating_sub(start_pos.1)
            } else {
                0
            };
            let first_line_style = format!(
                "top: {}em; left: {}ch; width: {}ch; height: {}em;",
                start_pos.0 as f32 * props.line_height,
                start_pos.1,
                first_line_length,
                props.line_height
            );
            highlights.push(html! {
                <div
                    class="absolute bg-blue-200 dark:bg-blue-800 bg-opacity-30 dark:bg-opacity-30 z-5"
                    style={first_line_style}
                ></div>
            });

            // Middle lines (full lines)
            for line_idx in start_pos.0 + 1..end_pos.0 {
                if line_idx >= self.lines.len() {
                    break;
                }
                let line_length = self.lines[line_idx].len();
                let line_style = format!(
                    "top: {}em; left: 0; width: {}ch; height: {}em;",
                    line_idx as f32 * props.line_height,
                    line_length,
                    props.line_height
                );
                highlights.push(html! {
                    <div
                        class="absolute bg-blue-200 dark:bg-blue-800 bg-opacity-30 dark:bg-opacity-30 z-5"
                        style={line_style}
                    ></div>
                });
            }

            // Last line (partial)
            let last_line_style = format!(
                "top: {}em; left: 0; width: {}ch; height: {}em;",
                end_pos.0 as f32 * props.line_height,
                end_pos.1,
                props.line_height
            );
            highlights.push(html! {
                <div
                    class="absolute bg-blue-200 dark:bg-blue-800 bg-opacity-30 dark:bg-opacity-30 z-5"
                    style={last_line_style}
                ></div>
            });
        }

        html! {
            <div class="pointer-events-none">
                <>{for highlights}</>
            </div>
        }
    }

    fn handle_keydown(&mut self, event: KeyboardEvent, ctx: &Context<Self>) {
        // No logging needed for production
        let key = event.key();

        if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            match key.as_str() {
                "Tab" => {
                    event.prevent_default();

                    let tab_size = " ".repeat(ctx.props().tab_size as usize);
                    let selection_start =
                        textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
                    let selection_end =
                        textarea.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;
                    let value = textarea.value();

                    if selection_start == selection_end {
                        // No selection, just insert a tab
                        let new_value = format!(
                            "{}{}{}",
                            &value[0..selection_start],
                            tab_size,
                            &value[selection_end..]
                        );

                        textarea.set_value(&new_value);
                        textarea
                            .set_selection_start(Some((selection_start + tab_size.len()) as u32))
                            .unwrap();
                        textarea
                            .set_selection_end(Some((selection_start + tab_size.len()) as u32))
                            .unwrap();

                        ctx.link()
                            .send_message(CodeEditorMsg::CodeChanged(new_value));
                    } else {
                        // Handle multi-line indentation
                        let text_before = &value[..selection_start];
                        let selected_text = &value[selection_start..selection_end];
                        let text_after = &value[selection_end..];

                        if event.shift_key() {
                            // Unindent (not implemented for simplicity)
                            // But in a real implementation, this would remove tabs/spaces
                        } else {
                            // Add indentation to the beginning of each line
                            let indented = if selected_text.contains('\n') {
                                // Indent multiple lines
                                let lines = selected_text.split('\n');
                                let indented_lines: Vec<String> =
                                    lines.map(|line| format!("{}{}", tab_size, line)).collect();
                                indented_lines.join("\n")
                            } else {
                                // Indent single line
                                format!("{}{}", tab_size, selected_text)
                            };

                            let new_value = format!("{}{}{}", text_before, indented, text_after);
                            textarea.set_value(&new_value);

                            // Update selection range
                            let new_selection_end = selection_start + indented.len();
                            textarea
                                .set_selection_start(Some(selection_start as u32))
                                .unwrap();
                            textarea
                                .set_selection_end(Some(new_selection_end as u32))
                                .unwrap();

                            ctx.link()
                                .send_message(CodeEditorMsg::CodeChanged(new_value));
                        }
                    }
                }
                _ => {
                    // Let the browser handle other keys
                }
            }
        }
    }

    fn highlight_code(&mut self, ctx: &Context<Self>) {
        let props = ctx.props();
        let language: String = props.language.clone();
        let mut html_lines = Vec::new();

        // Create a temporary element to assist with syntax highlighting
        let temp_element = document().create_element("pre").unwrap();
        let _ = temp_element.set_class_name(&format!("language-{}", language));

        // Attempt to use Prism.js if available in the global scope
        let window = web_sys::window().unwrap();
        let has_prism = js_sys::Reflect::has(&window, &JsValue::from_str("Prism")).unwrap_or(false);

        for (i, line) in self.lines.iter().enumerate() {
            let line_clone = line.clone();
            let line_num = i + 1;

            if has_prism && !line_clone.is_empty() && line_clone.trim().len() > 0 {
                // Use Prism.js for syntax highlighting
                let code_element = document().create_element("code").unwrap();
                let _ = code_element.set_class_name(&format!("language-{}", language));
                code_element.set_text_content(Some(&line_clone));
                let _ = temp_element.set_inner_html("");
                let _ = temp_element.append_child(&code_element);

                // Call Prism's highlightElement method if available
                if let (Ok(prism), true) = (
                    js_sys::Reflect::get(&window, &JsValue::from_str("Prism")),
                    has_prism,
                ) {
                    if let Ok(highlight_fn) =
                        js_sys::Reflect::get(&prism, &JsValue::from_str("highlightElement"))
                    {
                        if let Ok(function) = highlight_fn.dyn_into::<js_sys::Function>() {
                            let _ = function.call1(&prism, &code_element.clone().into());
                        }
                    }
                }

                // Get the highlighted HTML and convert it to a safe format for Yew
                let highlighted_html = code_element.inner_html();
                html_lines.push(html! {
                    <div class="line">
                        <div class="line-content">
                            { Html::from_html_unchecked(highlighted_html.into()) }
                        </div>
                    </div>
                });
            } else {
                // Fallback to simple highlighting
                let tokens = self.tokenize_line(&line_clone, &language);
                html_lines.push(html! {
                    <div class="line">
                        <div class="line-content">
                            { for tokens.into_iter().map(|(token_type, text)| {
                                html! {
                                    <span class={format!("token {}", token_type)}>{ text }</span>
                                }
                            })}
                        </div>
                    </div>
                });
            }
        }

        // Update the highlighted HTML
        self.highlighted_html = html! {
            <>
                { for html_lines.into_iter() }
            </>
        };
    }
}
