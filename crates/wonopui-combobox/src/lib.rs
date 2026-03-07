//! Combobox component for WonopUI.
//!
//! A searchable dropdown component that combines an input with a dropdown menu.

use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Combobox component (shadcn v4)
pub mod classes {
    /// Root container
    pub const CONTAINER: &str = "relative";
    /// Trigger button - premium outline style with smooth transitions
    pub const BUTTON: &str = "flex h-9 w-full items-center justify-between gap-2 whitespace-nowrap rounded-md border border-zinc-200 bg-transparent px-3 py-2 text-sm shadow-xs transition-all duration-200 placeholder:text-zinc-500 focus-visible:border-zinc-950 focus-visible:ring-zinc-950/50 focus-visible:ring-[3px] focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:border-zinc-800 dark:placeholder:text-zinc-400 dark:focus-visible:border-zinc-300 dark:focus-visible:ring-zinc-300/50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 text-zinc-900 dark:text-zinc-50 *:data-[slot=combobox-icon]:text-zinc-500 dark:*:data-[slot=combobox-icon]:text-zinc-400";
    /// Open state styling
    pub const BUTTON_OPEN: &str = "border-zinc-950 ring-zinc-950/50 ring-[3px] dark:border-zinc-300 dark:ring-zinc-300/50";
    /// Disabled state
    pub const BUTTON_DISABLED: &str = "cursor-not-allowed opacity-50";
    /// Dropdown list container - premium popover style
    pub const LIST: &str = "absolute z-50 mt-2 w-full overflow-hidden rounded-xl border border-zinc-200 bg-white text-zinc-950 shadow-lg dark:border-zinc-800 dark:bg-zinc-950 dark:text-zinc-50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";
    /// List inner wrapper - more padding for premium feel
    pub const LIST_INNER: &str = "max-h-60 overflow-y-auto overflow-x-hidden p-1.5";
    /// Individual item - premium interactive styling with better spacing
    pub const ITEM: &str = "relative flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-3 py-2.5 text-sm outline-none transition-colors hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 text-zinc-700 dark:text-zinc-300";
    /// Selected item with check indicator
    pub const ITEM_SELECTED: &str = "relative flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-3 py-2.5 text-sm outline-none bg-zinc-100 text-zinc-900 dark:bg-zinc-800 dark:text-zinc-50 font-medium";
    /// Group heading with proper spacing
    pub const HEADING: &str = "px-3 py-2 text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide";
    /// Chevron icon
    pub const CHEVRON_ICON: &str = "size-4 shrink-0 opacity-50";
    /// Check icon for selected items
    pub const CHECK_ICON: &str = "size-4 shrink-0";
    /// Empty state
    pub const EMPTY: &str = "py-6 text-center text-sm text-zinc-500 dark:text-zinc-400";
}

/// Represents an item in the combobox
#[derive(Debug, Clone, PartialEq)]
pub enum ComboboxItem {
    /// A selectable option with (value, label)
    Option(String, String),
    /// A non-selectable heading
    Heading(String),
}

#[derive(Properties, PartialEq)]
pub struct ComboboxProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub options: Vec<ComboboxItem>,
    #[prop_or_default]
    pub on_select: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub tabindex: Option<i32>,
}

#[function_component(Combobox)]
pub fn combobox(props: &ComboboxProps) -> Html {
    let open = use_state(|| false);
    let value = use_state(|| props.value.clone().unwrap_or_default());
    let container_ref = use_node_ref();
    let is_mouse_down_inside = use_state(|| false);

    // Update internal value when prop changes
    {
        let value = value.clone();
        let prop_value = props.value.clone();
        use_effect_with(prop_value, move |prop_value| {
            if let Some(val) = prop_value {
                value.set(val.clone());
            }
            || ()
        });
    }

    // Close dropdown when clicking outside
    {
        let open = open.clone();
        let container_ref = container_ref.clone();

        use_effect_with((), move |_| {
            let open = open.clone();
            let container_ref = container_ref.clone();

            let document = web_sys::window().unwrap().document().unwrap();
            let listener = EventListener::new(&document, "mousedown", move |event| {
                if let Some(container) = container_ref.get() {
                    let target = event.target().unwrap();
                    if let Ok(node) = target.dyn_into::<web_sys::Node>() {
                        if !container.contains(Some(&node)) {
                            open.set(false);
                        }
                    }
                }
            });

            move || drop(listener)
        });
    }

    // Track mouse events inside the container
    let on_mousedown = {
        let is_mouse_down_inside = is_mouse_down_inside.clone();
        Callback::from(move |_: MouseEvent| {
            is_mouse_down_inside.set(true);
        })
    };

    let on_mouseup = {
        let is_mouse_down_inside = is_mouse_down_inside.clone();
        Callback::from(move |_: MouseEvent| {
            is_mouse_down_inside.set(false);
        })
    };

    // Close on blur, but only if the mouse is not down inside the container
    let on_blur = {
        let open = open.clone();
        let is_mouse_down_inside = is_mouse_down_inside.clone();
        Callback::from(move |_: FocusEvent| {
            if !*is_mouse_down_inside {
                let open_clone = open.clone();
                let timeout = Timeout::new(100, move || {
                    open_clone.set(false);
                });
                timeout.forget();
            }
        })
    };

    let on_select = {
        let value = value.clone();
        let open = open.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |selected_value: String| {
            value.set(selected_value.clone());
            open.set(false);
            on_select.emit(selected_value);
        })
    };

    let selected_label = props
        .options
        .iter()
        .find_map(|item| {
            if let ComboboxItem::Option(val, label) = item {
                if val == value.as_str() {
                    Some(label.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            props
                .placeholder
                .clone()
                .unwrap_or_else(|| "Select option...".to_string())
        });

    let toggle_open = {
        let open = open.clone();
        Callback::from(move |_: MouseEvent| open.set(!*open))
    };

    let custom_style = props.width.as_ref().map(|w| format!("width: {};", w));

    let button_class = merge_classes(&[
        classes::BUTTON,
        if *open { classes::BUTTON_OPEN } else { "" },
        if props.disabled {
            classes::BUTTON_DISABLED
        } else {
            ""
        },
    ]);

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div
            data-slot="combobox"
            class={container_class}
            ref={container_ref}
            onmousedown={on_mousedown}
            onmouseup={on_mouseup}
        >
            <button
                data-slot="combobox-trigger"
                data-state={if *open { "open" } else { "closed" }}
                id={props.id.clone()}
                name={props.name.clone()}
                class={button_class}
                type="button"
                role="combobox"
                aria-expanded={open.to_string()}
                aria-haspopup="listbox"
                aria-label={props.aria_label.clone()}
                onclick={toggle_open}
                onblur={on_blur}
                disabled={props.disabled}
                tabindex={props.tabindex.map(|t| t.to_string())}
                style={custom_style.clone()}
            >
                <span data-slot="combobox-value">{ selected_label }</span>
                <svg data-slot="combobox-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes::CHEVRON_ICON} aria-hidden="true">
                    <path d="m6 9 6 6 6-6"/>
                </svg>
            </button>
            {
                if *open {
                    html! {
                        <div
                            data-slot="combobox-content"
                            data-state="open"
                            class={classes::LIST}
                            style={custom_style}
                            role="listbox"
                        >
                            <div data-slot="combobox-list" class={classes::LIST_INNER}>
                                { for props.options.iter().map(|item| {
                                    match item {
                                        ComboboxItem::Option(val, label) => {
                                            let on_select = on_select.clone();
                                            let val = val.clone();
                                            let is_selected = *value == val;
                                            let item_class = if is_selected {
                                                classes::ITEM_SELECTED
                                            } else {
                                                classes::ITEM
                                            };
                                            html! {
                                                <div
                                                    data-slot="combobox-item"
                                                    data-selected={is_selected.then_some("true")}
                                                    class={item_class}
                                                    role="option"
                                                    aria-selected={is_selected.to_string()}
                                                    onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                                >
                                                    if is_selected {
                                                        <svg data-slot="combobox-check" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes::CHECK_ICON} aria-hidden="true">
                                                            <path d="M20 6 9 17l-5-5"/>
                                                        </svg>
                                                    } else {
                                                        <span class="size-4"></span>
                                                    }
                                                    <span>{ label }</span>
                                                </div>
                                            }
                                        },
                                        ComboboxItem::Heading(heading_text) => {
                                            html! {
                                                <div data-slot="combobox-group-heading" class={classes::HEADING}>
                                                    { heading_text }
                                                </div>
                                            }
                                        }
                                    }
                                }) }
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
