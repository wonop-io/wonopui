//! Confirm dialog component for WonopUI.
//!
//! A modal dialog with confirm and cancel buttons.

use wasm_bindgen::JsCast;
use wonopui_button::{Button, ButtonVariant};
use wonopui_core::*;

/// Default CSS classes for confirm dialog styling.
pub mod classes {
    /// Overlay/backdrop container.
    pub const OVERLAY: &str = "fixed inset-0 z-50 flex items-center justify-center bg-zinc-900/80 dark:bg-zinc-950/90 backdrop-blur-sm";

    /// Dialog content container.
    pub const CONTENT: &str = "bg-white dark:bg-zinc-800 rounded-lg shadow-lg max-w-md w-full mx-4 border border-zinc-200 dark:border-zinc-700";

    /// Dialog header section.
    pub const HEADER: &str = "p-4 border-b border-zinc-200 dark:border-zinc-700 flex items-center gap-3";

    /// Dialog title.
    pub const TITLE: &str = "text-lg font-semibold text-zinc-900 dark:text-zinc-100";

    /// Dialog body/message section.
    pub const BODY: &str = "p-4 text-sm text-zinc-600 dark:text-zinc-400";

    /// Dialog footer with buttons.
    pub const FOOTER: &str = "p-4 border-t border-zinc-200 dark:border-zinc-700 flex justify-end gap-2";

    /// Icon container.
    pub const ICON: &str = "shrink-0 w-6 h-6";

    /// Default/question icon color.
    pub const ICON_DEFAULT: &str = "text-zinc-500 dark:text-zinc-400";

    /// Destructive icon color.
    pub const ICON_DESTRUCTIVE: &str = "text-red-500 dark:text-red-400";

    /// Warning icon color.
    pub const ICON_WARNING: &str = "text-amber-500 dark:text-amber-400";

    /// Info icon color.
    pub const ICON_INFO: &str = "text-blue-500 dark:text-blue-400";
}

/// Confirm dialog variant.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ConfirmDialogVariant {
    /// Default confirmation style (neutral/question).
    #[default]
    Default,
    /// Destructive/dangerous action style (red warning).
    Destructive,
    /// Warning style (yellow warning icon).
    Warning,
    /// Info style (blue info icon).
    Info,
}

/// Properties for the ConfirmDialog component.
#[derive(Properties, PartialEq)]
pub struct ConfirmDialogProps {
    /// Whether the dialog is visible.
    pub visible: bool,

    /// Dialog title.
    pub title: String,

    /// Dialog message/description.
    pub message: String,

    /// Text for the confirm button.
    #[prop_or("Confirm".to_string())]
    pub confirm_text: String,

    /// Text for the cancel button.
    #[prop_or("Cancel".to_string())]
    pub cancel_text: String,

    /// Dialog variant (default, destructive, warning, or info).
    #[prop_or_default]
    pub variant: ConfirmDialogVariant,

    /// Callback when confirm is clicked.
    pub on_confirm: Callback<()>,

    /// Callback when cancel is clicked or dialog is dismissed.
    pub on_cancel: Callback<()>,

    /// Additional CSS classes for the dialog container.
    #[prop_or_default]
    pub class: Classes,

    /// Disable buttons and show spinner on confirm button.
    #[prop_or_default]
    pub loading: bool,
}

/// A modal confirmation dialog with confirm and cancel buttons.
///
/// # Example
///
/// ```rust
/// use wonopui_confirm_dialog::{ConfirmDialog, ConfirmDialogVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let visible = use_state(|| false);
///     let show = {
///         let visible = visible.clone();
///         Callback::from(move |_| visible.set(true))
///     };
///     let on_confirm = {
///         let visible = visible.clone();
///         Callback::from(move |_| {
///             // Do something...
///             visible.set(false);
///         })
///     };
///     let on_cancel = {
///         let visible = visible.clone();
///         Callback::from(move |_| visible.set(false))
///     };
///
///     html! {
///         <>
///             <button onclick={show}>{"Delete"}</button>
///             <ConfirmDialog
///                 visible={*visible}
///                 title="Delete Item"
///                 message="Are you sure you want to delete this item?"
///                 variant={ConfirmDialogVariant::Destructive}
///                 {on_confirm}
///                 {on_cancel}
///             />
///         </>
///     }
/// }
/// ```
#[function_component(ConfirmDialog)]
pub fn confirm_dialog(props: &ConfirmDialogProps) -> Html {
    let on_confirm = {
        let callback = props.on_confirm.clone();
        let loading = props.loading;
        Callback::from(move |_: MouseEvent| {
            if !loading {
                callback.emit(())
            }
        })
    };

    let on_cancel = {
        let callback = props.on_cancel.clone();
        let loading = props.loading;
        Callback::from(move |_: MouseEvent| {
            if !loading {
                callback.emit(())
            }
        })
    };

    let on_overlay_click = {
        let callback = props.on_cancel.clone();
        let loading = props.loading;
        Callback::from(move |e: MouseEvent| {
            if loading {
                return;
            }
            // Only close if clicking the overlay itself, not the dialog content
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    if element.class_list().contains("confirm-dialog-overlay") {
                        callback.emit(());
                    }
                }
            }
        })
    };

    let confirm_variant = match props.variant {
        ConfirmDialogVariant::Default => ButtonVariant::Primary,
        ConfirmDialogVariant::Destructive => ButtonVariant::Danger,
        ConfirmDialogVariant::Warning => ButtonVariant::Warning,
        ConfirmDialogVariant::Info => ButtonVariant::Primary,
    };

    let icon_class = match props.variant {
        ConfirmDialogVariant::Default => classes::ICON_DEFAULT,
        ConfirmDialogVariant::Destructive => classes::ICON_DESTRUCTIVE,
        ConfirmDialogVariant::Warning => classes::ICON_WARNING,
        ConfirmDialogVariant::Info => classes::ICON_INFO,
    };

    if !props.visible {
        return html! {};
    }

    let icon_svg = match props.variant {
        ConfirmDialogVariant::Default => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={classes!(classes::ICON, icon_class)}>
                <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
            </svg>
        },
        ConfirmDialogVariant::Destructive | ConfirmDialogVariant::Warning => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={classes!(classes::ICON, icon_class)}>
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
            </svg>
        },
        ConfirmDialogVariant::Info => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={classes!(classes::ICON, icon_class)}>
                <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
            </svg>
        },
    };

    html! {
        <div
            class={classes!(classes::OVERLAY, "confirm-dialog-overlay", props.class.clone())}
            onclick={on_overlay_click}
        >
            <div class={classes::CONTENT}>
                <div class={classes::HEADER}>
                    { icon_svg }
                    <h2 class={classes::TITLE}>{ &props.title }</h2>
                </div>
                <div class={classes::BODY}>
                    { &props.message }
                </div>
                <div class={classes::FOOTER}>
                    <Button
                        variant={ButtonVariant::Ghost}
                        onclick={on_cancel}
                        disabled={props.loading}
                    >
                        { props.cancel_text.clone() }
                    </Button>
                    <Button
                        variant={confirm_variant}
                        onclick={on_confirm}
                        loading={props.loading}
                    >
                        { props.confirm_text.clone() }
                    </Button>
                </div>
            </div>
        </div>
    }
}