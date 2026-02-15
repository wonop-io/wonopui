//! Textarea component for wonopui
//!
//! A multi-line text input component.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const TEXTAREA_BASE: &str = "w-full px-3 py-2 text-sm bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 dark:text-zinc-100 placeholder-gray-400 dark:placeholder-zinc-500 resize-y";
    pub const TEXTAREA_DISABLED: &str = "opacity-50 cursor-not-allowed";
}

#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub rows: Option<u32>,
    #[prop_or_default]
    pub cols: Option<u32>,
    #[prop_or_default]
    pub maxlength: Option<u32>,
    #[prop_or_default]
    pub minlength: Option<u32>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub readonly: bool,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let class = merge_classes(&[
        classes::TEXTAREA_BASE,
        if props.disabled {
            classes::TEXTAREA_DISABLED
        } else {
            ""
        },
        &props.class.to_string(),
    ]);

    html! {
        <textarea
            class={class}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            placeholder={props.placeholder.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
            disabled={props.disabled}
            rows={props.rows.map(|r| r.to_string())}
            cols={props.cols.map(|c| c.to_string())}
            maxlength={props.maxlength.map(|m| m.to_string())}
            minlength={props.minlength.map(|m| m.to_string())}
            required={props.required}
            readonly={props.readonly}
        />
    }
}
