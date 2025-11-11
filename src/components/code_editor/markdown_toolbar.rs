use yew::prelude::*;

/// Toolbar actions for markdown formatting
#[derive(Clone, PartialEq)]
pub enum MarkdownAction {
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
    HorizontalRule,
}

/// Properties for the markdown toolbar
#[derive(Properties, PartialEq, Clone)]
pub struct MarkdownToolbarProps {
    /// Callback when an action is triggered
    pub on_action: Callback<MarkdownAction>,

    /// Custom CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Theme name (light or dark)
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,

    /// Show or hide the toolbar
    #[prop_or(true)]
    pub visible: bool,
}

#[function_component(MarkdownToolbar)]
pub fn markdown_toolbar(props: &MarkdownToolbarProps) -> Html {
    let theme_class = if props.theme == "dark" {
        "dark"
    } else {
        "light"
    };

    if !props.visible {
        return html! {};
    }

    let on_action = props.on_action.clone();

    html! {
        <div
            class={classes!(
                props.class.clone(),
                "markdown-toolbar",
                "flex",
                "flex-wrap",
                "gap-1",
                "p-2",
                "bg-gray-100",
                "dark:bg-gray-800",
                "border",
                "border-gray-300",
                "dark:border-gray-700",
                "rounded",
                theme_class
            )}
        >
            // Text formatting
            <ToolbarButton
                icon="B"
                title="Bold (Ctrl+B)"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Bold))
                }}
                bold={true}
            />

            <ToolbarButton
                icon="I"
                title="Italic (Ctrl+I)"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Italic))
                }}
                italic={true}
            />

            <ToolbarButton
                icon="S"
                title="Strikethrough"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Strikethrough))
                }}
                strikethrough={true}
            />

            <ToolbarDivider />

            // Headings
            <ToolbarButton
                icon="H1"
                title="Heading 1"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Heading(1)))
                }}
            />

            <ToolbarButton
                icon="H2"
                title="Heading 2"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Heading(2)))
                }}
            />

            <ToolbarButton
                icon="H3"
                title="Heading 3"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Heading(3)))
                }}
            />

            <ToolbarDivider />

            // Links and images
            <ToolbarButton
                icon="🔗"
                title="Insert Link"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Link))
                }}
            />

            <ToolbarButton
                icon="🖼️"
                title="Insert Image"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Image))
                }}
            />

            <ToolbarDivider />

            // Code
            <ToolbarButton
                icon="`"
                title="Inline Code"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Code))
                }}
            />

            <ToolbarButton
                icon="```"
                title="Code Block"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::CodeBlock))
                }}
            />

            <ToolbarDivider />

            // Lists and quotes
            <ToolbarButton
                icon="•"
                title="Unordered List"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::UnorderedList))
                }}
            />

            <ToolbarButton
                icon="1."
                title="Ordered List"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::OrderedList))
                }}
            />

            <ToolbarButton
                icon="❝"
                title="Quote"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::Quote))
                }}
            />

            <ToolbarDivider />

            // Horizontal rule
            <ToolbarButton
                icon="─"
                title="Horizontal Rule"
                on_click={{
                    let on_action = on_action.clone();
                    Callback::from(move |_| on_action.emit(MarkdownAction::HorizontalRule))
                }}
            />
        </div>
    }
}

/// A toolbar button component
#[derive(Properties, PartialEq, Clone)]
struct ToolbarButtonProps {
    icon: String,
    title: String,
    on_click: Callback<MouseEvent>,
    #[prop_or(false)]
    bold: bool,
    #[prop_or(false)]
    italic: bool,
    #[prop_or(false)]
    strikethrough: bool,
}

#[function_component(ToolbarButton)]
fn toolbar_button(props: &ToolbarButtonProps) -> Html {
    let mut style_class = String::new();
    if props.bold {
        style_class.push_str("font-bold ");
    }
    if props.italic {
        style_class.push_str("italic ");
    }
    if props.strikethrough {
        style_class.push_str("line-through ");
    }

    html! {
        <button
            class={classes!(
                "toolbar-button",
                "px-3",
                "py-1",
                "text-sm",
                "font-medium",
                "text-gray-700",
                "dark:text-gray-300",
                "bg-white",
                "dark:bg-gray-700",
                "border",
                "border-gray-300",
                "dark:border-gray-600",
                "rounded",
                "hover:bg-gray-50",
                "dark:hover:bg-gray-600",
                "focus:outline-none",
                "focus:ring-2",
                "focus:ring-blue-500",
                "transition-colors",
                style_class
            )}
            title={props.title.clone()}
            onclick={props.on_click.clone()}
        >
            { &props.icon }
        </button>
    }
}

/// A divider between toolbar button groups
#[function_component(ToolbarDivider)]
fn toolbar_divider() -> Html {
    html! {
        <div class="w-px bg-gray-300 dark:bg-gray-600 my-1"></div>
    }
}

/// Helper functions to apply markdown formatting
pub mod formatting {
    use super::MarkdownAction;

    /// Apply markdown formatting to the selected text
    pub fn apply_formatting(
        text: &str,
        selection_start: usize,
        selection_end: usize,
        action: MarkdownAction,
    ) -> (String, usize, usize) {
        let before = &text[..selection_start];
        let selected = &text[selection_start..selection_end];
        let after = &text[selection_end..];

        match action {
            MarkdownAction::Bold => {
                let new_text = format!("{}**{}**{}", before, selected, after);
                let new_start = selection_start + 2;
                let new_end = selection_end + 2;
                (new_text, new_start, new_end)
            }
            MarkdownAction::Italic => {
                let new_text = format!("{}*{}*{}", before, selected, after);
                let new_start = selection_start + 1;
                let new_end = selection_end + 1;
                (new_text, new_start, new_end)
            }
            MarkdownAction::Strikethrough => {
                let new_text = format!("{}~~{}~~{}", before, selected, after);
                let new_start = selection_start + 2;
                let new_end = selection_end + 2;
                (new_text, new_start, new_end)
            }
            MarkdownAction::Heading(level) => {
                let hashes = "#".repeat(level as usize);
                let new_text = if selected.is_empty() {
                    format!("{}{} {}", before, hashes, after)
                } else {
                    format!("{}{} {}{}", before, hashes, selected, after)
                };
                let new_start = selection_start + level as usize + 1;
                let new_end = selection_end + level as usize + 1;
                (new_text, new_start, new_end)
            }
            MarkdownAction::Link => {
                let new_text = if selected.is_empty() {
                    format!("{}[Link text](url){}", before, after)
                } else {
                    format!("{}[{}](url){}", before, selected, after)
                };
                let new_start = selection_start + 1;
                let new_end = if selected.is_empty() {
                    selection_start + 10
                } else {
                    selection_end + 1
                };
                (new_text, new_start, new_end)
            }
            MarkdownAction::Image => {
                let new_text = if selected.is_empty() {
                    format!("{}![Alt text](url){}", before, after)
                } else {
                    format!("{}![{}](url){}", before, selected, after)
                };
                let new_start = selection_start + 2;
                let new_end = if selected.is_empty() {
                    selection_start + 11
                } else {
                    selection_end + 2
                };
                (new_text, new_start, new_end)
            }
            MarkdownAction::Code => {
                let new_text = format!("{}`{}`{}", before, selected, after);
                let new_start = selection_start + 1;
                let new_end = selection_end + 1;
                (new_text, new_start, new_end)
            }
            MarkdownAction::CodeBlock => {
                let new_text = if selected.is_empty() {
                    format!("{}```\n\n```{}", before, after)
                } else {
                    format!("{}```\n{}\n```{}", before, selected, after)
                };
                let new_start = selection_start + 4;
                let new_end = if selected.is_empty() {
                    selection_start + 4
                } else {
                    selection_end + 4
                };
                (new_text, new_start, new_end)
            }
            MarkdownAction::Quote => {
                let new_text = if selected.is_empty() {
                    format!("{}> {}", before, after)
                } else {
                    format!("{}> {}{}", before, selected, after)
                };
                let new_start = selection_start + 2;
                let new_end = selection_end + 2;
                (new_text, new_start, new_end)
            }
            MarkdownAction::UnorderedList => {
                let new_text = if selected.is_empty() {
                    format!("{}- {}", before, after)
                } else {
                    format!("{}- {}{}", before, selected, after)
                };
                let new_start = selection_start + 2;
                let new_end = selection_end + 2;
                (new_text, new_start, new_end)
            }
            MarkdownAction::OrderedList => {
                let new_text = if selected.is_empty() {
                    format!("{}1. {}", before, after)
                } else {
                    format!("{}1. {}{}", before, selected, after)
                };
                let new_start = selection_start + 3;
                let new_end = selection_end + 3;
                (new_text, new_start, new_end)
            }
            MarkdownAction::HorizontalRule => {
                let new_text = format!("{}---\n{}", before, after);
                let new_start = selection_start + 4;
                let new_end = selection_start + 4;
                (new_text, new_start, new_end)
            }
        }
    }
}
