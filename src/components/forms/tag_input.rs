#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use std::rc::Rc;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::closure::Closure;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TagInputProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub default_value: Vec<String>,
    #[prop_or_default]
    pub candidates: Option<Callback<String, Vec<String>>>,
    #[prop_or_default]
    pub onupdate: Option<Callback<Vec<String>>>,
    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(TagInput)]
pub fn tag_input(props: &TagInputProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = Rc::new(get_brandguide());

    let tags = use_state(|| props.default_value.clone());
    let candidate_tags = use_state(Vec::<String>::new);
    let input_ref = use_node_ref();
    let container_ref = use_node_ref();

    // Effect for setting up focus event listener
    {
        let container_ref = container_ref.clone();
        let input_ref = input_ref.clone();
        use_effect_with((), move |_| {
            let mut cleanup_needed = false;
            let mut event_closure = None;

            if let Some(container) = container_ref.cast::<HtmlElement>() {
                let input_ref = input_ref.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        let _ = input.focus();
                    }
                }) as Box<dyn FnMut(_)>);

                container
                    .add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())
                    .unwrap();

                cleanup_needed = true;
                event_closure = Some(closure);
            }

            move || {
                if cleanup_needed {
                    if let Some(closure) = event_closure {
                        drop(closure);
                    }
                }
            }
        });
    }

    let add_tag = {
        let tags = tags.clone();
        let candidate_tags = candidate_tags.clone();
        let props_onupdate = props.onupdate.clone();

        Callback::from(move |tag: String| {
            let mut new_tags = (*tags).clone();
            new_tags.push(tag);
            tags.set(new_tags.clone());
            candidate_tags.set(Vec::new());

            if let Some(ref onupdate) = props_onupdate {
                onupdate.emit(new_tags);
            }
        })
    };

    let remove_tag = {
        let tags = tags.clone();
        let props_onupdate = props.onupdate.clone();

        Callback::from(move |index: usize| {
            let mut new_tags = (*tags).clone();
            new_tags.remove(index);
            tags.set(new_tags.clone());

            if let Some(ref onupdate) = props_onupdate {
                onupdate.emit(new_tags);
            }
        })
    };

    let update_input = {
        let candidate_tags = candidate_tags.clone();
        let props_candidates = props.candidates.clone();

        Callback::from(move |value: String| {
            if let Some(ref candidates) = props_candidates {
                // Only show candidates if the input is not empty
                if value.is_empty() {
                    candidate_tags.set(Vec::new());
                } else {
                    let candidate_results = candidates.emit(value);
                    candidate_tags.set(candidate_results);
                }
            }
        })
    };

    let onkeypress = {
        let add_tag = add_tag.clone();

        Callback::from(move |e: KeyboardEvent| {
            #[cfg(not(feature = "ssr"))]
            {
                if e.key() == "Enter" {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let value = input.value();
                    if !value.is_empty() {
                        input.set_value("");
                        add_tag.emit(value);
                    }
                }
            }
        })
    };

    let oninput = {
        let update_input = update_input.clone();

        Callback::from(move |e: InputEvent| {
            #[cfg(not(feature = "ssr"))]
            {
                let input: HtmlInputElement = e.target_unchecked_into();
                update_input.emit(input.value());
            }
            #[cfg(feature = "ssr")]
            update_input.emit(String::new());
        })
    };

    html! {
        <div
            ref={container_ref}
            tabindex="0"
            class={classes!(&brandguide.tag_input_container)}
        >
            <div class={classes!(&brandguide.tag_input_tags_container)}>
                {for (*tags).iter().enumerate().map(|(index, tag)| {
                    let onclick = {
                        let remove_tag = remove_tag.clone();
                        let index = index;
                        Callback::from(move |_| remove_tag.emit(index))
                    };
                    html! {
                        <span class={classes!(&brandguide.tag_input_tag)}>
                            {tag}
                            <button onclick={onclick} class={classes!(&brandguide.tag_input_remove_button)}>{"×"}</button>
                        </span>
                    }
                })}
            </div>
            <input
                type="text"
                ref={input_ref}
                id={props.id.clone()}
                onkeypress={onkeypress}
                oninput={oninput}
                placeholder={props.placeholder.clone()}
                class={classes!(&brandguide.tag_input_input)}
            />
            <div class={classes!(&brandguide.tag_input_candidates_container)}>
                {for (*candidate_tags).iter().map(|candidate| {
                    let candidate_clone = candidate.clone();
                    let add_tag = add_tag.clone();
                    let onclick = Callback::from(move |_| add_tag.emit(candidate_clone.clone()));
                    html! {
                        <button onclick={onclick} class={classes!(&brandguide.tag_input_candidate_button)}>{candidate}</button>
                    }
                })}
            </div>
        </div>
    }
}

// Snippets to update brandguide:
// ("tag_input_container".to_string(), "cursor-text flex flex-col space-y-1 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500".to_string()),
// ("tag_input_tags_container".to_string(), "flex flex-wrap gap-2".to_string()),
// ("tag_input_tag".to_string(), "bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300".to_string()),
// ("tag_input_remove_button".to_string(), "ml-1 text-blue-600 rounded-full hover:text-blue-800 dark:text-blue-300 dark:hover:text-blue-100 cursor-pointer".to_string()),
// ("tag_input_input".to_string(), "bg-transparent outline-none focus:outline-none flex-grow".to_string()),
// ("tag_input_candidates_container".to_string(), "flex flex-wrap gap-2".to_string()),
// ("tag_input_candidate_button".to_string(), "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800".to_string()),
//
// pub tag_input_container: ClassesContainer<T>,
// pub tag_input_tags_container: ClassesContainer<T>,
// pub tag_input_tag: ClassesContainer<T>,
// pub tag_input_remove_button: ClassesContainer<T>,
// pub tag_input_input: ClassesContainer<T>,
// pub tag_input_candidates_container: ClassesContainer<T>,
// pub tag_input_candidate_button: ClassesContainer<T>,
//
// tag_input_container: self.tag_input_container.to_owned(),
// tag_input_tags_container: self.tag_input_tags_container.to_owned(),
// tag_input_tag: self.tag_input_tag.to_owned(),
// tag_input_remove_button: self.tag_input_remove_button.to_owned(),
// tag_input_input: self.tag_input_input.to_owned(),
// tag_input_candidates_container: self.tag_input_candidates_container.to_owned(),
// tag_input_candidate_button: self.tag_input_candidate_button.to_owned(),
//
// tag_input_container: default_config_hm
// .get("tag_input_container")
// .expect("Template parameter missing")
// .clone(),
// tag_input_tags_container: default_config_hm
// .get("tag_input_tags_container")
// .expect("Template parameter missing")
// .clone(),
// tag_input_tag: default_config_hm
// .get("tag_input_tag")
// .expect("Template parameter missing")
// .clone(),
// tag_input_remove_button: default_config_hm
// .get("tag_input_remove_button")
// .expect("Template parameter missing")
// .clone(),
// tag_input_input: default_config_hm
// .get("tag_input_input")
// .expect("Template parameter missing")
// .clone(),
// tag_input_candidates_container: default_config_hm
// .get("tag_input_candidates_container")
// .expect("Template parameter missing")
// .clone(),
// tag_input_candidate_button: default_config_hm
// .get("tag_input_candidate_button")
// .expect("Template parameter missing")
// .clone(),
