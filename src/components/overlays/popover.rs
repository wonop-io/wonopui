#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::{BrandGuideType, ClassesStr};
#[cfg(not(feature = "ssr"))]
use gloo_console as console;
use std::rc::Rc;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
use yew::function_component;
use yew::html;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PopoverState {
    pub is_open: bool,
    pub toggle: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct PopoverProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let is_open = use_state(|| false);
    let div_ref = use_node_ref();
    let toggle = {
        let is_open = is_open.clone();
        let div_ref = div_ref.clone();
        Callback::from(move |_| {
            let new_value = !*is_open;
            is_open.set(new_value);
            #[cfg(not(feature = "ssr"))]
            if new_value {
                if let Some(element) = div_ref.cast::<web_sys::HtmlElement>() {
                    element.focus().unwrap();
                }
            }
        })
    };

    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    let close = Callback::from(|_| {});

    let state = Rc::new(PopoverState {
        is_open: *is_open,
        toggle,
    });

    html! {
        <ContextProvider<Rc<PopoverState>> context={state}>
            <div ref={div_ref} class={classes!(&brandguide.popover_container, props.class.clone())} tabindex="0" onfocusout={close}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<PopoverState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct PopoverTriggerProps {
    pub children: Children,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(PopoverTrigger)]
pub fn popover_trigger(props: &PopoverTriggerProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");

    let onclick = {
        let toggle = state.toggle.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    html! {
        <div class={classes!(&brandguide.popover_trigger, props.class.clone())} {onclick}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PopoverContentProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(PopoverPosition::SouthMiddle)]
    pub position: PopoverPosition,
}

#[derive(Clone, PartialEq)]
pub enum PopoverPosition {
    NorthStart,
    NorthMiddle,
    NorthEnd,
    SouthStart,
    SouthMiddle,
    SouthEnd,
    EastStart,
    EastMiddle,
    EastEnd,
    WestStart,
    WestMiddle,
    WestEnd,
}

impl PopoverPosition {
    fn to_class<'a>(&'a self, brandguide: &'a BrandGuideType) -> &ClassesStr {
        match self {
            PopoverPosition::NorthStart => &brandguide.popover_position_north_start,
            PopoverPosition::NorthMiddle => &brandguide.popover_position_north_middle,
            PopoverPosition::NorthEnd => &brandguide.popover_position_north_end,
            PopoverPosition::SouthStart => &brandguide.popover_position_south_start,
            PopoverPosition::SouthMiddle => &brandguide.popover_position_south_middle,
            PopoverPosition::SouthEnd => &brandguide.popover_position_south_end,
            PopoverPosition::EastStart => &brandguide.popover_position_east_start,
            PopoverPosition::EastMiddle => &brandguide.popover_position_east_middle,
            PopoverPosition::EastEnd => &brandguide.popover_position_east_end,
            PopoverPosition::WestStart => &brandguide.popover_position_west_start,
            PopoverPosition::WestMiddle => &brandguide.popover_position_west_middle,
            PopoverPosition::WestEnd => &brandguide.popover_position_west_end,
        }
    }
}

#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");

    if !state.is_open {
        return html! {};
    }

    let position_class = props.position.to_class(&brandguide);

    html! {
        <div class={classes!(&brandguide.popover_content, position_class, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

// Snippets to update brandguide:
// ("popover_container".to_string(), "relative".to_string()),
// ("popover_trigger".to_string(), "cursor-pointer".to_string()),
// ("popover_content".to_string(), "absolute bg-white border border-gray-300 rounded-md shadow-lg p-4 z-10 dark:bg-zinc-800 dark:border-zinc-700".to_string()),
// ("popover_position_north_start".to_string(), "bottom-full left-0 transform translate-x-0".to_string()),
// ("popover_position_north_middle".to_string(), "bottom-full left-1/2 transform -translate-x-1/2".to_string()),
// ("popover_position_north_end".to_string(), "bottom-full right-0 transform translate-x-0".to_string()),
// ("popover_position_south_start".to_string(), "top-full left-0 transform translate-x-0".to_string()),
// ("popover_position_south_middle".to_string(), "top-full left-1/2 transform -translate-x-1/2".to_string()),
// ("popover_position_south_end".to_string(), "top-full right-0 transform".to_string()),
// ("popover_position_east_start".to_string(), "top-0 left-full transform translate-y-0".to_string()),
// ("popover_position_east_middle".to_string(), "top-1/2 left-full transform -translate-y-1/2".to_string()),
// ("popover_position_east_end".to_string(), "bottom-0 left-full transform".to_string()),
// ("popover_position_west_start".to_string(), "top-0 right-full transform translate-y-0".to_string()),
// ("popover_position_west_middle".to_string(), "top-1/2 right-full transform -translate-y-1/2".to_string()),
// ("popover_position_west_end".to_string(), "bottom-0 right-full transform".to_string()),
//
// pub popover_container: ClassesContainer<T>,
// pub popover_trigger: ClassesContainer<T>,
// pub popover_content: ClassesContainer<T>,
// pub popover_position_north_start: ClassesContainer<T>,
// pub popover_position_north_middle: ClassesContainer<T>,
// pub popover_position_north_end: ClassesContainer<T>,
// pub popover_position_south_start: ClassesContainer<T>,
// pub popover_position_south_middle: ClassesContainer<T>,
// pub popover_position_south_end: ClassesContainer<T>,
// pub popover_position_east_start: ClassesContainer<T>,
// pub popover_position_east_middle: ClassesContainer<T>,
// pub popover_position_east_end: ClassesContainer<T>,
// pub popover_position_west_start: ClassesContainer<T>,
// pub popover_position_west_middle: ClassesContainer<T>,
// pub popover_position_west_end: ClassesContainer<T>,
//
// popover_container: self.popover_container.to_owned(),
// popover_trigger: self.popover_trigger.to_owned(),
// popover_content: self.popover_content.to_owned(),
// popover_position_north_start: self.popover_position_north_start.to_owned(),
// popover_position_north_middle: self.popover_position_north_middle.to_owned(),
// popover_position_north_end: self.popover_position_north_end.to_owned(),
// popover_position_south_start: self.popover_position_south_start.to_owned(),
// popover_position_south_middle: self.popover_position_south_middle.to_owned(),
// popover_position_south_end: self.popover_position_south_end.to_owned(),
// popover_position_east_start: self.popover_position_east_start.to_owned(),
// popover_position_east_middle: self.popover_position_east_middle.to_owned(),
// popover_position_east_end: self.popover_position_east_end.to_owned(),
// popover_position_west_start: self.popover_position_west_start.to_owned(),
// popover_position_west_middle: self.popover_position_west_middle.to_owned(),
// popover_position_west_end: self.popover_position_west_end.to_owned(),
//
// popover_container: default_config_hm
// .get("popover_container")
// .expect("Template parameter missing")
// .clone(),
// popover_trigger: default_config_hm
// .get("popover_trigger")
// .expect("Template parameter missing")
// .clone(),
// popover_content: default_config_hm
// .get("popover_content")
// .expect("Template parameter missing")
// .clone(),
// popover_position_north_start: default_config_hm
// .get("popover_position_north_start")
// .expect("Template parameter missing")
// .clone(),
// popover_position_north_middle: default_config_hm
// .get("popover_position_north_middle")
// .expect("Template parameter missing")
// .clone(),
// popover_position_north_end: default_config_hm
// .get("popover_position_north_end")
// .expect("Template parameter missing")
// .clone(),
// popover_position_south_start: default_config_hm
// .get("popover_position_south_start")
// .expect("Template parameter missing")
// .clone(),
// popover_position_south_middle: default_config_hm
// .get("popover_position_south_middle")
// .expect("Template parameter missing")
// .clone(),
// popover_position_south_end: default_config_hm
// .get("popover_position_south_end")
// .expect("Template parameter missing")
// .clone(),
// popover_position_east_start: default_config_hm
// .get("popover_position_east_start")
// .expect("Template parameter missing")
// .clone(),
// popover_position_east_middle: default_config_hm
// .get("popover_position_east_middle")
// .expect("Template parameter missing")
// .clone(),
// popover_position_east_end: default_config_hm
// .get("popover_position_east_end")
// .expect("Template parameter missing")
// .clone(),
// popover_position_west_start: default_config_hm
// .get("popover_position_west_start")
// .expect("Template parameter missing")
// .clone(),
// popover_position_west_middle: default_config_hm
// .get("popover_position_west_middle")
// .expect("Template parameter missing")
// .clone(),
// popover_position_west_end: default_config_hm
// .get("popover_position_west_end")
// .expect("Template parameter missing")
// .clone(),
