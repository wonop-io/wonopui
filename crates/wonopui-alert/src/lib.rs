//! Alert component for WonopUI.
//!
//! A feedback component for displaying important messages.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for alert styling.
/// Based on shadcn/ui v4 alert component with premium styling.
pub mod classes {
    /// Base alert styles - premium rounded design with flex layout
    pub const BASE: &str = "relative w-full rounded-xl border p-4 text-sm flex gap-3 items-start shadow-sm";

    /// Default alert variant (neutral).
    pub const DEFAULT: &str = "bg-zinc-50 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 border-zinc-200 dark:border-zinc-800";

    /// Destructive/Error alert variant - premium red styling
    pub const DESTRUCTIVE: &str = "bg-red-50 dark:bg-red-950/30 text-red-900 dark:text-red-100 border-red-200 dark:border-red-900/50";

    /// Success alert variant - premium green styling
    pub const SUCCESS: &str = "bg-emerald-50 dark:bg-emerald-950/30 text-emerald-900 dark:text-emerald-100 border-emerald-200 dark:border-emerald-900/50";

    /// Warning alert variant - premium amber styling
    pub const WARNING: &str = "bg-amber-50 dark:bg-amber-950/30 text-amber-900 dark:text-amber-100 border-amber-200 dark:border-amber-900/50";

    /// Error alert variant (alias for DESTRUCTIVE).
    pub const ERROR: &str = DESTRUCTIVE;

    /// Info alert variant - premium blue styling
    pub const INFO: &str = "bg-blue-50 dark:bg-blue-950/30 text-blue-900 dark:text-blue-100 border-blue-200 dark:border-blue-900/50";

    /// Icon container - centered and properly sized
    pub const ICON: &str = "flex-shrink-0 size-5";
    
    /// Icon colors per variant
    pub const ICON_INFO: &str = "text-blue-600 dark:text-blue-400";
    pub const ICON_SUCCESS: &str = "text-emerald-600 dark:text-emerald-400";
    pub const ICON_WARNING: &str = "text-amber-600 dark:text-amber-400";
    pub const ICON_ERROR: &str = "text-red-600 dark:text-red-400";

    /// Content container
    pub const CONTENT: &str = "flex-1 min-w-0";

    /// Alert title styles - premium typography
    pub const TITLE: &str = "font-semibold text-sm leading-tight";

    /// Alert description styles - muted text with proper spacing
    pub const DESCRIPTION: &str = "mt-1 text-sm opacity-90 leading-relaxed";
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

    let icon_class = match variant {
        AlertVariant::Info => classes::ICON_INFO,
        AlertVariant::Success => classes::ICON_SUCCESS,
        AlertVariant::Warning => classes::ICON_WARNING,
        AlertVariant::Error => classes::ICON_ERROR,
    };

    let icon = match variant {
        AlertVariant::Info => html! {
            <svg class={classes!(classes::ICON, icon_class)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        },
        AlertVariant::Success => html! {
            <svg class={classes!(classes::ICON, icon_class)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        },
        AlertVariant::Warning => html! {
            <svg class={classes!(classes::ICON, icon_class)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
            </svg>
        },
        AlertVariant::Error => html! {
            <svg class={classes!(classes::ICON, icon_class)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        },
    };

    html! {
        <div
            data-slot="alert"
            data-variant={format!("{:?}", variant).to_lowercase()}
            role="alert"
            class={classes!(classes::BASE, variant_class, props.class.clone())}
        >
            { icon }
            <div class={classes::CONTENT}>
                if let Some(title) = &props.title {
                    <div data-slot="alert-title" class={classes::TITLE}>{ title }</div>
                }
                <div data-slot="alert-description" class={classes::DESCRIPTION}>
                    { for props.children.iter() }
                </div>
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
        <div data-slot="alert-title" class={class}>
            { for props.children.iter() }
        </div>
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
        <div data-slot="alert-description" class={class}>
            { for props.children.iter() }
        </div>
    }
}
