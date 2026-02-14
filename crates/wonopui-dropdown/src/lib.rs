//! Dropdown component for wonopui
//!
//! A dropdown menu built on top of the Popover component.

use std::rc::Rc;
use wonopui_core::merge_classes;
use wonopui_popover::{Popover, PopoverContent, PopoverPosition, PopoverState, PopoverTrigger};
use yew::prelude::*;

pub mod classes {
    pub const DROPDOWN_CONTENT: &str = "py-1 min-w-[160px]";
    pub const DROPDOWN_ITEM: &str = "flex items-center px-4 py-2 text-sm text-gray-700 dark:text-zinc-300 hover:bg-gray-100 dark:hover:bg-zinc-700 cursor-pointer";
    pub const DROPDOWN_ITEM_DISABLED: &str = "opacity-50 cursor-not-allowed hover:bg-transparent dark:hover:bg-transparent";
    pub const DROPDOWN_ITEM_ICON: &str = "mr-2 w-4 h-4";
    pub const DROPDOWN_ITEM_WIDGET: &str = "px-4 py-2";
    pub const DROPDOWN_SEPARATOR: &str = "my-1 border-t border-gray-200 dark:border-zinc-600";
    pub const DROPDOWN_HEADING: &str = "px-4 py-2 text-xs font-semibold text-gray-500 dark:text-zinc-400 uppercase tracking-wider";
}

// Re-export PopoverPosition for convenience
pub use wonopui_popover::PopoverPosition as DropdownPosition;

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    #[prop_or_default]
    pub items: Vec<DropdownItem>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(PopoverPosition::SouthMiddle)]
    pub position: PopoverPosition,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub full_width: bool,
}

#[derive(PartialEq, Clone)]
pub enum DropdownItem {
    Action {
        label: String,
        icon: Option<Html>,
        onclick: Callback<MouseEvent>,
        disabled: bool,
    },
    Widget(Html),
    Separator,
    Heading {
        label: String,
    },
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    let popover_content_class = merge_classes(&[
        classes::DROPDOWN_CONTENT,
        if props.full_width { "w-full" } else { "" },
    ]);

    html! {
        <Popover class={props.class.clone()}>
            <PopoverTrigger>
                { props.children.clone() }
            </PopoverTrigger>
            <PopoverContent class={classes!(popover_content_class)} position={props.position.clone()}>
                { for props.items.iter().map(|item| {
                    match item {
                        DropdownItem::Separator => {
                            html! { <hr class={classes::DROPDOWN_SEPARATOR} /> }
                        },
                        _ => {
                            html! {
                                <DropdownItemComponent
                                    content={item.clone()}
                                    full_width={props.full_width}
                                />
                            }
                        }
                    }
                }) }
            </PopoverContent>
        </Popover>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownItemComponentProps {
    pub content: DropdownItem,
    #[prop_or_default]
    pub full_width: bool,
}

#[function_component(DropdownItemComponent)]
fn dropdown_item_component(props: &DropdownItemComponentProps) -> Html {
    let popover_state =
        use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");

    match &props.content {
        DropdownItem::Action {
            label,
            icon,
            onclick,
            disabled,
        } => {
            let onclick = {
                let onclick = onclick.clone();
                let toggle = popover_state.toggle.clone();
                if *disabled {
                    Callback::from(|_| {})
                } else {
                    Callback::from(move |e: MouseEvent| {
                        onclick.emit(e);
                        toggle.emit(());
                    })
                }
            };

            let item_class = merge_classes(&[
                classes::DROPDOWN_ITEM,
                if props.full_width { "w-full" } else { "" },
                if *disabled { classes::DROPDOWN_ITEM_DISABLED } else { "" },
            ]);

            html! {
                <div class={item_class} {onclick}>
                    { if let Some(icon) = icon {
                        html! { <span class={classes::DROPDOWN_ITEM_ICON}>{ icon.clone() }</span> }
                    } else {
                        html! {}
                    }}
                    <span>{ label }</span>
                </div>
            }
        }
        DropdownItem::Widget(content) => {
            html! {
                <div class={classes::DROPDOWN_ITEM_WIDGET}>
                    { content.clone() }
                </div>
            }
        }
        DropdownItem::Heading { label } => {
            html! {
                <div class={classes::DROPDOWN_HEADING}>
                    <span>{ label }</span>
                </div>
            }
        }
        DropdownItem::Separator => {
            html! {}
        }
    }
}
