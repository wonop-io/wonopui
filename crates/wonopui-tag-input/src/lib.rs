//! TagInput component for wonopui
//!
//! An input field for entering multiple tags/labels.

use web_sys::HtmlInputElement;
use yew::TargetCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const TAG_INPUT_CONTAINER: &str = "flex flex-wrap gap-2 p-2 bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 rounded-md focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500";
    pub const TAG_INPUT_TAGS_CONTAINER: &str = "flex flex-wrap gap-2";
    pub const TAG_INPUT_TAG: &str = "inline-flex items-center gap-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-sm font-medium px-2.5 py-0.5 rounded";
    pub const TAG_INPUT_REMOVE_BUTTON: &str = "text-blue-600 dark:text-blue-300 hover:text-blue-800 dark:hover:text-blue-100 cursor-pointer";
    pub const TAG_INPUT_INPUT: &str = "flex-1 min-w-[120px] bg-transparent outline-none text-sm text-gray-900 dark:text-zinc-100 placeholder-gray-400 dark:placeholder-zinc-500";
    pub const TAG_INPUT_CANDIDATES_CONTAINER: &str = "flex flex-wrap gap-2 mt-2";
    pub const TAG_INPUT_CANDIDATE_BUTTON: &str = "text-white bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 font-medium rounded-md text-sm px-3 py-1.5";
}

#[derive(Properties, PartialEq, Clone)]
pub struct TagInputProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub default_value: Vec<String>,
    #[prop_or_default]
    pub candidates: Option<Callback<String, Vec<String>>>,
    #[prop_or_default]
    pub onchange: Callback<Vec<String>>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub max_tags: Option<usize>,
}

#[function_component(TagInput)]
pub fn tag_input(props: &TagInputProps) -> Html {
    let tags = use_state(|| props.default_value.clone());
    let candidate_tags = use_state(Vec::<String>::new);
    let input_ref = use_node_ref();

    let add_tag = {
        let tags = tags.clone();
        let candidate_tags = candidate_tags.clone();
        let onchange = props.onchange.clone();
        let max_tags = props.max_tags;
        let input_ref = input_ref.clone();

        Callback::from(move |tag: String| {
            let mut new_tags = (*tags).clone();
            
            // Check if we've reached max tags
            if let Some(max) = max_tags {
                if new_tags.len() >= max {
                    return;
                }
            }
            
            // Don't add duplicates
            if !new_tags.contains(&tag) {
                new_tags.push(tag);
                tags.set(new_tags.clone());
                candidate_tags.set(Vec::new());
                onchange.emit(new_tags);
            }
            
            // Clear the input
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.set_value("");
            }
        })
    };

    let remove_tag = {
        let tags = tags.clone();
        let onchange = props.onchange.clone();

        Callback::from(move |index: usize| {
            let mut new_tags = (*tags).clone();
            new_tags.remove(index);
            tags.set(new_tags.clone());
            onchange.emit(new_tags);
        })
    };

    let update_candidates = {
        let candidate_tags = candidate_tags.clone();
        let props_candidates = props.candidates.clone();

        Callback::from(move |value: String| {
            if let Some(ref candidates) = props_candidates {
                if value.is_empty() {
                    candidate_tags.set(Vec::new());
                } else {
                    let candidate_results = candidates.emit(value);
                    candidate_tags.set(candidate_results);
                }
            }
        })
    };

    let onkeydown = {
        let add_tag = add_tag.clone();
        let tags = tags.clone();
        let remove_tag = remove_tag.clone();

        Callback::from(move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            if e.key() == "Enter" {
                e.prevent_default();
                if !value.is_empty() {
                    add_tag.emit(value);
                }
            } else if e.key() == "Backspace" && value.is_empty() && !tags.is_empty() {
                // Remove last tag when backspace on empty input
                remove_tag.emit(tags.len() - 1);
            }
        })
    };

    let oninput = {
        let update_candidates = update_candidates.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            update_candidates.emit(input.value());
        })
    };

    let onfocus_container = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        })
    };

    let container_class = merge_classes(&[
        classes::TAG_INPUT_CONTAINER,
        if props.disabled { "opacity-50 cursor-not-allowed" } else { "cursor-text" },
        &props.class.to_string(),
    ]);

    html! {
        <div class="flex flex-col gap-2">
            <div
                tabindex="0"
                onclick={onfocus_container}
                class={container_class}
            >
                { for (*tags).iter().enumerate().map(|(index, tag)| {
                    let onclick = {
                        let remove_tag = remove_tag.clone();
                        Callback::from(move |_| remove_tag.emit(index))
                    };
                    html! {
                        <span class={classes::TAG_INPUT_TAG}>
                            { tag }
                            if !props.disabled {
                                <button 
                                    onclick={onclick} 
                                    class={classes::TAG_INPUT_REMOVE_BUTTON}
                                    type="button"
                                >
                                    {"×"}
                                </button>
                            }
                        </span>
                    }
                })}
                <input
                    type="text"
                    ref={input_ref}
                    id={props.id.clone()}
                    onkeydown={onkeydown}
                    oninput={oninput}
                    placeholder={if tags.is_empty() { props.placeholder.clone() } else { String::new() }}
                    class={classes::TAG_INPUT_INPUT}
                    disabled={props.disabled}
                />
            </div>
            if !candidate_tags.is_empty() {
                <div class={classes::TAG_INPUT_CANDIDATES_CONTAINER}>
                    { for (*candidate_tags).iter().map(|candidate| {
                        let candidate_clone = candidate.clone();
                        let add_tag = add_tag.clone();
                        let onclick = Callback::from(move |_| add_tag.emit(candidate_clone.clone()));
                        html! {
                            <button 
                                onclick={onclick} 
                                class={classes::TAG_INPUT_CANDIDATE_BUTTON}
                                type="button"
                            >
                                { candidate }
                            </button>
                        }
                    })}
                </div>
            }
        </div>
    }
}
