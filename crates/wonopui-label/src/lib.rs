//! Label component for wonopui
//!
//! A label component for form fields.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for label styling.
/// Based on shadcn/ui v4 label component with premium styling.
pub mod classes {
    /// Base label styles - matches shadcn v4 Label component.
    /// Premium look with proper letter-spacing, line-height, and margin.
    pub const LABEL_BASE: &str = "flex items-center gap-2 text-sm leading-6 font-medium tracking-tight select-none text-zinc-950 dark:text-zinc-50 mb-2 group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50";
    
    /// Description text styles with proper spacing.
    pub const LABEL_DESCRIPTION: &str = "text-[13px] leading-5 text-zinc-500 dark:text-zinc-400 mt-1.5";
    
    /// Required indicator styles.
    pub const REQUIRED_INDICATOR: &str = "text-red-500 dark:text-red-400 font-normal";
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
            data-slot="label"
            class={class}
            for={props.for_id.clone()}
        >
            if let Some(description) = &props.description {
                <div class="flex flex-col gap-1">
                    <span class="flex items-center gap-1.5">
                        { props.children.clone() }
                        if props.required {
                            <span class={classes::REQUIRED_INDICATOR}>{"*"}</span>
                        }
                    </span>
                    <span class={classes::LABEL_DESCRIPTION}>
                        { description }
                    </span>
                </div>
            } else {
                <>
                    { props.children.clone() }
                    if props.required {
                        <span class={classes::REQUIRED_INDICATOR}>{"*"}</span>
                    }
                </>
            }
        </label>
    }
}
