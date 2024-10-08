#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use std::rc::Rc;
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
}

#[function_component(Select)]
pub fn select<T: Clone + PartialEq + ToString + 'static>(props: &SelectProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let is_open = use_state(|| false);
    let selected = use_state(|| props.selected.clone());

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
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
        .unwrap_or_default();

    html! {
        <div class={&brandguide.select_container} id={props.id.clone()}>
            <button
                type="button"
                class={&brandguide.select_trigger}
                onclick={{
                    let toggle = state.toggle.clone();
                    move |_| toggle.emit(())
                }}
            >
                <span class={&brandguide.select_trigger_placeholder}>{ selected_label }</span>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={&brandguide.select_trigger_icon} aria-hidden="true">
                    <path d="m6 9 6 6 6-6"></path>
                </svg>
            </button>
            if *is_open {
                <div class={&brandguide.select_content_container}>
                    <ul class={&brandguide.select_content_list}>
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
                            html! {
                                <li
                                    class={&brandguide.select_item}
                                    onclick={on_click}
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
