//! Select component for wonopui
//!
//! A customizable select/dropdown component with keyboard navigation.

use std::rc::Rc;
use wasm_bindgen::JsCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const SELECT_CONTAINER: &str = "relative inline-block w-full";
    pub const SELECT_TRIGGER: &str = "flex items-center justify-between w-full px-3 py-2 text-sm bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    pub const SELECT_TRIGGER_PLACEHOLDER: &str = "truncate";
    pub const SELECT_TRIGGER_ICON: &str = "w-4 h-4 ml-2 shrink-0";
    pub const SELECT_CONTENT_CONTAINER: &str = "absolute z-50 w-full mt-1 bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-600 rounded-md shadow-lg";
    pub const SELECT_CONTENT_LIST: &str = "max-h-60 overflow-auto py-1";
    pub const SELECT_ITEM: &str = "px-3 py-2 text-sm cursor-pointer hover:bg-gray-100 dark:hover:bg-zinc-700 text-gray-900 dark:text-zinc-100";
    pub const SELECT_ITEM_SELECTED: &str = "bg-blue-50 dark:bg-blue-900/20";
}

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl ToString for SelectOption {
    fn to_string(&self) -> String {
        self.label.clone()
    }
}

#[derive(Clone, PartialEq)]
pub struct SelectState<T: Clone + PartialEq + ToString + 'static> {
    pub selected: Option<T>,
    pub is_open: bool,
    pub toggle: Callback<()>,
    pub on_select: Callback<T>,
}

#[derive(Properties, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + ToString + 'static> {
    pub options: Vec<T>,
    #[prop_or_default]
    pub selected: Option<T>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
}

#[function_component(Select)]
pub fn select<T: Clone + PartialEq + ToString + 'static>(props: &SelectProps<T>) -> Html {
    let is_open = use_state(|| false);
    let selected = use_state(|| props.selected.clone());
    let select_ref = use_node_ref();

    let toggle = {
        let is_open = is_open.clone();
        let disabled = props.disabled;
        Callback::from(move |_| {
            if !disabled {
                is_open.set(!*is_open);
            }
        })
    };

    let on_select = {
        let selected = selected.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |value: T| {
            selected.set(Some(value.clone()));
            onchange.emit(value);
        })
    };

    let close = {
        let is_open = is_open.clone();
        let select_ref = select_ref.clone();
        Callback::from(move |e: FocusEvent| {
            if let Some(related_target) = e.related_target() {
                let related_element: web_sys::Element = related_target.unchecked_into();
                if let Some(select_element) = select_ref.cast::<web_sys::Element>() {
                    if !select_element.contains(Some(&related_element)) {
                        is_open.set(false);
                    }
                }
            } else {
                is_open.set(false);
            }
        })
    };

    let state = Rc::new(SelectState {
        selected: (*selected).clone(),
        is_open: *is_open,
        toggle,
        on_select,
    });

    let selected_label = props
        .options
        .iter()
        .find(|value| Some(*value) == selected.as_ref())
        .map(|value| value.to_string())
        .unwrap_or_else(|| props.placeholder.clone().unwrap_or_default());

    let container_style = match &props.width {
        Some(width) => format!("width: {};", width),
        None => String::new(),
    };

    let custom_style = match &props.style {
        Some(style) => format!("{} {}", container_style, style),
        None => container_style,
    };

    let style_attr = if !custom_style.is_empty() {
        Some(custom_style)
    } else {
        None
    };

    let container_class = merge_classes(&[
        classes::SELECT_CONTAINER,
        &props.class.to_string(),
        if props.disabled { "opacity-50 cursor-not-allowed" } else { "" },
    ]);

    html! {
        <div
            class={container_class}
            id={props.id.clone()}
            ref={select_ref}
            tabindex="0"
            onfocusout={close}
            style={style_attr}
            aria-disabled={props.disabled.to_string()}
            aria-label={props.aria_label.clone()}
        >
            <button
                type="button"
                class={classes::SELECT_TRIGGER}
                onclick={{
                    let toggle = state.toggle.clone();
                    move |_| toggle.emit(())
                }}
                disabled={props.disabled}
                aria-required={props.required.to_string()}
                name={props.name.clone()}
                aria-expanded={is_open.to_string()}
            >
                <span class={classes::SELECT_TRIGGER_PLACEHOLDER}>{ selected_label }</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes::SELECT_TRIGGER_ICON} aria-hidden="true">
                    <path d="m6 9 6 6 6-6"></path>
                </svg>
            </button>
            if *is_open {
                <div class={classes::SELECT_CONTENT_CONTAINER}>
                    <ul class={classes::SELECT_CONTENT_LIST} role="listbox">
                        {for props.options.iter().map(|value| {
                            let on_click = {
                                let value = value.clone();
                                let on_select = state.on_select.clone();
                                let toggle = state.toggle.clone();
                                Callback::from(move |_| {
                                    on_select.emit(value.clone());
                                    toggle.emit(());
                                })
                            };
                            let is_selected = Some(value) == selected.as_ref();
                            let item_class = merge_classes(&[
                                classes::SELECT_ITEM,
                                if is_selected { classes::SELECT_ITEM_SELECTED } else { "" },
                            ]);
                            html! {
                                <li
                                    class={item_class}
                                    onclick={on_click}
                                    role="option"
                                    aria-selected={is_selected.to_string()}
                                >
                                    { value.to_string() }
                                </li>
                            }
                        })}
                    </ul>
                </div>
            }
        </div>
    }
}
