//! Divider component for wonopui
//!
//! A horizontal divider with optional text.
//! Styled to match shadcn/ui v4 design system (Separator).

use yew::prelude::*;

/// Default CSS classes for divider/separator styling.
/// Based on shadcn/ui v4 separator component with premium styling.
pub mod classes {
    /// Container styles for divider with text.
    pub const DIVIDER_CONTAINER: &str = "relative my-6";
    
    /// Horizontal separator line - premium gradient effect.
    pub const DIVIDER_HORIZONTAL: &str = "bg-gradient-to-r from-transparent via-zinc-300 to-transparent dark:via-zinc-700 shrink-0 h-px w-full";
    
    /// Vertical separator line - premium gradient effect.
    pub const DIVIDER_VERTICAL: &str = "bg-gradient-to-b from-transparent via-zinc-300 to-transparent dark:via-zinc-700 shrink-0 h-full w-px";
    
    /// Simple horizontal separator (no gradient).
    pub const DIVIDER_SIMPLE: &str = "bg-zinc-200 dark:bg-zinc-800 shrink-0 h-px w-full";
    
    /// Legacy alias for horizontal divider.
    pub const DIVIDER_LINE: &str = DIVIDER_SIMPLE;
    
    /// Text container styles (for dividers with text).
    pub const DIVIDER_TEXT_CONTAINER: &str = "relative flex justify-center text-sm font-medium leading-6";
    
    /// Text styles with premium appearance.
    pub const DIVIDER_TEXT: &str = "bg-white dark:bg-zinc-950 px-4 text-zinc-400 dark:text-zinc-500 uppercase text-xs tracking-wider font-semibold";
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
            <div
                data-slot="separator-container"
                class={format!("{} {}", classes::DIVIDER_CONTAINER, props.class)}
            >
                <div class="absolute inset-0 flex items-center" aria-hidden="true">
                    <hr
                        data-slot="separator"
                        data-orientation="horizontal"
                        class={classes::DIVIDER_LINE}
                        style={props.style.clone()}
                    />
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
            <hr
                data-slot="separator"
                data-orientation="horizontal"
                class={format!("{} {} {}", classes::DIVIDER_LINE, classes::DIVIDER_CONTAINER, props.class)}
                style={props.style.clone()}
            />
        }
    }
}
