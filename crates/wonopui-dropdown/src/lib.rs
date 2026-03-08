//! Dropdown component for wonopui
//!
//! A dropdown menu built on top of the Popover component.

use std::rc::Rc;
use wonopui_core::merge_classes;
use wonopui_popover::{Popover, PopoverContent, PopoverPosition, PopoverState, PopoverTrigger};
use yew::prelude::*;

/// Default CSS classes for dropdown styling (shadcn v4 style).
pub mod classes {
    /// Content container styles - premium with more padding.
    pub const DROPDOWN_CONTENT: &str = "p-1.5 min-w-[10rem] overflow-hidden";
    
    /// Item styles - premium with better spacing and transitions.
    pub const DROPDOWN_ITEM: &str = "relative flex cursor-default select-none items-center gap-3 rounded-lg px-3 py-2.5 text-sm text-zinc-700 dark:text-zinc-300 outline-none transition-all duration-150 focus:bg-zinc-100 dark:focus:bg-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-900 dark:hover:text-zinc-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-zinc-500 dark:[&_svg:not([class*='text-'])]:text-zinc-400";
    
    /// Destructive item variant.
    pub const DROPDOWN_ITEM_DESTRUCTIVE: &str = "text-red-600 dark:text-red-400 focus:bg-red-50 dark:focus:bg-red-950/50 focus:text-red-600 dark:focus:text-red-400 [&_svg]:!text-red-600 dark:[&_svg]:!text-red-400";
    
    /// Disabled item styles.
    pub const DROPDOWN_ITEM_DISABLED: &str = "pointer-events-none opacity-50";
    
    /// Item icon styles.
    pub const DROPDOWN_ITEM_ICON: &str = "size-4 shrink-0 flex items-center justify-center";
    
    /// Widget container styles.
    pub const DROPDOWN_ITEM_WIDGET: &str = "px-3 py-2";
    
    /// Separator styles.
    pub const DROPDOWN_SEPARATOR: &str = "my-1.5 h-px bg-zinc-200 dark:bg-zinc-800";
    
    /// Heading/label styles - premium uppercase styling.
    pub const DROPDOWN_HEADING: &str = "px-3 py-2 text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide";
    
    /// Shortcut text styles.
    pub const DROPDOWN_SHORTCUT: &str = "ml-auto text-xs tracking-widest text-zinc-400 dark:text-zinc-500";
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
                            html! { <hr data-slot="dropdown-menu-separator" class={classes::DROPDOWN_SEPARATOR} /> }
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
                if *disabled {
                    classes::DROPDOWN_ITEM_DISABLED
                } else {
                    ""
                },
            ]);

            html! {
                <div data-slot="dropdown-menu-item" class={item_class} {onclick}>
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
                <div data-slot="dropdown-menu-widget" class={classes::DROPDOWN_ITEM_WIDGET}>
                    { content.clone() }
                </div>
            }
        }
        DropdownItem::Heading { label } => {
            html! {
                <div data-slot="dropdown-menu-label" class={classes::DROPDOWN_HEADING}>
                    <span>{ label }</span>
                </div>
            }
        }
        DropdownItem::Separator => {
            html! {}
        }
    }
}
