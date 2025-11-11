use yew::prelude::*;

/// A component that renders markdown as HTML preview
#[derive(Properties, PartialEq, Clone)]
pub struct MarkdownPreviewProps {
    /// The markdown content to preview
    #[prop_or_default]
    pub markdown: String,

    /// Theme name (light or dark)
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,

    /// Custom CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Show or hide the preview
    #[prop_or(true)]
    pub visible: bool,
}

#[function_component(MarkdownPreview)]
pub fn markdown_preview(props: &MarkdownPreviewProps) -> Html {
    let theme_class = if props.theme == "dark" {
        "dark"
    } else {
        "light"
    };

    if !props.visible {
        return html! {};
    }

    let rendered_html = render_markdown(&props.markdown);

    html! {
        <>
            <MarkdownPreviewStyles />
            <div
                class={classes!(
                    props.class.clone(),
                    "markdown-preview",
                    "p-4",
                    "overflow-auto",
                    "bg-white",
                    "dark:bg-gray-900",
                    "text-gray-900",
                    "dark:text-gray-100",
                    "border",
                    "border-gray-300",
                    "dark:border-gray-700",
                    "rounded",
                    theme_class
                )}
            >
                { Html::from_html_unchecked(rendered_html.into()) }
            </div>
        </>
    }
}

/// Simple markdown to HTML renderer
fn render_markdown(markdown: &str) -> String {
    let mut html = String::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = Vec::<String>::new();
    let mut in_list = false;
    let mut list_items = Vec::<String>::new();

    for line in lines {
        // Code blocks
        if line.starts_with("```") {
            if in_code_block {
                // End code block
                html.push_str(&format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>\n",
                    code_block_lang,
                    escape_html(&code_block_content.join("\n"))
                ));
                code_block_content.clear();
                code_block_lang.clear();
                in_code_block = false;
            } else {
                // Start code block
                code_block_lang = line.trim_start_matches("```").trim().to_string();
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            code_block_content.push(line.to_string());
            continue;
        }

        // Close list if needed
        if !line.trim_start().starts_with('-')
            && !line.trim_start().starts_with('*')
            && !line.trim_start().starts_with('+')
            && !line.chars().next().map_or(false, |c| c.is_digit(10))
            && in_list
        {
            html.push_str("<ul>\n");
            for item in &list_items {
                html.push_str(&format!("<li>{}</li>\n", render_inline(item)));
            }
            html.push_str("</ul>\n");
            list_items.clear();
            in_list = false;
        }

        // Headers
        if line.starts_with('#') {
            let hash_count = line.chars().take_while(|c| *c == '#').count();
            if hash_count <= 6 {
                let content = line.trim_start_matches('#').trim();
                html.push_str(&format!(
                    "<h{} class=\"markdown-h{}\">{}</h{}>\n",
                    hash_count,
                    hash_count,
                    render_inline(content),
                    hash_count
                ));
                continue;
            }
        }

        // Horizontal rules
        if line.trim() == "---" || line.trim() == "***" || line.trim() == "___" {
            html.push_str("<hr class=\"markdown-hr\" />\n");
            continue;
        }

        // Lists
        if line.trim_start().starts_with("- ")
            || line.trim_start().starts_with("* ")
            || line.trim_start().starts_with("+ ")
        {
            in_list = true;
            let content = line
                .trim_start()
                .trim_start_matches('-')
                .trim_start_matches('*')
                .trim_start_matches('+')
                .trim();
            list_items.push(content.to_string());
            continue;
        }

        // Numbered lists
        if let Some(pos) = line.find(". ") {
            let prefix = &line[..pos];
            if prefix.chars().all(|c| c.is_digit(10)) {
                in_list = true;
                let content = line[pos + 2..].trim();
                list_items.push(content.to_string());
                continue;
            }
        }

        // Blockquotes
        if line.trim_start().starts_with("> ") {
            let content = line.trim_start().trim_start_matches('>').trim();
            html.push_str(&format!(
                "<blockquote class=\"markdown-blockquote\">{}</blockquote>\n",
                render_inline(content)
            ));
            continue;
        }

        // Paragraphs
        if !line.trim().is_empty() {
            html.push_str(&format!("<p>{}</p>\n", render_inline(line)));
        } else {
            html.push_str("<br/>\n");
        }
    }

    // Close any remaining list
    if in_list {
        html.push_str("<ul>\n");
        for item in &list_items {
            html.push_str(&format!("<li>{}</li>\n", render_inline(item)));
        }
        html.push_str("</ul>\n");
    }

    html
}

/// Render inline markdown elements (bold, italic, links, etc.)
fn render_inline(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    let mut buffer = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if chars.peek() == Some(&'*') {
                    // Bold
                    chars.next(); // consume second *
                    if !buffer.is_empty() {
                        result.push_str(&escape_html(&buffer));
                        buffer.clear();
                    }
                    let mut bold_text = String::new();
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'*') {
                            chars.next(); // consume closing *
                            break;
                        }
                        bold_text.push(c);
                    }
                    result.push_str(&format!("<strong>{}</strong>", escape_html(&bold_text)));
                } else {
                    // Italic
                    if !buffer.is_empty() {
                        result.push_str(&escape_html(&buffer));
                        buffer.clear();
                    }
                    let mut italic_text = String::new();
                    while let Some(c) = chars.next() {
                        if c == '*' {
                            break;
                        }
                        italic_text.push(c);
                    }
                    result.push_str(&format!("<em>{}</em>", escape_html(&italic_text)));
                }
            }
            '_' => {
                // Italic with underscore
                if !buffer.is_empty() {
                    result.push_str(&escape_html(&buffer));
                    buffer.clear();
                }
                let mut italic_text = String::new();
                while let Some(c) = chars.next() {
                    if c == '_' {
                        break;
                    }
                    italic_text.push(c);
                }
                result.push_str(&format!("<em>{}</em>", escape_html(&italic_text)));
            }
            '`' => {
                // Inline code
                if !buffer.is_empty() {
                    result.push_str(&escape_html(&buffer));
                    buffer.clear();
                }
                let mut code_text = String::new();
                while let Some(c) = chars.next() {
                    if c == '`' {
                        break;
                    }
                    code_text.push(c);
                }
                result.push_str(&format!("<code>{}</code>", escape_html(&code_text)));
            }
            '[' => {
                // Links
                if !buffer.is_empty() {
                    result.push_str(&escape_html(&buffer));
                    buffer.clear();
                }
                let mut link_text = String::new();
                while let Some(c) = chars.next() {
                    if c == ']' {
                        break;
                    }
                    link_text.push(c);
                }
                if chars.peek() == Some(&'(') {
                    chars.next(); // consume (
                    let mut url = String::new();
                    while let Some(c) = chars.next() {
                        if c == ')' {
                            break;
                        }
                        url.push(c);
                    }
                    result.push_str(&format!(
                        "<a href=\"{}\" class=\"markdown-link\">{}</a>",
                        escape_html(&url),
                        escape_html(&link_text)
                    ));
                } else {
                    result.push_str(&format!("[{}]", escape_html(&link_text)));
                }
            }
            '!' => {
                // Images
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume [
                    if !buffer.is_empty() {
                        result.push_str(&escape_html(&buffer));
                        buffer.clear();
                    }
                    let mut alt_text = String::new();
                    while let Some(c) = chars.next() {
                        if c == ']' {
                            break;
                        }
                        alt_text.push(c);
                    }
                    if chars.peek() == Some(&'(') {
                        chars.next(); // consume (
                        let mut url = String::new();
                        while let Some(c) = chars.next() {
                            if c == ')' {
                                break;
                            }
                            url.push(c);
                        }
                        result.push_str(&format!(
                            "<img src=\"{}\" alt=\"{}\" class=\"markdown-image\" />",
                            escape_html(&url),
                            escape_html(&alt_text)
                        ));
                    } else {
                        result.push_str(&format!("![{}]", escape_html(&alt_text)));
                    }
                } else {
                    buffer.push(ch);
                }
            }
            '~' => {
                // Strikethrough
                if chars.peek() == Some(&'~') {
                    chars.next(); // consume second ~
                    if !buffer.is_empty() {
                        result.push_str(&escape_html(&buffer));
                        buffer.clear();
                    }
                    let mut strike_text = String::new();
                    while let Some(c) = chars.next() {
                        if c == '~' && chars.peek() == Some(&'~') {
                            chars.next(); // consume closing ~
                            break;
                        }
                        strike_text.push(c);
                    }
                    result.push_str(&format!("<del>{}</del>", escape_html(&strike_text)));
                } else {
                    buffer.push(ch);
                }
            }
            _ => {
                buffer.push(ch);
            }
        }
    }

    if !buffer.is_empty() {
        result.push_str(&escape_html(&buffer));
    }

    result
}

/// Escape HTML special characters
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Styles for the markdown preview
#[function_component(MarkdownPreviewStyles)]
fn markdown_preview_styles() -> Html {
    html! {
        <style>
            {r#"
            .markdown-preview {
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
                line-height: 1.6;
            }

            .markdown-preview h1,
            .markdown-preview .markdown-h1 {
                @apply text-3xl font-bold mb-4 mt-6;
            }

            .markdown-preview h2,
            .markdown-preview .markdown-h2 {
                @apply text-2xl font-bold mb-3 mt-5;
            }

            .markdown-preview h3,
            .markdown-preview .markdown-h3 {
                @apply text-xl font-bold mb-2 mt-4;
            }

            .markdown-preview h4,
            .markdown-preview .markdown-h4 {
                @apply text-lg font-bold mb-2 mt-3;
            }

            .markdown-preview h5,
            .markdown-preview .markdown-h5 {
                @apply text-base font-bold mb-2 mt-2;
            }

            .markdown-preview h6,
            .markdown-preview .markdown-h6 {
                @apply text-sm font-bold mb-2 mt-2;
            }

            .markdown-preview p {
                @apply mb-4;
            }

            .markdown-preview ul,
            .markdown-preview ol {
                @apply mb-4 ml-6;
            }

            .markdown-preview ul {
                @apply list-disc;
            }

            .markdown-preview ol {
                @apply list-decimal;
            }

            .markdown-preview li {
                @apply mb-1;
            }

            .markdown-preview code {
                @apply bg-gray-100 dark:bg-gray-800 text-pink-600 dark:text-pink-400 px-1 py-0.5 rounded font-mono text-sm;
            }

            .markdown-preview pre {
                @apply bg-gray-100 dark:bg-gray-800 p-4 rounded mb-4 overflow-x-auto;
            }

            .markdown-preview pre code {
                @apply bg-transparent p-0 text-gray-900 dark:text-gray-100;
            }

            .markdown-preview .markdown-blockquote,
            .markdown-preview blockquote {
                @apply border-l-4 border-gray-300 dark:border-gray-700 pl-4 italic text-gray-600 dark:text-gray-400 mb-4;
            }

            .markdown-preview .markdown-hr,
            .markdown-preview hr {
                @apply border-t border-gray-300 dark:border-gray-700 my-6;
            }

            .markdown-preview .markdown-link,
            .markdown-preview a {
                @apply text-blue-600 dark:text-blue-400 underline hover:text-blue-800 dark:hover:text-blue-300;
            }

            .markdown-preview .markdown-image,
            .markdown-preview img {
                @apply max-w-full h-auto rounded shadow-md my-4;
            }

            .markdown-preview strong {
                @apply font-bold;
            }

            .markdown-preview em {
                @apply italic;
            }

            .markdown-preview del {
                @apply line-through;
            }
            "#}
        </style>
    }
}
