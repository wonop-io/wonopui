//! Window controls (traffic lights) component for WonopUI.
//!
//! macOS-style window control buttons for Tauri/desktop apps.

use wonopui_core::*;

/// Default CSS classes for window controls styling.
pub mod classes {
    /// Container for the traffic light buttons.
    pub const CONTAINER: &str = "flex items-center gap-2";

    /// Base button styles.
    pub const BUTTON: &str = "w-3 h-3 rounded-full transition-colors cursor-default";

    /// Close button (red).
    pub const CLOSE: &str = "bg-red-500 hover:bg-red-600";

    /// Minimize button (yellow).
    pub const MINIMIZE: &str = "bg-yellow-500 hover:bg-yellow-600";

    /// Maximize button (green).
    pub const MAXIMIZE: &str = "bg-green-500 hover:bg-green-600";

    /// Disabled/inactive button.
    pub const DISABLED: &str = "bg-zinc-300 dark:bg-zinc-600 cursor-not-allowed";

    /// Hover container class for showing icons.
    pub const HOVER_ICONS: &str = "group";
}

/// Window control button type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowControlButton {
    Close,
    Minimize,
    Maximize,
}

/// Properties for the WindowControls component.
#[derive(Properties, PartialEq)]
pub struct WindowControlsProps {
    /// Callback when close button is clicked.
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Callback when minimize button is clicked.
    #[prop_or_default]
    pub on_minimize: Option<Callback<()>>,

    /// Callback when maximize button is clicked.
    #[prop_or_default]
    pub on_maximize: Option<Callback<()>>,

    /// Whether to disable the close button.
    #[prop_or(false)]
    pub close_disabled: bool,

    /// Whether to disable the minimize button.
    #[prop_or(false)]
    pub minimize_disabled: bool,

    /// Whether to disable the maximize button.
    #[prop_or(false)]
    pub maximize_disabled: bool,

    /// Whether to show icons on hover.
    #[prop_or(false)]
    pub show_icons: bool,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// macOS-style window control buttons (traffic lights).
///
/// # Example
///
/// ```rust
/// use wonopui_window_controls::WindowControls;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let on_close = Callback::from(|_| {
///         // Close the window
///     });
///     let on_minimize = Callback::from(|_| {
///         // Minimize the window
///     });
///     let on_maximize = Callback::from(|_| {
///         // Maximize the window
///     });
///
///     html! {
///         <WindowControls
///             {on_close}
///             {on_minimize}
///             {on_maximize}
///         />
///     }
/// }
/// ```
#[function_component(WindowControls)]
pub fn window_controls(props: &WindowControlsProps) -> Html {
    let on_close = {
        let callback = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(ref cb) = callback {
                cb.emit(());
            }
        })
    };

    let on_minimize = {
        let callback = props.on_minimize.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(ref cb) = callback {
                cb.emit(());
            }
        })
    };

    let on_maximize = {
        let callback = props.on_maximize.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(ref cb) = callback {
                cb.emit(());
            }
        })
    };

    let close_class = if props.close_disabled {
        classes::DISABLED
    } else {
        classes::CLOSE
    };

    let minimize_class = if props.minimize_disabled {
        classes::DISABLED
    } else {
        classes::MINIMIZE
    };

    let maximize_class = if props.maximize_disabled {
        classes::DISABLED
    } else {
        classes::MAXIMIZE
    };

    html! {
        <div class={classes!(classes::CONTAINER, props.class.clone())}>
            <button
                class={classes!(classes::BUTTON, close_class)}
                onclick={on_close}
                disabled={props.close_disabled}
                aria-label="Close window"
                title="Close"
            />
            <button
                class={classes!(classes::BUTTON, minimize_class)}
                onclick={on_minimize}
                disabled={props.minimize_disabled}
                aria-label="Minimize window"
                title="Minimize"
            />
            <button
                class={classes!(classes::BUTTON, maximize_class)}
                onclick={on_maximize}
                disabled={props.maximize_disabled}
                aria-label="Maximize window"
                title="Maximize"
            />
        </div>
    }
}
