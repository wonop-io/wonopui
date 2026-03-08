//! Select component for wonopui
//!
//! A customizable select/dropdown component with keyboard navigation.
//! Styled to match shadcn/ui v4 design system.

use std::fmt;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for select styling.
/// Based on shadcn/ui v4 select component.
pub mod classes {
    /// Container styles.
    pub const SELECT_CONTAINER: &str = "relative inline-block w-fit";
    
    /// Full width container.
    pub const SELECT_CONTAINER_FULL_WIDTH: &str = "w-full";
    
    /// Trigger button styles - matches shadcn v4 SelectTrigger.
    /// Uses shadcn v4 focus pattern: focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    pub const SELECT_TRIGGER: &str = "flex h-10 w-fit items-center justify-between gap-2 rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent dark:bg-zinc-950/30 dark:hover:bg-zinc-950/50 px-3.5 py-2 text-sm whitespace-nowrap shadow-xs transition-[color,box-shadow] duration-200 outline-none focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 text-zinc-900 dark:text-zinc-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
    
    /// Compact variant trigger.
    pub const SELECT_TRIGGER_COMPACT: &str = "flex items-center justify-between w-full bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 rounded shadow-sm focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    
    /// Ghost variant trigger.
    pub const SELECT_TRIGGER_GHOST: &str = "flex items-center justify-between w-full bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed";
    
    /// Extra small size.
    pub const SIZE_XSMALL: &str = "px-2 py-1 text-xs h-7";
    
    /// Small size.
    pub const SIZE_SMALL: &str = "px-2.5 py-1.5 text-sm h-8";
    
    /// Medium size.
    pub const SIZE_MEDIUM: &str = "px-3 py-2 text-sm h-10";
    
    /// Large size.
    pub const SIZE_LARGE: &str = "px-4 py-2.5 text-base h-11";
    
    /// Placeholder text styles.
    pub const SELECT_TRIGGER_PLACEHOLDER: &str = "line-clamp-1 flex items-center gap-2 text-zinc-500 dark:text-zinc-400";
    
    /// Selected value text styles.
    pub const SELECT_TRIGGER_VALUE: &str = "line-clamp-1 flex items-center gap-2";
    
    /// Chevron icon styles.
    pub const SELECT_TRIGGER_ICON: &str = "size-4 opacity-50 shrink-0";
    
    /// Extra small icon.
    pub const SELECT_TRIGGER_ICON_XSMALL: &str = "w-3 h-3 ml-1 shrink-0";
    
    /// Content container styles - matches shadcn v4 SelectContent with animations.
    pub const SELECT_CONTENT_CONTAINER: &str = "absolute z-50 w-full mt-1 min-w-[8rem] overflow-hidden rounded-md border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 shadow-md animate-in fade-in-0 zoom-in-95 data-[side=bottom]:slide-in-from-top-2";
    
    /// Search input container
    pub const SELECT_SEARCH_CONTAINER: &str = "px-2 pt-2 pb-1 sticky top-0 bg-white dark:bg-zinc-950";
    
    /// Search input styles
    pub const SELECT_SEARCH_INPUT: &str = "flex h-9 w-full rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent px-3 py-2 text-sm placeholder:text-zinc-500 dark:placeholder:text-zinc-400 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] outline-none";
    
    /// Content list styles.
    pub const SELECT_CONTENT_LIST: &str = "max-h-60 overflow-x-hidden overflow-y-auto p-1 scroll-my-1";
    
    /// Item styles - matches shadcn v4 SelectItem.
    pub const SELECT_ITEM: &str = "relative flex w-full cursor-default select-none items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-none focus:bg-zinc-100 dark:focus:bg-zinc-800 focus:text-zinc-900 dark:focus:text-zinc-50 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-900 dark:text-zinc-50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    
    /// Selected item indicator container.
    pub const SELECT_ITEM_INDICATOR: &str = "absolute right-2 flex size-3.5 items-center justify-center";
    
    /// Selected item styles.
    pub const SELECT_ITEM_SELECTED: &str = "bg-zinc-100 dark:bg-zinc-800";
    
    /// Label styles for groups.
    pub const SELECT_LABEL: &str = "px-2 py-1.5 text-xs text-zinc-500 dark:text-zinc-400";
    
    /// Separator styles.
    pub const SELECT_SEPARATOR: &str = "bg-zinc-200 dark:bg-zinc-800 pointer-events-none -mx-1 my-1 h-px";
    
    /// No results message
    pub const SELECT_NO_RESULTS: &str = "py-6 text-center text-sm text-zinc-500 dark:text-zinc-400";
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
    /// Enable search/filter functionality in the dropdown
    #[prop_or_default]
    pub searchable: bool,
    /// Placeholder text for search input
    #[prop_or("Search...".to_string())]
    pub search_placeholder: String,
    /// Default selected value (used on initial render)
    #[prop_or_default]
    pub default_value: Option<T>,
}

#[function_component(Select)]
pub fn select<T: Clone + PartialEq + ToString + 'static>(props: &SelectProps<T>) -> Html {
    let is_open = use_state(|| false);
    // Use default_value if selected is None, otherwise use selected
    let initial_value = props.selected.clone().or_else(|| props.default_value.clone());
    let selected = use_state(|| initial_value);
    let search_query = use_state(String::new);
    let select_ref = use_node_ref();
    let search_input_ref = use_node_ref();

    // Reset search when dropdown closes
    {
        let search_query = search_query.clone();
        let is_open_val = *is_open;
        use_effect_with(is_open_val, move |&open| {
            if !open {
                search_query.set(String::new());
            }
            || ()
        });
    }

    // Focus search input when dropdown opens
    {
        let search_input_ref = search_input_ref.clone();
        let is_open_val = *is_open;
        let searchable = props.searchable;
        use_effect_with(is_open_val, move |&open| {
            if open && searchable {
                if let Some(input) = search_input_ref.cast::<web_sys::HtmlInputElement>() {
                    let _ = input.focus();
                }
            }
            || ()
        });
    }

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

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
            }
        })
    };

    let state = Rc::new(SelectState {
        selected: (*selected).clone(),
        is_open: *is_open,
        toggle,
        on_select,
    });

    // Filter options based on search query
    let filtered_options: Vec<&T> = if props.searchable && !search_query.is_empty() {
        let query_lower = search_query.to_lowercase();
        props.options.iter()
            .filter(|opt| opt.to_string().to_lowercase().contains(&query_lower))
            .collect()
    } else {
        props.options.iter().collect()
    };

    let selected_label = props
        .options
        .iter()
        .find(|value| Some(*value) == selected.as_ref())
        .map(|value| value.to_string());

    let has_selection = selected_label.is_some();
    let display_label = selected_label.clone().unwrap_or_else(|| props.placeholder.clone().unwrap_or_default());

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
        SelectVariant::Default => classes::SELECT_TRIGGER,
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

    let value_class = if has_selection {
        classes::SELECT_TRIGGER_VALUE
    } else {
        classes::SELECT_TRIGGER_PLACEHOLDER
    };

    html! {
        <div
            data-slot="select"
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
                data-slot="select-trigger"
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
                <span data-slot="select-value" class={value_class}>{ display_label }</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={icon_class} aria-hidden="true">
                    <path d="m6 9 6 6 6-6"></path>
                </svg>
            </button>
            if *is_open {
                <div data-slot="select-content" data-side="bottom" class={classes::SELECT_CONTENT_CONTAINER}>
                    if props.searchable {
                        <div class={classes::SELECT_SEARCH_CONTAINER}>
                            <input
                                ref={search_input_ref}
                                type="text"
                                class={classes::SELECT_SEARCH_INPUT}
                                placeholder={props.search_placeholder.clone()}
                                value={(*search_query).clone()}
                                oninput={on_search_input}
                            />
                        </div>
                    }
                    <ul class={classes::SELECT_CONTENT_LIST} role="listbox">
                        if filtered_options.is_empty() {
                            <li class={classes::SELECT_NO_RESULTS}>{"No results found"}</li>
                        } else {
                            {for filtered_options.iter().map(|value| {
                                let value_str = value.to_string();
                                // Check if it's a separator (empty string with special handling)
                                if value_str.is_empty() {
                                    return html! {
                                        <li class={classes::SELECT_SEPARATOR}></li>
                                    };
                                }

                                let on_click = {
                                    let value = (*value).clone();
                                    let on_select = state.on_select.clone();
                                    let toggle = state.toggle.clone();
                                    Callback::from(move |_| {
                                        on_select.emit(value.clone());
                                        toggle.emit(());
                                    })
                                };
                                let is_selected = Some(*value) == selected.as_ref();
                                let item_class = merge_classes(&[
                                    classes::SELECT_ITEM,
                                    if is_selected { classes::SELECT_ITEM_SELECTED } else { "" },
                                ]);
                                html! {
                                    <li
                                        data-slot="select-item"
                                        class={item_class}
                                        onclick={on_click}
                                        role="option"
                                        aria-selected={is_selected.to_string()}
                                    >
                                        if is_selected {
                                            <span data-slot="select-item-indicator" class={classes::SELECT_ITEM_INDICATOR}>
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-4">
                                                    <path d="M20 6 9 17l-5-5"></path>
                                                </svg>
                                            </span>
                                        }
                                        { value_str }
                                    </li>
                                }
                            })}
                        }
                    </ul>
                </div>
            }
        </div>
    }
}