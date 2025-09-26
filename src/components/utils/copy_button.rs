use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

#[cfg(all(feature = "BrowserProvider", feature = "WindowProvider"))]
use crate::components::utils::{use_browser_window as use_window, use_clipboard};

#[cfg(all(feature = "BrowserProvider", not(feature = "WindowProvider")))]
use crate::components::utils::{use_window, use_clipboard};

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
}

/// A button that copies text to the clipboard when clicked
///
/// # Example
/// ```
/// use wonopui::components::CopyButton;
///
/// fn some_component() -> Html {
///     html! {
///         <CopyButton
///             copy_text="Hello, World!"
///             copied_text={Some("Copied!".to_string())}
///             class="p-2 bg-blue-500 text-white rounded"
///         >
///             {"Copy to clipboard"}
///         </CopyButton>
///     }
/// }
/// ```
#[function_component(CopyButton)]
pub fn copy_button(props: &CopyButtonProps) -> Html {
    let is_copied = use_state(|| false);
    let timeout_handle = use_mut_ref(|| None::<i32>);
    let window = use_window();
    let clipboard_ops = use_clipboard();

    let onclick = {
        let is_copied = is_copied.clone();
        let timeout_handle = timeout_handle.clone();
        let copy_text = props.copy_text.clone();
        let copied_timeout_ms = props.copied_timeout_ms;
        let user_onclick = props.onclick.clone();
        let window = window.clone();
        let clipboard_ops = clipboard_ops.clone();

        Callback::from(move |e: MouseEvent| {
            // Call user's onclick handler if provided
            if let Some(callback) = user_onclick.clone() {
                callback.emit(e.clone());
            }

            // Copy text to clipboard
            if window.is_none() {
                return;
            }

            // Use clipboard API through BrowserProvider
            let copy_text_clone = copy_text.clone();
            let clipboard_ops = clipboard_ops.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = clipboard_ops.copy_text(&copy_text_clone).await;
            });

            // Update copied state
            is_copied.set(true);

            let window = window.as_ref().unwrap();

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

    let button_text = if *is_copied {
        match &props.copied_text {
            Some(text) => text.clone(),
            None => "Copied!".to_string(),
        }
    } else {
        match props.children.iter().count() {
            0 => "Copy".to_string(),
            _ => String::new(), // Will render children instead
        }
    };

    html! {
        <button
            class={props.class.clone()}
            onclick={onclick}
            type="button"
        >
            if *is_copied {
                if let Some(text) = &props.copied_text {
                    { text }
                } else {
                    { "Copied!" }
                }
            } else if props.children.iter().count() > 0 {
                { for props.children.iter() }
            } else {
                { "Copy" }
            }
        </button>
    }
}
