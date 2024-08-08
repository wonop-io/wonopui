use crate::config::BRANDGUIDE;
use std::rc::Rc;
use yew::function_component;
use yew::html;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use gloo_console as console;

#[derive(Clone, PartialEq)]
pub struct PopoverState {
    pub is_open: bool,
    pub toggle: Callback<()>,
    // pub close: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct PopoverProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}
#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let is_open = use_state(|| false);
    let div_ref = use_node_ref();
    let toggle = {
        let is_open = is_open.clone();
        let div_ref = div_ref.clone();
        Callback::from(move |_| {
            let new_value = !*is_open;
            is_open.set(new_value);
            if new_value {
                if let Some(element) = div_ref.cast::<web_sys::HtmlElement>() {
                    element.focus().unwrap();
                }
            }
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
        //  close,
    });

    html! {
        <ContextProvider<Rc<PopoverState>> context={state}>
            <div ref={div_ref} class={classes!(BRANDGUIDE.popover_container, props.class.clone())} tabindex="0" onfocusout={close}>
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
    let state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");

    let onclick = {
        let toggle = state.toggle.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    html! {
        <div class={classes!(BRANDGUIDE.popover_trigger, props.class.clone())} {onclick}>
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
    fn to_class(&self) -> &'static str {
        match self {
            PopoverPosition::NorthStart => "bottom-full left-0 transform translate-x-0",
            PopoverPosition::NorthMiddle => "bottom-full left-1/2 transform -translate-x-1/2",
            PopoverPosition::NorthEnd => "bottom-full right-0 transform translate-x-0",
            PopoverPosition::SouthStart => "top-full left-0 transform translate-x-0",
            PopoverPosition::SouthMiddle => "top-full left-1/2 transform -translate-x-1/2",
            PopoverPosition::SouthEnd => "top-full right-0 transform",
            PopoverPosition::EastStart => "top-0 left-full transform translate-y-0",
            PopoverPosition::EastMiddle => "top-1/2 left-full transform -translate-y-1/2",
            PopoverPosition::EastEnd => "bottom-0 left-full transform",
            PopoverPosition::WestStart => "top-0 right-full transform translate-y-0",
            PopoverPosition::WestMiddle => "top-1/2 right-full transform -translate-y-1/2",
            PopoverPosition::WestEnd => "bottom-0 right-full transform",
        }
    }
}

#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");

    if !state.is_open {
        return html! {};
    }

    let position_class = props.position.to_class();

    html! {
        <div class={classes!(BRANDGUIDE.popover_content, position_class, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

// Proposed new brandguide class for popover_content:
// popover_content: "absolute bg-white border border-gray-300 rounded-md shadow-lg p-4 z-10 dark:bg-zinc-800 dark:border-zinc-700".to_string(),

// New entries in the brand guide:
// popover_container: "relative",
// popover_trigger: "cursor-pointer",
// popover_content: "absolute bg-white border border-gray-300 rounded-md shadow-lg p-4 z-10"
