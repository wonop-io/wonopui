use std::collections::HashMap;

use yew::prelude::*;
use crate::components::code_editor::{CodeEditor, CodeEditorProps, Diff, DiffType, Annotation, AnnotationType, TypeHint};

#[function_component(CodeEditorExample)]
pub fn code_editor_example() -> Html {
    let code = use_state(|| r#"fn example_function(x: i32) -> i32 {
    // This is a comment
    let result = x * 2;
    
    if result > 10 {
        println!("Result is greater than 10: {}", result);
    } else {
        println!("Result is less than or equal to 10: {}", result);
    }
    
    // Return the calculated value
    result
}"#.to_string());
    
    let language = use_state(|| Some("rust".to_string()));
    
    let theme = use_state(|| "light".to_string());
    
    let diffs = use_state(|| vec![
        Diff::added(3),
        Diff::removed(8),
        Diff::modified(5).with_column_range(4, 12).with_message("This condition was changed")
    ]);
    
    let annotations = use_state(|| vec![
        Annotation::error(8, "This line will be removed in the next version")
            .with_column_range(8, 23),
        Annotation::warning(3, "Unused variable")
            .with_column_range(8, 14),
        Annotation::info(1, "Function signature")
            .inline()
    ]);
    
    let type_hints = use_state(|| vec![
        TypeHint::new(3, ": i32").at_column(19),
        TypeHint::new(1, ": fn(i32) -> i32").at_column(19)
    ]);
    
    let on_code_change = {
        let code = code.clone();
        Callback::from(move |new_code: String| {
            code.set(new_code);
        })
    };
    
    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            if *theme == "light" {
                theme.set("dark".to_string());
            } else {
                theme.set("light".to_string());
            }
        })
    };
    
    let change_language = {
        let language = language.clone();
        Callback::from(move |event: Event| {
            let select = event.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let value = select.value();
            if value == "plain" {
                language.set(None);
            } else {
                language.set(Some(value));
            }
        })
    };
    
    html! {
        <div class="p-4 max-w-4xl mx-auto">
            <h1 class="text-2xl font-bold mb-4">{"Code Editor Example"}</h1>
            
            <div class="mb-4 flex space-x-4">
                <div>
                    <label class="block text-sm font-medium mb-1">{"Language"}</label>
                    <select
                        class="block w-full border border-gray-300 rounded p-2"
                        onchange={change_language}
                    >
                        <option value="plain" selected={language.is_none()}>{"Plain Text"}</option>
                        <option value="rust" selected={language.as_ref().map_or(false, |l| l == "rust")}>{"Rust"}</option>
                        <option value="javascript" selected={language.as_ref().map_or(false, |l| l == "javascript")}>{"JavaScript"}</option>
                        <option value="typescript" selected={language.as_ref().map_or(false, |l| l == "typescript")}>{"TypeScript"}</option>
                    </select>
                </div>
                
                <div>
                    <label class="block text-sm font-medium mb-1">{"Theme"}</label>
                    <button
                        class="block w-full border border-gray-300 rounded p-2"
                        onclick={toggle_theme}
                    >
                        {if *theme == "light" { "Switch to Dark" } else { "Switch to Light" }}
                    </button>
                </div>
            </div>
            
            <div class="border rounded p-2 bg-gray-50 dark:bg-gray-800">
                <CodeEditor
                    code={(*code).clone()}
                    language={(*language).clone()}
                    theme={(*theme).clone()}
                    show_line_numbers=true
                    font_size=14
                    line_height=1.5
                    tab_size=4
                    max_height=400
                    line_wrap=true
                    diffs={(*diffs).clone()}
                    annotations={(*annotations).clone()}
                    type_hints={(*type_hints).clone()}
                    on_change={on_code_change}
                />
            </div>
            
            <div class="mt-4">
                <h2 class="text-lg font-semibold mb-2">{"Current Code"}</h2>
                <pre class="bg-gray-100 dark:bg-gray-700 p-4 rounded overflow-auto">{(*code).clone()}</pre>
            </div>
        </div>
    }
}