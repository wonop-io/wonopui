//! Command component for WonopUI.
//!
//! A command palette / search component with keyboard navigation.

use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Command component (shadcn v4)
pub mod classes {
    /// Root container - premium border and shadow
    pub const CONTAINER: &str = "flex h-full w-full flex-col overflow-hidden rounded-xl bg-white text-zinc-950 shadow-lg dark:bg-zinc-950 dark:text-zinc-50 border border-zinc-200 dark:border-zinc-800";
    /// Input wrapper with border and better spacing
    pub const INPUT_WRAPPER: &str = "flex items-center gap-3 border-b border-zinc-200 px-4 dark:border-zinc-800";
    /// Search icon - larger for better alignment
    pub const ICON: &str = "size-5 shrink-0 text-zinc-400 dark:text-zinc-500";
    /// Input field with no outline
    pub const INPUT: &str = "flex h-12 w-full bg-transparent py-3 text-sm outline-none placeholder:text-zinc-500 disabled:cursor-not-allowed disabled:opacity-50 dark:placeholder:text-zinc-400 text-zinc-900 dark:text-zinc-50";
    /// List container with padding
    pub const LIST: &str = "max-h-[300px] overflow-y-auto overflow-x-hidden scroll-py-1 p-1.5";
    /// Individual item - premium interactive styling with better spacing
    pub const ITEM: &str = "relative flex cursor-pointer select-none items-center gap-3 rounded-lg px-3 py-2.5 text-sm outline-none transition-colors hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 text-zinc-700 dark:text-zinc-300";
    /// Selected/highlighted item
    pub const SELECTED_ITEM: &str = "relative flex cursor-pointer select-none items-center gap-3 rounded-lg px-3 py-2.5 text-sm outline-none bg-zinc-100 text-zinc-900 dark:bg-zinc-800 dark:text-zinc-50";
    /// Icon within item - properly sized for alignment
    pub const ITEM_ICON: &str = "size-4 shrink-0 text-zinc-500 dark:text-zinc-400 flex items-center justify-center";
    /// Empty state message
    pub const EMPTY: &str = "py-8 text-center text-sm text-zinc-500 dark:text-zinc-400";
    /// Group container
    pub const GROUP: &str = "overflow-hidden text-zinc-950 dark:text-zinc-50";
    /// Group heading with better spacing
    pub const GROUP_HEADING: &str = "px-3 py-2 text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide";
    /// Separator between groups
    pub const SEPARATOR: &str = "my-1.5 h-px bg-zinc-200 dark:bg-zinc-800";
    /// Shortcut key display
    pub const SHORTCUT: &str = "ml-auto text-xs tracking-widest text-zinc-400 dark:text-zinc-500";
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
            let new_value = input.value();
            let search_value = new_value.to_lowercase();
            value.set(new_value);
            let new_filtered: Vec<CommandOption<T>> = options
                .iter()
                .filter(|(_, keywords, label, _)| {
                    keywords.to_lowercase().contains(&search_value)
                        || label.to_lowercase().contains(&search_value)
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
        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
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
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);
    let empty_message = props
        .empty_message
        .clone()
        .unwrap_or_else(|| "No results found.".to_string());

    html! {
        <div data-slot="command" ref={div_ref} class={container_class} tabindex="0" onfocusout={close}>
            <div data-slot="command-input-wrapper" class={classes::INPUT_WRAPPER}>
                <svg data-slot="command-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes::ICON} aria-hidden="true">
                    <circle cx="11" cy="11" r="8"/>
                    <path d="m21 21-4.3-4.3"/>
                </svg>
                <input
                    data-slot="command-input"
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
                    type="text"
                    role="combobox"
                    aria-expanded={is_open.to_string()}
                    aria-autocomplete="list"
                />
            </div>
            {
                if *is_open {
                    html! {
                        <div data-slot="command-list" class={classes::LIST} role="listbox">
                            {
                                if filtered_options.is_empty() {
                                    html! {
                                        <div data-slot="command-empty" class={classes::EMPTY}>
                                            { empty_message.clone() }
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div data-slot="command-group" class={classes::GROUP}>
                                            { filtered_options.iter().enumerate().map(|(index, (val, _, label, icon))| {
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
                                                        data-slot="command-item"
                                                        data-selected={is_selected.then_some("true")}
                                                        class={item_class}
                                                        onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                                        role="option"
                                                        aria-selected={is_selected.to_string()}
                                                    >
                                                        if let Some(icon) = icon {
                                                            <span data-slot="command-item-icon" class={classes::ITEM_ICON}>
                                                                { icon.clone() }
                                                            </span>
                                                        }
                                                        <span>{ label }</span>
                                                    </div>
                                                }
                                            }).collect::<Html>() }
                                        </div>
                                    }
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
