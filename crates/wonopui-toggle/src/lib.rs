//! Toggle component for wonopui
//!
//! A two-state button that can be either on or off.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for toggle styling.
/// Based on shadcn/ui v4 toggle component.
pub mod classes {
    /// Base toggle styles - matches shadcn v4 Toggle component.
    /// Uses rounded-lg for rounded square appearance with proper padding.
    pub const TOGGLE_BASE: &str = "inline-flex items-center justify-center gap-2 rounded-lg text-sm font-medium transition-all duration-200 outline-none focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 whitespace-nowrap";

    /// Default variant styles (no border).
    pub const TOGGLE_DEFAULT: &str = "bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-900 dark:hover:text-zinc-100";

    /// Outline variant styles (with border).
    pub const TOGGLE_OUTLINE: &str = "border border-zinc-200 dark:border-zinc-800 bg-transparent shadow-xs hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-900 dark:hover:text-zinc-50";

    /// Checked/pressed state styles (shadcn: data-[state=on]).
    pub const TOGGLE_CHECKED: &str = "bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-50";

    /// Unchecked state styles.
    pub const TOGGLE_UNCHECKED: &str = "text-zinc-500 dark:text-zinc-400";

    /// Disabled state styles.
    pub const TOGGLE_DISABLED: &str = "pointer-events-none opacity-50";

    // Size variants - matching shadcn v4 with generous padding
    /// Default size with good padding for rounded square look.
    pub const TOGGLE_SIZE_DEFAULT: &str = "h-10 min-w-10 px-4 py-2.5";

    /// Small size with proper padding.
    pub const TOGGLE_SM: &str = "h-9 min-w-9 px-3 py-2";

    /// Large size with proper padding.
    pub const TOGGLE_LG: &str = "h-12 min-w-12 px-5 py-3";
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
            ToggleSize::Default => classes::TOGGLE_SIZE_DEFAULT,
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
        if props.disabled {
            classes::TOGGLE_DISABLED
        } else {
            ""
        },
        &props.class.to_string(),
    ]);

    html! {
        <button
            data-slot="toggle"
            data-state={if *pressed { "on" } else { "off" }}
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
