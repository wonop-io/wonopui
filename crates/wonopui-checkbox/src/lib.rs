//! Checkbox component for WonopUI.
//!
//! A toggle input component for boolean values.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for checkbox styling.
/// Based on shadcn/ui v4 checkbox component.
pub mod classes {
    /// Base checkbox styles - matches shadcn v4 Checkbox component.
    /// Uses shadcn v4 focus pattern: focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    pub const BASE: &str = "peer size-4 shrink-0 rounded-[4px] border border-zinc-200 dark:border-zinc-800 dark:bg-zinc-950/30 shadow-xs transition-shadow duration-200 outline-none focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50";

    /// Checked state styles.
    pub const CHECKED: &str = "bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 border-zinc-900 dark:border-zinc-50";

    /// Unchecked state styles.
    pub const UNCHECKED: &str = "border-zinc-200 dark:border-zinc-800";

    /// Disabled state styles.
    pub const DISABLED: &str = "cursor-not-allowed opacity-50";

    /// Label styles.
    pub const LABEL: &str = "ml-2 text-sm font-medium leading-none text-zinc-900 dark:text-zinc-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

    /// Indicator wrapper styles for the checkmark.
    pub const INDICATOR: &str = "grid place-content-center text-current transition-none";
}

/// Properties for the Checkbox component.
#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    /// Checkbox ID attribute.
    #[prop_or_default]
    pub id: String,

    /// Whether the checkbox is checked.
    #[prop_or_default]
    pub checked: bool,

    /// Callback when toggled.
    #[prop_or_default]
    pub on_toggle: Callback<MouseEvent>,

    /// Whether the checkbox is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// A toggle input component for boolean values.
///
/// # Example
///
/// ```rust
/// use wonopui_checkbox::Checkbox;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let checked = use_state(|| false);
///     let on_toggle = {
///         let checked = checked.clone();
///         Callback::from(move |_| checked.set(!*checked))
///     };
///
///     html! {
///         <Checkbox checked={*checked} on_toggle={on_toggle} />
///     }
/// }
/// ```
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let state_class = if props.checked {
        classes::CHECKED
    } else {
        classes::UNCHECKED
    };

    let disabled_class = if props.disabled {
        classes::DISABLED
    } else {
        ""
    };

    html! {
        <button
            data-slot="checkbox"
            data-state={if props.checked { "checked" } else { "unchecked" }}
            type="button"
            role="checkbox"
            aria-checked={props.checked.to_string()}
            value="on"
            class={classes!(
                classes::BASE,
                state_class,
                disabled_class,
                props.class.clone(),
            )}
            id={props.id.clone()}
            onclick={props.on_toggle.clone()}
            disabled={props.disabled}
        >
            if props.checked {
                <span data-slot="checkbox-indicator" class={classes::INDICATOR}>
                    <svg class="size-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                    </svg>
                </span>
            }
        </button>
    }
}
