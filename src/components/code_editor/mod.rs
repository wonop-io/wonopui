use std::collections::HashMap;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

// Internal modules
pub mod annotation;
pub mod diff;
pub mod styles;
pub mod type_hint;

pub use annotation::{Annotation, AnnotationType};
pub use diff::{Diff, DiffType};
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
        let lines = self.code.lines().collect::<Vec<_>>();

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
                        // Line numbers
                        if props.show_line_numbers {
                            <div class="flex-none bg-gray-100 dark:bg-gray-800 px-3 py-2 text-gray-500 dark:text-gray-400 text-right select-none border-r border-gray-300 dark:border-gray-700">
                                { for lines.iter().enumerate().map(|(i, _)| {
                                    html! { <div key={i} class="leading-[inherit]">{ i + 1 }</div> }
                                }) }
                            </div>
                        }

                        // Editor area
                        <div class="flex-1 relative">
                            // Syntax highlighting overlay - exactly matches textarea positioning
                            <div 
                                class="absolute inset-0 p-2 m-0 pointer-events-none overflow-hidden whitespace-pre-wrap break-words text-gray-900 dark:text-gray-100"
                                style={format!("{} font-family: inherit; font-size: inherit; line-height: inherit;", editor_style)}
                            >
                                { self.render_highlighted_code(ctx) }
                            </div>

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
            result.push(html! { <span class="text-gray-500 dark:text-gray-400 italic">{ comment_part }</span> });
            
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
                    result.push(html! { <span class="text-green-600 dark:text-green-400">{ current_word.clone() }</span> });
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
                        result.push(html! { <span class="text-gray-600 dark:text-gray-400">{ ch.to_string() }</span> });
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
                result.push(html! { <span class="text-green-600 dark:text-green-400">{ current_word }</span> });
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
            result.push(html! { <span class="text-blue-600 dark:text-blue-400 font-semibold">{ word.to_string() }</span> });
        } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
            result.push(html! { <span class="text-purple-600 dark:text-purple-400">{ word.to_string() }</span> });
        } else {
            result.push(html! { { word.to_string() } });
        }
    }
}