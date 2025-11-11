use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::ClassesStr;

/// File attachment structure
#[derive(Clone, PartialEq, Debug)]
pub struct FileAttachment {
    pub id: String,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub url: Option<String>,
}

/// Markdown editor mode
#[derive(Clone, PartialEq, Debug)]
pub enum EditorMode {
    Edit,
    Preview,
    Split,
}

/// Toolbar button action
#[derive(Clone, PartialEq, Debug)]
pub enum ToolbarAction {
    Bold,
    Italic,
    Strikethrough,
    Heading(u8),
    Link,
    Image,
    Code,
    CodeBlock,
    Quote,
    UnorderedList,
    OrderedList,
    TaskList,
    Table,
    HorizontalRule,
}

/// Properties for the MarkdownEditor component
#[derive(Properties, PartialEq, Clone)]
pub struct MarkdownEditorProps {
    /// Initial markdown content
    #[prop_or_default]
    pub value: String,

    /// Callback when content changes
    #[prop_or_default]
    pub on_change: Callback<String>,

    /// Placeholder text
    #[prop_or_else(|| "Write your markdown here...".to_string())]
    pub placeholder: String,

    /// Whether the editor is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Show toolbar
    #[prop_or(true)]
    pub show_toolbar: bool,

    /// Show mode switcher (Edit/Preview/Split)
    #[prop_or(true)]
    pub show_mode_switcher: bool,

    /// Initial editor mode
    #[prop_or(EditorMode::Split)]
    pub initial_mode: EditorMode,

    /// Enable file attachments
    #[prop_or(false)]
    pub enable_attachments: bool,

    /// Callback when files are attached
    #[prop_or_default]
    pub on_attach: Callback<Vec<FileAttachment>>,

    /// Current file attachments
    #[prop_or_default]
    pub attachments: Vec<FileAttachment>,

    /// Enable role/user selection
    #[prop_or(false)]
    pub enable_mentions: bool,

    /// Available users/roles for mentions
    #[prop_or_default]
    pub mention_options: Vec<(String, String)>, // (id, display_name)

    /// Callback when a user/role is mentioned
    #[prop_or_default]
    pub on_mention: Callback<String>,

    /// Minimum height in pixels
    #[prop_or(300)]
    pub min_height: u32,

    /// Maximum height in pixels (0 = no limit)
    #[prop_or(0)]
    pub max_height: u32,

    /// Custom CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Auto-save interval in milliseconds (0 = disabled)
    #[prop_or(0)]
    pub autosave_interval: u32,

    /// Callback for auto-save
    #[prop_or_default]
    pub on_autosave: Callback<String>,

    /// Enable keyboard shortcuts
    #[prop_or(true)]
    pub enable_shortcuts: bool,

    /// Custom CSS for preview
    #[prop_or_default]
    pub preview_css: String,

    /// Show character/word count
    #[prop_or(true)]
    pub show_stats: bool,
}

pub enum MarkdownEditorMsg {
    UpdateContent(String),
    SetMode(EditorMode),
    ApplyFormat(ToolbarAction),
    AttachFile(Vec<FileAttachment>),
    RemoveAttachment(String),
    InsertMention(String),
    HandleKeyDown(KeyboardEvent),
    AutoSave,
    UpdateStats,
}

pub struct MarkdownEditor {
    content: String,
    mode: EditorMode,
    textarea_ref: NodeRef,
    preview_ref: NodeRef,
    word_count: usize,
    char_count: usize,
    line_count: usize,
}

impl Component for MarkdownEditor {
    type Message = MarkdownEditorMsg;
    type Properties = MarkdownEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let content = ctx.props().value.clone();
        let (word_count, char_count, line_count) = Self::calculate_stats(&content);

        Self {
            content,
            mode: ctx.props().initial_mode.clone(),
            textarea_ref: NodeRef::default(),
            preview_ref: NodeRef::default(),
            word_count,
            char_count,
            line_count,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MarkdownEditorMsg::UpdateContent(new_content) => {
                self.content = new_content.clone();
                let (word_count, char_count, line_count) = Self::calculate_stats(&new_content);
                self.word_count = word_count;
                self.char_count = char_count;
                self.line_count = line_count;
                ctx.props().on_change.emit(new_content);
                true
            }
            MarkdownEditorMsg::SetMode(mode) => {
                self.mode = mode;
                true
            }
            MarkdownEditorMsg::ApplyFormat(action) => {
                self.apply_formatting(action);
                ctx.props().on_change.emit(self.content.clone());
                true
            }
            MarkdownEditorMsg::AttachFile(files) => {
                ctx.props().on_attach.emit(files);
                true
            }
            MarkdownEditorMsg::RemoveAttachment(_id) => {
                // Handled by parent component
                false
            }
            MarkdownEditorMsg::InsertMention(mention) => {
                self.insert_text(&format!("@{}", mention));
                ctx.props().on_mention.emit(mention);
                true
            }
            MarkdownEditorMsg::HandleKeyDown(event) => {
                if ctx.props().enable_shortcuts {
                    self.handle_keyboard_shortcut(ctx, event)
                } else {
                    false
                }
            }
            MarkdownEditorMsg::AutoSave => {
                ctx.props().on_autosave.emit(self.content.clone());
                false
            }
            MarkdownEditorMsg::UpdateStats => {
                let (word_count, char_count, line_count) = Self::calculate_stats(&self.content);
                self.word_count = word_count;
                self.char_count = char_count;
                self.line_count = line_count;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class={classes!("wonop-markdown-editor", "flex", "flex-col", "border", "rounded-lg", "overflow-hidden", props.class.clone())}>
                // Toolbar
                {if props.show_toolbar {
                    self.render_toolbar(ctx)
                } else {
                    html! {}
                }}

                // Mode Switcher
                {if props.show_mode_switcher {
                    self.render_mode_switcher(ctx)
                } else {
                    html! {}
                }}

                // Editor Area
                <div class="flex-1 flex overflow-hidden" style={format!("min-height: {}px;", props.min_height)}>
                    {self.render_editor_content(ctx)}
                </div>

                // Attachments
                {if props.enable_attachments && !props.attachments.is_empty() {
                    self.render_attachments(ctx)
                } else {
                    html! {}
                }}

                // Status Bar
                {if props.show_stats {
                    self.render_status_bar(ctx)
                } else {
                    html! {}
                }}
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().value != self.content {
            self.content = ctx.props().value.clone();
            let (word_count, char_count, line_count) = Self::calculate_stats(&self.content);
            self.word_count = word_count;
            self.char_count = char_count;
            self.line_count = line_count;
            true
        } else {
            false
        }
    }
}

impl MarkdownEditor {
    fn render_toolbar(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="flex items-center gap-1 p-2 border-b bg-gray-50 flex-wrap">
                // Text formatting
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Bold (Ctrl+B)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Bold))}
                >
                    <span class="font-bold">{"B"}</span>
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Italic (Ctrl+I)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Italic))}
                >
                    <span class="italic">{"I"}</span>
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Strikethrough (Ctrl+Shift+X)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Strikethrough))}
                >
                    <span class="line-through">{"S"}</span>
                </button>

                <div class="w-px h-6 bg-gray-300 mx-1"></div>

                // Headings
                {for (1..=3).map(|level| {
                    let level_clone = level;
                    html! {
                        <button
                            class="p-2 hover:bg-gray-200 rounded transition-colors"
                            title={format!("Heading {} (Ctrl+{})", level, level)}
                            onclick={link.callback(move |_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Heading(level_clone)))}
                        >
                            {format!("H{}", level)}
                        </button>
                    }
                })}

                <div class="w-px h-6 bg-gray-300 mx-1"></div>

                // Insert elements
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Link (Ctrl+K)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Link))}
                >
                    {"🔗"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Image (Ctrl+Shift+I)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Image))}
                >
                    {"🖼️"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Code (Ctrl+`)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Code))}
                >
                    {"</>"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Code Block (Ctrl+Shift+C)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::CodeBlock))}
                >
                    {"```"}
                </button>

                <div class="w-px h-6 bg-gray-300 mx-1"></div>

                // Lists
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Unordered List (Ctrl+Shift+U)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::UnorderedList))}
                >
                    {"• List"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Ordered List (Ctrl+Shift+O)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::OrderedList))}
                >
                    {"1. List"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Task List (Ctrl+Shift+T)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::TaskList))}
                >
                    {"☐ Task"}
                </button>

                <div class="w-px h-6 bg-gray-300 mx-1"></div>

                // Other
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Quote (Ctrl+Shift+Q)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Quote))}
                >
                    {"Quote"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Table (Ctrl+Shift+T)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::Table))}
                >
                    {"Table"}
                </button>
                <button
                    class="p-2 hover:bg-gray-200 rounded transition-colors"
                    title="Horizontal Rule (Ctrl+Shift+H)"
                    onclick={link.callback(|_| MarkdownEditorMsg::ApplyFormat(ToolbarAction::HorizontalRule))}
                >
                    {"---"}
                </button>

                // File attachment button
                {if ctx.props().enable_attachments {
                    html! {
                        <>
                            <div class="w-px h-6 bg-gray-300 mx-1"></div>
                            <button
                                class="p-2 hover:bg-gray-200 rounded transition-colors"
                                title="Attach File"
                                onclick={link.callback(|_| {
                                    // Trigger file input click
                                    MarkdownEditorMsg::UpdateStats
                                })}
                            >
                                {"📎 Attach"}
                            </button>
                        </>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }

    fn render_mode_switcher(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="flex items-center gap-2 p-2 border-b bg-gray-100">
                <button
                    class={classes!(
                        "px-3", "py-1", "rounded", "transition-colors",
                        if self.mode == EditorMode::Edit { "bg-blue-500 text-white" } else { "bg-white hover:bg-gray-200" }
                    )}
                    onclick={link.callback(|_| MarkdownEditorMsg::SetMode(EditorMode::Edit))}
                >
                    {"Edit"}
                </button>
                <button
                    class={classes!(
                        "px-3", "py-1", "rounded", "transition-colors",
                        if self.mode == EditorMode::Preview { "bg-blue-500 text-white" } else { "bg-white hover:bg-gray-200" }
                    )}
                    onclick={link.callback(|_| MarkdownEditorMsg::SetMode(EditorMode::Preview))}
                >
                    {"Preview"}
                </button>
                <button
                    class={classes!(
                        "px-3", "py-1", "rounded", "transition-colors",
                        if self.mode == EditorMode::Split { "bg-blue-500 text-white" } else { "bg-white hover:bg-gray-200" }
                    )}
                    onclick={link.callback(|_| MarkdownEditorMsg::SetMode(EditorMode::Split))}
                >
                    {"Split"}
                </button>
            </div>
        }
    }

    fn render_editor_content(&self, ctx: &Context<Self>) -> Html {

        match self.mode {
            EditorMode::Edit => {
                html! {
                    <div class="flex-1 flex flex-col">
                        {self.render_textarea(ctx)}
                    </div>
                }
            }
            EditorMode::Preview => {
                html! {
                    <div class="flex-1 flex flex-col">
                        {self.render_preview(ctx)}
                    </div>
                }
            }
            EditorMode::Split => {
                html! {
                    <>
                        <div class="flex-1 border-r">
                            {self.render_textarea(ctx)}
                        </div>
                        <div class="flex-1">
                            {self.render_preview(ctx)}
                        </div>
                    </>
                }
            }
        }
    }

    fn render_textarea(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();

        let oninput = link.callback(|e: InputEvent| {
            let target = e.target().unwrap();
            let textarea = target.dyn_into::<HtmlTextAreaElement>().unwrap();
            MarkdownEditorMsg::UpdateContent(textarea.value())
        });

        let onkeydown = link.callback(|e: KeyboardEvent| {
            MarkdownEditorMsg::HandleKeyDown(e)
        });

        html! {
            <textarea
                ref={self.textarea_ref.clone()}
                class="w-full h-full p-4 resize-none focus:outline-none font-mono"
                value={self.content.clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                {oninput}
                {onkeydown}
            />
        }
    }

    fn render_preview(&self, _ctx: &Context<Self>) -> Html {
        let html_content = self.markdown_to_html(&self.content);

        // Use Html::from_html_unchecked to render the markdown HTML
        let inner_html = Html::from_html_unchecked(AttrValue::from(html_content));

        html! {
            <div
                ref={self.preview_ref.clone()}
                class="w-full h-full p-4 overflow-auto prose prose-sm max-w-none"
            >
                { inner_html }
            </div>
        }
    }

    fn render_attachments(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        html! {
            <div class="border-t p-2 bg-gray-50">
                <div class="text-sm font-semibold mb-2">{"Attachments:"}</div>
                <div class="flex flex-wrap gap-2">
                    {for props.attachments.iter().map(|attachment| {
                        let attachment_id = attachment.id.clone();
                        html! {
                            <div class="flex items-center gap-2 px-3 py-1 bg-white border rounded">
                                <span class="text-sm">{&attachment.name}</span>
                                <span class="text-xs text-gray-500">{format!("({:.1} KB)", attachment.size as f64 / 1024.0)}</span>
                                <button
                                    class="text-red-500 hover:text-red-700"
                                    onclick={link.callback(move |_| MarkdownEditorMsg::RemoveAttachment(attachment_id.clone()))}
                                >
                                    {"×"}
                                </button>
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }

    fn render_status_bar(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex items-center justify-between px-4 py-2 border-t bg-gray-50 text-xs text-gray-600">
                <div class="flex gap-4">
                    <span>{format!("Lines: {}", self.line_count)}</span>
                    <span>{format!("Words: {}", self.word_count)}</span>
                    <span>{format!("Characters: {}", self.char_count)}</span>
                </div>
                <div>
                    <span>{"Markdown"}</span>
                </div>
            </div>
        }
    }

    fn markdown_to_html(&self, markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    fn calculate_stats(content: &str) -> (usize, usize, usize) {
        let line_count = content.lines().count();
        let char_count = content.chars().count();
        let word_count = content
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .count();

        (word_count, char_count, line_count)
    }

    fn apply_formatting(&mut self, action: ToolbarAction) {
        if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            let start = textarea.selection_start().unwrap_or(None).unwrap_or(0) as usize;
            let end = textarea.selection_end().unwrap_or(None).unwrap_or(0) as usize;
            let selected_text = if start != end {
                self.content[start..end].to_string()
            } else {
                String::new()
            };

            let (before_cursor, after_cursor) = self.content.split_at(start);
            let after_selection = if end > start {
                &self.content[end..]
            } else {
                after_cursor
            };

            let (insert_text, cursor_offset) = match action {
                ToolbarAction::Bold => {
                    if selected_text.is_empty() {
                        ("**bold text**".to_string(), 2)
                    } else {
                        (format!("**{}**", selected_text), selected_text.len() + 4)
                    }
                }
                ToolbarAction::Italic => {
                    if selected_text.is_empty() {
                        ("*italic text*".to_string(), 1)
                    } else {
                        (format!("*{}*", selected_text), selected_text.len() + 2)
                    }
                }
                ToolbarAction::Strikethrough => {
                    if selected_text.is_empty() {
                        ("~~strikethrough~~".to_string(), 2)
                    } else {
                        (format!("~~{}~~", selected_text), selected_text.len() + 4)
                    }
                }
                ToolbarAction::Heading(level) => {
                    let prefix = "#".repeat(level as usize);
                    if selected_text.is_empty() {
                        (format!("{} Heading {}", prefix, level), prefix.len() + 1)
                    } else {
                        (format!("{} {}", prefix, selected_text), prefix.len() + selected_text.len() + 1)
                    }
                }
                ToolbarAction::Link => {
                    if selected_text.is_empty() {
                        ("[link text](url)".to_string(), 1)
                    } else {
                        (format!("[{}](url)", selected_text), selected_text.len() + 3)
                    }
                }
                ToolbarAction::Image => {
                    if selected_text.is_empty() {
                        ("![alt text](image-url)".to_string(), 2)
                    } else {
                        (format!("![{}](image-url)", selected_text), selected_text.len() + 4)
                    }
                }
                ToolbarAction::Code => {
                    if selected_text.is_empty() {
                        ("`code`".to_string(), 1)
                    } else {
                        (format!("`{}`", selected_text), selected_text.len() + 2)
                    }
                }
                ToolbarAction::CodeBlock => {
                    if selected_text.is_empty() {
                        ("```\ncode\n```".to_string(), 4)
                    } else {
                        (format!("```\n{}\n```", selected_text), selected_text.len() + 8)
                    }
                }
                ToolbarAction::Quote => {
                    if selected_text.is_empty() {
                        ("> Quote".to_string(), 2)
                    } else {
                        (format!("> {}", selected_text), selected_text.len() + 2)
                    }
                }
                ToolbarAction::UnorderedList => {
                    ("- List item".to_string(), 2)
                }
                ToolbarAction::OrderedList => {
                    ("1. List item".to_string(), 3)
                }
                ToolbarAction::TaskList => {
                    ("- [ ] Task item".to_string(), 6)
                }
                ToolbarAction::Table => {
                    ("| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |".to_string(), 2)
                }
                ToolbarAction::HorizontalRule => {
                    ("---\n".to_string(), 4)
                }
            };

            self.content = format!("{}{}{}", before_cursor, insert_text, after_selection);

            // Update textarea value and cursor position
            let _ = textarea.set_value(&self.content);
            let new_cursor_pos = (start + cursor_offset) as u32;
            let _ = textarea.set_selection_start(Some(new_cursor_pos));
            let _ = textarea.set_selection_end(Some(new_cursor_pos));
            let _ = textarea.focus();
        }
    }

    fn insert_text(&mut self, text: &str) {
        if let Some(textarea) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            let start = textarea.selection_start().unwrap_or(None).unwrap_or(0) as usize;
            let (before_cursor, after_cursor) = self.content.split_at(start);
            self.content = format!("{}{}{}", before_cursor, text, after_cursor);

            let _ = textarea.set_value(&self.content);
            let new_cursor_pos = (start + text.len()) as u32;
            let _ = textarea.set_selection_start(Some(new_cursor_pos));
            let _ = textarea.set_selection_end(Some(new_cursor_pos));
        }
    }

    fn handle_keyboard_shortcut(&mut self, ctx: &Context<Self>, event: KeyboardEvent) -> bool {
        let ctrl_or_meta = event.ctrl_key() || event.meta_key();

        if !ctrl_or_meta {
            return false;
        }

        let action = match event.key().as_str() {
            "b" if !event.shift_key() => Some(ToolbarAction::Bold),
            "i" if !event.shift_key() => Some(ToolbarAction::Italic),
            "k" if !event.shift_key() => Some(ToolbarAction::Link),
            "`" if !event.shift_key() => Some(ToolbarAction::Code),
            "x" if event.shift_key() => Some(ToolbarAction::Strikethrough),
            "i" if event.shift_key() => Some(ToolbarAction::Image),
            "c" if event.shift_key() => Some(ToolbarAction::CodeBlock),
            "u" if event.shift_key() => Some(ToolbarAction::UnorderedList),
            "o" if event.shift_key() => Some(ToolbarAction::OrderedList),
            "t" if event.shift_key() => Some(ToolbarAction::TaskList),
            "q" if event.shift_key() => Some(ToolbarAction::Quote),
            "h" if event.shift_key() => Some(ToolbarAction::HorizontalRule),
            "1" if !event.shift_key() => Some(ToolbarAction::Heading(1)),
            "2" if !event.shift_key() => Some(ToolbarAction::Heading(2)),
            "3" if !event.shift_key() => Some(ToolbarAction::Heading(3)),
            _ => None,
        };

        if let Some(action) = action {
            event.prevent_default();
            self.apply_formatting(action);
            ctx.props().on_change.emit(self.content.clone());
            true
        } else {
            false
        }
    }
}
