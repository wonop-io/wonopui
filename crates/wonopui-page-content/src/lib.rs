//! Page Content component for WonopUI.
//!
//! A wrapper component for main page content.

use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the PageContent component
pub mod classes {
    pub const CONTAINER: &str = "bg-white dark:bg-zinc-900 overflow-y-auto h-full min-h-full rounded-md";
}

#[derive(Properties, PartialEq, Clone)]
pub struct PageContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(PageContent)]
pub fn page_content(props: &PageContentProps) -> Html {
    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);
    
    html! {
        <main class={container_class}>
            { for props.children.iter() }
        </main>
    }
}
