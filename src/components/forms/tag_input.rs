#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use std::cell::RefCell;
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

pub struct TagInputInner {
    tags: Rc<RefCell<Vec<String>>>,
    input_ref: NodeRef,
    container_ref: NodeRef,
    candidate_tags: Rc<RefCell<Vec<String>>>,
}

pub enum Msg {
    AddTag(String),
    RemoveTag(usize),
    UpdateInput(String),
    UpdateCandidates(Vec<String>),
    FocusInput,
}

#[derive(Properties, PartialEq)]
pub struct TagInputInnerProps {
    pub brandguide: Rc<BrandGuideType>,
    pub props: TagInputProps,
}

impl Component for TagInputInner {
    type Message = Msg;
    type Properties = TagInputInnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tags: Rc::new(RefCell::new(ctx.props().props.default_value.clone())),
            input_ref: NodeRef::default(),
            container_ref: NodeRef::default(),
            candidate_tags: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTag(tag) => {
                self.tags.borrow_mut().push(tag);
                if let Some(ref onupdate) = ctx.props().props.onupdate {
                    onupdate.emit(self.tags.borrow().clone());
                }
                self.candidate_tags.borrow_mut().clear();
                #[cfg(not(feature = "ssr"))]
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
                true
            }
            Msg::RemoveTag(index) => {
                self.tags.borrow_mut().remove(index);
                if let Some(ref onupdate) = ctx.props().props.onupdate {
                    onupdate.emit(self.tags.borrow().clone());
                }
                true
            }
            Msg::UpdateInput(value) => {
                if let Some(ref candidates) = ctx.props().props.candidates {
                    let callback = ctx.link().callback(Msg::UpdateCandidates);
                    callback.emit(candidates.emit(value));
                }
                false
            }
            Msg::UpdateCandidates(candidates) => {
                *self.candidate_tags.borrow_mut() = candidates;
                true
            }
            Msg::FocusInput => {
                #[cfg(not(feature = "ssr"))]
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let brandguide = &ctx.props().brandguide;

        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            #[cfg(not(feature = "ssr"))]
            {
                if e.key() == "Enter" {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let value = input.value();
                    if !value.is_empty() {
                        input.set_value("");
                        return Some(Msg::AddTag(value));
                    }
                }
            }
            None
        });

        let oninput = ctx.link().callback(|e: InputEvent| {
            #[cfg(not(feature = "ssr"))]
            {
                let input: HtmlInputElement = e.target_unchecked_into();
                return Msg::UpdateInput(input.value());
            }
            #[cfg(feature = "ssr")]
            Msg::UpdateInput(String::new())
        });

        let onfocus = ctx.link().callback(|_| Msg::FocusInput);

        let candidate_tags = self.candidate_tags.borrow();

        html! {
            <div
                ref={self.container_ref.clone()}
                {onfocus}
                tabindex="0"
                class={classes!(&brandguide.tag_input_container)}
            >
                <div class={classes!(&brandguide.tag_input_tags_container)}>
                    {for self.tags.borrow().iter().enumerate().map(|(index, tag)| {
                        let onclick = ctx.link().callback(move |_| Msg::RemoveTag(index));
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
                    ref={self.input_ref.clone()}
                    id={ctx.props().props.id.clone()}
                    {onkeypress}
                    {oninput}
                    placeholder={ctx.props().props.placeholder.clone()}
                    class={classes!(&brandguide.tag_input_input)}
                />
                <div class={classes!(&brandguide.tag_input_candidates_container)}>
                    {for candidate_tags.iter().map(|candidate| {
                        let candidate_clone = candidate.clone();
                        let onclick = ctx.link().callback(move |_| Msg::AddTag(candidate_clone.clone()));
                        html! {
                            <button onclick={onclick} class={classes!(&brandguide.tag_input_candidate_button)}>{candidate}</button>
                        }
                    })}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        #[cfg(not(feature = "ssr"))]
        if first_render {
            if let Some(container) = self.container_ref.cast::<HtmlElement>() {
                let link = ctx.link().clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                    link.send_message(Msg::FocusInput);
                }) as Box<dyn FnMut(_)>);
                container
                    .add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
            }
        }
    }
}

#[function_component(TagInput)]
pub fn tag_input(props: &TagInputProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = Rc::new(get_brandguide());

    html! {
        <TagInputInner brandguide={brandguide} props={props.clone()} />
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
