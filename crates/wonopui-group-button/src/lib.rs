//! GroupButton component for wonopui
//!
//! A group of mutually exclusive toggle buttons (like a segmented control).

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the GroupButton component (shadcn v4 - like Toggle Group)
pub mod classes {
    /// Container - muted background with rounded corners
    pub const GROUP_BUTTON_CONTAINER: &str = "inline-flex items-center rounded-lg bg-zinc-100 p-1 dark:bg-zinc-800";
    /// Vertical orientation
    pub const GROUP_BUTTON_VERTICAL: &str = "flex-col";
    /// Individual trigger button
    pub const GROUP_BUTTON_TRIGGER: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 px-3 py-1.5 rounded-md focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none";
    /// Active state - elevated with shadow
    pub const GROUP_BUTTON_TRIGGER_ACTIVE: &str = "bg-white text-zinc-950 shadow-sm dark:bg-zinc-950 dark:text-zinc-50";
    /// Inactive state - muted text with hover
    pub const GROUP_BUTTON_TRIGGER_INACTIVE: &str = "text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-50";
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
        if is_active {
            classes::GROUP_BUTTON_TRIGGER_ACTIVE
        } else {
            classes::GROUP_BUTTON_TRIGGER_INACTIVE
        },
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
