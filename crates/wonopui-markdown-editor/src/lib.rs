//! Markdown Editor component for WonopUI.
//!
//! A basic markdown editor with preview support.
//! Note: Full markdown parsing would require a markdown library.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the MarkdownEditor component
pub mod classes {
    pub const CONTAINER: &str = "flex flex-col border rounded-md overflow-hidden";
    pub const TOOLBAR: &str = "flex gap-1 p-2 border-b bg-muted/50";
    pub const TOOLBAR_BUTTON: &str = "px-2 py-1 rounded hover:bg-accent text-sm font-medium";
    pub const CONTENT: &str = "flex flex-1 min-h-[200px]";
    pub const EDITOR: &str = "flex-1 p-3 resize-none outline-none border-none";
    pub const PREVIEW: &str = "flex-1 p-3 prose dark:prose-invert max-w-none overflow-auto";
    pub const SPLIT: &str = "w-px bg-border";
}

#[derive(Clone, PartialEq, Default)]
pub enum EditorMode {
    #[default]
    Edit,
    Preview,
    Split,
}

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub mode: EditorMode,
    /// Whether to show the toolbar
    #[prop_or(true)]
    pub show_toolbar: bool,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let content = use_state(|| props.value.clone());
    let mode = use_state(|| props.mode.clone());

    // Sync with prop changes
    {
        let content = content.clone();
        let prop_value = props.value.clone();
        use_effect_with(prop_value, move |value| {
            content.set(value.clone());
            || ()
        });
    }

    let oninput = {
        let content = content.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(textarea) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                let value = textarea.value();
                content.set(value.clone());
                onchange.emit(value);
            }
        })
    };

    let set_mode_edit = {
        let mode = mode.clone();
        Callback::from(move |_: MouseEvent| mode.set(EditorMode::Edit))
    };

    let set_mode_preview = {
        let mode = mode.clone();
        Callback::from(move |_: MouseEvent| mode.set(EditorMode::Preview))
    };

    let set_mode_split = {
        let mode = mode.clone();
        Callback::from(move |_: MouseEvent| mode.set(EditorMode::Split))
    };

    // Simple markdown to HTML conversion (very basic)
    let preview_html = simple_markdown_to_html(&content);

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);
    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Write markdown here...".to_string());

    html! {
        <div class={container_class}>
            if props.show_toolbar {
                <div class={classes::TOOLBAR}>
                    <button
                        class={classes::TOOLBAR_BUTTON}
                        onclick={set_mode_edit}
                    >
                        {"Edit"}
                    </button>
                    <button
                        class={classes::TOOLBAR_BUTTON}
                        onclick={set_mode_preview}
                    >
                        {"Preview"}
                    </button>
                    <button
                        class={classes::TOOLBAR_BUTTON}
                        onclick={set_mode_split}
                    >
                        {"Split"}
                    </button>
                </div>
            }
            <div class={classes::CONTENT}>
                { match *mode {
                    EditorMode::Edit => html! {
                        <textarea
                            class={classes::EDITOR}
                            value={(*content).clone()}
                            oninput={oninput}
                            placeholder={placeholder}
                        />
                    },
                    EditorMode::Preview => html! {
                        <div class={classes::PREVIEW}>
                            { Html::from_html_unchecked(preview_html.into()) }
                        </div>
                    },
                    EditorMode::Split => html! {
                        <>
                            <textarea
                                class={classes::EDITOR}
                                value={(*content).clone()}
                                oninput={oninput}
                                placeholder={placeholder}
                            />
                            <div class={classes::SPLIT} />
                            <div class={classes::PREVIEW}>
                                { Html::from_html_unchecked(preview_html.into()) }
                            </div>
                        </>
                    },
                } }
            </div>
        </div>
    }
}

/// Very simple markdown to HTML conversion
/// For production use, consider using a proper markdown library
fn simple_markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();

        if let Some(content) = trimmed.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>", content));
        } else if let Some(content) = trimmed.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>", content));
        } else if let Some(content) = trimmed.strip_prefix("### ") {
            html.push_str(&format!("<h3>{}</h3>", content));
        } else if let Some(content) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            html.push_str(&format!("<li>{}</li>", content));
        } else if trimmed.starts_with("```") {
            html.push_str("<pre><code>");
        } else if trimmed == "```" {
            html.push_str("</code></pre>");
        } else if trimmed.is_empty() {
            html.push_str("<br/>");
        } else {
            // Handle inline formatting
            let formatted = trimmed
                .replace("**", "<strong>") // Bold (simplified)
                .replace("*", "<em>"); // Italic (simplified)
            html.push_str(&format!("<p>{}</p>", formatted));
        }
    }

    html
}
