//! Select component for wonopui
//!
//! A customizable select/dropdown component with keyboard navigation.

use std::fmt;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const SELECT_CONTAINER: &str = "relative inline-block";
    pub const SELECT_CONTAINER_FULL_WIDTH: &str = "w-full";
    
    // Default variant
    pub const SELECT_TRIGGER_DEFAULT: &str = "flex items-center justify-between w-full bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    
    // Compact variant
    pub const SELECT_TRIGGER_COMPACT: &str = "flex items-center justify-between w-full bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 rounded shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    
    // Ghost variant
    pub const SELECT_TRIGGER_GHOST: &str = "flex items-center justify-between w-full bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    
    // Sizes
    pub const SIZE_XSMALL: &str = "px-2 py-1 text-xs";
    pub const SIZE_SMALL: &str = "px-2.5 py-1.5 text-sm";
    pub const SIZE_MEDIUM: &str = "px-3 py-2 text-sm";
    pub const SIZE_LARGE: &str = "px-4 py-2.5 text-base";
    
    pub const SELECT_TRIGGER_PLACEHOLDER: &str = "truncate text-gray-900 dark:text-zinc-100";
    pub const SELECT_TRIGGER_PLACEHOLDER_EMPTY: &str = "truncate text-gray-400 dark:text-zinc-500";
    pub const SELECT_TRIGGER_ICON: &str = "w-4 h-4 ml-2 shrink-0";
    pub const SELECT_TRIGGER_ICON_XSMALL: &str = "w-3 h-3 ml-1 shrink-0";
    pub const SELECT_CONTENT_CONTAINER: &str = "absolute z-50 w-full mt-1 bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-600 rounded-md shadow-lg";
    pub const SELECT_CONTENT_LIST: &str = "max-h-60 overflow-auto py-1";
    pub const SELECT_ITEM: &str = "px-3 py-2 text-sm cursor-pointer hover:bg-gray-100 dark:hover:bg-zinc-700 text-gray-900 dark:text-zinc-100";
    pub const SELECT_ITEM_SELECTED: &str = "bg-blue-50 dark:bg-blue-900/20";
    pub const SELECT_ITEM_DISABLED: &str = "opacity-50 cursor-not-allowed";
    pub const SELECT_SEPARATOR: &str = "border-t border-gray-200 dark:border-zinc-600 my-1";
    pub const SELECT_LABEL: &str = "block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5";
}

/// Select variant determines the visual style.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SelectVariant {
    #[default]
    Default,
    Compact,
    Ghost,
}

/// Select size determines the dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SelectSize {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Create an option where label equals value.
    pub fn simple(value: impl Into<String>) -> Self {
        let v = value.into();
        Self {
            value: v.clone(),
            label: v,
            disabled: false,
        }
    }

    /// Create a disabled separator line.
    pub fn separator() -> Self {
        Self {
            value: String::new(),
            label: String::new(),
            disabled: true,
        }
    }

    /// Check if this option is a separator.
    pub fn is_separator(&self) -> bool {
        self.value.is_empty() && self.label.is_empty() && self.disabled
    }
}

impl fmt::Display for SelectOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
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
    /// Visual variant of the select.
    #[prop_or_default]
    pub variant: SelectVariant,
    /// Size of the select.
    #[prop_or_default]
    pub size: SelectSize,
    /// Label text above the select.
    #[prop_or_default]
    pub label: Option<String>,
    /// Make select full width (w-full).
    #[prop_or_default]
    pub full_width: bool,
    /// Maximum width CSS value (e.g., "120px").
    #[prop_or_default]
    pub max_width: Option<String>,
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
        .map(|value| value.to_string());

    let has_selection = selected_label.is_some();
    let display_label = selected_label.unwrap_or_else(|| props.placeholder.clone().unwrap_or_default());

    // Build container style
    let mut style_parts = Vec::new();
    if let Some(width) = &props.width {
        style_parts.push(format!("width: {};", width));
    }
    if let Some(max_width) = &props.max_width {
        style_parts.push(format!("max-width: {};", max_width));
    }
    if let Some(custom_style) = &props.style {
        style_parts.push(custom_style.clone());
    }
    let style_attr = if !style_parts.is_empty() {
        Some(style_parts.join(" "))
    } else {
        None
    };

    let trigger_variant_class = match props.variant {
        SelectVariant::Default => classes::SELECT_TRIGGER_DEFAULT,
        SelectVariant::Compact => classes::SELECT_TRIGGER_COMPACT,
        SelectVariant::Ghost => classes::SELECT_TRIGGER_GHOST,
    };

    let size_class = match props.size {
        SelectSize::XSmall => classes::SIZE_XSMALL,
        SelectSize::Small => classes::SIZE_SMALL,
        SelectSize::Medium => classes::SIZE_MEDIUM,
        SelectSize::Large => classes::SIZE_LARGE,
    };

    let icon_class = match props.size {
        SelectSize::XSmall => classes::SELECT_TRIGGER_ICON_XSMALL,
        _ => classes::SELECT_TRIGGER_ICON,
    };

    let container_class = merge_classes(&[
        classes::SELECT_CONTAINER,
        if props.full_width { classes::SELECT_CONTAINER_FULL_WIDTH } else { "" },
        &props.class.to_string(),
        if props.disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            ""
        },
    ]);

    let trigger_class = merge_classes(&[
        trigger_variant_class,
        size_class,
    ]);

    let placeholder_class = if has_selection {
        classes::SELECT_TRIGGER_PLACEHOLDER
    } else {
        classes::SELECT_TRIGGER_PLACEHOLDER_EMPTY
    };

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
            if let Some(label) = &props.label {
                <label class={classes::SELECT_LABEL}>{ label }</label>
            }
            <button
                type="button"
                class={trigger_class}
                onclick={{
                    let toggle = state.toggle.clone();
                    move |_| toggle.emit(())
                }}
                disabled={props.disabled}
                aria-required={props.required.to_string()}
                name={props.name.clone()}
                aria-expanded={is_open.to_string()}
            >
                <span class={placeholder_class}>{ display_label }</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={icon_class} aria-hidden="true">
                    <path d="m6 9 6 6 6-6"></path>
                </svg>
            </button>
            if *is_open {
                <div class={classes::SELECT_CONTENT_CONTAINER}>
                    <ul class={classes::SELECT_CONTENT_LIST} role="listbox">
                        {for props.options.iter().map(|value| {
                            let value_str = value.to_string();
                            // Check if it's a separator (empty string with special handling)
                            if value_str.is_empty() {
                                return html! {
                                    <li class={classes::SELECT_SEPARATOR}></li>
                                };
                            }

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
                                    { value_str }
                                </li>
                            }
                        })}
                    </ul>
                </div>
            }
        </div>
    }
}