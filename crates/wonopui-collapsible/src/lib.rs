//! Collapsible component for wonopui
//!
//! A component that can be expanded or collapsed to show or hide content.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for collapsible styling (shadcn v4).
pub mod classes {
    /// Container styles - add spacing between items
    pub const COLLAPSIBLE_CONTAINER: &str = "space-y-2";
    /// Trigger styles - interactive with focus state
    pub const COLLAPSIBLE_TRIGGER: &str = "cursor-pointer select-none transition-all duration-200 [&[data-state=open]>svg.collapsible-icon]:rotate-180";
    /// Content styles - smooth animation with top padding
    pub const COLLAPSIBLE_CONTENT: &str = "overflow-hidden pt-2 space-y-2 transition-all duration-200 data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down";
    /// Header styles - flexible layout with padding
    pub const COLLAPSIBLE_HEADER: &str = "flex items-center justify-between gap-4 py-2";
    /// Title styles - premium typography
    pub const COLLAPSIBLE_TITLE: &str = "text-sm font-semibold text-zinc-900 dark:text-zinc-50 tracking-tight";
    /// Item styles - premium card-like appearance with margin
    pub const COLLAPSIBLE_ITEM: &str = "rounded-lg border border-zinc-200 bg-white px-4 py-3 text-sm text-zinc-900 shadow-xs dark:border-zinc-800 dark:bg-zinc-950 dark:text-zinc-50";
    /// Chevron icon for trigger
    pub const COLLAPSIBLE_ICON: &str = "collapsible-icon size-4 shrink-0 text-zinc-500 transition-transform duration-200 dark:text-zinc-400";
}

#[derive(Clone, PartialEq)]
pub struct CollapsibleState {
    pub is_open: bool,
    pub toggle: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let is_open = use_state(|| props.open);

    let toggle = {
        let is_open = is_open.clone();
        let on_open_change = props.on_open_change.clone();
        Callback::from(move |_| {
            let new_state = !*is_open;
            is_open.set(new_state);
            on_open_change.emit(new_state);
        })
    };

    let state = Rc::new(CollapsibleState {
        is_open: *is_open,
        toggle,
    });

    let class = merge_classes(&[classes::COLLAPSIBLE_CONTAINER, &props.class.to_string()]);

    html! {
        <ContextProvider<Rc<CollapsibleState>> context={state.clone()}>
            <div data-slot="collapsible" data-state={if *is_open { "open" } else { "closed" }} class={class}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<CollapsibleState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTriggerProps {
    #[prop_or_default]
    pub as_child: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CollapsibleTrigger)]
pub fn collapsible_trigger(props: &CollapsibleTriggerProps) -> Html {
    let state = use_context::<Rc<CollapsibleState>>().expect("CollapsibleState not found");

    let onclick = {
        let toggle = state.toggle.clone();
        Callback::from(move |_: MouseEvent| toggle.emit(()))
    };

    let class = merge_classes(&[classes::COLLAPSIBLE_TRIGGER, &props.class.to_string()]);

    html! {
        <button
            data-slot="collapsible-trigger"
            data-state={if state.is_open { "open" } else { "closed" }}
            onclick={onclick}
            class={class}
            type="button"
            aria-expanded={state.is_open.to_string()}
        >
            { for props.children.iter() }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleContentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleContent)]
pub fn collapsible_content(props: &CollapsibleContentProps) -> Html {
    let state = use_context::<Rc<CollapsibleState>>().expect("CollapsibleState not found");

    if !state.is_open {
        return html! {};
    }

    let class = merge_classes(&[classes::COLLAPSIBLE_CONTENT, &props.class.to_string()]);

    html! {
        <div data-slot="collapsible-content" data-state="open" class={class} role="region">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleHeaderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleHeader)]
pub fn collapsible_header(props: &CollapsibleHeaderProps) -> Html {
    let class = merge_classes(&[classes::COLLAPSIBLE_HEADER, &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTitleProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleTitle)]
pub fn collapsible_title(props: &CollapsibleTitleProps) -> Html {
    let class = merge_classes(&[classes::COLLAPSIBLE_TITLE, &props.class.to_string()]);

    html! {
        <h4 class={class}>
            { for props.children.iter() }
        </h4>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleItemProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleItem)]
pub fn collapsible_item(props: &CollapsibleItemProps) -> Html {
    let class = merge_classes(&[classes::COLLAPSIBLE_ITEM, &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}
