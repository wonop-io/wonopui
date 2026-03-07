//! Progress bar component for WonopUI.
//!
//! A customizable progress indicator with optional labels.

use wonopui_core::*;

/// Default CSS classes for progress bar styling.
pub mod classes {
    /// Container styles.
    pub const CONTAINER: &str = "w-full";

    /// Labels container (between value and max).
    pub const LABELS: &str = "flex justify-between mb-1 text-sm";

    /// Label text styles.
    pub const LABEL: &str = "text-zinc-700 dark:text-zinc-300";

    /// Value text styles.
    pub const VALUE: &str = "text-zinc-600 dark:text-zinc-400";

    /// Track (background) styles.
    pub const TRACK: &str = "w-full bg-zinc-200 dark:bg-zinc-700 rounded-full overflow-hidden";

    /// Small track height.
    pub const TRACK_SM: &str = "h-1";

    /// Medium track height (default).
    pub const TRACK_MD: &str = "h-2";

    /// Large track height.
    pub const TRACK_LG: &str = "h-3";

    /// Bar (fill) styles.
    pub const BAR: &str = "h-full rounded-full transition-all duration-300 ease-out";

    /// Default bar color.
    pub const BAR_DEFAULT: &str = "bg-blue-600 dark:bg-blue-500";

    /// Success bar color.
    pub const BAR_SUCCESS: &str = "bg-emerald-600 dark:bg-emerald-500";

    /// Warning bar color.
    pub const BAR_WARNING: &str = "bg-amber-600 dark:bg-amber-500";

    /// Error bar color.
    pub const BAR_ERROR: &str = "bg-red-600 dark:bg-red-500";
}

/// Progress bar size.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ProgressSize {
    /// Small progress bar.
    Sm,
    /// Medium progress bar (default).
    #[default]
    Md,
    /// Large progress bar.
    Lg,
}

/// Progress bar variant/color.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ProgressVariant {
    /// Default blue color.
    #[default]
    Default,
    /// Success green color.
    Success,
    /// Warning amber color.
    Warning,
    /// Error red color.
    Error,
}

/// Properties for the Progress component.
#[derive(Properties, PartialEq)]
pub struct ProgressProps {
    /// Current progress value.
    pub value: f64,

    /// Maximum value (default: 100).
    #[prop_or(100.0)]
    pub max: f64,

    /// Size of the progress bar.
    #[prop_or_default]
    pub size: ProgressSize,

    /// Color variant.
    #[prop_or_default]
    pub variant: ProgressVariant,

    /// Optional label to show above the progress bar.
    #[prop_or_default]
    pub label: Option<String>,

    /// Whether to show the value text.
    #[prop_or(false)]
    pub show_value: bool,

    /// Format function for the value display.
    /// Receives (value, max) and returns a string.
    #[prop_or_default]
    pub value_format: Option<Callback<(f64, f64), String>>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// A progress bar component with optional labels.
///
/// # Example
///
/// ```rust
/// use wonopui_progress::{Progress, ProgressVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <div>
///             <Progress value={50.0} />
///             <Progress
///                 value={75.0}
///                 label="Upload progress"
///                 show_value={true}
///                 variant={ProgressVariant::Success}
///             />
///         </div>
///     }
/// }
/// ```
#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let percentage = (props.value / props.max * 100.0).min(100.0).max(0.0);

    let track_size_class = match props.size {
        ProgressSize::Sm => classes::TRACK_SM,
        ProgressSize::Md => classes::TRACK_MD,
        ProgressSize::Lg => classes::TRACK_LG,
    };

    let bar_variant_class = match props.variant {
        ProgressVariant::Default => classes::BAR_DEFAULT,
        ProgressVariant::Success => classes::BAR_SUCCESS,
        ProgressVariant::Warning => classes::BAR_WARNING,
        ProgressVariant::Error => classes::BAR_ERROR,
    };

    let value_text = if props.show_value {
        if let Some(ref format_fn) = props.value_format {
            Some(format_fn.emit((props.value, props.max)))
        } else {
            Some(format!("{:.0}%", percentage))
        }
    } else {
        None
    };

    let has_labels = props.label.is_some() || value_text.is_some();

    html! {
        <div
            class={classes!(classes::CONTAINER, props.class.clone())}
            role="progressbar"
            aria-valuenow={props.value.to_string()}
            aria-valuemin="0"
            aria-valuemax={props.max.to_string()}
        >
            if has_labels {
                <div class={classes::LABELS}>
                    if let Some(ref label) = props.label {
                        <span class={classes::LABEL}>{ label }</span>
                    } else {
                        <span></span>
                    }
                    if let Some(ref value) = value_text {
                        <span class={classes::VALUE}>{ value }</span>
                    }
                </div>
            }
            <div class={classes!(classes::TRACK, track_size_class)}>
                <div
                    class={classes!(classes::BAR, bar_variant_class)}
                    style={format!("width: {}%", percentage)}
                />
            </div>
        </div>
    }
}
