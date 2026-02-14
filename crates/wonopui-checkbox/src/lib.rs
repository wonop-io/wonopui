//! Checkbox component for WonopUI.
//!
//! A toggle input component for boolean values.

use wonopui_core::*;

/// Default CSS classes for checkbox styling.
pub mod classes {
    /// Base checkbox styles.
    pub const BASE: &str = "h-4 w-4 shrink-0 rounded-sm border border-zinc-700 ring-offset-background focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500 text-indigo-600 dark:text-indigo-400";
    
    /// Checked state styles.
    pub const CHECKED: &str = "bg-indigo-500/90 dark:bg-indigo-600 text-zinc-100 border-transparent";
    
    /// Unchecked state styles.
    pub const UNCHECKED: &str = "border-zinc-300 dark:border-zinc-600";
    
    /// Disabled state styles.
    pub const DISABLED: &str = "opacity-50 cursor-not-allowed";
    
    /// Label styles.
    pub const LABEL: &str = "ml-2 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";
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
            type="button"
            role="checkbox"
            aria-checked={props.checked.to_string()}
            value="on"
            class={classes!(
                "peer",
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
                <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                </svg>
            }
        </button>
    }
}
