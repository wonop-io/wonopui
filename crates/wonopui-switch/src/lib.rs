//! Switch component for WonopUI.
//!
//! A toggle switch component for boolean inputs.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for switch styling.
/// Based on shadcn/ui v4 switch component.
pub mod classes {
    /// Base switch styles - matches shadcn v4 Switch component.
    /// Uses shadcn v4 focus pattern: focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    pub const BASE: &str = "peer group/switch inline-flex h-[1.15rem] w-8 shrink-0 cursor-pointer items-center rounded-full border border-transparent shadow-xs transition-all duration-200 outline-none focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50";

    /// Checked state background - using a primary color that contrasts with white thumb.
    pub const CHECKED: &str = "bg-zinc-900 dark:bg-zinc-300";

    /// Unchecked state background.
    pub const UNCHECKED: &str = "bg-zinc-200 dark:bg-zinc-700";

    /// Disabled state styles.
    pub const DISABLED: &str = "cursor-not-allowed opacity-50";

    /// Switch thumb (the moving circle) - matches shadcn v4.
    /// The thumb should always be white to contrast with the track.
    pub const THUMB: &str = "pointer-events-none block size-4 rounded-full bg-white shadow-sm ring-0 transition-transform duration-200";

    /// Thumb position when checked.
    pub const TRANSLATE_CHECKED: &str = "translate-x-[calc(100%-2px)]";

    /// Thumb position when unchecked.
    pub const TRANSLATE_UNCHECKED: &str = "translate-x-0";

    /// Label styles.
    pub const LABEL: &str = "text-sm font-medium leading-none text-zinc-900 dark:text-zinc-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

    /// Small switch size.
    pub const SIZE_SM: &str = "h-3.5 w-6";

    /// Small thumb size.
    pub const THUMB_SM: &str = "size-3";
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
            data-slot="switch"
            data-state={if *checked { "checked" } else { "unchecked" }}
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
                data-slot="switch-thumb"
                data-state={if *checked { "checked" } else { "unchecked" }}
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
