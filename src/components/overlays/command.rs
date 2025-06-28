#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CommandProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub options: Vec<(T, String, String, Option<Html>)>, // (value, keywords, label, icon)
    #[prop_or_default]
    pub on_select: Callback<T>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Command)]
pub fn command<T: Clone + PartialEq + 'static>(props: &CommandProps<T>) -> Html
where
    T: Clone + PartialEq + 'static,
{
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let is_open = use_state(|| false);
    let value = use_state(|| String::new());
    let filtered_options = use_state(|| props.options.clone());
    let div_ref = use_node_ref();
    let input_ref = use_node_ref();
    let selected_index = use_state(|| 0);

    // Auto-focus logic with use_effect_with
    {
        let is_open = is_open.clone();
        let input_ref = input_ref.clone();
        use_effect_with(
            (*is_open, input_ref.clone()),
            move |(is_open, input_ref)| {
                if *is_open {
                    if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                        let _ = input.focus();
                    }
                }
                || {}
            },
        );
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
        Callback::from(move |_| {
            let new_value = !*is_open;
            is_open.set(new_value);
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
            let new_filtered = options
                .iter()
                .filter(|(_, keywords, label, _)| {
                    keywords.to_lowercase().contains(&new_value)
                        || label.to_lowercase().contains(&new_value)
                })
                .cloned()
                .collect::<Vec<_>>();
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
                if *is_open {
                    selected_index.set((*selected_index + 1) % filtered_options.len());
                    e.prevent_default();
                }
            }
            "ArrowUp" => {
                if *is_open {
                    selected_index.set(
                        (*selected_index as usize + filtered_options.len() - 1)
                            % filtered_options.len(),
                    );
                    e.prevent_default();
                }
            }
            _ => {}
        })
    };

    html! {
        <div ref={div_ref} class={classes!(&brandguide.command_container, props.class.clone())} tabindex="0" onfocusout={close}>
            <div class={classes!(&brandguide.command_input_wrapper)}>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes!(&brandguide.command_icon)}>
                    <circle cx="11" cy="11" r="8"></circle>
                    <path d="m21 21-4.3-4.3"></path>
                </svg>
                <input
                    ref={input_ref}
                    class={classes!(&brandguide.command_input)}
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
                        <div class={classes!(&brandguide.command_list)} role="listbox">
                            {
                                if filtered_options.is_empty() {
                                    html! {
                                        <div class={classes!(&brandguide.command_item)}>
                                            { "No results available" }
                                        </div>
                                    }
                                } else {
                                    filtered_options.iter().enumerate().map(|(index, (val, _, label, icon))| {
                                        let on_select = on_select.clone();
                                        let val = val.clone();
                                        let is_selected = index == *selected_index;
                                        html! {
                                            <div
                                                class={classes!(if is_selected { &brandguide.command_selected_item } else { &brandguide.command_item })}
                                                onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                                role="option"
                                                aria-selected={is_selected.to_string()}
                                            >
                                                if let Some(icon) = icon {
                                                    <span class={classes!(&brandguide.command_item_icon)}>
                                                        {icon.clone()}
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
