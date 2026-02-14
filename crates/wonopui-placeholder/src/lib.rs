//! Placeholder component for wonopui
//!
//! A placeholder component with a dashed border pattern, useful for
//! indicating content areas during development or empty states.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const PLACEHOLDER_CONTAINER: &str = "relative overflow-hidden rounded-md border w-full h-full text-zinc-700 dark:text-zinc-300 dark:border-zinc-600 flex justify-center items-center min-h-[100px]";
    pub const PLACEHOLDER_SVG: &str = "absolute inset-0 h-full w-full stroke-gray-900/10 dark:stroke-zinc-200/10";
    pub const PLACEHOLDER_TEXT: &str = "p-2 z-10 bg-white dark:bg-zinc-800 rounded-md text-sm";
}

#[derive(Properties, PartialEq)]
pub struct PlaceholderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("Content placeholder".to_string())]
    pub text: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Placeholder)]
pub fn placeholder(props: &PlaceholderProps) -> Html {
    let container_class = merge_classes(&[
        classes::PLACEHOLDER_CONTAINER,
        &props.class.to_string(),
    ]);

    html! {
        <div class={container_class}>
            <svg class={classes::PLACEHOLDER_SVG} fill="none">
                <defs>
                    <pattern id="dash" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
                        <line x1="0" y1="0" x2="0" y2="10" />
                    </pattern>
                </defs>
                <rect stroke="none" fill="url(#dash)" width="100%" height="100%"></rect>
            </svg>
            <div class={classes::PLACEHOLDER_TEXT}>
                if props.children.is_empty() {
                    { &props.text }
                } else {
                    { for props.children.iter() }
                }
            </div>
        </div>
    }
}
