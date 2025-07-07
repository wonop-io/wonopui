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
use wonopui::*;
use yew::prelude::*;
//
// Define our custom block type implementation
#[derive(Clone, PartialEq)]
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
        arguments: String,
        update_block: Callback<Self>,
        onkeydown: Callback<KeyboardEvent>,
        onfocus: Callback<FocusEvent>,
        onblur: Callback<FocusEvent>,
        has_focus: bool,
    ) -> Html {
        match self {
            EditorBlockType::Paragraph(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            log::info!("Storing: {} vs {}", new_content, content_clone);
                            update_block.emit(EditorBlockType::Paragraph(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <ParagraphBlock
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::Heading1(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::Heading1(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <Heading1Block
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::Heading2(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::Heading2(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <Heading2Block
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::Heading3(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::Heading3(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <Heading3Block
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::BulletList(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::BulletList(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <BulletListBlock
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::NumberedList(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::NumberedList(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <NumberedListBlock
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::Quote(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::Quote(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <QuoteBlock
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::CodeBlock(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::CodeBlock(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <CodeBlockBlock
                        content={content.clone()}
                        on_input={Callback::noop()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
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
            EditorBlockType::FileBlock(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::FileBlock(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <FileBlockBlock
                        content={content.clone()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::UrlBlock(content) => {
                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block.emit(EditorBlockType::UrlBlock(new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <UrlBlockBlock
                        content={content.clone()}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
            EditorBlockType::Role(current_role, content) => {
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

                let custom_onblur = {
                    let update_block = update_block.clone();
                    let onblur = onblur.clone();
                    let content_clone = content.clone();
                    let role_type = current_role.clone();
                    Callback::from(move |e: FocusEvent| {
                        let input = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_content = input.inner_text();
                        if new_content != content_clone {
                            update_block
                                .emit(EditorBlockType::Role(role_type.clone(), new_content));
                        }
                        onblur.emit(e);
                    })
                };

                html! {
                    <RoleBlock
                        role_type={current_role.clone()}
                        content={content.clone()}
                        onclick={onclick}
                        onkeydown={onkeydown.clone()}
                        onfocus={onfocus.clone()}
                        onblur={custom_onblur}
                        has_focus={has_focus}
                    />
                }
            }
        }
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
        match self {
            EditorBlockType::Paragraph(content) => content.is_empty(),
            EditorBlockType::Heading1(content) => content.is_empty(),
            EditorBlockType::Heading2(content) => content.is_empty(),
            EditorBlockType::Heading3(content) => content.is_empty(),
            EditorBlockType::BulletList(content) => content.is_empty(),
            EditorBlockType::NumberedList(content) => content.is_empty(),
            EditorBlockType::Quote(content) => content.is_empty(),
            EditorBlockType::CodeBlock(content) => content.is_empty(),
            EditorBlockType::Divider => true,
            EditorBlockType::FileBlock(content) => content.is_empty(),
            EditorBlockType::UrlBlock(content) => content.is_empty(),
            EditorBlockType::Role(_, content) => content.is_empty(),
        }
    }
}
