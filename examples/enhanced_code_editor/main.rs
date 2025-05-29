use std::collections::HashMap;
use yew::prelude::*;
use wonopui::components::code_editor::{
    CodeEditor, CodeEditorProps, Diff, Annotation, AnnotationType, TypeHint
};

#[function_component(App)]
fn app() -> Html {
    let initial_code = r#"use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    
    // Insert some values
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    
    // Access a value
    if let Some(value) = map.get("key2") {
        println!("Value for key2: {}", value);
    }
    
    // Iterate over the map
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}"#.to_string();

    let diffs = vec![
        Diff::added(5).with_message("Added comment"),
        Diff::modified(6).with_column_range(4, 15),
        Diff::removed(15).with_message("Removed unnecessary newline"),
    ];
    
    let annotations = vec![
        Annotation::warning(7, "Consider using a more descriptive variable name").with_column_range(18, 23),
        Annotation::error(10, "Potential panic if key doesn't exist").with_column_range(13, 32),
        Annotation::info(14, "This is a standard pattern for iterating over HashMaps").inline(),
    ];
    
    let type_hints = vec![
        TypeHint::new(3, "HashMap<&str, &str>").at_column(14),
        TypeHint::new(6, "&str").at_column(15),
        TypeHint::new(6, "&str").at_column(31),
        TypeHint::new(10, "Option<&&str>").at_column(24),
    ];
    
    let on_change = Callback::from(|code: String| {
        // In a real app, you'd handle code changes here
        log::info!("Code changed: {}", code.len());
    });

    // Define a custom keymap
    let mut keymap = HashMap::new();
    keymap.insert(
        "Ctrl+/".to_string(),
        Callback::from(|event: KeyboardEvent| {
            event.prevent_default();
            log::info!("Comment/uncomment shortcut pressed");
            // In a real app, you'd implement comment toggling here
        })
    );
    keymap.insert(
        "Ctrl+d".to_string(),
        Callback::from(|event: KeyboardEvent| {
            event.prevent_default();
            log::info!("Duplicate line shortcut pressed");
            // In a real app, you'd implement line duplication here
        })
    );

    html! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">{"Enhanced Code Editor Example"}</h1>
            
            <div class="mb-6">
                <div class="bg-gray-100 dark:bg-gray-800 p-3 mb-3 rounded text-sm">
                    <p class="font-semibold mb-1">{"Enhanced Features:"}</p>
                    <ul class="list-disc pl-5">
                        <li>{"Fixed scrolling synchronization"}</li>
                        <li>{"Visible cursor with consistent blinking"}</li>
                        <li>{"Multi-cursor support (Hold Alt/Option + Click)"}</li>
                        <li>{"Custom keymap support (Ctrl+/ and Ctrl+D defined)"}</li>
                        <li>{"Pure Tailwind CSS styling"}</li>
                    </ul>
                </div>
                <CodeEditor 
                    code={initial_code.clone()}
                    language={"rust"}
                    theme={"light"}
                    diffs={diffs}
                    annotations={annotations}
                    type_hints={type_hints}
                    show_line_numbers={true}
                    on_change={on_change}
                    class={"border border-gray-300 dark:border-gray-700 rounded shadow-sm"}
                    style={"min-height: 400px;"}
                    enable_multi_cursor={true}
                    enable_keymap={true}
                    keymap={Some(keymap)}
                />
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <h2 class="text-lg font-semibold mb-2">{"Dark Theme"}</h2>
                    <CodeEditor 
                        code={"// Dark theme example\nfunction hello() {\n  console.log('Hello world');\n  return true;\n}"}
                        language={"javascript"}
                        theme={"dark"}
                        show_line_numbers={true}
                        class={"border border-gray-700 rounded shadow-sm"}
                        style={"height: 200px;"}
                    />
                </div>
                
                <div>
                    <h2 class="text-lg font-semibold mb-2">{"No Line Numbers"}</h2>
                    <CodeEditor 
                        code={"# Python example without line numbers\ndef calculate(a, b):\n    return a * b + a / b if b != 0 else 0\n\nresult = calculate(5, 10)\nprint(f\"Result: {result}\")"}
                        language={"python"}
                        show_line_numbers={false}
                        class={"border border-gray-300 dark:border-gray-700 rounded shadow-sm"}
                        style={"height: 200px;"}
                    />
                </div>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}