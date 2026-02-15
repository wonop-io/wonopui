//! Badge component for WonopUI.
//!
//! A small status indicator component.

use wonopui_core::*;

/// Default CSS classes for badge styling.
pub mod classes {
    /// Base badge styles.
    pub const BASE: &str = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-hidden focus:ring-2 focus:ring-zinc-950 focus:ring-offset-2 dark:focus:ring-zinc-300";

    /// Default badge variant.
    pub const DEFAULT: &str = "border-transparent bg-zinc-900 text-zinc-50 hover:bg-zinc-900/80 dark:bg-zinc-50 dark:text-zinc-900 dark:hover:bg-zinc-50/80";

    /// Success badge variant.
    pub const SUCCESS: &str = "border-transparent bg-emerald-100 text-emerald-900 dark:bg-emerald-900/30 dark:text-emerald-400";

    /// Warning badge variant.
    pub const WARNING: &str =
        "border-transparent bg-amber-100 text-amber-900 dark:bg-amber-900/30 dark:text-amber-400";

    /// Error badge variant.
    pub const ERROR: &str =
        "border-transparent bg-red-100 text-red-900 dark:bg-red-900/30 dark:text-red-400";

    /// Info badge variant.
    pub const INFO: &str =
        "border-transparent bg-blue-100 text-blue-900 dark:bg-blue-900/30 dark:text-blue-400";
}

/// Badge variant determines the visual style.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}

/// Backwards compatibility alias for BadgeVariant
pub type BadgeType = BadgeVariant;

/// Properties for the Badge component.
#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    /// Badge variant.
    #[prop_or_default]
    pub variant: BadgeVariant,

    /// Backwards compatibility alias for variant.
    #[prop_or_default]
    pub badge_type: Option<BadgeVariant>,

    /// Badge label text. If provided, will be displayed instead of children.
    #[prop_or_default]
    pub label: Option<String>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Badge content (children).
    #[prop_or_default]
    pub children: Children,
}

/// A small status indicator component.
///
/// # Example
///
/// ```rust
/// use wonopui_badge::{Badge, BadgeVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Badge variant={BadgeVariant::Success}>
///             {"Active"}
///         </Badge>
///     }
/// }
/// ```
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    // Support both `variant` and `badge_type` props for backwards compatibility
    let variant = props.badge_type.unwrap_or(props.variant);

    let variant_class = match variant {
        BadgeVariant::Default => classes::DEFAULT,
        BadgeVariant::Success => classes::SUCCESS,
        BadgeVariant::Warning => classes::WARNING,
        BadgeVariant::Error => classes::ERROR,
        BadgeVariant::Info => classes::INFO,
    };

    html! {
        <span class={classes!(classes::BASE, variant_class, props.class.clone())}>
            if let Some(label) = &props.label {
                { label }
            } else {
                { for props.children.iter() }
            }
        </span>
    }
}
