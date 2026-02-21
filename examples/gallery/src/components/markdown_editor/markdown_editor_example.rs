use wonopui::prelude::*;
use wonopui::MarkdownEditor;
use yew::prelude::*;
use super::editor_block_type::EditorBlockType;

#[function_component(MarkdownEditorExample)]
pub fn markdown_editor_example() -> Html {
    // Initial blocks with some demo content
    let initial_blocks = vec![
        EditorBlockType::Heading1("Hello World".to_string()),
        EditorBlockType::Paragraph("Start typing here...".to_string()),
        EditorBlockType::BulletList("List item".to_string()),
        EditorBlockType::Paragraph("**Bold text** and *italic* formatting.".to_string()),
    ];
    
    let blocks = use_state(|| initial_blocks.clone());

    let on_change = {
        let blocks = blocks.clone();
        Callback::from(move |new_blocks: Vec<EditorBlockType>| {
            blocks.set(new_blocks);
        })
    };

    html! {
        <div class="w-full">
            <div class="mb-4 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-md">
                <p class="text-sm text-blue-800 dark:text-blue-500">
                    {"A block-based markdown editor with drag & drop, undo/redo, and inline formatting."}
                </p>
                <p class="mt-2 text-xs text-blue-600 dark:text-blue-400">
                    {"Try typing '/' to insert new block types, or use keyboard shortcuts: Ctrl+B (bold), Ctrl+I (italic), Ctrl+Z (undo)"}
                </p>
            </div>
            <MarkdownEditor<EditorBlockType>
                initial_content={initial_blocks}
                on_change={on_change}
                auto_focus={true}
                show_block_actions={true}
            />
            <div class="mt-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-md">
                <h3 class="text-sm font-semibold mb-2">{"Editor Content (Debug View):"}</h3>
                <pre class="text-xs overflow-auto max-h-[200px]">
                    {format!("{} blocks", blocks.len())}
                </pre>
            </div>
        </div>
    }
}
