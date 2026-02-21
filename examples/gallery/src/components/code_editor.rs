use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use gloo_console as console;
use std::collections::HashMap;
use web_sys::KeyboardEvent;
use wonopui::prelude::*;
use yew::prelude::*;

#[function_component(CodeEditorThemeEditor)]
pub fn code_editor_theme_editor() -> Html {
    // Using Tailwind CSS classes for styling
    let fields = vec![
        ("bg-white dark:bg-gray-900".to_string(), "Editor Background".to_string()),
        (
            "text-gray-900 dark:text-gray-100".to_string(),
            "Editor Text".to_string(),
        ),
        (
            "bg-gray-100 dark:bg-gray-800".to_string(),
            "Line Numbers Background".to_string(),
        ),
        (
            "text-gray-500 dark:text-gray-400".to_string(),
            "Line Numbers Text".to_string(),
        ),
        (
            "bg-blue-500".to_string(),
            "Cursor".to_string(),
        ),
        (
            "bg-blue-200 dark:bg-blue-800/30".to_string(),
            "Selection".to_string(),
        ),
        (
            "bg-green-200 dark:bg-green-900/30".to_string(),
            "Added Diff".to_string(),
        ),
        (
            "bg-red-200 dark:bg-red-900/30".to_string(),
            "Removed Diff".to_string(),
        ),
        (
            "bg-yellow-200 dark:bg-yellow-900/30".to_string(),
            "Modified Diff".to_string(),
        ),
        (
            "border-b-2 border-red-500".to_string(),
            "Error Annotation".to_string(),
        ),
        (
            "border-b-2 border-yellow-500".to_string(),
            "Warning Annotation".to_string(),
        ),
        (
            "border-b-2 border-blue-500".to_string(),
            "Info Annotation".to_string(),
        ),
    ];

    let preview = html! {
        <div>
            <p>{"Theme editor for code editor using Tailwind CSS"}</p>
            <p>{"Customize the colors and appearance of the code editor"}</p>
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CodeEditorDocumentation)]
pub fn code_editor_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "CodeEditor Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The CodeEditor component is a high-performance syntax highlighting editor that supports code editing, line numbers, inline diffs, annotations, and type hints. It uses syntect for syntax highlighting." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="space-y-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">{"Original Code"}</h3>
                            <CodeEditor
                                code={r#"use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new(); // HashMap<_, _>

    // This might panic
    map.insert("key1", "value1");
    // Error: This function might panic
}"#}
                                language="rust"
                                theme="light"
                                show_line_numbers=true
                                diffs={vec![
                                    Diff::modified(6),
                                    Diff::removed(7)
                                ]}
                                annotations={vec![
                                    Annotation::error(8, "This function might panic").inline()
                                ]}
                                type_hints={vec![
                                    TypeHint::new(4, "HashMap<&str, &str>").at_column(23)
                                ]}
                                font_size={"14px"}
                                line_height={"1.5"}
                                class="border border-gray-300 dark:border-gray-700 rounded-sm shadow-xs"
                            />
                        </div>
                        
                        <div class="relative">
                            <h3 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">{"Diff View - Proposed Changes"}</h3>
                            <CodeEditor
                                code={r#"use std::collections::HashMap;

fn main() {
    let mut data_map = HashMap::new(); // HashMap<String, String>
    
    // Initialize with proper error handling
    data_map.insert("key1", "updated_value1");
    data_map.insert("key2", "new_value2");
    
    // This line was removed - old panic code
    
    // Safe access with error handling  
    match data_map.get("key1") {
        Some(value) => println!("Found: {}", value),
        None => println!("Key not found"),
    }
}"#}
                                language="rust"
                                theme="light"
                                show_line_numbers=true
                                diffs={vec![
                                    // First diff block - variable rename
                                    Diff::modified(4).with_message("Improved variable name"),
                                    
                                    // Second diff block - new initialization code
                                    Diff::added(6).with_message("Added helpful comment"),
                                    Diff::added(7).with_message("Updated insert call"), 
                                    Diff::added(8).with_message("Added second insert"),
                                    
                                    // Third diff block - removed old code
                                    Diff::removed(10).with_message("Removed unsafe panic code"),
                                    
                                    // Fourth diff block - new safe code
                                    Diff::added(12).with_message("Added safe error handling comment"),
                                    Diff::added(13).with_message("Safe pattern matching"),
                                    Diff::added(14).with_message("Handle Some case"),
                                    Diff::added(15).with_message("Handle None case"),
                                    Diff::added(16).with_message("Close match block")
                                ]}
                                annotations={vec![
                                    Annotation::info(4, "Better variable name for clarity").with_column_range(12, 20),
                                    Annotation::success(6, "Added helpful initialization comment").inline(),
                                    Annotation::warning(10, "This line represents removed code").with_column_range(4, 50),
                                    Annotation::success(13, "Replaced potential panic with safe pattern matching").with_column_range(4, 38)
                                ]}
                                type_hints={vec![
                                    TypeHint::new(4, "HashMap<&str, &str>").at_column(36),
                                    TypeHint::new(14, "Option<&&str>").at_column(8)
                                ]}
                                font_size={"14px"}
                                line_height={"1.5"}
                                class="border border-emerald-300 dark:border-emerald-700 rounded-sm shadow-md bg-emerald-50 dark:bg-emerald-950"
                            />
                        </div>
                    </div>
                }}
                customize={html! {
                    <CodeEditorThemeEditor />
                }}
                code="// Import the CodeEditor component and related types
use wonopui::components::code_editor::{CodeEditor, Diff, DiffType, Annotation, TypeHint};
use std::collections::HashMap;

// Create a CodeEditor component with syntax highlighting and features
<CodeEditor
    code={r#\"use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new(); // HashMap<_, _>

    // Added comment
    map.insert(\"key1\", \"value1\");
    // Error: This function might panic
}\"#}
    language=\"rust\"
    theme=\"light\"                     // Light/dark themes are now supported
    show_line_numbers={true}
    diffs={vec![
        Diff::new(5, DiffType::Added),
        Diff::new(6, DiffType::Modified)
    ]}
    annotations={vec![
        Annotation::error(7, \"This function might panic\").inline()
    ]}
    type_hints={vec![
        TypeHint::new(3, \"HashMap<String, String>\").at_column(23)
    ]}
    on_change={Callback::from(|code: String| {
        console::log!(\"Code changed:\", &code);
    })}
    // Enhanced features
    enable_multi_cursor={true}          // Enable multi-cursor support (Alt+Click)
    enable_keymap={true}                // Enable custom keymaps
    keymap={                            // Optional custom keymaps
        let mut keymap = HashMap::new();
        keymap.insert(\"Ctrl+/\".to_string(), 
            Callback::from(|e| { e.prevent_default(); }));
        Some(keymap)
    }
    class=\"border border-gray-300 dark:border-gray-700 rounded-sm shadow-xs\"
/>"
            />

            <Features features={vec![
                "Syntax highlighting based on syntect",
                "Real-time editable code with performant rendering",
                "Line numbers with toggle support",
                "Inline diffs to show code changes (added, removed, modified)",
                "True diff view mode with proper line number mapping",
                "Accept/Reject controls for code change proposals",
                "Annotations for errors, warnings, and information",
                "Type hints for showing type information at specific positions",
                "Customizable themes and styles",
                "Synchronized scrolling for all editor elements",
                "Multi-cursor support for advanced editing",
                "Custom keymap support for personalized shortcuts",
                "Tab management and indentation support",
                "Tailwind CSS styling for consistent design",
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="CodeEditor"
                description="Props for the CodeEditor component."
                props={vec![
                    ("code", "String", "The code content to edit/display."),
                    ("language", "String", "The language for syntax highlighting (e.g., 'rust', 'javascript')."),
                    ("theme", "String", "The theme name ('light' or 'dark')."),
                    ("show_line_numbers", "bool", "Whether to show line numbers (default: true)."),
                    ("font_size", "u8", "Font size in pixels (default: 14)."),
                    ("font_family", "String", "Font family (default: 'JetBrains Mono, monospace')."),
                    ("line_height", "f32", "Line height multiplier (default: 1.5)."),
                    ("class", "Classes", "Additional CSS classes."),
                    ("style", "String", "Inline CSS styles."),
                    ("on_change", "Option<Callback<String>>", "Callback when code changes."),
                    ("on_focus", "Option<Callback<FocusEvent>>", "Callback when editor receives focus."),
                    ("on_blur", "Option<Callback<FocusEvent>>", "Callback when editor loses focus."),
                    ("tab_size", "u8", "Tab size in spaces (default: 4)."),
                    ("read_only", "bool", "Whether editor is in read-only mode (default: false)."),
                    ("max_height", "u32", "Maximum height of editor in pixels (0 for unlimited)."),
                    ("line_wrap", "bool", "Enable line wrapping (default: true)."),
                    ("enable_multi_cursor", "bool", "Enable multiple cursors with Alt+Click (default: false)."),
                    ("enable_keymap", "bool", "Enable custom keymap support (default: false)."),
                    ("keymap", "Option<HashMap<String, Callback<KeyboardEvent>>>", "Custom keyboard shortcuts."),
                    ("diff_view", "bool", "Enable diff view mode with proper line number mapping (default: false)."),
                    ("original_line_numbers", "Vec<Option<usize>>", "Maps display lines to original line numbers for diff view."),
                ]}
            />

            <NotesSection
                title={"Recently Added Features".to_string()}
                notes={vec![
                    "Diff view mode: Proper side-by-side diff with accurate line number mapping".to_string(),
                    "Accept/Reject controls: Interactive buttons to approve or dismiss code changes".to_string(),
                    "Enhanced line numbering: Shows original line numbers or gaps for added/removed lines".to_string(),
                    "Multi-cursor support: Create and manage multiple cursors with Alt+Click".to_string(),
                    "Custom keymap: Define your own keyboard shortcuts".to_string(),
                    "Fixed scrolling: Properly synchronized scrolling of line numbers and content".to_string(),
                    "Tailwind CSS: Complete integration with Tailwind CSS for consistent styling".to_string(),
                    "Improved cursor visibility: Better cursor rendering and blinking animation".to_string(),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The CodeEditor component uses syntect for syntax highlighting.".to_string(),
                    "Multi-cursor can be activated by holding Alt key while clicking in the editor.".to_string(),
                    "Custom keymaps can be defined for specialized editing operations.".to_string(),
                    "For monospaced fonts, 'JetBrains Mono', 'Fira Code', or 'Source Code Pro' are recommended.".to_string(),
                    "Line numbers, annotations, and diffs all scroll in sync with the main editor content.".to_string(),
                    "The component is optimized for performance with large files.".to_string(),
                ]}
            />

            <h3 class="text-xl font-semibold mt-6 mb-3 text-zinc-900 dark:text-white">{"Keyboard Shortcuts"}</h3>
            <div class="bg-gray-100 dark:bg-gray-800 p-4 rounded-sm mb-6">
                <p class="text-sm mb-2">{"The code editor supports custom keyboard shortcuts through the keymap property:"}</p>
                <ul class="list-disc ml-6 text-sm text-gray-700 dark:text-gray-300">
                    <li>{"Ctrl+/ - Toggle comment (in the example)"}</li>
                    <li>{"Ctrl+D - Duplicate line (in the example)"}</li>
                    <li>{"Alt+Click - Add multiple cursors"}</li>
                    <li>{"Ctrl+Alt+Click - Clear all cursors"}</li>
                    <li>{"You can define your own shortcuts with the keymap property"}</li>
                </ul>
            </div>
                
            <StylingSection
            component_name={"CodeEditor".to_string()}
            class_descriptions={vec![
                ("editor-container".to_string(), "Container for the entire editor (Tailwind: flex relative rounded-sm overflow-hidden)".to_string()),
                ("line-numbers".to_string(), "Line numbers column (Tailwind: flex-none w-12 text-right bg-gray-100 dark:bg-gray-800)".to_string()),
                ("code-display".to_string(), "Main code display area (Tailwind: p-2 whitespace-pre)".to_string()),
                ("cursor-element".to_string(), "Editor cursor (Tailwind: absolute w-[2px] bg-blue-500 animate-blink)".to_string()),
                ("diff-added".to_string(), "Added line highlight (Tailwind: bg-green-200 dark:bg-green-900/30)".to_string()),
                ("diff-removed".to_string(), "Removed line highlight (Tailwind: bg-red-200 dark:bg-red-900/30)".to_string()),
                ("diff-modified".to_string(), "Modified line highlight (Tailwind: bg-yellow-200 dark:bg-yellow-900/30)".to_string()),
                ("annotation-error".to_string(), "Error annotation (Tailwind: border-b-2 border-red-500)".to_string()),
                ("annotation-warning".to_string(), "Warning annotation (Tailwind: border-b-2 border-yellow-500)".to_string()),
                ("annotation-info".to_string(), "Info annotation (Tailwind: border-b-2 border-blue-500)".to_string()),
            ]}
            />
        </Container>
    }
}
