use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(MarkdownRendererDocumentation)]
pub fn markdown_renderer_documentation() -> Html {
    let sample_markdown = r#"# Hello World

This is **bold** and *italic* text.

## Features

- Item one
- Item two
- Item three

> This is a blockquote
"#;

    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "MarkdownRenderer Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "Renders markdown content to styled HTML. Supports common markdown features including headers, lists, code blocks, and more." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="border rounded-lg p-4 bg-zinc-50 dark:bg-zinc-800">
                        <MarkdownRenderer content={sample_markdown.to_string()} />
                    </div>
                }}
                code={r##"let markdown = r#"# Hello

This is **bold** text."#;

<MarkdownRenderer content={markdown.to_string()} />"##.to_string()}
            />

            <Features features={vec![
                "Full CommonMark support",
                "Tables support (enabled by default)",
                "Strikethrough support",
                "Task lists support",
                "Smart punctuation (optional)",
                "Prose styling with Tailwind"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="MarkdownRenderer"
                description="Props for the MarkdownRenderer component."
                props={vec![
                    ("content", "String", "The markdown content to render"),
                    ("tables", "bool", "Enable tables support. Default: true"),
                    ("strikethrough", "bool", "Enable strikethrough support. Default: true"),
                    ("tasklists", "bool", "Enable task lists support. Default: true"),
                    ("smart_punctuation", "bool", "Enable smart punctuation. Default: false"),
                    ("class", "Classes", "Additional CSS classes"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Uses pulldown-cmark for markdown parsing.".to_string(),
                    "Styling uses Tailwind prose classes for consistent typography.".to_string(),
                    "For syntax highlighting in code blocks, additional setup may be required.".to_string(),
                ]}
            />
        </Container>
    }
}
