//! Page Content component for WonopUI.
//!
//! A wrapper component for main page content.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the PageContent component (premium styling)
pub mod classes {
    /// Main container with padding and premium styling.
    pub const CONTAINER: &str =
        "bg-white dark:bg-zinc-950 overflow-y-auto h-full min-h-full rounded-xl p-6 md:p-8 lg:p-10 shadow-sm border border-zinc-100 dark:border-zinc-800/50";
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
