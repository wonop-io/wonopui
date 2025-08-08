// editor_block_type.rs
use crate::components::markdown_editor::blocks::{
    AssistantRoleIcon, BulletListBlock, BulletListIcon, CodeBlockBlock, CodeBlockIcon,
    DividerBlock, DividerIcon, FileBlockBlock, FileBlockIcon, Heading1Block, Heading1Icon,
    Heading2Block, Heading2Icon, Heading3Block, Heading3Icon, NumberedListBlock, NumberedListIcon,
    ParagraphBlock, ParagraphIcon, QuoteBlock, QuoteIcon, RoleBlock, RoleType, SystemRoleIcon,
    UrlBlockBlock, UrlBlockIcon, UserRoleIcon,
};
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};
use wonopui::prelude::*;
use wonopui::ContentEditableWithCommands;
use wonopui::*;
use yew::prelude::*;
//
// Define our custom block type implementation
#[derive(Debug, Clone, PartialEq)]
pub enum EditorBlockType {
    Paragraph(String),
    Heading1(String),
    Heading2(String),
    Heading3(String),
    BulletList(String),
    NumberedList(String),
    Quote(String),
    CodeBlock(String),
    Divider,
    FileBlock(String),
    UrlBlock(String),
    Role(RoleType, String),
}

impl BlockTrait for EditorBlockType {
    fn new_block() -> Self {
        EditorBlockType::Paragraph(String::new())
    }
    
    fn to_markdown(&self) -> String {
        match self {
            EditorBlockType::Paragraph(content) => content.clone(),
            EditorBlockType::Heading1(content) => format!("# {}", content),
            EditorBlockType::Heading2(content) => format!("## {}", content),
            EditorBlockType::Heading3(content) => format!("### {}", content),
            EditorBlockType::BulletList(content) => format!("- {}", content),
            EditorBlockType::NumberedList(content) => format!("1. {}", content),
            EditorBlockType::Quote(content) => format!("> {}", content),
            EditorBlockType::CodeBlock(content) => format!("```\n{}\n```", content),
            EditorBlockType::Divider => "---".to_string(),
            EditorBlockType::FileBlock(path) => format!("![File]({})", path),
            EditorBlockType::UrlBlock(url) => format!("[{}]({})", url, url),
            EditorBlockType::Role(role_type, content) => {
                format!("**{}**: {}", role_type.to_string(), content)
            }
        }
    }

    fn icon(&self) -> Html {
        match self {
            EditorBlockType::Paragraph(_) => html! { <ParagraphIcon /> },
            EditorBlockType::Heading1(_) => html! { <Heading1Icon /> },
            EditorBlockType::Heading2(_) => html! { <Heading2Icon /> },
            EditorBlockType::Heading3(_) => html! { <Heading3Icon /> },
            EditorBlockType::BulletList(_) => html! { <BulletListIcon /> },
            EditorBlockType::NumberedList(_) => html! { <NumberedListIcon /> },
            EditorBlockType::Quote(_) => html! { <QuoteIcon /> },
            EditorBlockType::CodeBlock(_) => html! { <CodeBlockIcon /> },
            EditorBlockType::Divider => html! { <DividerIcon /> },
            EditorBlockType::FileBlock(_) => html! { <FileBlockIcon /> },
            EditorBlockType::UrlBlock(_) => html! { <UrlBlockIcon /> },
            EditorBlockType::Role(role_type, _) => match role_type {
                RoleType::System => html! { <SystemRoleIcon /> },
                RoleType::Assistant => html! { <AssistantRoleIcon /> },
                RoleType::User => html! { <UserRoleIcon /> },
            },
        }
    }

    fn name(&self) -> String {
        match self {
            EditorBlockType::Paragraph(_) => "Paragraph".to_string(),
            EditorBlockType::Heading1(_) => "Heading 1".to_string(),
            EditorBlockType::Heading2(_) => "Heading 2".to_string(),
            EditorBlockType::Heading3(_) => "Heading 3".to_string(),
            EditorBlockType::BulletList(_) => "Bullet List".to_string(),
            EditorBlockType::NumberedList(_) => "Numbered List".to_string(),
            EditorBlockType::Quote(_) => "Quote".to_string(),
            EditorBlockType::CodeBlock(_) => "Code Block".to_string(),
            EditorBlockType::Divider => "Divider".to_string(),
            EditorBlockType::FileBlock(_) => "File Upload".to_string(),
            EditorBlockType::UrlBlock(_) => "URL".to_string(),
            EditorBlockType::Role(role_type, _) => format!("Role: {}", role_type.to_string()),
        }
    }

    fn render(
        &self,
        update_block: Callback<Self>,
        onkeydown: Callback<KeyboardEvent>,
        onfocus: Callback<FocusEvent>,
        onblur: Callback<FocusEvent>,
        has_focus: bool,
    ) -> Html {
        // Get command options for blocks that support commands
        let command_options = Self::search(None)
            .into_iter()
            .map(|block_type| {
                let name = block_type.name();
                let keywords = name.to_lowercase();
                let icon = Some(block_type.icon());
                (block_type, keywords, name, icon)
            })
            .collect::<Vec<_>>();

        // Create an on_input callback that updates content immediately
        let on_input = {
            let update_block = update_block.clone();
            let self_type = self.clone();

            Callback::from(move |content: String| {
                // Create a new block of the same type with updated content
                let new_block = match &self_type {
                    EditorBlockType::Paragraph(_) => EditorBlockType::Paragraph(content),
                    EditorBlockType::Heading1(_) => EditorBlockType::Heading1(content),
                    EditorBlockType::Heading2(_) => EditorBlockType::Heading2(content),
                    EditorBlockType::Heading3(_) => EditorBlockType::Heading3(content),
                    EditorBlockType::BulletList(_) => EditorBlockType::BulletList(content),
                    EditorBlockType::NumberedList(_) => EditorBlockType::NumberedList(content),
                    EditorBlockType::Quote(_) => EditorBlockType::Quote(content),
                    EditorBlockType::CodeBlock(_) => EditorBlockType::CodeBlock(content),
                    EditorBlockType::Divider => EditorBlockType::Divider,
                    EditorBlockType::FileBlock(_) => EditorBlockType::FileBlock(content),
                    EditorBlockType::UrlBlock(_) => EditorBlockType::UrlBlock(content),
                    EditorBlockType::Role(role_type, _) => {
                        EditorBlockType::Role(role_type.clone(), content)
                    }
                };

                update_block.emit(new_block);
            })
        };
        let content = match self {
            EditorBlockType::Paragraph(content) => content.clone(),
            EditorBlockType::Heading1(content) => content.clone(),
            EditorBlockType::Heading2(content) => content.clone(),
            EditorBlockType::Heading3(content) => content.clone(),
            EditorBlockType::BulletList(content) => content.clone(),
            EditorBlockType::NumberedList(content) => content.clone(),
            EditorBlockType::Quote(content) => content.clone(),
            EditorBlockType::CodeBlock(content) => content.clone(),
            EditorBlockType::Divider => String::new(),
            EditorBlockType::FileBlock(content) => content.clone(),
            EditorBlockType::UrlBlock(content) => content.clone(),
            EditorBlockType::Role(_, content) => content.clone(),
        };
        log::info!("Content: {}", content);
        match self {
            EditorBlockType::Paragraph(_) => {
                html! {
                    <ParagraphBlock
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::Heading1(_) => {
                html! {
                    <Heading1Block
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::Heading2(_) => {
                html! {
                    <Heading2Block
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::Heading3(_) => {
                html! {
                    <Heading3Block
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::BulletList(_) => {
                html! {
                    <BulletListBlock
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::NumberedList(_) => {
                html! {
                    <NumberedListBlock
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::Quote(_) => {
                html! {
                    <QuoteBlock
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::CodeBlock(_) => {
                html! {
                    <CodeBlockBlock
                        content={content.clone()}
                        on_input={on_input}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                        command_triggers={Self::command_triggers()}
                        command_options={command_options}
                        on_command_select={update_block.clone()}
                    />
                }
            }
            EditorBlockType::Divider => {
                html! {
                    <DividerBlock
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::FileBlock(_) => {
                html! {
                    <FileBlockBlock
                        content={content.clone()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::UrlBlock(_) => {
                html! {
                    <UrlBlockBlock
                        content={content.clone()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::Role(current_role, _) => {
                let update_block_cb = update_block.clone();
                let onclick = {
                    let current_role = current_role.clone();
                    let content = content.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        let element = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();

                        // Cycle through the roles
                        let new_role = match current_role {
                            RoleType::System => RoleType::Assistant,
                            RoleType::Assistant => RoleType::User,
                            RoleType::User => RoleType::System,
                        };

                        // Update the data attribute
                        element
                            .set_attribute("data-role", &new_role.to_string())
                            .unwrap();

                        // Update text content
                        element.set_text_content(Some(&new_role.to_string()));

                        // Update the block type
                        update_block_cb.emit(EditorBlockType::Role(new_role, content.clone()));
                    })
                };

                html! {
                    <RoleBlock
                        role_type={current_role.clone()}
                        content={content.clone()}
                        onclick={onclick}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={onblur.clone()}
                        has_focus={has_focus}
                    />
                }
            }
        }
    }

    fn command_triggers() -> Vec<String> {
        vec!["/".to_string()]
    }

    fn search(query: Option<String>) -> Vec<Self> {
        let block_types = vec![
            EditorBlockType::Paragraph(String::new()),
            EditorBlockType::Heading1(String::new()),
            EditorBlockType::Heading2(String::new()),
            EditorBlockType::Heading3(String::new()),
            EditorBlockType::BulletList(String::new()),
            EditorBlockType::NumberedList(String::new()),
            EditorBlockType::Quote(String::new()),
            EditorBlockType::CodeBlock(String::new()),
            EditorBlockType::Divider,
            EditorBlockType::FileBlock(String::new()),
            EditorBlockType::UrlBlock(String::new()),
            EditorBlockType::Role(RoleType::System, String::new()),
            EditorBlockType::Role(RoleType::Assistant, String::new()),
            EditorBlockType::Role(RoleType::User, String::new()),
        ];

        if let Some(query) = query {
            if query.is_empty() {
                return block_types;
            }
            let query = query.to_lowercase();
            block_types
                .into_iter()
                .filter(|block_type| {
                    let name = block_type.name().to_lowercase();
                    name.contains(&query)
                })
                .collect()
        } else {
            block_types
        }
    }

    fn can_delete(&self) -> bool {
        let blank = String::new();
        let content = match self {
            EditorBlockType::Paragraph(content) => content,
            EditorBlockType::Heading1(content) => content,
            EditorBlockType::Heading2(content) => content,
            EditorBlockType::Heading3(content) => content,
            EditorBlockType::BulletList(content) => content,
            EditorBlockType::NumberedList(content) => content,
            EditorBlockType::Quote(content) => content,
            EditorBlockType::CodeBlock(content) => content,
            EditorBlockType::Divider => &blank,
            EditorBlockType::FileBlock(content) => content,
            EditorBlockType::UrlBlock(content) => content,
            EditorBlockType::Role(_, content) => content,
        };

        content.is_empty() || content == "\n" || content == "\r\n"
    }
}
