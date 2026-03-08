//! Textarea component for wonopui
//!
//! A multi-line text input component.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for textarea styling.
/// Based on shadcn/ui v4 textarea component.
pub mod classes {
    /// Base textarea styles - matches shadcn v4 Textarea component.
    /// Uses shadcn v4 focus pattern: focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    pub const TEXTAREA_BASE: &str = "flex field-sizing-content min-h-16 w-full rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent dark:bg-zinc-950/30 px-3 py-2 text-base md:text-sm shadow-xs transition-[color,box-shadow] duration-200 outline-none placeholder:text-zinc-500 dark:placeholder:text-zinc-400 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 text-zinc-900 dark:text-zinc-50";
    /// Disabled state styles.
    pub const TEXTAREA_DISABLED: &str = "cursor-not-allowed opacity-50";
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
            data-slot="textarea"
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
