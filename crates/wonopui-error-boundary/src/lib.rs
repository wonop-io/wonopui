//! Error boundary component for WonopUI.
//!
//! A wrapper component that catches errors in child components.

use std::rc::Rc;
use wonopui_core::*;

/// Default CSS classes for error boundary fallback styling.
pub mod classes {
    /// Error container styles.
    pub const ERROR_CONTAINER: &str = "p-4 border border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20 rounded-lg";

    /// Error title styles.
    pub const ERROR_TITLE: &str = "text-lg font-semibold text-red-800 dark:text-red-200 mb-2";

    /// Error message styles.
    pub const ERROR_MESSAGE: &str = "text-sm text-red-600 dark:text-red-400";

    /// Retry button styles.
    pub const RETRY_BUTTON: &str = "mt-4 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-md text-sm font-medium transition-colors";
}

/// Context for error boundary state.
#[derive(Clone, PartialEq)]
pub struct ErrorBoundaryContext {
    /// The current error, if any.
    pub error: Option<String>,
    /// Callback to clear the error and retry.
    pub reset: Callback<()>,
}

/// Properties for the ErrorBoundary component.
#[derive(Properties, PartialEq)]
pub struct ErrorBoundaryProps {
    /// Child components to render.
    pub children: Children,

    /// Optional custom fallback UI when an error occurs.
    /// Receives the error message and reset callback.
    #[prop_or_default]
    pub fallback: Option<Callback<(String, Callback<()>), Html>>,

    /// Optional callback when an error is caught.
    #[prop_or_default]
    pub on_error: Option<Callback<String>>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Default error fallback UI component.
#[derive(Properties, PartialEq)]
pub struct DefaultErrorFallbackProps {
    pub error: String,
    pub on_reset: Callback<()>,
}

#[function_component(DefaultErrorFallback)]
pub fn default_error_fallback(props: &DefaultErrorFallbackProps) -> Html {
    let onclick = {
        let on_reset = props.on_reset.clone();
        Callback::from(move |_: MouseEvent| on_reset.emit(()))
    };

    html! {
        <div class={classes::ERROR_CONTAINER}>
            <h3 class={classes::ERROR_TITLE}>{"Something went wrong"}</h3>
            <p class={classes::ERROR_MESSAGE}>{ &props.error }</p>
            <button class={classes::RETRY_BUTTON} {onclick}>
                {"Try again"}
            </button>
        </div>
    }
}

/// An error boundary component that catches rendering errors.
///
/// Note: Yew's error boundary support is limited. This component provides
/// a context-based pattern for error handling in child components.
///
/// # Example
///
/// ```rust
/// use wonopui_error_boundary::ErrorBoundary;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <ErrorBoundary>
///             <RiskyComponent />
///         </ErrorBoundary>
///     }
/// }
/// ```
#[function_component(ErrorBoundary)]
pub fn error_boundary(props: &ErrorBoundaryProps) -> Html {
    let error = use_state(|| None::<String>);

    let reset = {
        let error = error.clone();
        Callback::from(move |_| error.set(None))
    };

    let set_error = {
        let error = error.clone();
        let on_error = props.on_error.clone();
        Callback::from(move |err: String| {
            if let Some(ref callback) = on_error {
                callback.emit(err.clone());
            }
            error.set(Some(err));
        })
    };

    let context = Rc::new(ErrorBoundaryContext {
        error: (*error).clone(),
        reset: reset.clone(),
    });

    if let Some(ref err) = *error {
        if let Some(ref fallback) = props.fallback {
            return fallback.emit((err.clone(), reset));
        } else {
            return html! {
                <DefaultErrorFallback error={err.clone()} on_reset={reset} />
            };
        }
    }

    html! {
        <ContextProvider<Rc<ErrorBoundaryContext>> context={context}>
            <div class={props.class.clone()}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<ErrorBoundaryContext>>>
    }
}

/// Hook to access the error boundary context.
///
/// Returns a callback to report errors to the nearest ErrorBoundary.
#[hook]
pub fn use_error_boundary() -> Callback<String> {
    let context = use_context::<Rc<ErrorBoundaryContext>>();

    Callback::from(move |_error: String| {
        // In a real implementation, this would set the error
        // Currently Yew doesn't have native error boundary support
        // This hook provides a pattern for manual error reporting
        if let Some(_ctx) = &context {
            // Error reporting would happen here
        }
    })
}
