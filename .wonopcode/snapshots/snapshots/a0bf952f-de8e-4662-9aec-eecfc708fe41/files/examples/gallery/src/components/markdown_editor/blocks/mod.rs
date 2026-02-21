use crate::components::markdown_editor::EditorBlockType;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};
use wonopui::prelude::*;
use wonopui::*;
use yew::prelude::*;

// Icon components for each block type
#[function_component(ParagraphIcon)]
pub fn paragraph_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="17" y1="10" x2="3" y2="10"></line>
            <line x1="21" y1="6" x2="3" y2="6"></line>
            <line x1="21" y1="14" x2="3" y2="14"></line>
            <line x1="17" y1="18" x2="3" y2="18"></line>
        </svg>
    }
}

#[function_component(Heading1Icon)]
pub fn heading1_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M4 12h8"></path>
            <path d="M4 18V6"></path>
            <path d="M12 18V6"></path>
            <path d="m17 12 3 4"></path>
            <path d="m20 12-3 4"></path>
        </svg>
    }
}

#[function_component(Heading2Icon)]
pub fn heading2_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M4 12h8"></path>
            <path d="M4 18V6"></path>
            <path d="M12 18V6"></path>
            <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"></path>
        </svg>
    }
}

#[function_component(Heading3Icon)]
pub fn heading3_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M4 12h8"></path>
            <path d="M4 18V6"></path>
            <path d="M12 18V6"></path>
            <path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"></path>
            <path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"></path>
        </svg>
    }
}

#[function_component(BulletListIcon)]
pub fn bullet_list_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="9" y1="6" x2="20" y2="6"></line>
            <line x1="9" y1="12" x2="20" y2="12"></line>
            <line x1="9" y1="18" x2="20" y2="18"></line>
            <circle cx="4" cy="6" r="2"></circle>
            <circle cx="4" cy="12" r="2"></circle>
            <circle cx="4" cy="18" r="2"></circle>
        </svg>
    }
}

#[function_component(NumberedListIcon)]
pub fn numbered_list_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="10" y1="6" x2="21" y2="6"></line>
            <line x1="10" y1="12" x2="21" y2="12"></line>
            <line x1="10" y1="18" x2="21" y2="18"></line>
            <path d="M4 6h1v4"></path>
            <path d="M4 10h2"></path>
            <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path>
        </svg>
    }
}

#[function_component(QuoteIcon)]
pub fn quote_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
            <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>
        </svg>
    }
}

#[function_component(CodeBlockIcon)]
pub fn code_block_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="16 18 22 12 16 6"></polyline>
            <polyline points="8 6 2 12 8 18"></polyline>
        </svg>
    }
}

#[function_component(DividerIcon)]
pub fn divider_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" y1="12" x2="21" y2="12"></line>
        </svg>
    }
}

#[function_component(ChecklistIcon)]
pub fn checklist_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="5" width="6" height="6" rx="1"></rect>
            <path d="m3 17 2 2 4-4"></path>
            <path d="M13 6h8"></path>
            <path d="M13 12h8"></path>
            <path d="M13 18h8"></path>
        </svg>
    }
}

#[function_component(TableIcon)]
pub fn table_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 3v18"></path>
            <rect width="18" height="18" x="3" y="3" rx="2"></rect>
            <path d="M3 9h18"></path>
            <path d="M3 15h18"></path>
        </svg>
    }
}

#[function_component(FileBlockIcon)]
pub fn file_block_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <line x1="12" y1="18" x2="12" y2="12"></line>
            <line x1="9" y1="15" x2="15" y2="15"></line>
        </svg>
    }
}

#[function_component(UrlBlockIcon)]
pub fn url_block_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
        </svg>
    }
}

#[function_component(SystemRoleIcon)]
pub fn system_role_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
            <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
            <line x1="6" y1="6" x2="6.01" y2="6"></line>
            <line x1="6" y1="18" x2="6.01" y2="18"></line>
        </svg>
    }
}

#[function_component(AssistantRoleIcon)]
pub fn assistant_role_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 2a8 8 0 0 1 8 8v1a7 7 0 0 1-7 7h-1a7 7 0 0 1-7-7v-1a8 8 0 0 1 8-8Z"></path>
            <path d="M19.07 19.07a8 8 0 0 1-11.31 0"></path>
            <path d="M9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
            <path d="M17 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
        </svg>
    }
}

#[function_component(UserRoleIcon)]
pub fn user_role_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
            <circle cx="12" cy="7" r="4"></circle>
        </svg>
    }
}

// Define RoleType enum
#[derive(Debug, Clone, PartialEq)]
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

// Generic Block Component for editable blocks with ContentEditableWithCommands
#[derive(Properties, PartialEq)]
pub struct GenericBlockProps<T: Clone + PartialEq + 'static> {
    pub tag: &'static str,
    #[prop_or_default]
    pub content: String,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub has_focus: bool,
    #[prop_or(true)]
    pub contenteditable: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(T, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<T>,
}

#[function_component(GenericBlock)]
pub fn generic_block<T: Clone + PartialEq + 'static>(props: &GenericBlockProps<T>) -> Html {
    let node_ref = use_node_ref();

    // Check if we should use ContentEditableWithCommands
    let use_commands = !props.command_triggers.is_empty() && !props.command_options.is_empty();

    // Effect for focus when not using ContentEditableWithCommands
    // Always create the effect at the top level and conditionally check inside
    use_effect_with(
        (
            props.has_focus,
            node_ref.clone(),
            props.content.clone(),
            use_commands,
        ),
        |(has_focus, node_ref, content, use_commands)| {
            // Only update the HTML if we're not using ContentEditableWithCommands
            if !*use_commands {
                if let Some(element) = node_ref.cast::<HtmlElement>() {
                    // Instead of using set_text_content which doesn't preserve newlines,
                    // use innerHTML to preserve formatting
                    element.set_inner_html(&content.replace("\n", "<br>"));

                    if *has_focus {
                        let _ = element.focus();
                    } else {
                        let _ = element.blur();
                    }
                }
            }

            || ()
        },
    );

    // Convert input callback
    let on_input_string = {
        let on_input = props.on_input.clone();
        Callback::from(move |content: String| {
            on_input.emit(content);
        })
    };

    // if use_commands && props.contenteditable {
    html! {
        <ContentEditableWithCommands<T>
            tag={props.tag}
            content={props.content.clone()}
            class={props.classes.clone()}
            placeholder=""
            is_active={props.has_focus}
            on_input={on_input_string}
            on_keydown={props.onkeydown.clone()}
            on_focus={props.onfocus.clone()}
            on_blur={props.onblur.clone()}
            command_triggers={props.command_triggers.clone()}
            command_options={props.command_options.clone()}
            on_command_select={props.on_command_select.clone()}
        />
    }
    /*
    } else {
        html! {
            <@{props.tag}
                ref={node_ref}
                key="editable"
                class={props.classes.clone()}
                contenteditable={props.contenteditable.to_string()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlElement>();
                    let content = input.inner_text();
                    on_input_string.emit(content);
                })}
                onkeydown={props.onkeydown.clone()}
                onfocus={props.onfocus.clone()}
                onblur={props.onblur.clone()}
            >
                {props.children.clone()}
            </@>
        }
    }
    */
}

// Component for paragraph block rendering
#[derive(Properties, PartialEq)]
pub struct ParagraphProps {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(ParagraphBlock)]
pub fn paragraph_block(props: &ParagraphProps) -> Html {
    html! {
        <GenericBlock<EditorBlockType>
            tag="div"
            content={props.content.clone()}
            classes={classes!(
                "px-3",
                "py-2",
                "min-h-[1.75em]",
                "w-full",
                "text-sm",
                "leading-relaxed",
                "text-foreground",
                "outline-none",
                "focus:outline-none",
                "[&[contenteditable=true]:empty:before]:content-[attr(data-placeholder)]",
                "[&[contenteditable=true]:empty:before]:text-muted-foreground"
            )}
            on_input={props.on_input.clone()}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            has_focus={props.has_focus}
            command_triggers={props.command_triggers.clone()}
            command_options={props.command_options.clone()}
            on_command_select={props.on_command_select.clone()}
        />
    }
}

// Component for heading1 block rendering
#[derive(Properties, PartialEq)]
pub struct Heading1Props {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(Heading1Block)]
pub fn heading1_block(props: &Heading1Props) -> Html {
    html! {
        <GenericBlock<EditorBlockType>
            tag="div"
            content={props.content.clone()}
            classes={classes!(
                "px-3",
                "py-2",
                "min-h-[1.75em]",
                "w-full",
                "scroll-m-20",
                "text-3xl",
                "font-bold",
                "tracking-tight",
                "text-foreground",
                "outline-none",
                "focus:outline-none"
            )}
            on_input={props.on_input.clone()}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            has_focus={props.has_focus}
            command_triggers={props.command_triggers.clone()}
            command_options={props.command_options.clone()}
            on_command_select={props.on_command_select.clone()}
        />
    }
}

// Component for heading2 block rendering
#[derive(Properties, PartialEq)]
pub struct Heading2Props {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(Heading2Block)]
pub fn heading2_block(props: &Heading2Props) -> Html {
    html! {
        <GenericBlock<EditorBlockType>
            tag="div"
            content={props.content.clone()}
            classes={classes!(
                "px-3",
                "py-2",
                "min-h-[1.75em]",
                "w-full",
                "scroll-m-20",
                "text-2xl",
                "font-semibold",
                "tracking-tight",
                "text-foreground",
                "border-b",
                "border-border",
                "pb-2",
                "outline-none",
                "focus:outline-none"
            )}
            on_input={props.on_input.clone()}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            has_focus={props.has_focus}
            command_triggers={props.command_triggers.clone()}
            command_options={props.command_options.clone()}
            on_command_select={props.on_command_select.clone()}
        />
    }
}

// Component for heading3 block rendering
#[derive(Properties, PartialEq)]
pub struct Heading3Props {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(Heading3Block)]
pub fn heading3_block(props: &Heading3Props) -> Html {
    html! {
        <GenericBlock<EditorBlockType>
            tag="div"
            content={props.content.clone()}
            classes={classes!(
                "px-3",
                "py-2",
                "min-h-[1.75em]",
                "w-full",
                "scroll-m-20",
                "text-xl",
                "font-semibold",
                "tracking-tight",
                "text-foreground",
                "outline-none",
                "focus:outline-none"
            )}
            on_input={props.on_input.clone()}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            has_focus={props.has_focus}
            command_triggers={props.command_triggers.clone()}
            command_options={props.command_options.clone()}
            on_command_select={props.on_command_select.clone()}
        />
    }
}

// Component for bullet list block rendering
#[derive(Properties, PartialEq)]
pub struct BulletListProps {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(BulletListBlock)]
pub fn bullet_list_block(props: &BulletListProps) -> Html {
    html! {
        <div class="flex items-start gap-2 px-3 py-1">
            // Bullet point
            <div class="mt-2.5 h-1.5 w-1.5 shrink-0 rounded-full bg-foreground" />
            <GenericBlock<EditorBlockType>
                tag="div"
                content={props.content.clone()}
                classes={classes!(
                    "py-1",
                    "min-h-[1.75em]",
                    "w-full",
                    "text-sm",
                    "leading-relaxed",
                    "text-foreground",
                    "outline-none",
                    "focus:outline-none"
                )}
                on_input={props.on_input.clone()}
                onkeydown={props.onkeydown.clone()}
                onfocus={props.onfocus.clone()}
                onblur={props.onblur.clone()}
                has_focus={props.has_focus}
                command_triggers={props.command_triggers.clone()}
                command_options={props.command_options.clone()}
                on_command_select={props.on_command_select.clone()}
            />
        </div>
    }
}

// Component for numbered list block rendering
#[derive(Properties, PartialEq)]
pub struct NumberedListProps {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(NumberedListBlock)]
pub fn numbered_list_block(props: &NumberedListProps) -> Html {
    html! {
        <div class="flex items-start gap-2 px-3 py-1">
            // Number indicator (would need actual number from parent in real impl)
            <span class="mt-1 text-sm font-medium text-muted-foreground select-none min-w-[1.5em]">{"1."}</span>
            <GenericBlock<EditorBlockType>
                tag="div"
                content={props.content.clone()}
                classes={classes!(
                    "py-1",
                    "min-h-[1.75em]",
                    "w-full",
                    "text-sm",
                    "leading-relaxed",
                    "text-foreground",
                    "outline-none",
                    "focus:outline-none"
                )}
                on_input={props.on_input.clone()}
                onkeydown={props.onkeydown.clone()}
                onfocus={props.onfocus.clone()}
                onblur={props.onblur.clone()}
                has_focus={props.has_focus}
                command_triggers={props.command_triggers.clone()}
                command_options={props.command_options.clone()}
                on_command_select={props.on_command_select.clone()}
            />
        </div>
    }
}

// Component for quote block rendering
#[derive(Properties, PartialEq)]
pub struct QuoteProps {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(QuoteBlock)]
pub fn quote_block(props: &QuoteProps) -> Html {
    html! {
        <GenericBlock<EditorBlockType>
            tag="div"
            content={props.content.clone()}
            classes={classes!(
                "px-4",
                "py-2",
                "min-h-[1.75em]",
                "w-full",
                "border-l-2",
                "border-border",
                "bg-muted/50",
                "text-sm",
                "italic",
                "text-muted-foreground",
                "outline-none",
                "focus:outline-none"
            )}
            on_input={props.on_input.clone()}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            has_focus={props.has_focus}
            command_triggers={props.command_triggers.clone()}
            command_options={props.command_options.clone()}
            on_command_select={props.on_command_select.clone()}
        />
    }
}

// Component for code block rendering
#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    pub content: String,
    pub on_input: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(CodeBlockBlock)]
pub fn code_block_block(props: &CodeBlockProps) -> Html {
    html! {
        <div class="relative rounded-lg border border-border bg-muted">
            // Code language badge (optional - for future enhancement)
            <div class="absolute right-2 top-2">
                <span class="text-xs text-muted-foreground font-medium px-2 py-0.5 rounded bg-background/50">
                    {"code"}
                </span>
            </div>
            <GenericBlock<EditorBlockType>
                tag="div"
                content={props.content.clone()}
                classes={classes!(
                    "p-4",
                    "min-h-[3em]",
                    "w-full",
                    "font-mono",
                    "text-sm",
                    "leading-relaxed",
                    "text-foreground",
                    "whitespace-pre-wrap",
                    "outline-none",
                    "focus:outline-none"
                )}
                on_input={props.on_input.clone()}
                onkeydown={props.onkeydown.clone()}
                onfocus={props.onfocus.clone()}
                onblur={props.onblur.clone()}
                has_focus={props.has_focus}
                command_triggers={props.command_triggers.clone()}
                command_options={props.command_options.clone()}
                on_command_select={props.on_command_select.clone()}
            />
        </div>
    }
}

// Component for divider block
#[derive(Properties, PartialEq)]
pub struct DividerProps {
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
}

#[function_component(DividerBlock)]
pub fn divider_block(props: &DividerProps) -> Html {
    html! {
        <div 
            class="px-3 py-4"
            tabindex="0"
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
        >
            <hr class="border-t border-border" />
        </div>
    }
}

// Component for checklist block rendering
#[derive(Properties, PartialEq)]
pub struct ChecklistProps {
    pub content: String,
    pub checked: bool,
    pub on_input: Callback<String>,
    pub on_toggle: Callback<bool>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
    #[prop_or_default]
    pub command_triggers: Vec<String>,
    #[prop_or_default]
    pub command_options: Vec<(EditorBlockType, String, String, Option<Html>)>,
    #[prop_or_default]
    pub on_command_select: Callback<EditorBlockType>,
}

#[function_component(ChecklistBlock)]
pub fn checklist_block(props: &ChecklistProps) -> Html {
    let checked = props.checked;
    let on_toggle = props.on_toggle.clone();
    
    let on_checkbox_click = Callback::from(move |_: MouseEvent| {
        on_toggle.emit(!checked);
    });

    html! {
        <div class="flex items-start gap-2 p-2">
            <button
                type="button"
                onclick={on_checkbox_click}
                class={classes!(
                    "flex-shrink-0",
                    "w-5",
                    "h-5",
                    "mt-0.5",
                    "rounded",
                    "border-2",
                    "border-zinc-300",
                    "dark:border-zinc-600",
                    "flex",
                    "items-center",
                    "justify-center",
                    "cursor-pointer",
                    "transition-colors",
                    if props.checked { "bg-blue-500 border-blue-500" } else { "bg-transparent" }
                )}
            >
                if props.checked {
                    <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path>
                    </svg>
                }
            </button>
            <div class={classes!(
                "flex-1",
                if props.checked { "line-through text-zinc-400 dark:text-zinc-500" } else { "" }
            )}>
                <GenericBlock<EditorBlockType>
                    tag="div"
                    content={props.content.clone()}
                    classes={classes!(
                        "min-h-[1.5em]",
                        "w-full",
                        "outline-hidden",
                        "focus:outline-hidden"
                    )}
                    on_input={props.on_input.clone()}
                    onkeydown={props.onkeydown.clone()}
                    onfocus={props.onfocus.clone()}
                    onblur={props.onblur.clone()}
                    has_focus={props.has_focus}
                    command_triggers={props.command_triggers.clone()}
                    command_options={props.command_options.clone()}
                    on_command_select={props.on_command_select.clone()}
                />
            </div>
        </div>
    }
}

// Component for file block rendering
#[derive(Properties, PartialEq)]
pub struct FileBlockProps {
    pub content: String,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
}

#[function_component(FileBlockBlock)]
pub fn file_block_block(props: &FileBlockProps) -> Html {
    let node_ref = use_node_ref();

    // Effect to handle focusing/blurring based on has_focus prop
    use_effect_with(
        (props.has_focus, node_ref.clone()),
        |(has_focus, node_ref)| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                if *has_focus {
                    let _ = element.focus();
                } else {
                    let _ = element.blur();
                }
            }
            || ()
        },
    );

    html! {
        <div
            ref={node_ref}
            tabindex="0"
            class={classes!(
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
                "outline-hidden",
                "focus:outline-hidden"
            )}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                <polyline points="14 2 14 8 20 8"></polyline>
                <line x1="12" y1="18" x2="12" y2="12"></line>
                <line x1="9" y1="15" x2="15" y2="15"></line>
            </svg>
            <div>
                if props.content.is_empty() {
                    <span>{"Click to upload a file"}</span>
                } else {
                    <span>{props.content.clone()}</span>
                }
            </div>
        </div>
    }
}

// Component for URL block rendering
#[derive(Properties, PartialEq)]
pub struct UrlBlockProps {
    pub content: String,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
}

#[function_component(UrlBlockBlock)]
pub fn url_block_block(props: &UrlBlockProps) -> Html {
    let node_ref = use_node_ref();
    let url = if props.content.is_empty() {
        "https://".to_string()
    } else {
        props.content.clone()
    };

    // Effect to handle focusing/blurring based on has_focus prop
    use_effect_with(
        (props.has_focus, node_ref.clone()),
        |(has_focus, node_ref)| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                if *has_focus {
                    let _ = element.focus();
                } else {
                    let _ = element.blur();
                }
            }
            || ()
        },
    );

    html! {
        <div
            ref={node_ref}
            tabindex="0"
            class={classes!(
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
                "outline-hidden",
                "focus:outline-hidden"
            )}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
            </svg>
            <a href={url.clone()} target="_blank" class="text-blue-500 hover:underline">
                {url}
            </a>
        </div>
    }
}

// Component for role block rendering
#[derive(Properties, PartialEq)]
pub struct RoleProps {
    pub role_type: RoleType,
    pub content: String,
    pub onclick: Callback<MouseEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
}

#[function_component(RoleBlock)]
pub fn role_block(props: &RoleProps) -> Html {
    let node_ref = use_node_ref();
    let badge_color = match props.role_type {
        RoleType::System => {
            "bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300"
        }
        RoleType::Assistant => {
            "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300"
        }
        RoleType::User => "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300",
    };

    // Effect to handle focusing/blurring based on has_focus prop
    use_effect_with(
        (props.has_focus, node_ref.clone()),
        |(has_focus, node_ref)| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                if *has_focus {
                    let _ = element.focus();
                } else {
                    let _ = element.blur();
                }
            }
            || ()
        },
    );

    html! {
        <div
            ref={node_ref}
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
                badge_color,
                "outline-hidden"  // Add this to remove focus outline if needed
            )}
            data-role={props.role_type.to_string()}
            onclick={props.onclick.clone()}
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            tabindex="0"  // Make the div focusable
        >
            {props.role_type.to_string()}
        </div>
    }
}

// Table data structure
#[derive(Debug, Clone, PartialEq)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Default for TableData {
    fn default() -> Self {
        Self {
            headers: vec!["Column 1".to_string(), "Column 2".to_string(), "Column 3".to_string()],
            rows: vec![
                vec!["".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string()],
            ],
        }
    }
}

impl TableData {
    /// Parse table data from markdown table format
    pub fn from_content(content: &str) -> Self {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Self::default();
        }

        // Parse header row
        let headers: Vec<String> = lines[0]
            .split('|')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();

        if headers.is_empty() {
            return Self::default();
        }

        // Skip separator line if present
        let data_start = if lines.len() > 1 && lines[1].contains('-') { 2 } else { 1 };

        // Parse data rows
        let rows: Vec<Vec<String>> = lines[data_start..]
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let cells: Vec<String> = line
                    .split('|')
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string())
                    .collect();
                // Pad with empty strings if needed
                let mut padded = cells;
                while padded.len() < headers.len() {
                    padded.push(String::new());
                }
                padded
            })
            .collect();

        if rows.is_empty() {
            let num_cols = headers.len();
            Self {
                headers,
                rows: vec![vec!["".to_string(); num_cols]],
            }
        } else {
            Self { headers, rows }
        }
    }

    /// Convert table data to markdown table format
    pub fn to_content(&self) -> String {
        let mut result = String::new();

        // Header row
        result.push_str("| ");
        result.push_str(&self.headers.join(" | "));
        result.push_str(" |\n");

        // Separator row
        result.push_str("| ");
        result.push_str(&self.headers.iter().map(|_| "---").collect::<Vec<_>>().join(" | "));
        result.push_str(" |\n");

        // Data rows
        for row in &self.rows {
            result.push_str("| ");
            result.push_str(&row.join(" | "));
            result.push_str(" |\n");
        }

        result.trim_end().to_string()
    }

    /// Add a new row to the table
    pub fn add_row(&mut self) {
        self.rows.push(vec!["".to_string(); self.headers.len()]);
    }

    /// Add a new column to the table
    pub fn add_column(&mut self) {
        self.headers.push(format!("Column {}", self.headers.len() + 1));
        for row in &mut self.rows {
            row.push(String::new());
        }
    }

    /// Remove a row from the table
    pub fn remove_row(&mut self, index: usize) {
        if self.rows.len() > 1 && index < self.rows.len() {
            self.rows.remove(index);
        }
    }

    /// Remove a column from the table
    pub fn remove_column(&mut self, index: usize) {
        if self.headers.len() > 1 && index < self.headers.len() {
            self.headers.remove(index);
            for row in &mut self.rows {
                if index < row.len() {
                    row.remove(index);
                }
            }
        }
    }
}

// Component for table block rendering
#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub content: String,
    pub on_update: Callback<String>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub has_focus: bool,
}

#[function_component(TableBlock)]
pub fn table_block(props: &TableProps) -> Html {
    let table_data = use_state(|| TableData::from_content(&props.content));
    let node_ref = use_node_ref();

    // Effect to parse content changes
    {
        let table_data = table_data.clone();
        let content = props.content.clone();
        use_effect_with(content, move |content| {
            table_data.set(TableData::from_content(content));
            || ()
        });
    }

    // Effect for focus management
    use_effect_with(
        (props.has_focus, node_ref.clone()),
        |(has_focus, node_ref)| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                if *has_focus {
                    let _ = element.focus();
                }
            }
            || ()
        },
    );

    let on_header_change = {
        let table_data = table_data.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |(index, value): (usize, String)| {
            let mut data = (*table_data).clone();
            if index < data.headers.len() {
                data.headers[index] = value;
                on_update.emit(data.to_content());
                table_data.set(data);
            }
        })
    };

    let on_cell_change = {
        let table_data = table_data.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |(row, col, value): (usize, usize, String)| {
            let mut data = (*table_data).clone();
            if row < data.rows.len() && col < data.rows[row].len() {
                data.rows[row][col] = value;
                on_update.emit(data.to_content());
                table_data.set(data);
            }
        })
    };

    let on_add_row = {
        let table_data = table_data.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |_: MouseEvent| {
            let mut data = (*table_data).clone();
            data.add_row();
            on_update.emit(data.to_content());
            table_data.set(data);
        })
    };

    let on_add_column = {
        let table_data = table_data.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |_: MouseEvent| {
            let mut data = (*table_data).clone();
            data.add_column();
            on_update.emit(data.to_content());
            table_data.set(data);
        })
    };

    html! {
        <div 
            ref={node_ref}
            tabindex="0"
            class="w-full overflow-x-auto p-2"
            onkeydown={props.onkeydown.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
        >
            <table class="w-full border-collapse border border-zinc-300 dark:border-zinc-600">
                <thead>
                    <tr class="bg-zinc-100 dark:bg-zinc-800">
                        {
                            table_data.headers.iter().enumerate().map(|(i, header)| {
                                let on_header_change = on_header_change.clone();
                                let value = header.clone();
                                html! {
                                    <th class="border border-zinc-300 dark:border-zinc-600 p-2">
                                        <input
                                            type="text"
                                            value={value}
                                            class="w-full bg-transparent border-none outline-none font-bold text-center"
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                on_header_change.emit((i, input.value()));
                                            })}
                                        />
                                    </th>
                                }
                            }).collect::<Html>()
                        }
                    </tr>
                </thead>
                <tbody>
                    {
                        table_data.rows.iter().enumerate().map(|(row_idx, row)| {
                            html! {
                                <tr class="hover:bg-zinc-50 dark:hover:bg-zinc-800/50">
                                    {
                                        row.iter().enumerate().map(|(col_idx, cell)| {
                                            let on_cell_change = on_cell_change.clone();
                                            let value = cell.clone();
                                            html! {
                                                <td class="border border-zinc-300 dark:border-zinc-600 p-2">
                                                    <input
                                                        type="text"
                                                        value={value}
                                                        class="w-full bg-transparent border-none outline-none"
                                                        oninput={Callback::from(move |e: InputEvent| {
                                                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                            on_cell_change.emit((row_idx, col_idx, input.value()));
                                                        })}
                                                    />
                                                </td>
                                            }
                                        }).collect::<Html>()
                                    }
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
            <div class="flex gap-2 mt-2">
                <button
                    type="button"
                    onclick={on_add_row}
                    class="text-xs px-2 py-1 bg-zinc-200 dark:bg-zinc-700 rounded hover:bg-zinc-300 dark:hover:bg-zinc-600"
                >
                    {"+ Add Row"}
                </button>
                <button
                    type="button"
                    onclick={on_add_column}
                    class="text-xs px-2 py-1 bg-zinc-200 dark:bg-zinc-700 rounded hover:bg-zinc-300 dark:hover:bg-zinc-600"
                >
                    {"+ Add Column"}
                </button>
            </div>
        </div>
    }
}
