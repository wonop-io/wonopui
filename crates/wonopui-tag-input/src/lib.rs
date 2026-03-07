//! TagInput component for wonopui
//!
//! An input field for entering multiple tags/labels.

use web_sys::HtmlInputElement;
use wonopui_core::merge_classes;
use yew::prelude::*;
use yew::TargetCast;

/// CSS classes for the TagInput component (shadcn v4)
pub mod classes {
    /// Container - premium input field appearance
    pub const TAG_INPUT_CONTAINER: &str = "flex flex-wrap items-center gap-1.5 min-h-9 w-full rounded-md border border-zinc-200 bg-transparent px-2.5 py-1.5 text-sm shadow-xs transition-all duration-200 focus-within:border-zinc-950 focus-within:ring-zinc-950/50 focus-within:ring-[3px] focus-within:outline-none dark:border-zinc-800 dark:focus-within:border-zinc-300 dark:focus-within:ring-zinc-300/50";
    /// Tags wrapper
    pub const TAG_INPUT_TAGS_CONTAINER: &str = "flex flex-wrap gap-1.5";
    /// Individual tag - badge-like styling
    pub const TAG_INPUT_TAG: &str = "inline-flex items-center gap-1 rounded-md border border-zinc-200 bg-zinc-100 px-2 py-0.5 text-xs font-medium text-zinc-900 transition-colors dark:border-zinc-700 dark:bg-zinc-800 dark:text-zinc-50";
    /// Remove button on tag
    pub const TAG_INPUT_REMOVE_BUTTON: &str = "ml-0.5 rounded-sm text-zinc-500 transition-colors hover:text-zinc-900 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-950 dark:text-zinc-400 dark:hover:text-zinc-50 dark:focus-visible:ring-zinc-300";
    /// Input field within container
    pub const TAG_INPUT_INPUT: &str = "flex-1 min-w-[80px] bg-transparent outline-none text-sm text-zinc-900 placeholder:text-zinc-500 dark:text-zinc-50 dark:placeholder:text-zinc-400";
    /// Candidates container
    pub const TAG_INPUT_CANDIDATES_CONTAINER: &str = "flex flex-wrap gap-1.5 mt-2";
    /// Candidate suggestion button
    pub const TAG_INPUT_CANDIDATE_BUTTON: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 border border-zinc-200 bg-white shadow-xs hover:bg-zinc-50 hover:text-zinc-900 dark:border-zinc-800 dark:bg-zinc-950 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 h-7 px-2.5 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none text-zinc-700 dark:text-zinc-300";
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
        if props.disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-text"
        },
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
