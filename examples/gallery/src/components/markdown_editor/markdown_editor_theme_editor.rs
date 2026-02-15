use crate::api_section::ApiSection;
use crate::components::example_code::ExampleCode;
use crate::components::markdown_editor::EditorBlockType;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(MarkdownEditorThemeEditor)]
pub fn markdown_editor_theme_editor() -> Html {
    let fields = vec![
        (
            "markdown_editor_container".to_string(),
            "Editor Container".to_string(),
        ),
        (
            "markdown_editor_blocks_container".to_string(),
            "Blocks Container".to_string(),
        ),
        (
            "markdown_editor_block".to_string(),
            "Editor Block".to_string(),
        ),
        (
            "markdown_editor_block_active".to_string(),
            "Active Block".to_string(),
        ),
        (
            "markdown_editor_paragraph".to_string(),
            "Paragraph Block".to_string(),
        ),
        (
            "markdown_editor_heading1".to_string(),
            "Heading 1 Block".to_string(),
        ),
        (
            "markdown_editor_heading2".to_string(),
            "Heading 2 Block".to_string(),
        ),
        (
            "markdown_editor_heading3".to_string(),
            "Heading 3 Block".to_string(),
        ),
        (
            "markdown_editor_bullet_list".to_string(),
            "Bullet List Block".to_string(),
        ),
        (
            "markdown_editor_numbered_list".to_string(),
            "Numbered List Block".to_string(),
        ),
        (
            "markdown_editor_quote".to_string(),
            "Quote Block".to_string(),
        ),
        ("markdown_editor_code".to_string(), "Code Block".to_string()),
        (
            "markdown_editor_divider".to_string(),
            "Divider Block".to_string(),
        ),
        (
            "markdown_editor_file_block".to_string(),
            "File Block".to_string(),
        ),
        (
            "markdown_editor_url_block".to_string(),
            "URL Block".to_string(),
        ),
        (
            "markdown_editor_role_block".to_string(),
            "Role Block".to_string(),
        ),
        (
            "markdown_editor_role_system".to_string(),
            "System Role Badge".to_string(),
        ),
        (
            "markdown_editor_role_assistant".to_string(),
            "Assistant Role Badge".to_string(),
        ),
        (
            "markdown_editor_role_user".to_string(),
            "User Role Badge".to_string(),
        ),
        (
            "markdown_command_menu_container".to_string(),
            "Command Menu Container".to_string(),
        ),
    ];

    let preview = html! {
        <MarkdownEditor
            class="h-[400px]"
            placeholder={"Start typing..."}
            value="# Example Heading\n\nStart typing here...\n\n- List item example\n\n**System**: System message"
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}
