use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::prelude::*;
use wonopui::*;
use yew::prelude::*;

#[function_component(BasicMarkdownEditorDemo)]
pub fn basic_markdown_editor_demo() -> Html {
    let value = use_state(|| r#"# Welcome to the Markdown Editor!

This is a **powerful** markdown editor with *live preview*.

## Features

- **Bold**, *italic*, and ~~strikethrough~~ text
- Headers (H1-H6)
- Lists (ordered and unordered)
- Links and images
- Code blocks with syntax highlighting
- Tables
- Task lists
- And much more!

Try editing this text to see the live preview update!

## Code Example

```rust
fn main() {
    println!("Hello, World!");
}
```

## Task List

- [x] Create markdown editor
- [x] Add live preview
- [ ] Add more features
"#.to_string());

    let on_change = {
        let value = value.clone();
        Callback::from(move |new_value: String| {
            value.set(new_value);
        })
    };

    html! {
        <div class="w-full">
            <MarkdownEditor
                value={(*value).clone()}
                {on_change}
                placeholder="Write your markdown here..."
                show_toolbar={true}
                show_mode_switcher={true}
                initial_mode={EditorMode::Split}
                min_height={500}
                show_stats={true}
            />
        </div>
    }
}

#[function_component(EditOnlyModeDemo)]
pub fn edit_only_mode_demo() -> Html {
    let value = use_state(|| r#"# Edit Mode Only

This editor is in edit-only mode."#.to_string());

    let on_change = {
        let value = value.clone();
        Callback::from(move |new_value: String| {
            value.set(new_value);
        })
    };

    html! {
        <MarkdownEditor
            value={(*value).clone()}
            {on_change}
            initial_mode={EditorMode::Edit}
            show_mode_switcher={false}
            min_height={300}
        />
    }
}

#[function_component(PreviewOnlyModeDemo)]
pub fn preview_only_mode_demo() -> Html {
    let value = r#"# Preview Mode Only

This editor is in preview-only mode, showing **rendered** markdown.

## Features Demonstrated

- Beautiful typography
- Proper heading hierarchy
- **Bold** and *italic* text
- `Inline code`

```rust
// Code blocks with syntax highlighting
let x = 42;
```

> Blockquotes work too!

1. Ordered lists
2. Are supported
3. Automatically

- Unordered lists
- Work great
- As well
"#.to_string();

    html! {
        <MarkdownEditor
            value={value}
            initial_mode={EditorMode::Preview}
            show_toolbar={false}
            show_mode_switcher={false}
            min_height={300}
        />
    }
}

#[function_component(WithAttachmentsDemo)]
pub fn with_attachments_demo() -> Html {
    let value = use_state(|| r#"# Document with Attachments

This editor supports file attachments!"#.to_string());
    let attachments = use_state(|| vec![
        FileAttachment {
            id: "1".to_string(),
            name: "document.pdf".to_string(),
            size: 1024 * 256, // 256 KB
            mime_type: "application/pdf".to_string(),
            url: Some("https://example.com/document.pdf".to_string()),
        },
        FileAttachment {
            id: "2".to_string(),
            name: "image.png".to_string(),
            size: 1024 * 512, // 512 KB
            mime_type: "image/png".to_string(),
            url: Some("https://example.com/image.png".to_string()),
        },
    ]);

    let on_change = {
        let value = value.clone();
        Callback::from(move |new_value: String| {
            value.set(new_value);
        })
    };

    let on_attach = {
        let attachments = attachments.clone();
        Callback::from(move |new_files: Vec<FileAttachment>| {
            let mut current = (*attachments).clone();
            current.extend(new_files);
            attachments.set(current);
        })
    };

    html! {
        <MarkdownEditor
            value={(*value).clone()}
            {on_change}
            enable_attachments={true}
            attachments={(*attachments).clone()}
            {on_attach}
            min_height={400}
        />
    }
}

#[function_component(WithMentionsDemo)]
pub fn with_mentions_demo() -> Html {
    let value = use_state(|| r#"# Collaborative Document

Type @ to mention team members!"#.to_string());
    let mention_options = vec![
        ("user1".to_string(), "Alice Johnson".to_string()),
        ("user2".to_string(), "Bob Smith".to_string()),
        ("user3".to_string(), "Charlie Brown".to_string()),
        ("team1".to_string(), "Engineering Team".to_string()),
        ("team2".to_string(), "Design Team".to_string()),
    ];

    let on_change = {
        let value = value.clone();
        Callback::from(move |new_value: String| {
            value.set(new_value);
        })
    };

    let on_mention = Callback::from(move |mentioned: String| {
        log::info!("Mentioned: {}", mentioned);
    });

    html! {
        <MarkdownEditor
            value={(*value).clone()}
            {on_change}
            enable_mentions={true}
            {mention_options}
            {on_mention}
            min_height={400}
        />
    }
}

#[function_component(CustomHeightDemo)]
pub fn custom_height_demo() -> Html {
    let value = use_state(|| r#"# Compact Editor

This editor has custom height constraints."#.to_string());

    let on_change = {
        let value = value.clone();
        Callback::from(move |new_value: String| {
            value.set(new_value);
        })
    };

    html! {
        <MarkdownEditor
            value={(*value).clone()}
            {on_change}
            min_height={200}
            max_height={400}
        />
    }
}

#[function_component(DisabledDemo)]
pub fn disabled_demo() -> Html {
    let value = r#"# Disabled Editor

This editor is in read-only mode."#.to_string();

    html! {
        <MarkdownEditor
            {value}
            disabled={true}
            min_height={200}
        />
    }
}

#[function_component(MarkdownEditorDocumentation)]
pub fn markdown_editor_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen pb-12">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Markdown Editor Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "A feature-rich, best-in-class markdown editor with live preview, formatting toolbar, file attachments, and collaborative features. Built for performance and usability." }
            </p>

            // Features Section
            <Features
                features={vec![
                    "MarkdownEditor",
                ]}
            />

            // Basic Example
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "Basic Example" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "The default configuration with split-screen mode, toolbar, and statistics." }
            </p>
            <ExampleCode
                preview={html! { <BasicMarkdownEditorDemo /> }}
                code={r##"
use wonopui::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| r#"# Hello World

Write **markdown** here!"#.to_string());

    let on_change = {
        let value = value.clone();
        Callback::from(move |new_value: String| {
            value.set(new_value);
        })
    };

    html! {
        <MarkdownEditor
            value={(*value).clone()}
            {on_change}
            placeholder="Write your markdown here..."
            show_toolbar={true}
            show_mode_switcher={true}
            initial_mode={EditorMode::Split}
            min_height={500}
            show_stats={true}
        />
    }
}
                "##}
            />

            // Edit Only Mode
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "Edit-Only Mode" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Use the editor in edit-only mode without the mode switcher." }
            </p>
            <ExampleCode
                preview={html! { <EditOnlyModeDemo /> }}
                code={r#"
<MarkdownEditor
    value={(*value).clone()}
    on_change={on_change}
    initial_mode={EditorMode::Edit}
    show_mode_switcher={false}
    min_height={300}
/>
                "#}
            />

            // Preview Only Mode
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "Preview-Only Mode" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Display rendered markdown without editing capabilities." }
            </p>
            <ExampleCode
                preview={html! { <PreviewOnlyModeDemo /> }}
                code={r#"
<MarkdownEditor
    value={markdown_content}
    initial_mode={EditorMode::Preview}
    show_toolbar={false}
    show_mode_switcher={false}
    min_height={300}
/>
                "#}
            />

            // With Attachments
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "With File Attachments" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Enable file attachment support for documents, images, and more." }
            </p>
            <ExampleCode
                preview={html! { <WithAttachmentsDemo /> }}
                code={r#"
let attachments = use_state(|| vec![
    FileAttachment {
        id: "1".to_string(),
        name: "document.pdf".to_string(),
        size: 1024 * 256,
        mime_type: "application/pdf".to_string(),
        url: Some("https://example.com/document.pdf".to_string()),
    },
]);

let on_attach = {
    let attachments = attachments.clone();
    Callback::from(move |new_files: Vec<FileAttachment>| {
        let mut current = (*attachments).clone();
        current.extend(new_files);
        attachments.set(current);
    })
};

html! {
    <MarkdownEditor
        value={(*value).clone()}
        on_change={on_change}
        enable_attachments={true}
        attachments={(*attachments).clone()}
        {on_attach}
    />
}
                "#}
            />

            // With Mentions
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "With User/Role Mentions" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Enable mentions for collaborative editing with autocomplete." }
            </p>
            <ExampleCode
                preview={html! { <WithMentionsDemo /> }}
                code={r#"
let mention_options = vec![
    ("user1".to_string(), "Alice Johnson".to_string()),
    ("user2".to_string(), "Bob Smith".to_string()),
    ("team1".to_string(), "Engineering Team".to_string()),
];

let on_mention = Callback::from(|mentioned: String| {
    log::info!("Mentioned: {}", mentioned);
});

html! {
    <MarkdownEditor
        value={(*value).clone()}
        on_change={on_change}
        enable_mentions={true}
        {mention_options}
        {on_mention}
    />
}
                "#}
            />

            // Custom Height
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "Custom Height Constraints" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Set minimum and maximum height constraints for the editor." }
            </p>
            <ExampleCode
                preview={html! { <CustomHeightDemo /> }}
                code={r#"
<MarkdownEditor
    value={(*value).clone()}
    on_change={on_change}
    min_height={200}
    max_height={400}
/>
                "#}
            />

            // Disabled State
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">
                { "Disabled/Read-Only State" }
            </h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Display markdown in a read-only state." }
            </p>
            <ExampleCode
                preview={html! { <DisabledDemo /> }}
                code={r#"
<MarkdownEditor
    value={value}
    disabled={true}
    min_height={200}
/>
                "#}
            />

            // API Documentation
            <ApiSection
                title="MarkdownEditor API"
                description="Complete list of properties for the MarkdownEditor component."
                props={vec![
                    ("value", "String", "The markdown content to display and edit. Default: \"\""),
                    ("on_change", "Callback<String>", "Callback fired when the content changes."),
                    ("placeholder", "String", "Placeholder text for the editor. Default: \"Write your markdown here...\""),
                    ("disabled", "bool", "Whether the editor is in read-only mode. Default: false"),
                    ("show_toolbar", "bool", "Show the formatting toolbar. Default: true"),
                    ("show_mode_switcher", "bool", "Show the Edit/Preview/Split mode switcher. Default: true"),
                    ("initial_mode", "EditorMode", "Initial viewing mode (Edit, Preview, or Split). Default: EditorMode::Split"),
                    ("enable_attachments", "bool", "Enable file attachment functionality. Default: false"),
                    ("on_attach", "Callback<Vec<FileAttachment>>", "Callback fired when files are attached."),
                    ("attachments", "Vec<FileAttachment>", "Current file attachments to display. Default: []"),
                    ("enable_mentions", "bool", "Enable user/role mention functionality. Default: false"),
                    ("mention_options", "Vec<(String, String)>", "Available users/roles for mentions (id, name). Default: []"),
                    ("on_mention", "Callback<String>", "Callback fired when a user/role is mentioned."),
                    ("min_height", "u32", "Minimum height of the editor in pixels. Default: 300"),
                    ("max_height", "u32", "Maximum height of the editor (0 = no limit). Default: 0"),
                    ("class", "Classes", "Custom CSS classes to apply."),
                    ("autosave_interval", "u32", "Auto-save interval in milliseconds (0 = disabled). Default: 0"),
                    ("on_autosave", "Callback<String>", "Callback for auto-save functionality."),
                    ("enable_shortcuts", "bool", "Enable keyboard shortcuts. Default: true"),
                    ("preview_css", "String", "Custom CSS for the preview pane. Default: \"\""),
                    ("show_stats", "bool", "Show word/character/line count statistics. Default: true"),
                ]}
            />

            // Keyboard Shortcuts
            <NotesSection
                title="Keyboard Shortcuts"
                notes={vec![
                    "**Ctrl+B** - Bold text".to_string(),
                    "**Ctrl+I** - Italic text".to_string(),
                    "**Ctrl+K** - Insert link".to_string(),
                    "**Ctrl+`** - Inline code".to_string(),
                    "**Ctrl+Shift+X** - Strikethrough".to_string(),
                    "**Ctrl+Shift+I** - Insert image".to_string(),
                    "**Ctrl+Shift+C** - Code block".to_string(),
                    "**Ctrl+Shift+U** - Unordered list".to_string(),
                    "**Ctrl+Shift+O** - Ordered list".to_string(),
                    "**Ctrl+Shift+T** - Task list".to_string(),
                    "**Ctrl+Shift+Q** - Quote".to_string(),
                    "**Ctrl+Shift+H** - Horizontal rule".to_string(),
                    "**Ctrl+1/2/3** - Heading level 1/2/3".to_string(),
                ]}
            />

            // Usage Notes
            <NotesSection
                title="Usage Notes"
                notes={vec![
                    "The editor uses **pulldown-cmark** for markdown parsing, supporting CommonMark and GitHub-flavored markdown.".to_string(),
                    "Split mode provides real-time preview as you type.".to_string(),
                    "File attachments require implementing the upload logic in the `on_attach` callback.".to_string(),
                    "Mentions support requires providing a list of available users/roles.".to_string(),
                    "The editor is fully accessible with ARIA labels and keyboard navigation.".to_string(),
                    "Auto-save functionality can be enabled with a custom interval.".to_string(),
                    "Statistics are updated in real-time as you type.".to_string(),
                    "The preview pane can be styled with custom CSS via `preview_css` prop.".to_string(),
                ]}
            />
        </Container>
    }
}
