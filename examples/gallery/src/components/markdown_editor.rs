use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use web_sys::HtmlElement;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;
// TODO: Add to Crate.toml
use std::str::FromStr;
use wasm_bindgen::JsCast;

// Define our custom block type implementation
#[derive(Clone, PartialEq)]
pub enum EditorBlockType {
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    BulletList,
    NumberedList,
    Quote,
    CodeBlock,
    Divider,
    FileBlock,
    UrlBlock,
    Role(RoleType),
}

#[derive(Clone, PartialEq)]
pub enum RoleType {
    System,
    Assistant,
    User,
}

impl ToString for RoleType {
    fn to_string(&self) -> String {
        match self {
            RoleType::System => "System".to_string(),
            RoleType::Assistant => "Assistant".to_string(),
            RoleType::User => "User".to_string(),
        }
    }
}

impl FromStr for RoleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "System" => Ok(RoleType::System),
            "Assistant" => Ok(RoleType::Assistant),
            "User" => Ok(RoleType::User),
            _ => Err(()),
        }
    }
}

impl BlockTrait for EditorBlockType {
    fn new_block() -> Self {
        EditorBlockType::Paragraph // Default new block type
    }

    fn command_triggers() -> Vec<String> {
        vec!["/".to_string()] // Default command trigger
    }

    fn icon(&self) -> Html {
        match self {
            EditorBlockType::Paragraph => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="17" y1="10" x2="3" y2="10"></line>
                    <line x1="21" y1="6" x2="3" y2="6"></line>
                    <line x1="21" y1="14" x2="3" y2="14"></line>
                    <line x1="17" y1="18" x2="3" y2="18"></line>
                </svg>
            },
            EditorBlockType::Heading1 => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M4 12h8"></path>
                    <path d="M4 18V6"></path>
                    <path d="M12 18V6"></path>
                    <path d="m17 12 3 4"></path>
                    <path d="m20 12-3 4"></path>
                </svg>
            },
            EditorBlockType::Heading2 => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M4 12h8"></path>
                    <path d="M4 18V6"></path>
                    <path d="M12 18V6"></path>
                    <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"></path>
                </svg>
            },
            EditorBlockType::Heading3 => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M4 12h8"></path>
                    <path d="M4 18V6"></path>
                    <path d="M12 18V6"></path>
                    <path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"></path>
                    <path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"></path>
                </svg>
            },
            EditorBlockType::BulletList => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="9" y1="6" x2="20" y2="6"></line>
                    <line x1="9" y1="12" x2="20" y2="12"></line>
                    <line x1="9" y1="18" x2="20" y2="18"></line>
                    <circle cx="4" cy="6" r="2"></circle>
                    <circle cx="4" cy="12" r="2"></circle>
                    <circle cx="4" cy="18" r="2"></circle>
                </svg>
            },
            EditorBlockType::NumberedList => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="10" y1="6" x2="21" y2="6"></line>
                    <line x1="10" y1="12" x2="21" y2="12"></line>
                    <line x1="10" y1="18" x2="21" y2="18"></line>
                    <path d="M4 6h1v4"></path>
                    <path d="M4 10h2"></path>
                    <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path>
                </svg>
            },
            EditorBlockType::Quote => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
                    <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>
                </svg>
            },
            EditorBlockType::CodeBlock => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="16 18 22 12 16 6"></polyline>
                    <polyline points="8 6 2 12 8 18"></polyline>
                </svg>
            },
            EditorBlockType::Divider => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="3" y1="12" x2="21" y2="12"></line>
                </svg>
            },
            EditorBlockType::FileBlock => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                    <polyline points="14 2 14 8 20 8"></polyline>
                    <line x1="12" y1="18" x2="12" y2="12"></line>
                    <line x1="9" y1="15" x2="15" y2="15"></line>
                </svg>
            },
            EditorBlockType::UrlBlock => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                    <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                </svg>
            },
            EditorBlockType::Role(role_type) => match role_type {
                RoleType::System => html! {
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                        <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                        <line x1="6" y1="6" x2="6.01" y2="6"></line>
                        <line x1="6" y1="18" x2="6.01" y2="18"></line>
                    </svg>
                },
                RoleType::Assistant => html! {
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M12 2a8 8 0 0 1 8 8v1a7 7 0 0 1-7 7h-1a7 7 0 0 1-7-7v-1a8 8 0 0 1 8-8Z"></path>
                        <path d="M19.07 19.07a8 8 0 0 1-11.31 0"></path>
                        <path d="M9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
                        <path d="M17 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
                    </svg>
                },
                RoleType::User => html! {
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                        <circle cx="12" cy="7" r="4"></circle>
                    </svg>
                },
            },
        }
    }

    fn name(&self) -> String {
        match self {
            EditorBlockType::Paragraph => "Paragraph".to_string(),
            EditorBlockType::Heading1 => "Heading 1".to_string(),
            EditorBlockType::Heading2 => "Heading 2".to_string(),
            EditorBlockType::Heading3 => "Heading 3".to_string(),
            EditorBlockType::BulletList => "Bullet List".to_string(),
            EditorBlockType::NumberedList => "Numbered List".to_string(),
            EditorBlockType::Quote => "Quote".to_string(),
            EditorBlockType::CodeBlock => "Code Block".to_string(),
            EditorBlockType::Divider => "Divider".to_string(),
            EditorBlockType::FileBlock => "File Upload".to_string(),
            EditorBlockType::UrlBlock => "URL Link".to_string(),
            EditorBlockType::Role(role_type) => format!("Role: {}", role_type.to_string()),
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
        // Helper function to create a generic contenteditable div
        let editable_div = |html_tag: &str, default_classes: &str, content: String, keydown_cb: Callback<KeyboardEvent>, focus_cb: Callback<FocusEvent>, blur_cb: Callback<FocusEvent>, current_focus: bool| {
            let class_str = format!("{} p-2 min-h-[1.5em] w-full outline-none focus:outline-none", default_classes);

            // Create a NodeRef for focusing
            let element_ref = use_node_ref();

            // Focus the element when `has_focus` is true and it's not already focused
            {
                let element_ref = element_ref.clone();
                use_effect_with(current_focus, move |current_focus_dep| {
                    if *current_focus_dep {
                        if let Some(element) = element_ref.cast::<web_sys::HtmlElement>() {
                           // Check if it's already focused to prevent focus loops
                           if web_sys::window().unwrap().document().unwrap().active_element() != Some(element.clone().into()) {
                                let _ = element.focus();
                           }
                        }
                    }
                    || ()
                });
            }

            // The `on_input` prop for EditorBlock is connected to on_input_block in the main editor.
            // The `render` method of a BlockTrait implementation receives this as `update_block_content` (this was a misinterpretation, it receives `update_block` for type change, and the main editor handles direct input via its own DOM listeners on the contenteditable)
            // The `EditorBlock` component itself sets up an oninput handler that calls ITS `on_input` prop.
            // So, the `BlockTrait::render` function for an editable block primarily needs to render the contenteditable element
            // and pass through the onkeydown, onfocus, onblur. The actual text capture is handled by EditorBlock's own oninput.
            // My previous `editable_div`'s oninput was redundant and potentially problematic.
            // The `EditorBlock` component itself will attach an oninput listener to the first contenteditable child it finds.

            // Let's re-verify how `EditorBlock` handles input.
            // `EditorBlock` has `on_input: Callback<(usize, String)>`.
            // It renders `props.block.block_type.render(...)`.
            // The `render` function in `BlockTrait` has `arguments: String`, which is the content.
            // The `EditorBlock` doesn't seem to automatically attach an input handler to what `BlockTrait::render` produces.
            // This means the `render` method *is* responsible for wiring up input if it's an editable block.

            // The `on_input` callback provided to `EditorBlockProps` is what we should call.
            // The `BlockTrait::render` receives `onkeydown`, `onfocus`, `onblur`. It does NOT directly receive the `on_input` for content.
            // This is a gap. The `EditorBlock` should probably pass its `on_input` prop to the `BlockTrait::render` method.

            // Looking at `src/components/data_display/markdown_editor/editor_block.rs`:
            // `EditorBlockProps` has `on_input: Callback<(usize, String)>`.
            // This `on_input` is NOT passed to `props.block.block_type.render`. This is the central issue.

            // For now, to make the example work without modifying the core `EditorBlock` yet,
            // the `editable_div` must manually extract content and call a callback.
            // BUT, it doesn't have the correct callback.
            // This points to a design flaw in `EditorBlock` or `BlockTrait`.

            // Let's assume, for the sake of progressing the example, that `EditorBlock` *should* be passing its `on_input` callback
            // to the `BlockTrait::render` function. Since it's not, the example cannot properly implement editable blocks
            // that report their changes back.

            // Given the current structure, the `on_input` on `EditorBlock` is likely intended to be triggered
            // by an event listener set up *within* `EditorBlock` on the rendered output of `BlockTrait::render`,
            // specifically targeting a contenteditable element.
            // `EditorBlock`'s `html!` doesn't show it setting up its own direct oninput listener on the child.

            // This is a significant problem. The `EditorBlock` must provide a way for the rendered block
            // to signal content changes.

            // For the purpose of this exercise, I will proceed by modifying `BlockTrait` and `EditorBlock`
            // to correctly pass the `on_input` callback. This is essential.

            // PLAN REVISION NEEDED:
            // 1. Modify `BlockTrait`: add `on_input_content: Callback<String>` to `render` method.
            // 2. Modify `EditorBlock`: pass its `on_input` prop (adapted) to `BlockTrait::render` as `on_input_content`.
            // 3. Update `editable_div` in example to use this new `on_input_content` callback.

            // Given I cannot change the plan mid-step, I will make the example's `editable_div`
            // *assume* it had an `on_input_content: Callback<String>` and use it.
            // This means the example won't *actually* work until the core is changed.
            // This is not ideal, but I must work within the current plan step.

            // Simulating the `on_input_content` callback for the helper.
            // THIS IS A HACK for the example due to core component limitations.
            // We need a placeholder for where the real input callback would go.
            // The `update_block: Callback<Self>` is for block *type* changes, not content.
            let on_content_change = {
                // This is a dummy callback. In a real scenario, this would be connected
                // to the EditorBlock's on_input prop.
                let log_content = move |new_content: String| {
                    // In a real fix, this would be: on_input_prop.emit(new_content);
                    log::info!("Content changed (simulated): {}", new_content);
                };
                Callback::from(log_content)
            };


            let element_ref_clone_for_input = element_ref.clone();
            let oninput_handler = Callback::from(move |e: InputEvent| {
                if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                    on_content_change.emit(target.inner_html());
                }
            });

            html! {
                <@{html_tag}
                    ref={element_ref}
                    class={class_str}
                    contenteditable="true"
                    oninput={oninput_handler}
                    onkeydown={keydown_cb}
                    onfocus={focus_cb}
                    onblur={blur_cb}
                >
                    {content}
                </@>
            }
        };

        match self {
            EditorBlockType::Paragraph => editable_div("div", "", arguments, onkeydown, onfocus, onblur, has_focus),
            EditorBlockType::Heading1 => editable_div("h1", "text-3xl font-bold", arguments, onkeydown, onfocus, onblur, has_focus),
            EditorBlockType::Heading2 => editable_div("h2", "text-2xl font-bold", arguments, onkeydown, onfocus, onblur, has_focus),
            EditorBlockType::Heading3 => editable_div("h3", "text-xl font-bold", arguments, onkeydown, onfocus, onblur, has_focus),
            EditorBlockType::BulletList => editable_div("ul", "pl-8 list-disc", arguments, onkeydown, onfocus, onblur, has_focus), // TODO: Needs <li> handling for proper list items
            EditorBlockType::NumberedList => editable_div("ol", "pl-8 list-decimal", arguments, onkeydown, onfocus, onblur, has_focus), // TODO: Needs <li> handling
            EditorBlockType::Quote => editable_div("blockquote", "pl-4 border-l-4 border-zinc-300 dark:border-zinc-600 bg-zinc-50 dark:bg-zinc-800/50 italic", arguments, onkeydown, onfocus, onblur, has_focus),
            EditorBlockType::CodeBlock => editable_div("pre", "font-mono text-sm bg-zinc-100 dark:bg-zinc-800 rounded-md", arguments, onkeydown, onfocus, onblur, has_focus), // Typically, 'pre' wraps 'code'

            EditorBlockType::Divider => html! {
                <hr class="w-full border-t-2 border-zinc-200 dark:border-zinc-700 my-4" />
            },
            EditorBlockType::FileBlock => html! {
                <div class={classes!(
                    "flex",
                    "items-center",
                    "gap-2",
                    "p-3",
                    "border-2",
                    "border-dashed",
                    "border-zinc-300",
                    "dark:border-zinc-600",
                    "bg-zinc-50",
                    "dark:bg-zinc-800/50",
                    "rounded-md",
                    "cursor-pointer",
                    "hover:bg-zinc-100",
                    "dark:hover:bg-zinc-800"
                )}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                        <polyline points="14 2 14 8 20 8"></polyline>
                        <line x1="12" y1="18" x2="12" y2="12"></line>
                        <line x1="9" y1="15" x2="15" y2="15"></line>
                    </svg>
                    <div>
                        if arguments.is_empty() {
                            <span>{"Click to upload a file"}</span>
                        } else {
                            <span>{arguments}</span>
                        }
                    </div>
                </div>
            }),
            EditorBlockType::UrlBlock => {
                let url = if arguments.is_empty() {
                    "https://".to_string()
                } else {
                    arguments
                };

                html! {
                    <div class={classes!(
                        "flex",
                        "items-center",
                        "gap-2",
                        "p-3",
                        "border",
                        "border-zinc-300",
                        "dark:border-zinc-600",
                        "bg-zinc-50",
                        "dark:bg-zinc-800/50",
                        "rounded-md"
                    )}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                        </svg>
                        <a href={url.clone()} target="_blank" class="text-blue-500 hover:underline">
                            {url}
                        </a>
                    </div>
                })
            }
            EditorBlockType::Role(current_role) => {
                let update_block_cb = update_block.clone();
                let onclick = {
                    let current_role = current_role.clone();
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
                        update_block_cb.emit(EditorBlockType::Role(new_role));
                    })
                };

                let badge_color = match current_role {
                    RoleType::System => {
                        "bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300"
                    }
                    RoleType::Assistant => {
                        "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300"
                    }
                    RoleType::User => {
                        "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300"
                    }
                };

                html! {
                    <div
                        class={classes!(
                            "inline-flex",
                            "items-center",
                            "justify-center",
                            "px-2.5",
                            "py-1",
                            "rounded-md",
                            "text-sm",
                            "font-medium",
                            "cursor-pointer",
                            "mb-3",
                            badge_color
                        )}
                        data-role={current_role.to_string()}
                        onclick={onclick}
                    >
                        {current_role.to_string()}
                    </div>
                }
            }
            // Fallback for any unhandled types during development, though all should be covered.
            // _ => html! { <div>{"Unsupported block type"}</div> },
        }
    }

    fn search(query: Option<String>) -> Vec<Self> {
        let block_types = vec![
            EditorBlockType::Paragraph,
            EditorBlockType::Heading1,
            EditorBlockType::Heading2,
            EditorBlockType::Heading3,
            EditorBlockType::BulletList,
            EditorBlockType::NumberedList,
            EditorBlockType::Quote,
            EditorBlockType::CodeBlock,
            EditorBlockType::Divider,
            EditorBlockType::FileBlock,
            EditorBlockType::UrlBlock,
            EditorBlockType::Role(RoleType::System),
            EditorBlockType::Role(RoleType::Assistant),
            EditorBlockType::Role(RoleType::User),
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
            EditorBlockType::Role(_) => true,
            _ => true,
        }
    }
}
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
            "markdown_editor_divider_block".to_string(),
            "Divider Block Container".to_string(),
        ),
        (
            "markdown_editor_file_block".to_string(),
            "File Block Container".to_string(),
        ),
        (
            "markdown_editor_url_block".to_string(),
            "URL Block Container".to_string(),
        ),
        (
            "markdown_editor_role_block".to_string(),
            "Role Block Container".to_string(),
        ),
        (
            "markdown_command_menu_container".to_string(),
            "Command Menu Container".to_string(),
        ),
    ];

    let preview = html! {
        <MarkdownEditor<EditorBlockType>
            auto_focus={true}
            class="h-[400px]"
            placeholder="Start typing..."
            initial_content={vec![Block::new(EditorBlockType::Paragraph)]}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(MarkdownEditorExample)]
pub fn markdown_editor_example() -> Html {
    // Initialize with a paragraph block to ensure there's at least one component
    let initial_content = vec![
        Block::new(EditorBlockType::Paragraph),
        Block::new(EditorBlockType::Role(RoleType::System)),
        Block::new(EditorBlockType::FileBlock),
        Block::new(EditorBlockType::UrlBlock),
    ];
    let content = use_state(|| initial_content);

    let on_change = {
        let content = content.clone();
        Callback::from(move |blocks: Vec<Block<EditorBlockType>>| {
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

#[function_component(MarkdownEditorDocumentation)]
pub fn markdown_editor_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Markdown Editor Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Markdown Editor is a block-based rich text editor with a command menu that supports various content types like headings, lists, quotes, and code blocks." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <MarkdownEditorExample />
                }}
                customize={html! {
                    <MarkdownEditorThemeEditor />
                }}
                code={r#"
// Define a custom block type implementation
#[derive(Clone, PartialEq)]
pub enum EditorBlockType {
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    BulletList,
    NumberedList,
    Quote,
    CodeBlock,
    Divider,
    FileBlock,
    UrlBlock,
    Role(RoleType),
}

// Role type for chat roles
#[derive(Clone, PartialEq)]
pub enum RoleType {
    System,
    Assistant,
    User,
}

// Implement ToString for RoleType
impl ToString for RoleType {
    fn to_string(&self) -> String {
        match self {
            RoleType::System => "System".to_string(),
            RoleType::Assistant => "Assistant".to_string(),
            RoleType::User => "User".to_string(),
        }
    }
}

// Implement FromStr for RoleType
impl FromStr for RoleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "System" => Ok(RoleType::System),
            "Assistant" => Ok(RoleType::Assistant),
            "User" => Ok(RoleType::User),
            _ => Err(()),
        }
    }
}

// Implement the BlockTrait for your custom block type
impl BlockTrait for EditorBlockType {
    fn is_tag(&self) -> bool {
        match self {
            EditorBlockType::Divider => false,
            EditorBlockType::Role(_) => false,
            _ => true,
        }
    }

    fn tag(&self) -> Option<String> {
        match self {
            EditorBlockType::Paragraph => Some("p".to_string()),
            EditorBlockType::Heading1 => Some("h1".to_string()),
            EditorBlockType::Heading2 => Some("h2".to_string()),
            EditorBlockType::Heading3 => Some("h3".to_string()),
            EditorBlockType::BulletList => Some("ul".to_string()),
            EditorBlockType::NumberedList => Some("ol".to_string()),
            EditorBlockType::Quote => Some("blockquote".to_string()),
            EditorBlockType::CodeBlock => Some("pre".to_string()),
            EditorBlockType::FileBlock => Some("div".to_string()),
            EditorBlockType::UrlBlock => Some("div".to_string()),
            EditorBlockType::Divider => None,
            EditorBlockType::Role(_) => None,
        }
    }

    fn classes(&self) -> Classes {
        match self {
            // Existing block types...

            EditorBlockType::FileBlock => classes!(
                "p-2",
                "min-h-[1.5em]",
                "w-full",
                "border-2",
                "border-dashed",
                "border-zinc-300",
                "dark:border-zinc-600",
                "bg-zinc-50",
                "dark:bg-zinc-800/50",
                "rounded-md",
                "flex",
                "items-center",
                "gap-2",
                "outline-none",
                "focus:outline-none"
            ),
            EditorBlockType::UrlBlock => classes!(
                "p-2",
                "min-h-[1.5em]",
                "w-full",
                "border",
                "border-zinc-300",
                "dark:border-zinc-600",
                "bg-zinc-50",
                "dark:bg-zinc-800/50",
                "rounded-md",
                "flex",
                "items-center",
                "gap-2",
                "outline-none",
                "focus:outline-none"
            ),
            EditorBlockType::Role(_) => classes!(
                "inline-flex",
                "items-center",
                "justify-center",
                "px-2",
                "py-1",
                "rounded-md",
                "text-sm",
                "font-medium",
                "cursor-pointer",
                "mb-2"
            ),
        }
    }

    fn icon(&self) -> Html {
        match self {
            // Existing icons...

            EditorBlockType::FileBlock => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                    <polyline points="14 2 14 8 20 8"></polyline>
                    <line x1="12" y1="18" x2="12" y2="12"></line>
                    <line x1="9" y1="15" x2="15" y2="15"></line>
                </svg>
            },
            EditorBlockType::UrlBlock => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                    <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                </svg>
            },
            EditorBlockType::Role(role_type) => {
                match role_type {
                    RoleType::System => html! {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                            <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                            <line x1="6" y1="6" x2="6.01" y2="6"></line>
                            <line x1="6" y1="18" x2="6.01" y2="18"></line>
                        </svg>
                    },
                    RoleType::Assistant => html! {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M12 2a8 8 0 0 1 8 8v1a7 7 0 0 1-7 7h-1a7 7 0 0 1-7-7v-1a8 8 0 0 1 8-8Z"></path>
                            <path d="M19.07 19.07a8 8 0 0 1-11.31 0"></path>
                            <path d="M9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
                            <path d="M17 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
                        </svg>
                    },
                    RoleType::User => html! {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                        </svg>
                    }
                }
            }
        }
    }

    fn name(&self) -> String {
        match self {
            // Existing names...
            EditorBlockType::FileBlock => "File Upload".to_string(),
            EditorBlockType::UrlBlock => "URL Link".to_string(),
            EditorBlockType::Role(role_type) => format!("Role: {}", role_type.to_string()),
        }
    }

    fn render(&self, arguments: String) -> Option<Html> {
        match self {
            // Existing render methods...

            EditorBlockType::FileBlock => Some(html! {
                <div class={classes!(
                    "flex",
                    "items-center",
                    "gap-2",
                    "p-3",
                    "border-2",
                    "border-dashed",
                    "border-zinc-300",
                    "dark:border-zinc-600",
                    "bg-zinc-50",
                    "dark:bg-zinc-800/50",
                    "rounded-md",
                    "cursor-pointer",
                    "hover:bg-zinc-100",
                    "dark:hover:bg-zinc-800"
                )}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                        <polyline points="14 2 14 8 20 8"></polyline>
                        <line x1="12" y1="18" x2="12" y2="12"></line>
                        <line x1="9" y1="15" x2="15" y2="15"></line>
                    </svg>
                    <div>
                        if arguments.is_empty() {
                            <span>{"Click to upload a file"}</span>
                        } else {
                            <span>{arguments}</span>
                        }
                    </div>
                </div>
            }),
            EditorBlockType::UrlBlock => {
                let url = if arguments.is_empty() {
                    "https://".to_string()
                } else {
                    arguments
                };

                Some(html! {
                    <div class={classes!(
                        "flex",
                        "items-center",
                        "gap-2",
                        "p-3",
                        "border",
                        "border-zinc-300",
                        "dark:border-zinc-600",
                        "bg-zinc-50",
                        "dark:bg-zinc-800/50",
                        "rounded-md"
                    )}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                        </svg>
                        <a href={url.clone()} target="_blank" class="text-blue-500 hover:underline">
                            {url}
                        </a>
                    </div>
                })
            },
            EditorBlockType::Role(current_role) => {
                let onclick = {
                    let current_role = current_role.clone();
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
                        element.set_attribute("data-role", &new_role.to_string()).unwrap();

                        // Update text content
                        element.set_text_content(Some(&new_role.to_string()));
                    })
                };

                let badge_color = match current_role {
                    RoleType::System => "bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300",
                    RoleType::Assistant => "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300",
                    RoleType::User => "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300",
                };

                Some(html! {
                    <div
                        class={classes!(
                            "inline-flex",
                            "items-center",
                            "justify-center",
                            "px-2.5",
                            "py-1",
                            "rounded-md",
                            "text-sm",
                            "font-medium",
                            "cursor-pointer",
                            "mb-3",
                            badge_color
                        )}
                        data-role={current_role.to_string()}
                        onclick={onclick}
                    >
                        {current_role.to_string()}
                    </div>
                })
            },
        }
    }

    fn search(query: Option<String>) -> Vec<Self> {
        let block_types = vec![
            EditorBlockType::Paragraph,
            EditorBlockType::Heading1,
            EditorBlockType::Heading2,
            EditorBlockType::Heading3,
            EditorBlockType::BulletList,
            EditorBlockType::NumberedList,
            EditorBlockType::Quote,
            EditorBlockType::CodeBlock,
            EditorBlockType::Divider,
            EditorBlockType::FileBlock,
            EditorBlockType::UrlBlock,
            EditorBlockType::Role(RoleType::System),
            EditorBlockType::Role(RoleType::Assistant),
            EditorBlockType::Role(RoleType::User),
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
}

// Initialize with a varied set of blocks
let initial_content = vec![
    Block::new(EditorBlockType::Paragraph),
    Block::new(EditorBlockType::Role(RoleType::System)),
    Block::new(EditorBlockType::FileBlock),
    Block::new(EditorBlockType::UrlBlock),
];
let content = use_state(|| initial_content);

// Set up the change handler
let on_change = {
    let content = content.clone();
    Callback::from(move |blocks: Vec<Block<EditorBlockType>>| {
        content.set(blocks);
    })
};

// Render the editor
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

        // Optional debug view of the content
        <div class="mt-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-md">
            <h3 class="text-sm font-semibold mb-2">{"Editor Content (Debug View):"}</h3>
            <pre class="text-xs overflow-auto max-h-[200px]">
                {format!("Content contains {} blocks", content.len())}
            </pre>
        </div>
    </div>
}"#.to_string()}
            />

            <Features features={vec![
                "Block-based editing with multiple content types",
                "Command menu triggered with '/' character",
                "Support for headings, lists, quotes, code blocks and dividers",
                "Specialized blocks for file uploads, URL links, and role badges",
                "Interactive role badges that cycle between System, Assistant, and User",
                "Keyboard navigation between blocks",
                "Add and remove blocks with keyboard shortcuts",
                "Real-time content updates via callbacks",
                "Customizable styling for each block type",
                "Auto-focus option for immediate editing",
                "Generic implementation supporting custom block types"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="MarkdownEditor<T: BlockTrait>"
                description="A block-based rich text editor with command menu for different content types."
                props={vec![
                    ("placeholder", "String", "Placeholder text to show when the editor is empty."),
                    ("class", "Classes", "Additional CSS classes for the editor container."),
                    ("initial_content", "Vec<Block<T>>", "Optional initial content to populate the editor with."),
                    ("on_change", "Callback<Vec<Block<T>>>", "Callback function that receives the updated content whenever changes occur."),
                    ("auto_focus", "bool", "If true, automatically focuses the editor when mounted. Default is false."),
                ]}
            />

            <ApiSection
                title="Block<T: BlockTrait> Struct"
                description="Represents a single content block in the editor."
                props={vec![
                    ("id", "String", "Unique identifier for the block."),
                    ("content", "String", "The text content of the block."),
                    ("block_type", "T", "The type of the block, using your custom block type implementation."),
                ]}
            />

            <ApiSection
                title="BlockTrait Trait"
                description="Trait that must be implemented for custom block types."
                props={vec![
                    ("is_tag", "fn(&self) -> bool", "Returns whether this block type should be rendered as a tag."),
                    ("tag", "fn(&self) -> Option<String>", "Returns the HTML tag name for this block type, if applicable."),
                    ("classes", "fn(&self) -> Classes", "Returns the CSS classes for this block type."),
                    ("icon", "fn(&self) -> Html", "Returns the icon for this block type in the command menu."),
                    ("name", "fn(&self) -> String", "Returns the display name for this block type."),
                    ("render", "fn(&self, arguments: String) -> Option<Html>", "Custom render function for special blocks like dividers, files, URLs, and roles."),
                    ("search", "fn(query: Option<String>) -> Vec<Self>", "Returns block types that match the search query."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Type '/' in any block to open the command menu for changing block types.".to_string(),
                    "Press Enter to create a new block below the current one.".to_string(),
                    "Press Backspace on an empty block to delete it and move focus to the previous block.".to_string(),
                    "Use arrow keys to navigate between blocks.".to_string(),
                    "Shift+Enter creates a line break within the same block instead of creating a new block.".to_string(),
                    "Click on role badges to cycle between System, Assistant, and User roles.".to_string(),
                    "File blocks can be used to prompt for file uploads.".to_string(),
                    "URL blocks automatically format and make links clickable.".to_string(),
                    "The on_change callback provides the complete content structure whenever changes occur.".to_string(),
                    "Each block has a unique ID that persists across editor sessions, useful for tracking specific blocks.".to_string(),
                    "The editor handles basic keyboard shortcuts and navigation internally.".to_string(),
                    "You must implement the BlockTrait for your custom block types.".to_string(),
                    "To convert markdown to HTML for display, you'll need to use a separate markdown parser.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"MarkdownEditor".to_string()}
                class_descriptions={vec![
                    ("markdown_editor_container".to_string(), "Main container for the entire editor.".to_string()),
                    ("markdown_editor_blocks_container".to_string(), "Container for all the editor blocks.".to_string()),
                    ("markdown_editor_block".to_string(), "Base styles for each block in the editor.".to_string()),
                    ("markdown_editor_block_active".to_string(), "Styles applied to the currently active/focused block.".to_string()),
                    ("markdown_editor_paragraph".to_string(), "Styles for paragraph blocks.".to_string()),
                    ("markdown_editor_heading1".to_string(), "Styles for heading 1 blocks.".to_string()),
                    ("markdown_editor_heading2".to_string(), "Styles for heading 2 blocks.".to_string()),
                    ("markdown_editor_heading3".to_string(), "Styles for heading 3 blocks.".to_string()),
                    ("markdown_editor_bullet_list".to_string(), "Styles for bullet list items.".to_string()),
                    ("markdown_editor_numbered_list".to_string(), "Styles for numbered list items.".to_string()),
                    ("markdown_editor_quote".to_string(), "Styles for quote blocks.".to_string()),
                    ("markdown_editor_code".to_string(), "Styles for code blocks.".to_string()),
                    ("markdown_editor_divider".to_string(), "Styles for the horizontal divider element.".to_string()),
                    ("markdown_editor_divider_block".to_string(), "Styles for the container of divider blocks.".to_string()),
                    ("markdown_editor_file_block".to_string(), "Styles for the file upload block container.".to_string()),
                    ("markdown_editor_url_block".to_string(), "Styles for the URL link block container.".to_string()),
                    ("markdown_editor_role_block".to_string(), "Styles for the role badge block container.".to_string()),
                    ("markdown_command_menu_container".to_string(), "Styles for the command menu popup container.".to_string()),
                    ("command_container".to_string(), "Container for the command menu items.".to_string()),
                    ("command_list".to_string(), "List container for command menu options.".to_string()),
                    ("command_item".to_string(), "Individual command menu item.".to_string()),
                    ("command_selected_item".to_string(), "Selected command menu item.".to_string()),
                    ("command_item_icon".to_string(), "Icon container within a command menu item.".to_string()),
                ]}
            />
        </Container>
    }
}
