use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::*;
use wonopui::code_editor::{DiffView, DiffViewMode};
use yew::prelude::*;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use wasm_bindgen::JsCast;

use super::diffview_languages::LanguageShowcase;
use super::diffview_examples::UseCaseExamples;
use super::diffview_playground::DiffViewPlayground;

#[function_component(DiffViewDocumentation)]
pub fn diffview_documentation() -> Html {
    let old_code = r#"use std::collections::HashMap;

// Simple calculator functions
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_difference(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let result = calculate_sum(5, 3);
    println!("Sum: {}", result);
}"#;

    let new_code = r#"use std::collections::HashMap;
use std::fmt::Debug;

// Calculator module with enhanced functionality
fn calculate_sum<T: std::ops::Add<Output = T> + Debug>(a: T, b: T) -> T {
    // Add logging for debugging
    println!("Calculating sum of {:?} and {:?}", a, b);
    a + b
}

fn calculate_product(a: i32, b: i32) -> i32 {
    println!("Calculating {} × {}", a, b);
    a * b
}

fn calculate_difference(a: i32, b: i32) -> i32 {
    println!("Calculating {} - {}", a, b);
    a - b
}

fn main() {
    let sum_result = calculate_sum(5, 3);
    let product_result = calculate_product(5, 3);
    let diff_result = calculate_difference(5, 3);
    
    println!("Sum: {}", sum_result);
    println!("Product: {}", product_result);
    println!("Difference: {}", diff_result);
}"#;

    let mode = use_state(|| DiffViewMode::SideBySide);
    let show_line_numbers = use_state(|| true);
    let unified_diff = use_state(|| false);
    let context_lines = use_state(|| 3usize);

    let mode_clone = mode.clone();
    let on_mode_change = Callback::from(move |e: Event| {
        let target = e.target();
        if let Some(select) = target.and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
            let value = select.value();
            mode_clone.set(match value.as_str() {
                "inline" => DiffViewMode::Inline,
                _ => DiffViewMode::SideBySide,
            });
        }
    });

    let show_line_numbers_clone = show_line_numbers.clone();
    let on_line_numbers_change = Callback::from(move |_| {
        show_line_numbers_clone.set(!*show_line_numbers_clone);
    });

    let unified_diff_clone = unified_diff.clone();
    let on_unified_diff_change = Callback::from(move |_| {
        unified_diff_clone.set(!*unified_diff_clone);
    });

    let context_lines_clone = context_lines.clone();
    let on_context_lines_change = Callback::from(move |e: Event| {
        let target = e.target();
        if let Some(input) = target.and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            if let Ok(value) = input.value().parse::<usize>() {
                context_lines_clone.set(value);
            }
        }
    });

    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "DiffView Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The DiffView component computes and displays differences between two text strings with syntax highlighting. It supports both side-by-side and inline view modes." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "🎯 Interactive Playground" }</h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Try the DiffView component with your own text and customize all options:" }
            </p>
            <DiffViewPlayground />
            
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "📚 Use Case Examples" }</h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "Real-world examples demonstrating common diff scenarios:" }
            </p>
            <UseCaseExamples />
            
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "Interactive Example" }</h2>
            
            // Controls
            <div class="mb-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-md space-y-4" role="region" aria-label="Diff viewer controls">
                <div class="flex flex-wrap gap-4">
                    <div>
                        <label for="view-mode-select" class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">
                            {"View Mode"}
                        </label>
                        <select 
                            id="view-mode-select"
                            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-sm bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
                            onchange={on_mode_change}
                            aria-label="Select diff view mode"
                        >
                            <option value="side-by-side">{"Side by Side"}</option>
                            <option value="inline">{"Inline"}</option>
                        </select>
                    </div>
                    
                    <div>
                        <label for="context-lines-input" class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">
                            {"Context Lines"}
                        </label>
                        <input 
                            id="context-lines-input"
                            type="number" 
                            min="0" 
                            max="10" 
                            value={context_lines.to_string()}
                            class="px-3 py-2 w-20 border border-gray-300 dark:border-gray-600 rounded-sm bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
                            onchange={on_context_lines_change}
                            aria-label="Number of context lines to show"
                            aria-describedby="context-lines-desc"
                        />
                    </div>
                    
                    <div class="flex items-end">
                        <label class="flex items-center" for="line-numbers-checkbox">
                            <input 
                                id="line-numbers-checkbox"
                                type="checkbox" 
                                checked={*show_line_numbers}
                                onchange={on_line_numbers_change}
                                class="mr-2"
                                aria-label="Toggle line numbers display"
                            />
                            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                {"Show Line Numbers"}
                            </span>
                        </label>
                    </div>
                    
                    <div class="flex items-end">
                        <label class="flex items-center" for="unified-diff-checkbox">
                            <input 
                                id="unified-diff-checkbox"
                                type="checkbox" 
                                checked={*unified_diff}
                                onchange={on_unified_diff_change}
                                class="mr-2"
                                aria-label="Toggle unified diff headers"
                                aria-describedby="unified-diff-desc"
                            />
                            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                {"Show Unified Diff Header"}
                            </span>
                        </label>
                    </div>
                </div>
            </div>

            // Live DiffView
            <div class="mb-8">
                <DiffView
                    old_text={old_code}
                    new_text={new_code}
                    mode={(*mode).clone()}
                    language="rust"
                    show_line_numbers={*show_line_numbers}
                    unified_diff={*unified_diff}
                    context_lines={*context_lines}
                    font_size={14}
                    line_height={1.6}
                    class="shadow-md"
                />
            </div>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Basic Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="space-y-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">{"Side-by-Side View"}</h3>
                            <DiffView
                                old_text={"fn hello() {\n    println!(\"Hello\");\n}"}
                                new_text={"fn hello() {\n    println!(\"Hello, World!\");\n    // Added comment\n}"}
                                mode={DiffViewMode::SideBySide}
                                language="rust"
                                show_line_numbers=true
                            />
                        </div>
                    </div>
                }}
                code={r#"use wonopui::code_editor::{DiffView, DiffViewMode};

#[function_component(App)]
fn app() -> Html {
    let old_text = "fn hello() {\n    println!(\"Hello\");\n}";
    let new_text = "fn hello() {\n    println!(\"Hello, World!\");\n    // Added comment\n}";
    
    html! {
        <DiffView
            old_text={old_text}
            new_text={new_text}
            mode={DiffViewMode::SideBySide}
            language="rust"
            show_line_numbers=true
        />
    }
}"#}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Multiple Languages Support" }</h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { "DiffView supports syntax highlighting for many programming languages using syntect:" }
            </p>
            <LanguageShowcase />
            
            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "Inline View Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="space-y-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">{"Inline Diff View"}</h3>
                            <DiffView
                                old_text={"const CONFIG = {\n    debug: false,\n    port: 3000\n};"}
                                new_text={"const CONFIG = {\n    debug: true,\n    port: 8080,\n    host: 'localhost'\n};"}
                                mode={DiffViewMode::Inline}
                                language="javascript"
                                show_line_numbers=true
                                unified_diff=true
                            />
                        </div>
                    </div>
                }}
                code={r#"use wonopui::code_editor::{DiffView, DiffViewMode};

#[function_component(App)]
fn app() -> Html {
    let old_text = "const CONFIG = {\n    debug: false,\n    port: 3000\n};";
    let new_text = "const CONFIG = {\n    debug: true,\n    port: 8080,\n    host: 'localhost'\n};";
    
    html! {
        <DiffView
            old_text={old_text}
            new_text={new_text}
            mode={DiffViewMode::Inline}
            language="javascript"
            show_line_numbers=true
            unified_diff=true
        />
    }
}"#}
            />

            <ApiSection
                title="DiffView"
                description="Props for the DiffView component."
                props={vec![
                    ("old_text", "String", "The original text to compare. Required prop that contains the base text for diff computation."),
                    ("new_text", "String", "The modified text to compare against. Required prop that will be diffed against old_text."),
                    ("mode", "DiffViewMode", "Display mode - SideBySide or Inline. SideBySide shows changes in two columns, Inline shows changes in a single column. Default: SideBySide"),
                    ("show_line_numbers", "bool", "Whether to show line numbers for both old and new text. Helps with code review and navigation. Default: true"),
                    ("context_lines", "usize", "Number of unchanged lines to show around changes. Lower values focus on changes, higher values provide more context. Range: 0-10. Default: 3"),
                    ("language", "String", "Programming language for syntax highlighting. Supports 25+ languages including rust, javascript, python, go, java. Default: \"rust\""),
                    ("theme", "String", "Color theme for syntax highlighting. Options: \"auto\" (follows system), \"light\", \"dark\". Default: \"auto\""),
                    ("unified_diff", "bool", "Show unified diff headers (@@ -1,3 +1,5 @@) that indicate line ranges. Useful for patch generation. Default: false"),
                    ("font_size", "u8", "Font size in pixels. Adjustable for readability. Range: 10-20. Default: 14"),
                    ("font_family", "String", "Font family for the diff view. Monospace fonts recommended. Default: \"JetBrains Mono, monospace\""),
                    ("line_height", "f32", "Line height multiplier for text spacing. Range: 1.0-2.0. Default: 1.5"),
                    ("word_diff", "bool", "Enable word-level diff highlighting within changed lines. Coming soon. Default: false"),
                    ("ignore_whitespace", "bool", "Ignore whitespace changes in diff computation. Coming soon. Default: false"),
                    ("collapsible_unchanged", "bool", "Allow collapsing of large unchanged sections. Coming soon. Default: false"),
                    ("on_line_click", "Callback<DiffLineInfo>", "Optional callback fired when a diff line is clicked. Receives line information including number, type, and content."),
                    ("class", "Classes", "Additional CSS classes to apply to the container. Useful for custom styling and theming."),
                ]}
            />

            <ApiSection
                title="DiffViewMode"
                description="Enum for display mode selection."
                props={vec![
                    ("SideBySide", "", "Displays old and new text in separate columns. Best for comparing structure and seeing changes in context."),
                    ("Inline", "", "Shows changes in a single column with additions and removals interleaved. Best for reviewing sequential changes."),
                ]}
            />
            
            <ApiSection
                title="DiffLineInfo"
                description="Information passed to the on_line_click callback."
                props={vec![
                    ("line_number", "Option<usize>", "Line number in the original or new text, depending on the change type."),
                    ("change_type", "ChangeType", "Type of change: Added, Removed, or Unchanged."),
                    ("content", "String", "The text content of the line."),
                    ("side", "DiffSide", "Which side of the diff: Old or New (only relevant in side-by-side mode)."),
                ]}
            />

            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "✨ Features" }</h2>
            <div class="grid md:grid-cols-2 gap-6">
                <div>
                    <h3 class="font-semibold text-zinc-800 dark:text-zinc-200 mb-2">{ "Core Features" }</h3>
                    <ul class="list-disc list-inside space-y-2 text-zinc-600 dark:text-zinc-400">
                <li>{"Computes differences using the Myers diff algorithm"}</li>
                <li>{"Side-by-side and inline view modes"}</li>
                <li>{"Syntax highlighting for multiple languages"}</li>
                <li>{"Configurable context lines"}</li>
                <li>{"Line number display"}</li>
                <li>{"Dark mode support"}</li>
                        <li>{"Responsive layout"}</li>
                    </ul>
                </div>
                <div>
                    <h3 class="font-semibold text-zinc-800 dark:text-zinc-200 mb-2">{ "Advanced Features" }</h3>
                    <ul class="list-disc list-inside space-y-2 text-zinc-600 dark:text-zinc-400">
                        <li>{"25+ supported programming languages"}</li>
                        <li>{"Multiple theme options"}</li>
                        <li>{"Word-level diff support (coming soon)"}</li>
                        <li>{"Collapsible unchanged sections (coming soon)"}</li>
                        <li>{"Export diff as unified format (coming soon)"}</li>
                        <li>{"Keyboard navigation support"}</li>
                    </ul>
                </div>
            </div>

            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "📝 Usage Notes" }</h2>
            <div class="bg-amber-50 dark:bg-amber-950 border border-amber-200 dark:border-amber-800 rounded-md p-4 space-y-2">
                <p>{ "The DiffView component uses the 'similar' crate to compute text differences efficiently." }</p>
                <p>{ "Large files may take a moment to process - consider implementing virtual scrolling for very large diffs." }</p>
                <p>{ "The component recomputes diffs when the input texts or context lines change." }</p>
            </div>

            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "🎨 Styling & Customization" }</h2>
            <div class="bg-purple-50 dark:bg-purple-950 border border-purple-200 dark:border-purple-800 rounded-md p-4">
                <p>{ "The DiffView component uses Tailwind classes for styling. Key classes include:" }</p>
                <ul class="list-disc list-inside space-y-2 mt-2">
                    <li><code>{"bg-emerald-50"}</code> { " - Added lines background" }</li>
                    <li><code>{"bg-rose-50"}</code> { " - Removed lines background" }</li>
                    <li><code>{"bg-amber-50"}</code> { " - Modified lines background" }</li>
                    <li><code>{"text-indigo-600"}</code> { " - Keyword highlighting" }</li>
                </ul>
            </div>
        </Container>
    }
}