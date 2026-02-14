use wonopui::prelude::*;
use wonopui::MarkdownEditor;
use yew::prelude::*;

#[function_component(MarkdownEditorExample)]
pub fn markdown_editor_example() -> Html {
    let content = use_state(|| "# Hello World\n\nStart typing here...\n\n- List item\n\n**Bold text**".to_string());

    let on_change = {
        let content = content.clone();
        Callback::from(move |text: String| {
            content.set(text);
        })
    };

    html! {
        <div class="w-full">
            <div class="mb-4 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-md">
                <p class="text-sm text-blue-800 dark:text-blue-500">
                    {"A markdown editor with live preview."}
                </p>
            </div>
            <MarkdownEditor
                value={(*content).clone()}
                onchange={on_change}
                placeholder={"Start typing..."}
                show_toolbar={true}
            />
            <div class="mt-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-md">
                <h3 class="text-sm font-semibold mb-2">{"Editor Content (Debug View):"}</h3>
                <pre class="text-xs overflow-auto max-h-[200px]">
                    {format!("Content: {} characters", content.len())}
                </pre>
            </div>
        </div>
    }
}
