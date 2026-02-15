//! Alert component for WonopUI.
//!
//! A feedback component for displaying important messages.

use wonopui_core::*;

/// Default CSS classes for alert styling.
pub mod classes {
    /// Base alert styles.
    pub const BASE: &str = "mx-auto max-w-4xl w-full p-4 rounded-md bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700";

    /// Success alert variant.
    pub const SUCCESS: &str =
        "text-zinc-800 dark:text-zinc-100 border-l-emerald-500 dark:border-l-emerald-500";

    /// Warning alert variant.
    pub const WARNING: &str =
        "text-zinc-800 dark:text-zinc-100 border-l-amber-500 dark:border-l-amber-500";

    /// Error alert variant.
    pub const ERROR: &str =
        "text-zinc-800 dark:text-zinc-100 border-l-red-500 dark:border-l-red-500";

    /// Info alert variant.
    pub const INFO: &str =
        "text-zinc-800 dark:text-zinc-100 border-l-indigo-500 dark:border-l-indigo-500";

    /// Alert title styles.
    pub const TITLE: &str = "font-semibold text-lg mb-2";

    /// Alert description styles.
    pub const DESCRIPTION: &str = "text-sm";
}

/// Alert variant determines the visual style and semantic meaning.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Backwards compatibility alias for AlertVariant
pub type AlertType = AlertVariant;

/// Properties for the Alert component.
#[derive(Properties, PartialEq)]
pub struct AlertProps {
    /// Alert variant.
    #[prop_or_default]
    pub variant: AlertVariant,

    /// Backwards compatibility alias for variant.
    #[prop_or_default]
    pub alert_type: Option<AlertVariant>,

    /// Alert title (optional).
    #[prop_or_default]
    pub title: Option<String>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Alert content (children).
    #[prop_or_default]
    pub children: Children,
}

/// A feedback component for displaying important messages.
///
/// # Example
///
/// ```rust
/// use wonopui_alert::{Alert, AlertVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Alert
///             variant={AlertVariant::Success}
///             title="Success!"
///         >
///             {"Your action was completed successfully."}
///         </Alert>
///     }
/// }
/// ```
#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    // Support both `variant` and `alert_type` props for backwards compatibility
    let variant = props.alert_type.unwrap_or(props.variant);

    let variant_class = match variant {
        AlertVariant::Info => classes::INFO,
        AlertVariant::Success => classes::SUCCESS,
        AlertVariant::Warning => classes::WARNING,
        AlertVariant::Error => classes::ERROR,
    };

    html! {
        <div class={classes!(classes::BASE, variant_class, props.class.clone())}>
            if let Some(title) = &props.title {
                <div class={classes::TITLE}>{ title }</div>
            }
            <div class={classes::DESCRIPTION}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

// ============================================================================
// AlertTitle and AlertDescription subcomponents
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct AlertTitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// Alert title component for use inside Alert
#[function_component(AlertTitle)]
pub fn alert_title(props: &AlertTitleProps) -> Html {
    let class = format!("{} {}", classes::TITLE, props.class.to_string());
    html! {
        <h5 class={class}>
            { for props.children.iter() }
        </h5>
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertDescriptionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// Alert description component for use inside Alert
#[function_component(AlertDescription)]
pub fn alert_description(props: &AlertDescriptionProps) -> Html {
    let class = format!("{} {}", classes::DESCRIPTION, props.class.to_string());
    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}
