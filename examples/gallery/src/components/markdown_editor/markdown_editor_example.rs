use super::*;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use wonopui::prelude::*;
use yew::prelude::*;

#[function_component(MarkdownEditorExample)]
pub fn markdown_editor_example() -> Html {
    // Initialize with a paragraph block to ensure there's at least one component
    let initial_content = vec![
        EditorBlockType::Heading1("Hello world".to_string()),
        EditorBlockType::Paragraph("Hello world".to_string()),
        EditorBlockType::Role(RoleType::System, String::new()),
        EditorBlockType::FileBlock(String::new()),
        EditorBlockType::UrlBlock(String::new()),
    ];
    let content = use_state(|| initial_content);

    let on_change = {
        let content = content.clone();
        Callback::from(move |blocks: Vec<EditorBlockType>| {
            content.set(blocks);
        })
    };

    html! {
        <div class="w-full">
            <div class="mb-4 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-md">
                <p class="text-sm text-blue-800 dark:text-blue-500">
                    {"Type '/' to see the command menu and try different block types! Try the new File, URL, and Role blocks."}
                </p>
            </div>
            <MarkdownEditor<EditorBlockType>
                auto_focus={true}
                placeholder="Start typing..."
                on_change={on_change}
                initial_content={(*content).clone()}
            />
            <div class="mt-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-md">
                <h3 class="text-sm font-semibold mb-2">{"Editor Content (Debug View):"}</h3>
                <pre class="text-xs overflow-auto max-h-[200px]">
                    {format!("Content contains {} blocks", content.len())}
                </pre>
            </div>
        </div>
    }
}
