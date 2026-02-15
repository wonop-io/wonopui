//! Mention Input component for WonopUI.
//!
//! A text input that supports @mentions with autocomplete suggestions.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the MentionInput component
pub mod classes {
    pub const CONTAINER: &str = "relative";
    pub const INPUT: &str = "w-full px-3 py-2 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-ring";
    pub const SUGGESTIONS: &str = "absolute z-50 mt-1 w-full bg-background border rounded-md shadow-lg max-h-48 overflow-auto";
    pub const SUGGESTION: &str = "px-3 py-2 cursor-pointer hover:bg-accent text-sm";
    pub const SUGGESTION_SELECTED: &str = "px-3 py-2 cursor-pointer bg-accent text-sm";
    pub const TAG: &str = "inline-flex items-center px-2 py-0.5 rounded-md text-sm font-medium bg-primary/10 text-primary";
}

#[derive(Properties, PartialEq)]
pub struct MentionInputProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    /// Callback to get mention candidates. Takes the search query, returns matching items.
    #[prop_or_default]
    pub get_candidates: Option<Callback<String, Vec<String>>>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
    /// The trigger character for mentions
    #[prop_or("@".to_string())]
    pub trigger: String,
}

#[function_component(MentionInput)]
pub fn mention_input(props: &MentionInputProps) -> Html {
    let input_value = use_state(|| props.value.clone());
    let is_showing_suggestions = use_state(|| false);
    let suggestions = use_state(Vec::<String>::new);
    let selected_index = use_state(|| 0usize);
    let search_query = use_state(String::new);

    // Sync with prop changes
    {
        let input_value = input_value.clone();
        let prop_value = props.value.clone();
        use_effect_with(prop_value, move |value| {
            input_value.set(value.clone());
            || ()
        });
    }

    let oninput = {
        let input_value = input_value.clone();
        let onchange = props.onchange.clone();
        let is_showing_suggestions = is_showing_suggestions.clone();
        let suggestions = suggestions.clone();
        let search_query = search_query.clone();
        let get_candidates = props.get_candidates.clone();
        let trigger = props.trigger.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let value = input.value();
                input_value.set(value.clone());
                onchange.emit(value.clone());

                // Check for trigger character
                if let Some(trigger_pos) = value.rfind(&trigger) {
                    let query = &value[trigger_pos + trigger.len()..];
                    if !query.contains(' ') && !query.is_empty() {
                        search_query.set(query.to_string());
                        if let Some(get_candidates) = &get_candidates {
                            let candidates = get_candidates.emit(query.to_string());
                            suggestions.set(candidates);
                            is_showing_suggestions.set(true);
                        }
                    } else {
                        is_showing_suggestions.set(false);
                    }
                } else {
                    is_showing_suggestions.set(false);
                }
            }
        })
    };

    let onkeydown = {
        let is_showing_suggestions = is_showing_suggestions.clone();
        let suggestions = suggestions.clone();
        let selected_index = selected_index.clone();
        let input_value = input_value.clone();
        let onchange = props.onchange.clone();
        let _search_query = search_query.clone(); // TODO: Use for filtering
        let trigger = props.trigger.clone();

        Callback::from(move |e: KeyboardEvent| {
            if *is_showing_suggestions {
                match e.key().as_str() {
                    "ArrowDown" => {
                        e.prevent_default();
                        if suggestions.len() > 0 {
                            selected_index.set((*selected_index + 1) % suggestions.len());
                        }
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        if suggestions.len() > 0 {
                            selected_index
                                .set((*selected_index + suggestions.len() - 1) % suggestions.len());
                        }
                    }
                    "Enter" | "Tab" => {
                        e.prevent_default();
                        if let Some(selected) = suggestions.get(*selected_index) {
                            // Replace the mention query with the selected item
                            let value = (*input_value).clone();
                            if let Some(trigger_pos) = value.rfind(&trigger) {
                                let new_value = format!(
                                    "{}{} ",
                                    &value[..trigger_pos + trigger.len()],
                                    selected
                                );
                                input_value.set(new_value.clone());
                                onchange.emit(new_value);
                            }
                            is_showing_suggestions.set(false);
                        }
                    }
                    "Escape" => {
                        is_showing_suggestions.set(false);
                    }
                    _ => {}
                }
            }
        })
    };

    let on_suggestion_click = {
        let is_showing_suggestions = is_showing_suggestions.clone();
        let input_value = input_value.clone();
        let onchange = props.onchange.clone();
        let trigger = props.trigger.clone();

        move |suggestion: String| {
            let is_showing_suggestions = is_showing_suggestions.clone();
            let input_value = input_value.clone();
            let onchange = onchange.clone();
            let trigger = trigger.clone();

            Callback::from(move |_: MouseEvent| {
                let value = (*input_value).clone();
                if let Some(trigger_pos) = value.rfind(&trigger) {
                    let new_value =
                        format!("{}{} ", &value[..trigger_pos + trigger.len()], suggestion);
                    input_value.set(new_value.clone());
                    onchange.emit(new_value);
                }
                is_showing_suggestions.set(false);
            })
        }
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            <input
                type="text"
                id={props.id.clone()}
                class={classes::INPUT}
                value={(*input_value).clone()}
                oninput={oninput}
                onkeydown={onkeydown}
                placeholder={props.placeholder.clone()}
            />

            if *is_showing_suggestions && !suggestions.is_empty() {
                <div class={classes::SUGGESTIONS}>
                    { for suggestions.iter().enumerate().map(|(idx, suggestion)| {
                        let suggestion_class = if idx == *selected_index {
                            classes::SUGGESTION_SELECTED
                        } else {
                            classes::SUGGESTION
                        };
                        let onclick = on_suggestion_click(suggestion.clone());
                        html! {
                            <div class={suggestion_class} onclick={onclick}>
                                { suggestion }
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
