//! Popover component for wonopui
//!
//! A floating content panel that appears next to a trigger element.

use std::rc::Rc;
use wasm_bindgen::JsCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for popover styling (shadcn v4 style).
pub mod classes {
    /// Container styles.
    pub const POPOVER_CONTAINER: &str = "relative inline-block";
    
    /// Trigger styles.
    pub const POPOVER_TRIGGER: &str = "cursor-pointer";
    
    /// Content styles - shadcn v4 popover content with animations.
    /// Uses z-[9999] to ensure it appears above sidebars and other fixed elements.
    pub const POPOVER_CONTENT: &str = "absolute z-[9999] w-72 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 p-4 shadow-xl outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";

    /// Header styles - shadcn v4 popover header.
    pub const POPOVER_HEADER: &str = "flex flex-col gap-1 text-sm";
    
    /// Title styles - shadcn v4 popover title.
    pub const POPOVER_TITLE: &str = "font-medium text-zinc-950 dark:text-zinc-50";
    
    /// Description styles - shadcn v4 popover description.
    pub const POPOVER_DESCRIPTION: &str = "text-sm text-zinc-500 dark:text-zinc-400";

    // Position classes with slide animations
    pub const POSITION_NORTH_START: &str = "bottom-full left-0 mb-2 data-[state=open]:slide-in-from-bottom-2";
    pub const POSITION_NORTH_MIDDLE: &str = "bottom-full left-1/2 -translate-x-1/2 mb-2 data-[state=open]:slide-in-from-bottom-2";
    pub const POSITION_NORTH_END: &str = "bottom-full right-0 mb-2 data-[state=open]:slide-in-from-bottom-2";
    pub const POSITION_SOUTH_START: &str = "top-full left-0 mt-2 data-[state=open]:slide-in-from-top-2";
    pub const POSITION_SOUTH_MIDDLE: &str = "top-full left-1/2 -translate-x-1/2 mt-2 data-[state=open]:slide-in-from-top-2";
    pub const POSITION_SOUTH_END: &str = "top-full right-0 mt-2 data-[state=open]:slide-in-from-top-2";
    pub const POSITION_EAST_START: &str = "left-full top-0 ml-2 data-[state=open]:slide-in-from-left-2";
    pub const POSITION_EAST_MIDDLE: &str = "left-full top-1/2 -translate-y-1/2 ml-2 data-[state=open]:slide-in-from-left-2";
    pub const POSITION_EAST_END: &str = "left-full bottom-0 ml-2 data-[state=open]:slide-in-from-left-2";
    pub const POSITION_WEST_START: &str = "right-full top-0 mr-2 data-[state=open]:slide-in-from-right-2";
    pub const POSITION_WEST_MIDDLE: &str = "right-full top-1/2 -translate-y-1/2 mr-2 data-[state=open]:slide-in-from-right-2";
    pub const POSITION_WEST_END: &str = "right-full bottom-0 mr-2 data-[state=open]:slide-in-from-right-2";
}

#[derive(Clone, PartialEq)]
pub struct PopoverState {
    pub is_open: bool,
    pub toggle: Callback<()>,
}

#[derive(Clone, PartialEq, Default)]
pub enum PopoverPosition {
    NorthStart,
    NorthMiddle,
    NorthEnd,
    SouthStart,
    #[default]
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
    pub fn to_class(&self) -> &'static str {
        match self {
            PopoverPosition::NorthStart => classes::POSITION_NORTH_START,
            PopoverPosition::NorthMiddle => classes::POSITION_NORTH_MIDDLE,
            PopoverPosition::NorthEnd => classes::POSITION_NORTH_END,
            PopoverPosition::SouthStart => classes::POSITION_SOUTH_START,
            PopoverPosition::SouthMiddle => classes::POSITION_SOUTH_MIDDLE,
            PopoverPosition::SouthEnd => classes::POSITION_SOUTH_END,
            PopoverPosition::EastStart => classes::POSITION_EAST_START,
            PopoverPosition::EastMiddle => classes::POSITION_EAST_MIDDLE,
            PopoverPosition::EastEnd => classes::POSITION_EAST_END,
            PopoverPosition::WestStart => classes::POSITION_WEST_START,
            PopoverPosition::WestMiddle => classes::POSITION_WEST_MIDDLE,
            PopoverPosition::WestEnd => classes::POSITION_WEST_END,
        }
    }
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

    let class = merge_classes(&[classes::POPOVER_CONTAINER, &props.class.to_string()]);

    html! {
        <ContextProvider<Rc<PopoverState>> context={state}>
            <div data-slot="popover" ref={div_ref} class={class} tabindex="0" onfocusout={close}>
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

    let class = merge_classes(&[classes::POPOVER_TRIGGER, &props.class.to_string()]);

    html! {
        <div data-slot="popover-trigger" class={class} {onclick}>
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

#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let state = use_context::<Rc<PopoverState>>().expect("no context found for PopoverState");

    if !state.is_open {
        return html! {};
    }

    let position_class = props.position.to_class();

    let class = merge_classes(&[
        classes::POPOVER_CONTENT,
        position_class,
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="popover-content" data-state="open" class={class}>
            { for props.children.iter() }
        </div>
    }
}
