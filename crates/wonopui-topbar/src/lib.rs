//! Topbar component for wonopui
//!
//! A sticky or fixed header bar for navigation and actions.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const TOPBAR_BASE: &str = "shrink-0 top-0 z-10 flex h-16 border-b border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900";
    pub const TOPBAR_STICKY: &str = "sticky";
    pub const TOPBAR_RELATIVE: &str = "relative";
    pub const TOPBAR_ABSOLUTE: &str = "absolute w-full";
    pub const TOPBAR_FIXED: &str = "fixed w-full";
}

#[derive(Clone, PartialEq, Default)]
pub enum TopbarPosition {
    #[default]
    Sticky,
    Relative,
    Absolute,
    Fixed,
}

impl TopbarPosition {
    pub fn to_class(&self) -> &'static str {
        match self {
            TopbarPosition::Sticky => classes::TOPBAR_STICKY,
            TopbarPosition::Relative => classes::TOPBAR_RELATIVE,
            TopbarPosition::Absolute => classes::TOPBAR_ABSOLUTE,
            TopbarPosition::Fixed => classes::TOPBAR_FIXED,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TopbarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub position: TopbarPosition,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Topbar)]
pub fn topbar(props: &TopbarProps) -> Html {
    let class = merge_classes(&[
        classes::TOPBAR_BASE,
        props.position.to_class(),
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

// Topbar sub-components

#[derive(Properties, PartialEq)]
pub struct TopbarStartProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TopbarStart)]
pub fn topbar_start(props: &TopbarStartProps) -> Html {
    let class = merge_classes(&["flex items-center gap-x-4 px-4", &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TopbarCenterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TopbarCenter)]
pub fn topbar_center(props: &TopbarCenterProps) -> Html {
    let class = merge_classes(&[
        "flex-1 flex items-center justify-center",
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TopbarEndProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TopbarEnd)]
pub fn topbar_end(props: &TopbarEndProps) -> Html {
    let class = merge_classes(&["flex items-center gap-x-4 px-4", &props.class.to_string()]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}
