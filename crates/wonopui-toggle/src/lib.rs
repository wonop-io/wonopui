//! Toggle component for wonopui
//!
//! A two-state button that can be either on or off.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const TOGGLE_BASE: &str = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-white transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-950 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
    pub const TOGGLE_DEFAULT: &str = "bg-transparent hover:bg-gray-100 dark:hover:bg-zinc-800 h-10 px-3";
    pub const TOGGLE_OUTLINE: &str = "border border-gray-200 dark:border-zinc-700 bg-transparent hover:bg-gray-100 dark:hover:bg-zinc-800 h-10 px-3";
    pub const TOGGLE_CHECKED: &str = "bg-gray-100 dark:bg-zinc-800 text-gray-900 dark:text-zinc-100";
    pub const TOGGLE_UNCHECKED: &str = "text-gray-500 dark:text-zinc-400";
    pub const TOGGLE_DISABLED: &str = "opacity-50 cursor-not-allowed";
    
    // Size variants
    pub const TOGGLE_SM: &str = "h-9 px-2.5";
    pub const TOGGLE_LG: &str = "h-11 px-5";
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl ToggleSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleSize::Sm => classes::TOGGLE_SM,
            ToggleSize::Default => classes::TOGGLE_DEFAULT,
            ToggleSize::Lg => classes::TOGGLE_LG,
        }
    }
}

impl ToggleVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            ToggleVariant::Default => classes::TOGGLE_DEFAULT,
            ToggleVariant::Outline => classes::TOGGLE_OUTLINE,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub pressed: bool,
    /// Alias for pressed (for backward compatibility)
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub on_pressed_change: Callback<bool>,
    /// Alias for on_pressed_change (for backward compatibility)
    #[prop_or_default]
    pub on_toggle: Callback<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: ToggleVariant,
    #[prop_or_default]
    pub size: ToggleSize,
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    // Use checked as alias for pressed if pressed is not set
    let initial_pressed = props.pressed || props.checked;
    let pressed = use_state(|| initial_pressed);

    // Sync with external prop changes
    {
        let pressed = pressed.clone();
        let prop_pressed = props.pressed || props.checked;
        use_effect_with(prop_pressed, move |&prop_pressed| {
            pressed.set(prop_pressed);
            || {}
        });
    }

    let state_class = if *pressed {
        classes::TOGGLE_CHECKED
    } else {
        classes::TOGGLE_UNCHECKED
    };

    let on_click = {
        let pressed = pressed.clone();
        let on_pressed_change = props.on_pressed_change.clone();
        let on_toggle = props.on_toggle.clone();
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                let new_state = !*pressed;
                pressed.set(new_state);
                on_pressed_change.emit(new_state);
                on_toggle.emit(new_state);
            }
        })
    };

    let class = merge_classes(&[
        classes::TOGGLE_BASE,
        props.variant.to_class(),
        state_class,
        if props.disabled { classes::TOGGLE_DISABLED } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <button
            type="button"
            role="switch"
            aria-pressed={pressed.to_string()}
            class={class}
            id={props.id.clone()}
            onclick={on_click}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </button>
    }
}
