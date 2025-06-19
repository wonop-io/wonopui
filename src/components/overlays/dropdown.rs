use super::popover::{Popover, PopoverContent, PopoverPosition, PopoverState, PopoverTrigger};
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use std::rc::Rc;
use yew::prelude::*;

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

#[cfg_attr(feature = "ssr", allow(unused_variables))]
#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let popover_content_class = if props.full_width {
        classes!(&brandguide.dropdown_content, "w-full")
    } else {
        classes!(&brandguide.dropdown_content)
    };

    html! {
        <Popover class={ props.class.clone()}>
            <PopoverTrigger>
                { props.children.clone() }
            </PopoverTrigger>
            <PopoverContent class={popover_content_class} position={props.position.clone()}>
                { for props.items.iter().map(|item| {
                    match item {
                        DropdownItem::Separator => {
                            html! { <hr class={classes!(&brandguide.dropdown_separator)} /> }
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

#[cfg_attr(feature = "ssr", allow(unused_variables))]
#[function_component(DropdownItemComponent)]
fn dropdown_item_component(props: &DropdownItemComponentProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
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
            let item_class = if props.full_width {
                classes!(&brandguide.dropdown_item, "w-full")
            } else {
                classes!(&brandguide.dropdown_item)
            };

            let class = if *disabled {
                classes!(&brandguide.dropdown_item_disabled)
            } else {
                Classes::default()
            };
            html! {
                <div class={classes!(class, item_class)} {onclick}>
                    { if let Some(icon) = icon {
                        html! { <span class={classes!(&brandguide.dropdown_item_icon)}>{ icon.clone() }</span> }
                    } else {
                        html! {}
                    }}
                    <span>{ label }</span>
                </div>
            }
        }
        DropdownItem::Widget(content) => {
            html! {
                <div class={classes!(&brandguide.dropdown_item_widget)}>
                    { content.clone() }
                </div>
            }
        }
        DropdownItem::Heading { label } => {
            html! {
                <div class={classes!(&brandguide.dropdown_heading)}>
                    <span>{ label }</span>
                </div>
            }
        }
        DropdownItem::Separator => {
            html! {}
        }
    }
}
