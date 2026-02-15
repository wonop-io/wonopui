//! CopyButton component for wonopui
//!
//! A button that copies text to the clipboard when clicked.

use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

pub mod classes {
    pub const COPY_BUTTON: &str = "inline-flex items-center justify-center gap-2 px-3 py-2 text-sm font-medium rounded-md bg-gray-100 dark:bg-zinc-800 hover:bg-gray-200 dark:hover:bg-zinc-700 text-gray-700 dark:text-zinc-300 transition-colors";
    pub const COPY_BUTTON_ICON: &str = "w-4 h-4";
}

#[derive(Properties, PartialEq)]
pub struct CopyButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub copy_text: String,
    #[prop_or_default]
    pub copied_text: Option<String>,
    #[prop_or(2000)]
    pub copied_timeout_ms: u32,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub show_icon: bool,
}

#[function_component(CopyButton)]
pub fn copy_button(props: &CopyButtonProps) -> Html {
    let is_copied = use_state(|| false);
    let timeout_handle = use_mut_ref(|| None::<i32>);

    let onclick = {
        let is_copied = is_copied.clone();
        let timeout_handle = timeout_handle.clone();
        let copy_text = props.copy_text.clone();
        let copied_timeout_ms = props.copied_timeout_ms;
        let user_onclick = props.onclick.clone();

        Callback::from(move |e: MouseEvent| {
            // Call user's onclick handler if provided
            if let Some(callback) = user_onclick.clone() {
                callback.emit(e.clone());
            }

            // Copy text to clipboard
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let _ = clipboard.write_text(&copy_text);
            } else {
                return;
            }

            // Update copied state
            is_copied.set(true);

            // Get window reference for timeout operations
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };

            // Clear existing timeout if it exists
            if let Some(handle) = *timeout_handle.borrow() {
                window.clear_timeout_with_handle(handle);
            }

            // Set timeout to reset copied state
            let is_copied_clone = is_copied.clone();
            let closure = Closure::once(move || {
                is_copied_clone.set(false);
            });

            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    copied_timeout_ms as i32,
                )
                .expect("Failed to set timeout");

            closure.forget();
            *timeout_handle.borrow_mut() = Some(handle);
        })
    };

    let button_class = if props.class.is_empty() {
        classes::COPY_BUTTON.to_string()
    } else {
        props.class.to_string()
    };

    html! {
        <button
            class={button_class}
            onclick={onclick}
            type="button"
        >
            if props.show_icon {
                if *is_copied {
                    // Checkmark icon
                    <svg class={classes::COPY_BUTTON_ICON} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                } else {
                    // Copy icon
                    <svg class={classes::COPY_BUTTON_ICON} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                        <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
                        <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
                    </svg>
                }
            }
            if *is_copied {
                if let Some(text) = &props.copied_text {
                    { text }
                } else {
                    { "Copied!" }
                }
            } else if props.children.is_empty() {
                { "Copy" }
            } else {
                { for props.children.iter() }
            }
        </button>
    }
}
