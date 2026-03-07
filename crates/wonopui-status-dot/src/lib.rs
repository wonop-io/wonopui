//! Status dot indicator component for WonopUI.
//!
//! A simple colored dot to indicate status.

use wonopui_core::*;

/// Default CSS classes for status dot styling.
pub mod classes {
    /// Base status dot styles.
    pub const BASE: &str = "inline-block rounded-full";

    /// Small dot size.
    pub const SIZE_SM: &str = "h-2 w-2";

    /// Medium dot size (default).
    pub const SIZE_MD: &str = "h-3 w-3";

    /// Large dot size.
    pub const SIZE_LG: &str = "h-4 w-4";

    /// Success variant (green).
    pub const SUCCESS: &str = "bg-emerald-500";

    /// Warning variant (amber).
    pub const WARNING: &str = "bg-amber-500";

    /// Error variant (red).
    pub const ERROR: &str = "bg-red-500";

    /// Info variant (blue).
    pub const INFO: &str = "bg-blue-500";

    /// Neutral variant (gray).
    pub const NEUTRAL: &str = "bg-zinc-400 dark:bg-zinc-500";

    /// Pulse animation class.
    pub const PULSE: &str = "animate-pulse";
}

/// Status dot variant determines the color.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StatusDotVariant {
    /// Success/active status (green).
    Success,
    /// Warning status (amber).
    Warning,
    /// Error/offline status (red).
    Error,
    /// Info status (blue).
    Info,
    /// Neutral/unknown status (gray) - default.
    #[default]
    Neutral,
}

/// Status dot size.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StatusDotSize {
    /// Small dot.
    Sm,
    /// Medium dot (default).
    #[default]
    Md,
    /// Large dot.
    Lg,
}

/// Properties for the StatusDot component.
#[derive(Properties, PartialEq)]
pub struct StatusDotProps {
    /// Color variant of the dot.
    #[prop_or_default]
    pub variant: StatusDotVariant,

    /// Size of the dot.
    #[prop_or_default]
    pub size: StatusDotSize,

    /// Whether to show a pulse animation.
    #[prop_or(false)]
    pub pulse: bool,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Accessible label for screen readers.
    #[prop_or_default]
    pub label: Option<String>,
}

/// A simple colored status indicator dot.
///
/// # Example
///
/// ```rust
/// use wonopui_status_dot::{StatusDot, StatusDotVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <div>
///             <StatusDot variant={StatusDotVariant::Success} />
///             <StatusDot variant={StatusDotVariant::Error} pulse={true} />
///         </div>
///     }
/// }
/// ```
#[function_component(StatusDot)]
pub fn status_dot(props: &StatusDotProps) -> Html {
    let variant_class = match props.variant {
        StatusDotVariant::Success => classes::SUCCESS,
        StatusDotVariant::Warning => classes::WARNING,
        StatusDotVariant::Error => classes::ERROR,
        StatusDotVariant::Info => classes::INFO,
        StatusDotVariant::Neutral => classes::NEUTRAL,
    };

    let size_class = match props.size {
        StatusDotSize::Sm => classes::SIZE_SM,
        StatusDotSize::Md => classes::SIZE_MD,
        StatusDotSize::Lg => classes::SIZE_LG,
    };

    let pulse_class = if props.pulse { classes::PULSE } else { "" };

    html! {
        <span
            class={classes!(classes::BASE, variant_class, size_class, pulse_class, props.class.clone())}
            role="status"
            aria-label={props.label.clone()}
        />
    }
}
