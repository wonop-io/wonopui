//! Collapsible component for wonopui
//!
//! A component that can be expanded or collapsed to show or hide content.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const COLLAPSIBLE_CONTAINER: &str = "";
    pub const COLLAPSIBLE_TRIGGER: &str = "cursor-pointer";
    pub const COLLAPSIBLE_CONTENT: &str = "";
    pub const COLLAPSIBLE_HEADER: &str = "flex items-center justify-between";
    pub const COLLAPSIBLE_TITLE: &str = "text-sm font-medium";
    pub const COLLAPSIBLE_ITEM: &str = "";
}

#[derive(Clone, PartialEq)]
pub struct CollapsibleState {
    pub is_open: bool,
    pub toggle: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let is_open = use_state(|| props.open);

    let toggle = {
        let is_open = is_open.clone();
        let on_open_change = props.on_open_change.clone();
        Callback::from(move |_| {
            let new_state = !*is_open;
            is_open.set(new_state);
            on_open_change.emit(new_state);
        })
    };

    let state = Rc::new(CollapsibleState {
        is_open: *is_open,
        toggle,
    });

    let class = merge_classes(&[classes::COLLAPSIBLE_CONTAINER, &props.class.to_string()]);

    html! {
        <ContextProvider<Rc<CollapsibleState>> context={state}>
            <div class={class}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<CollapsibleState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTriggerProps {
    #[prop_or_default]
    pub as_child: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CollapsibleTrigger)]
pub fn collapsible_trigger(props: &CollapsibleTriggerProps) -> Html {
    let state = use_context::<Rc<CollapsibleState>>().expect("CollapsibleState not found");

    let onclick = {
        let toggle = state.toggle.clone();
        Callback::from(move |_: MouseEvent| toggle.emit(()))
    };

    let class = merge_classes(&[classes::COLLAPSIBLE_TRIGGER, &props.class.to_string()]);

    html! {
        <div {onclick} class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleContentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleContent)]
pub fn collapsible_content(props: &CollapsibleContentProps) -> Html {
    let state = use_context::<Rc<CollapsibleState>>().expect("CollapsibleState not found");

    if !state.is_open {
        return html! {};
    }

    let class = merge_classes(&[classes::COLLAPSIBLE_CONTENT, &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleHeaderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleHeader)]
pub fn collapsible_header(props: &CollapsibleHeaderProps) -> Html {
    let class = merge_classes(&[classes::COLLAPSIBLE_HEADER, &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTitleProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleTitle)]
pub fn collapsible_title(props: &CollapsibleTitleProps) -> Html {
    let class = merge_classes(&[classes::COLLAPSIBLE_TITLE, &props.class.to_string()]);

    html! {
        <h4 class={class}>
            { for props.children.iter() }
        </h4>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleItemProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleItem)]
pub fn collapsible_item(props: &CollapsibleItemProps) -> Html {
    let class = merge_classes(&[classes::COLLAPSIBLE_ITEM, &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}
