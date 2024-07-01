use super::popover::{Popover, PopoverContent, PopoverPosition, PopoverTrigger};
use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    #[prop_or_default]
    pub items: Vec<DropdownItemProps>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(PopoverPosition::SouthMiddle)]
    pub position: PopoverPosition,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(PartialEq, Clone)]
pub struct DropdownItemProps {
    pub label: String,
    pub icon: Option<Html>,
    pub onclick: Callback<MouseEvent>,
    pub is_separator: bool,
}

impl Default for DropdownItemProps {
    fn default() -> Self {
        Self {
            label: String::new(),
            icon: None,
            onclick: Callback::from(|_| {}),
            is_separator: false,
        }
    }
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    html! {
        <Popover class={ props.class.clone()}>
            <PopoverTrigger>
                { props.children.clone() }
            </PopoverTrigger>
            <PopoverContent class={BRANDGUIDE.dropdown_content} position={props.position.clone()}>
                { for props.items.iter().map(|item| {
                    if item.is_separator {
                        html! { <hr class={classes!(BRANDGUIDE.dropdown_separator)} /> }
                    } else {
                        html! {
                            <div class={classes!(BRANDGUIDE.dropdown_item)} onclick={item.onclick.clone()}>
                                { if let Some(icon) = &item.icon {
                                    html! { <span class={classes!(BRANDGUIDE.dropdown_item_icon)}>{ icon.clone() }</span> }
                                } else {
                                    html! {}
                                }}
                                <span>{ &item.label }</span>
                            </div>
                        }
                    }
                }) }
            </PopoverContent>
        </Popover>
    }
}
