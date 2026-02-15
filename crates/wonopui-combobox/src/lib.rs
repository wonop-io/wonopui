//! Combobox component for WonopUI.
//!
//! A searchable dropdown component that combines an input with a dropdown menu.

use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Combobox component
pub mod classes {
    pub const CONTAINER: &str = "relative";
    pub const BUTTON: &str = "inline-flex items-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2 w-full justify-between";
    pub const BUTTON_OPEN: &str = "bg-accent text-accent-foreground";
    pub const BUTTON_DISABLED: &str = "opacity-50 cursor-not-allowed";
    pub const LIST: &str = "absolute z-50 mt-1 w-full bg-background border border-input rounded-md shadow-md max-h-60 overflow-auto";
    pub const ITEM: &str =
        "px-4 py-2 cursor-pointer hover:bg-accent hover:text-accent-foreground text-sm";
    pub const ITEM_SELECTED: &str =
        "px-4 py-2 cursor-pointer bg-accent text-accent-foreground text-sm";
    pub const HEADING: &str = "px-4 py-1 text-xs font-semibold text-foreground/70 uppercase";
    pub const CHEVRON_ICON: &str = "ml-2 h-4 w-4 shrink-0 opacity-50";
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
            class={container_class}
            ref={container_ref}
            onmousedown={on_mousedown}
            onmouseup={on_mouseup}
        >
            <button
                id={props.id.clone()}
                name={props.name.clone()}
                class={button_class}
                role="combobox"
                aria-expanded={open.to_string()}
                aria-label={props.aria_label.clone()}
                onclick={toggle_open}
                onblur={on_blur}
                disabled={props.disabled}
                tabindex={props.tabindex.map(|t| t.to_string())}
                style={custom_style.clone()}
            >
                { selected_label }
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes::CHEVRON_ICON}>
                    <path d="m7 15 5 5 5-5"/>
                    <path d="m7 9 5-5 5 5"/>
                </svg>
            </button>
            {
                if *open {
                    html! {
                        <div class={classes::LIST} style={custom_style}>
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
                                                class={item_class}
                                                onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                            >
                                                { label }
                                            </div>
                                        }
                                    },
                                    ComboboxItem::Heading(heading_text) => {
                                        html! {
                                            <div class={classes::HEADING}>
                                                { heading_text }
                                            </div>
                                        }
                                    }
                                }
                            }) }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
