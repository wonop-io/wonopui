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

    let editor_style = "min-height: 400px; max-width: 900px; margin: 0 auto;";

    html! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">{"WonopUI Code Editor Example"}</h1>
            
            <div class="mb-6">
                <label class="block text-gray-700 dark:text-gray-300 mb-2">{"Basic Editor"}</label>
                <CodeEditor 
                    code={initial_code.clone()}
                    language={"rust"}
                    theme={"InspiredGitHub"}
                    diffs={diffs}
                    annotations={annotations}
                    type_hints={type_hints}
                    show_line_numbers={true}
                    on_change={on_change}
                    class={classes!("border", "border-gray-300", "rounded")}
                    style={editor_style}
                />
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                <div>
                    <label class="block text-gray-700 dark:text-gray-300 mb-2">{"Read-only Mode"}</label>
                    <CodeEditor 
                        code={"// This editor is read-only\nfn hello() {\n    println!(\"Hello, world!\");\n}"}
                        language={"rust"}
                        read_only={true}
                        max_height={200}
                        class={classes!("border", "border-gray-300", "rounded")}
                    />
                </div>
                
                <div>
                    <label class="block text-gray-700 dark:text-gray-300 mb-2">{"No Line Numbers"}</label>
                    <CodeEditor 
                        code={"// This editor has no line numbers\nlet x = 42;\nconsole.log(x);"}
                        language={"javascript"}
                        show_line_numbers={false}
                        max_height={200}
                        class={classes!("border", "border-gray-300", "rounded")}
                    />
                </div>
            </div>
            
            <div class="mb-6">
                <label class="block text-gray-700 dark:text-gray-300 mb-2">{"Different Language & Theme"}</label>
                <CodeEditor 
                    code={"<!DOCTYPE html>\n<html>\n<head>\n    <title>Example</title>\n</head>\n<body>\n    <h1>Hello World</h1>\n    <p>This is an example HTML document.</p>\n</body>\n</html>"}
                    language={"html"}
                    theme={"base16-ocean.dark"}
                    show_line_numbers={true}
                    class={classes!("border", "border-gray-300", "rounded")}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}