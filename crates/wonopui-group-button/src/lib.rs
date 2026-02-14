//! GroupButton component for wonopui
//!
//! A group of mutually exclusive toggle buttons (like a segmented control).

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const GROUP_BUTTON_CONTAINER: &str = "inline-flex rounded-lg bg-gray-100 dark:bg-zinc-800 p-1";
    pub const GROUP_BUTTON_VERTICAL: &str = "flex-col";
    pub const GROUP_BUTTON_TRIGGER: &str = "inline-flex items-center justify-center whitespace-nowrap px-3 py-1.5 text-sm font-medium rounded-md transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
    pub const GROUP_BUTTON_TRIGGER_ACTIVE: &str = "bg-white dark:bg-zinc-700 text-gray-900 dark:text-zinc-100 shadow-sm";
    pub const GROUP_BUTTON_TRIGGER_INACTIVE: &str = "text-gray-600 dark:text-zinc-400 hover:text-gray-900 dark:hover:text-zinc-200";
}

#[derive(Clone, PartialEq, Default)]
pub enum GroupButtonDirection {
    #[default]
    Row,
    Column,
}

/// Backwards compatibility alias for GroupButtonDirection
pub type FlexDirection = GroupButtonDirection;

#[derive(Clone, PartialEq)]
pub struct GroupButtonState {
    pub active_button: String,
    pub set_active_button: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct GroupButtonProps {
    pub children: Children,
    pub default_value: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub direction: GroupButtonDirection,
    #[prop_or_default]
    pub on_change: Callback<String>,
}

#[function_component(GroupButton)]
pub fn group_button(props: &GroupButtonProps) -> Html {
    let active_button = use_state(|| props.default_value.clone());

    let set_active_button = {
        let active_button = active_button.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |new_button: String| {
            active_button.set(new_button.clone());
            on_change.emit(new_button);
        })
    };

    let state = Rc::new(GroupButtonState {
        active_button: (*active_button).clone(),
        set_active_button,
    });

    let direction_class = match props.direction {
        GroupButtonDirection::Row => "",
        GroupButtonDirection::Column => classes::GROUP_BUTTON_VERTICAL,
    };

    let class = merge_classes(&[
        classes::GROUP_BUTTON_CONTAINER,
        direction_class,
        &props.class.to_string(),
    ]);

    html! {
        <ContextProvider<Rc<GroupButtonState>> context={state}>
            <div role="group" class={class}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<GroupButtonState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct GroupButtonTriggerProps {
    pub value: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(GroupButtonTrigger)]
pub fn group_button_trigger(props: &GroupButtonTriggerProps) -> Html {
    let state =
        use_context::<Rc<GroupButtonState>>().expect("no context found for GroupButtonState");

    let onclick = {
        let set_active_button = state.set_active_button.clone();
        let value = props.value.clone();
        let user_onclick = props.onclick.clone();
        let disabled = props.disabled;
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                set_active_button.emit(value.clone());
                user_onclick.emit(e);
            }
        })
    };

    let is_active = state.active_button == props.value;

    let class = merge_classes(&[
        classes::GROUP_BUTTON_TRIGGER,
        if is_active { classes::GROUP_BUTTON_TRIGGER_ACTIVE } else { classes::GROUP_BUTTON_TRIGGER_INACTIVE },
        &props.class.to_string(),
    ]);

    html! {
        <button
            type="button"
            role="button"
            onclick={onclick}
            class={class}
            disabled={props.disabled}
            aria-pressed={is_active.to_string()}
        >
            { for props.children.iter() }
        </button>
    }
}
