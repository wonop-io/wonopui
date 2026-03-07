//! Status indicator component for WonopUI.
//!
//! A status indicator with an optional icon and label.

use wonopui_core::*;
pub use wonopui_status_dot::{StatusDotSize, StatusDotVariant};

/// Default CSS classes for status indicator styling.
pub mod classes {
    /// Base container styles.
    pub const CONTAINER: &str = "inline-flex items-center gap-2";

    /// Label text styles.
    pub const LABEL: &str = "text-sm text-zinc-700 dark:text-zinc-300";
}

/// Properties for the StatusIndicator component.
#[derive(Properties, PartialEq)]
pub struct StatusIndicatorProps {
    /// Color variant of the status dot.
    #[prop_or_default]
    pub variant: StatusDotVariant,

    /// Size of the status dot.
    #[prop_or_default]
    pub size: StatusDotSize,

    /// Whether to show a pulse animation.
    #[prop_or(false)]
    pub pulse: bool,

    /// Optional label text.
    #[prop_or_default]
    pub label: Option<String>,

    /// Optional icon (rendered before the dot).
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Children (rendered after the label).
    #[prop_or_default]
    pub children: Children,
}

/// A status indicator with optional icon and label.
///
/// # Example
///
/// ```rust
/// use wonopui_status_indicator::{StatusIndicator, StatusDotVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <StatusIndicator
///             variant={StatusDotVariant::Success}
///             label="Online"
///         />
///     }
/// }
/// ```
#[function_component(StatusIndicator)]
pub fn status_indicator(props: &StatusIndicatorProps) -> Html {
    html! {
        <div class={classes!(classes::CONTAINER, props.class.clone())}>
            if let Some(icon) = &props.icon {
                { icon.clone() }
            }
            <wonopui_status_dot::StatusDot
                variant={props.variant}
                size={props.size}
                pulse={props.pulse}
            />
            if let Some(label) = &props.label {
                <span class={classes::LABEL}>{ label }</span>
            }
            { for props.children.iter() }
        </div>
    }
}
