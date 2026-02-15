//! Content component for wonopui
//!
//! Main content area component with optional aside panel.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const CONTENT_MAIN: &str = "flex-1 overflow-y-auto";
    pub const CONTENT_WITH_ASIDE: &str = "md:mr-96";
    pub const CONTENT_ASIDE: &str = "fixed top-0 right-0 h-full w-96 bg-white dark:bg-zinc-800 border-l border-gray-200 dark:border-zinc-700 overflow-y-auto hidden md:block";
    pub const CONTENT_ASIDE_INNER: &str = "h-full";
    pub const CONTENT_PADDING: &str = "p-4 sm:p-6 lg:p-8";
}

#[derive(Properties, PartialEq)]
pub struct MainContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub expanding: bool,
    #[prop_or(true)]
    pub padding: bool,
    #[prop_or_default]
    pub aside: Option<Html>,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    let main_class = merge_classes(&[
        classes::CONTENT_MAIN,
        if props.padding {
            classes::CONTENT_PADDING
        } else {
            ""
        },
        if props.aside.is_some() {
            classes::CONTENT_WITH_ASIDE
        } else {
            ""
        },
        &props.class.to_string(),
    ]);

    html! {
        <>
            <main class={main_class}>
                { props.children.clone() }
            </main>
            if let Some(aside) = &props.aside {
                <aside class={classes::CONTENT_ASIDE}>
                    <div class={classes::CONTENT_ASIDE_INNER}>
                        { aside.clone() }
                    </div>
                </aside>
            }
        </>
    }
}

// Simple Content wrapper

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub padding: bool,
}

#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    let class = merge_classes(&[
        classes::CONTENT_MAIN,
        if props.padding {
            classes::CONTENT_PADDING
        } else {
            ""
        },
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { props.children.clone() }
        </div>
    }
}
