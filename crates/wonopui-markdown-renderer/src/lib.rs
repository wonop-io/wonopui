//! Markdown renderer component for WonopUI.
//!
//! Renders markdown content to HTML with styling.

use pulldown_cmark::{html, Options, Parser};
use wonopui_core::*;

/// Default CSS classes for markdown content styling.
pub mod classes {
    /// Container for markdown content with prose styling.
    pub const CONTAINER: &str = "prose prose-zinc dark:prose-invert max-w-none";

    /// Additional prose styles for better readability.
    pub const PROSE_STYLES: &str = concat!(
        "prose-headings:font-semibold ",
        "prose-h1:text-2xl prose-h2:text-xl prose-h3:text-lg ",
        "prose-p:my-2 prose-a:text-blue-600 dark:prose-a:text-blue-400 ",
        "prose-code:text-sm prose-code:bg-zinc-100 dark:prose-code:bg-zinc-800 ",
        "prose-code:px-1 prose-code:py-0.5 prose-code:rounded ",
        "prose-pre:bg-zinc-100 dark:prose-pre:bg-zinc-800 ",
        "prose-pre:p-4 prose-pre:rounded-lg prose-pre:overflow-x-auto ",
        "prose-blockquote:border-l-4 prose-blockquote:border-zinc-300 ",
        "dark:prose-blockquote:border-zinc-600 prose-blockquote:pl-4 ",
        "prose-ul:list-disc prose-ol:list-decimal prose-li:my-1"
    );
}

/// Properties for the MarkdownRenderer component.
#[derive(Properties, PartialEq)]
pub struct MarkdownRendererProps {
    /// The markdown content to render.
    pub content: String,

    /// Whether to enable tables support.
    #[prop_or(true)]
    pub tables: bool,

    /// Whether to enable strikethrough support.
    #[prop_or(true)]
    pub strikethrough: bool,

    /// Whether to enable task lists support.
    #[prop_or(true)]
    pub tasklists: bool,

    /// Whether to enable smart punctuation.
    #[prop_or(false)]
    pub smart_punctuation: bool,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Renders markdown content to styled HTML.
///
/// # Example
///
/// ```rust
/// use wonopui_markdown_renderer::MarkdownRenderer;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let markdown = "# Hello World\n\nThis is **bold** and *italic*.";
///     html! {
///         <MarkdownRenderer content={markdown.to_string()} />
///     }
/// }
/// ```
#[function_component(MarkdownRenderer)]
pub fn markdown_renderer(props: &MarkdownRendererProps) -> Html {
    let html_output = use_memo(props.content.clone(), |content| {
        let mut options = Options::empty();

        if props.tables {
            options.insert(Options::ENABLE_TABLES);
        }
        if props.strikethrough {
            options.insert(Options::ENABLE_STRIKETHROUGH);
        }
        if props.tasklists {
            options.insert(Options::ENABLE_TASKLISTS);
        }
        if props.smart_punctuation {
            options.insert(Options::ENABLE_SMART_PUNCTUATION);
        }

        let parser = Parser::new_ext(content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    });

    html! {
        <div
            class={classes!(classes::CONTAINER, classes::PROSE_STYLES, props.class.clone())}
        >
            { Html::from_html_unchecked(AttrValue::from((*html_output).clone())) }
        </div>
    }
}

/// Convert markdown string to HTML string.
///
/// This is a utility function for cases where you need the raw HTML.
pub fn markdown_to_html(content: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
