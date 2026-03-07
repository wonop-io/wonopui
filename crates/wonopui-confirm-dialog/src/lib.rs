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
    pub const HEADER: &str = "p-4 border-b border-zinc-200 dark:border-zinc-700";

    /// Dialog title.
    pub const TITLE: &str = "text-lg font-semibold text-zinc-900 dark:text-zinc-100";

    /// Dialog body/message section.
    pub const BODY: &str = "p-4 text-sm text-zinc-600 dark:text-zinc-400";

    /// Dialog footer with buttons.
    pub const FOOTER: &str = "p-4 border-t border-zinc-200 dark:border-zinc-700 flex justify-end gap-2";
}

/// Confirm dialog variant.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ConfirmDialogVariant {
    /// Default confirmation style.
    #[default]
    Default,
    /// Destructive/dangerous action style.
    Destructive,
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

    /// Dialog variant (default or destructive).
    #[prop_or_default]
    pub variant: ConfirmDialogVariant,

    /// Callback when confirm is clicked.
    pub on_confirm: Callback<()>,

    /// Callback when cancel is clicked or dialog is dismissed.
    pub on_cancel: Callback<()>,

    /// Additional CSS classes for the dialog container.
    #[prop_or_default]
    pub class: Classes,
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
        Callback::from(move |_: MouseEvent| callback.emit(()))
    };

    let on_cancel = {
        let callback = props.on_cancel.clone();
        Callback::from(move |_: MouseEvent| callback.emit(()))
    };

    let on_overlay_click = {
        let callback = props.on_cancel.clone();
        Callback::from(move |e: MouseEvent| {
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
    };

    if !props.visible {
        return html! {};
    }

    html! {
        <div
            class={classes!(classes::OVERLAY, "confirm-dialog-overlay", props.class.clone())}
            onclick={on_overlay_click}
        >
            <div class={classes::CONTENT}>
                <div class={classes::HEADER}>
                    <h2 class={classes::TITLE}>{ &props.title }</h2>
                </div>
                <div class={classes::BODY}>
                    { &props.message }
                </div>
                <div class={classes::FOOTER}>
                    <Button
                        variant={ButtonVariant::Ghost}
                        onclick={on_cancel}
                    >
                        { props.cancel_text.clone() }
                    </Button>
                    <Button
                        variant={confirm_variant}
                        onclick={on_confirm}
                    >
                        { props.confirm_text.clone() }
                    </Button>
                </div>
            </div>
        </div>
    }
}
