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
    #[prop_or_default]
    pub full_width: bool,
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
    let popover_content_class = if props.full_width {
        classes!(BRANDGUIDE.dropdown_content, "w-full")
    } else {
        classes!(BRANDGUIDE.dropdown_content)
    };

    html! {
        <Popover class={ props.class.clone()}>
            <PopoverTrigger>
                { props.children.clone() }
            </PopoverTrigger>
            <PopoverContent class={popover_content_class} position={props.position.clone()}>
                { for props.items.iter().map(|item| {
                    if item.is_separator {
                        html! { <hr class={classes!(BRANDGUIDE.dropdown_separator)} /> }
                    } else {
                        let item_onclick = item.onclick.clone();
                        html! {
                            <DropdownItemComponent
                                label={item.label.clone()}
                                icon={item.icon.clone()}
                                onclick={item_onclick}
                                is_separator={item.is_separator}
                                full_width={props.full_width}
                            />
                        }
                    }
                }) }
            </PopoverContent>
        </Popover>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownItemComponentProps {
    pub label: String,
    pub icon: Option<Html>,
    pub onclick: Callback<MouseEvent>,
    pub is_separator: bool,
    #[prop_or_default]
    pub full_width: bool,
}

#[function_component(DropdownItemComponent)]
fn dropdown_item_component(props: &DropdownItemComponentProps) -> Html {
    let popover_state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");
    
    let onclick = {
        let onclick = props.onclick.clone();
        let toggle = popover_state.toggle.clone();
        Callback::from(move |e: MouseEvent| {
            onclick.emit(e);
            toggle.emit(());
        })
    };

    let item_class = if props.full_width {
        classes!(BRANDGUIDE.dropdown_item, "w-full")
    } else {
        classes!(BRANDGUIDE.dropdown_item)
    };

    html! {
        <div class={item_class} onclick={onclick}>
            { if let Some(icon) = &props.icon {
                html! { <span class={classes!(BRANDGUIDE.dropdown_item_icon)}>{ icon.clone() }</span> }
            } else {
                html! {}
            }}
            <span>{ &props.label }</span>
        </div>
    }
}
