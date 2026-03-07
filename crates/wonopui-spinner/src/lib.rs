//! Spinner loading indicator component for WonopUI.
//!
//! A simple animated spinner for loading states.

use wonopui_core::*;

/// Default CSS classes for spinner styling.
pub mod classes {
    /// Base spinner styles with animation.
    pub const BASE: &str = "inline-block animate-spin rounded-full border-2 border-solid border-current border-r-transparent motion-reduce:animate-[spin_1.5s_linear_infinite]";

    /// Small spinner size.
    pub const SIZE_SM: &str = "h-4 w-4";

    /// Medium spinner size (default).
    pub const SIZE_MD: &str = "h-6 w-6";

    /// Large spinner size.
    pub const SIZE_LG: &str = "h-8 w-8";

    /// Extra large spinner size.
    pub const SIZE_XL: &str = "h-12 w-12";
}

/// Spinner size variants.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SpinnerSize {
    /// Small spinner (16px).
    Sm,
    /// Medium spinner (24px) - default.
    #[default]
    Md,
    /// Large spinner (32px).
    Lg,
    /// Extra large spinner (48px).
    Xl,
}

/// Properties for the Spinner component.
#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    /// Size of the spinner.
    #[prop_or_default]
    pub size: SpinnerSize,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Accessible label for screen readers.
    #[prop_or("Loading...".to_string())]
    pub label: String,
}

/// An animated loading spinner component.
///
/// # Example
///
/// ```rust
/// use wonopui_spinner::{Spinner, SpinnerSize};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <div>
///             <Spinner />
///             <Spinner size={SpinnerSize::Lg} />
///         </div>
///     }
/// }
/// ```
#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let size_class = match props.size {
        SpinnerSize::Sm => classes::SIZE_SM,
        SpinnerSize::Md => classes::SIZE_MD,
        SpinnerSize::Lg => classes::SIZE_LG,
        SpinnerSize::Xl => classes::SIZE_XL,
    };

    html! {
        <div
            class={classes!(classes::BASE, size_class, props.class.clone())}
            role="status"
            aria-label={props.label.clone()}
        >
            <span class="sr-only">{&props.label}</span>
        </div>
    }
}
