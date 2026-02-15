//! Label component for wonopui
//!
//! A label component for form fields.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const LABEL_BASE: &str = "text-sm font-medium text-gray-700 dark:text-zinc-300";
    pub const LABEL_DESCRIPTION: &str = "text-xs text-gray-500 dark:text-zinc-500 mt-1";
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub for_id: String,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or_default]
    pub required: bool,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let class = merge_classes(&[classes::LABEL_BASE, &props.class.to_string()]);

    html! {
        <label
            class={class}
            for={props.for_id.clone()}
        >
            if let Some(description) = &props.description {
                <div class="flex flex-col">
                    <div class="flex items-center gap-1">
                        { props.children.clone() }
                        if props.required {
                            <span class="text-red-500">{"*"}</span>
                        }
                    </div>
                    <div class={classes::LABEL_DESCRIPTION}>
                        { description }
                    </div>
                </div>
            } else {
                <span class="flex items-center gap-1">
                    { props.children.clone() }
                    if props.required {
                        <span class="text-red-500">{"*"}</span>
                    }
                </span>
            }
        </label>
    }
}
