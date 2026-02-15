//! Switch component for WonopUI.
//!
//! A toggle switch component for boolean inputs.

use wonopui_core::*;

/// Default CSS classes for switch styling.
pub mod classes {
    /// Base switch styles.
    pub const BASE: &str = "shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-hidden focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 flex w-12 h-6";

    /// Checked state background.
    pub const CHECKED: &str = "bg-indigo-600 dark:bg-indigo-500";

    /// Unchecked state background.
    pub const UNCHECKED: &str = "bg-zinc-200 dark:bg-zinc-700";

    /// Disabled state styles.
    pub const DISABLED: &str = "cursor-not-allowed opacity-50 dark:bg-zinc-600";

    /// Switch thumb (the moving circle).
    pub const THUMB: &str = "pointer-events-none inline-block h-5 w-5 text-zinc-700 transform rounded-full bg-white shadow-sm ring-0 transition duration-200 ease-in-out";

    /// Thumb position when checked.
    pub const TRANSLATE_CHECKED: &str = "translate-x-6";

    /// Thumb position when unchecked.
    pub const TRANSLATE_UNCHECKED: &str = "translate-x-0";

    /// Label styles.
    pub const LABEL: &str = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-zinc-700";
}

/// Properties for the SwitchButton component.
#[derive(Properties, PartialEq)]
pub struct SwitchButtonProps {
    /// Switch ID attribute.
    #[prop_or_default]
    pub id: String,

    /// Controlled checked state (overrides internal state).
    #[prop_or_default]
    pub checked: Option<bool>,

    /// Default checked value when uncontrolled.
    #[prop_or_default]
    pub default_value: bool,

    /// Callback when toggled.
    #[prop_or_default]
    pub on_toggle: Callback<MouseEvent>,

    /// Whether the switch is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Icon to show when switch is on.
    #[prop_or_default]
    pub on_icon: Option<Html>,

    /// Icon to show when switch is off.
    #[prop_or_default]
    pub off_icon: Option<Html>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// A toggle switch component for boolean inputs.
///
/// # Example
///
/// ```rust
/// use wonopui_switch::SwitchButton;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let enabled = use_state(|| false);
///     let on_toggle = {
///         let enabled = enabled.clone();
///         Callback::from(move |_| enabled.set(!*enabled))
///     };
///
///     html! {
///         <SwitchButton checked={Some(*enabled)} on_toggle={on_toggle} />
///     }
/// }
/// ```
#[function_component(SwitchButton)]
pub fn switch_button(props: &SwitchButtonProps) -> Html {
    let checked = use_state(|| props.checked.unwrap_or(props.default_value));

    use_effect_with(
        (checked.clone(), props.checked),
        |(checked, prop_checked)| {
            if let Some(prop_checked) = prop_checked {
                if (**checked) != *prop_checked {
                    checked.set(*prop_checked);
                }
            }
        },
    );

    let state_class = if *checked {
        classes::CHECKED
    } else {
        classes::UNCHECKED
    };

    let translate_class = if *checked {
        classes::TRANSLATE_CHECKED
    } else {
        classes::TRANSLATE_UNCHECKED
    };

    let disabled_class = if props.disabled {
        classes::DISABLED
    } else {
        ""
    };

    let on_click = {
        let checked = checked.clone();
        let on_toggle = props.on_toggle.clone();
        let disabled = props.disabled;
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                checked.set(!*checked);
                on_toggle.emit(e);
            }
        })
    };

    html! {
        <button
            type="button"
            role="switch"
            aria-checked={checked.to_string()}
            class={classes!(
                classes::BASE,
                state_class,
                disabled_class,
                props.class.clone(),
            )}
            id={props.id.clone()}
            onclick={on_click}
            disabled={props.disabled}
        >
            <span
                aria-hidden="true"
                class={classes!(classes::THUMB, translate_class)}
            >
                { if *checked {
                    props.on_icon.clone().unwrap_or_default()
                } else {
                    props.off_icon.clone().unwrap_or_default()
                }}
            </span>
        </button>
    }
}
