//! Divider component for wonopui
//!
//! A horizontal divider with optional text.

use yew::prelude::*;

pub mod classes {
    pub const DIVIDER_CONTAINER: &str = "relative my-6";
    pub const DIVIDER_LINE: &str = "w-full border-t border-gray-200 dark:border-zinc-700";
    pub const DIVIDER_TEXT_CONTAINER: &str =
        "relative flex justify-center text-sm font-medium leading-6";
    pub const DIVIDER_TEXT: &str =
        "bg-white dark:bg-zinc-900 px-6 text-gray-500 dark:text-zinc-400";
}

#[derive(Properties, PartialEq)]
pub struct DividerProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Divider)]
pub fn divider(props: &DividerProps) -> Html {
    let has_content = !props.text.is_empty() || !props.children.is_empty();

    if has_content {
        html! {
            <div class={format!("{} {}", classes::DIVIDER_CONTAINER, props.class)}>
                <div class="absolute inset-0 flex items-center" aria-hidden="true">
                    <hr class={classes::DIVIDER_LINE} style={props.style.clone()} />
                </div>
                <div class={classes::DIVIDER_TEXT_CONTAINER}>
                    <span class={classes::DIVIDER_TEXT}>
                        if !props.text.is_empty() {
                            { &props.text }
                        } else {
                            { props.children.clone() }
                        }
                    </span>
                </div>
            </div>
        }
    } else {
        html! {
            <hr class={format!("{} {} {}", classes::DIVIDER_LINE, classes::DIVIDER_CONTAINER, props.class)} style={props.style.clone()} />
        }
    }
}
