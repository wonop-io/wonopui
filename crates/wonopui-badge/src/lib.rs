//! Badge component for WonopUI.
//!
//! A small status indicator component.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for badge styling.
/// Based on shadcn/ui v4 badge component.
pub mod classes {
    /// Base badge styles - rounded-md for a rounded square look
    /// Uses focus-visible ring pattern from shadcn v4
    pub const BASE: &str = "inline-flex items-center justify-center rounded-md border border-transparent px-2.5 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 gap-1.5 transition-[color,box-shadow] duration-200 overflow-hidden [&>svg]:size-3 [&>svg]:pointer-events-none focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px]";

    /// Default/Primary badge variant (shadcn: default).
    pub const DEFAULT: &str = "bg-zinc-900 text-zinc-50 [a&]:hover:bg-zinc-800 dark:bg-zinc-50 dark:text-zinc-900 dark:[a&]:hover:bg-zinc-200";

    /// Secondary badge variant (shadcn: secondary).
    pub const SECONDARY: &str = "bg-zinc-100 text-zinc-900 [a&]:hover:bg-zinc-200 dark:bg-zinc-800 dark:text-zinc-50 dark:[a&]:hover:bg-zinc-700";

    /// Outline badge variant (shadcn: outline).
    pub const OUTLINE: &str = "border-zinc-200 dark:border-zinc-800 text-zinc-900 dark:text-zinc-50 [a&]:hover:bg-zinc-100 dark:[a&]:hover:bg-zinc-800";

    /// Destructive badge variant (shadcn: destructive).
    pub const DESTRUCTIVE: &str = "bg-red-500 text-white [a&]:hover:bg-red-600 focus-visible:ring-red-500/50 dark:bg-red-600 dark:[a&]:hover:bg-red-500 dark:focus-visible:ring-red-400/50";

    /// Success badge variant.
    pub const SUCCESS: &str = "bg-emerald-500 text-white [a&]:hover:bg-emerald-600 focus-visible:ring-emerald-500/50 dark:bg-emerald-600 dark:[a&]:hover:bg-emerald-500 dark:focus-visible:ring-emerald-400/50";

    /// Warning badge variant.
    pub const WARNING: &str = "bg-amber-500 text-white [a&]:hover:bg-amber-600 focus-visible:ring-amber-500/50 dark:bg-amber-600 dark:[a&]:hover:bg-amber-500 dark:focus-visible:ring-amber-400/50";

    /// Error badge variant (alias for DESTRUCTIVE).
    pub const ERROR: &str = DESTRUCTIVE;

    /// Info badge variant.
    pub const INFO: &str = "bg-blue-500 text-white [a&]:hover:bg-blue-600 focus-visible:ring-blue-500/50 dark:bg-blue-600 dark:[a&]:hover:bg-blue-500 dark:focus-visible:ring-blue-400/50";

    /// Ghost badge variant (shadcn: ghost).
    pub const GHOST: &str = "[a&]:hover:bg-zinc-100 dark:[a&]:hover:bg-zinc-800 [a&]:hover:text-zinc-900 dark:[a&]:hover:text-zinc-50";

    /// Link badge variant (shadcn: link).
    pub const LINK: &str = "text-zinc-900 dark:text-zinc-50 underline-offset-4 [a&]:hover:underline";
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
        <span
            data-slot="badge"
            data-variant={format!("{:?}", variant).to_lowercase()}
            class={classes!(classes::BASE, variant_class, props.class.clone())}
        >
            if let Some(label) = &props.label {
                { label }
            } else {
                { for props.children.iter() }
            }
        </span>
    }
}
