#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::{BrandGuideType, ClassesStr};
use gloo_console as console;
use std::rc::Rc;
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

    // Handle focus when state changes
    {
        let is_open = is_open.clone();
        let div_ref = div_ref.clone();
        use_effect_with((*is_open, div_ref.clone()), move |(is_open, div_ref)| {
            if *is_open {
                if let Some(element) = div_ref.cast::<web_sys::HtmlElement>() {
                    let _ = element.focus();
                }
            }
            || {}
        });
    }

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
