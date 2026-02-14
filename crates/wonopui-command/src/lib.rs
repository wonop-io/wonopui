//! Command component for WonopUI.
//!
//! A command palette / search component with keyboard navigation.

use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the Command component
pub mod classes {
    pub const CONTAINER: &str = "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground";
    pub const INPUT_WRAPPER: &str = "flex items-center border-b px-3";
    pub const ICON: &str = "mr-2 h-4 w-4 shrink-0 opacity-50";
    pub const INPUT: &str = "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";
    pub const LIST: &str = "max-h-[300px] overflow-y-auto overflow-x-hidden";
    pub const ITEM: &str = "relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground";
    pub const SELECTED_ITEM: &str = "relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none bg-accent text-accent-foreground";
    pub const ITEM_ICON: &str = "mr-2 h-4 w-4";
    pub const EMPTY: &str = "py-6 text-center text-sm text-muted-foreground";
    pub const GROUP: &str = "overflow-hidden p-1 text-foreground";
    pub const GROUP_HEADING: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
}

/// A command option with value, keywords for search, label, and optional icon
pub type CommandOption<T> = (T, String, String, Option<Html>);

#[derive(Properties, PartialEq)]
pub struct CommandProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub options: Vec<CommandOption<T>>,
    #[prop_or_default]
    pub on_select: Callback<T>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub empty_message: Option<String>,
}

#[function_component(Command)]
pub fn command<T: Clone + PartialEq + 'static>(props: &CommandProps<T>) -> Html {
    let is_open = use_state(|| false);
    let value = use_state(String::new);
    let filtered_options = use_state(|| props.options.clone());
    let div_ref = use_node_ref();
    let input_ref = use_node_ref();
    let selected_index = use_state(|| 0usize);

    // Auto-focus when opening
    {
        let is_open = is_open.clone();
        let input_ref = input_ref.clone();
        use_effect_with(*is_open, move |is_open| {
            if *is_open {
                if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                    let _ = input.focus();
                }
            }
            || {}
        });
    }

    let on_select = {
        let value = value.clone();
        let is_open = is_open.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |selected_value: T| {
            value.set(String::new());
            is_open.set(false);
            on_select.emit(selected_value);
        })
    };

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    let close = {
        let is_open = is_open.clone();
        let div_ref = div_ref.clone();
        Callback::from(move |e: FocusEvent| {
            if let Some(related_target) = e.related_target() {
                let related_element: web_sys::Element = related_target.unchecked_into();
                if let Some(div_element) = div_ref.cast::<web_sys::Element>() {
                    if !div_element.contains(Some(&related_element)) {
                        is_open.set(false);
                    }
                }
            } else {
                is_open.set(false);
            }
        })
    };

    let oninput = {
        let is_open = is_open.clone();
        let value = value.clone();
        let filtered_options = filtered_options.clone();
        let options = props.options.clone();
        let selected_index = selected_index.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if !*is_open {
                is_open.set(true);
            }
            let new_value = input.value().to_lowercase();
            value.set(new_value.clone());
            let new_filtered: Vec<CommandOption<T>> = options
                .iter()
                .filter(|(_, keywords, label, _)| {
                    keywords.to_lowercase().contains(&new_value)
                        || label.to_lowercase().contains(&new_value)
                })
                .cloned()
                .collect();
            filtered_options.set(new_filtered);
            selected_index.set(0);
        })
    };

    let onkeydown = {
        let is_open = is_open.clone();
        let filtered_options = filtered_options.clone();
        let selected_index = selected_index.clone();
        let on_select = on_select.clone();
        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Escape" => {
                    is_open.set(false);
                    e.prevent_default();
                }
                "Enter" => {
                    if *is_open {
                        if let Some((val, _, _, _)) = filtered_options.get(*selected_index) {
                            on_select.emit(val.clone());
                        }
                        e.prevent_default();
                    }
                }
                "ArrowDown" => {
                    if *is_open && !filtered_options.is_empty() {
                        selected_index.set((*selected_index + 1) % filtered_options.len());
                        e.prevent_default();
                    }
                }
                "ArrowUp" => {
                    if *is_open && !filtered_options.is_empty() {
                        selected_index.set(
                            (*selected_index + filtered_options.len() - 1) % filtered_options.len(),
                        );
                        e.prevent_default();
                    }
                }
                _ => {}
            }
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);
    let empty_message = props.empty_message.clone().unwrap_or_else(|| "No results found.".to_string());

    html! {
        <div ref={div_ref} class={container_class} tabindex="0" onfocusout={close}>
            <div class={classes::INPUT_WRAPPER}>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes::ICON}>
                    <circle cx="11" cy="11" r="8"/>
                    <path d="m21 21-4.3-4.3"/>
                </svg>
                <input
                    ref={input_ref}
                    class={classes::INPUT}
                    placeholder={props.placeholder.clone()}
                    autocomplete="off"
                    autocorrect="off"
                    spellcheck="false"
                    onclick={toggle.clone()}
                    oninput={oninput}
                    onkeydown={onkeydown}
                    value={(*value).clone()}
                />
            </div>
            {
                if *is_open {
                    html! {
                        <div class={classes::LIST} role="listbox">
                            {
                                if filtered_options.is_empty() {
                                    html! {
                                        <div class={classes::EMPTY}>
                                            { empty_message.clone() }
                                        </div>
                                    }
                                } else {
                                    filtered_options.iter().enumerate().map(|(index, (val, _, label, icon))| {
                                        let on_select = on_select.clone();
                                        let val = val.clone();
                                        let is_selected = index == *selected_index;
                                        let item_class = if is_selected {
                                            classes::SELECTED_ITEM
                                        } else {
                                            classes::ITEM
                                        };
                                        html! {
                                            <div
                                                class={item_class}
                                                onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                                role="option"
                                                aria-selected={is_selected.to_string()}
                                            >
                                                if let Some(icon) = icon {
                                                    <span class={classes::ITEM_ICON}>
                                                        { icon.clone() }
                                                    </span>
                                                }
                                                <span>{ label }</span>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
