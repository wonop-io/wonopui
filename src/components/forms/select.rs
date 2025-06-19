#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use std::rc::Rc;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
use yew::prelude::*;

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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
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

    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    let close = Callback::from(move |_: FocusEvent| {});

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

    let container_class = classes!(
        &brandguide.select_container,
        props.class.clone(),
        props.disabled.then_some("opacity-50 cursor-not-allowed")
    );

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
                class={&brandguide.select_trigger}
                onclick={{
                    let toggle = state.toggle.clone();
                    move |_| toggle.emit(())
                }}
                disabled={props.disabled}
                aria-required={props.required.to_string()}
                name={props.name.clone()}
                aria-expanded={is_open.to_string()}
            >
                <span class={&brandguide.select_trigger_placeholder}>{ selected_label }</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={&brandguide.select_trigger_icon} aria-hidden="true">
                    <path d="m6 9 6 6 6-6"></path>
                </svg>
            </button>
            if *is_open {
                <div class={&brandguide.select_content_container}>
                    <ul class={&brandguide.select_content_list} role="listbox">
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
                            html! {
                                <li
                                    class={classes!(&brandguide.select_item, is_selected.then_some("bg-blue-50 dark:bg-blue-900/20"))}
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

// New component names to add to BRANDGUIDE:
// select_container: "relative inline-block text-left"
// select_trigger: "flex h-10 items-center justify-between rounded-md border border-gray-300 bg-white px-3 py-2 text-sm ring-offset-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-[180px]"
// select_trigger_placeholder: "pointer-events-none"
// select_trigger_icon: "lucide lucide-chevron-down h-4 w-4 opacity-50"
// select_content_container: "absolute mt-1 w-full rounded-md bg-white shadow-lg z-10"
// select_content_list: "max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm"
// select_group: "text-gray-900"
// select_label: "px-4 py-2 text-sm text-gray-700"
// select_item: "text-gray-900 cursor-pointer select-none relative py-2 pl-3 pr-9 hover:bg-gray-100"
