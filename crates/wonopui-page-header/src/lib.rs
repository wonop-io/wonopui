//! Page Header component for WonopUI.
//!
//! A header component for pages with title and action buttons.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the PageHeader component
pub mod classes {
    pub const CONTAINER: &str = "flex justify-between items-end mb-8";
    pub const TITLE: &str = "text-2xl font-bold";
    pub const ACTIONS: &str = "flex space-x-2";
}

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            <span class={classes::TITLE}>{ &props.title }</span>
            <div class={classes::ACTIONS}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
