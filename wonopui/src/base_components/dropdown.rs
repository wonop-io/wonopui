use super::popover::{Popover, PopoverContent, PopoverPosition, PopoverTrigger, PopoverState};
use crate::config::BRANDGUIDE;
use yew::prelude::*;
use std::rc::Rc;

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

#[derive(PartialEq, Clone, Properties)]
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
                        let item_onclick = item.onclick.clone();
                        html! {
                            <DropdownItem
                                label={item.label.clone()}
                                icon={item.icon.clone()}
                                onclick={item_onclick}
                                is_separator={item.is_separator}
                            />
                        }
                    }
                }) }
            </PopoverContent>
        </Popover>
    }
}


#[function_component(DropdownItem)]
fn dropdown_item(props: &DropdownItemProps) -> Html {
    let popover_state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");
    
    let onclick = {
        let onclick = props.onclick.clone();
        let toggle = popover_state.toggle.clone();
        Callback::from(move |e: MouseEvent| {
            onclick.emit(e);
            toggle.emit(());
        })
    };

    html! {
        <div class={classes!(BRANDGUIDE.dropdown_item)} onclick={onclick}>
            { if let Some(icon) = &props.icon {
                html! { <span class={classes!(BRANDGUIDE.dropdown_item_icon)}>{ icon.clone() }</span> }
            } else {
                html! {}
            }}
            <span>{ &props.label }</span>
        </div>
    }
}
