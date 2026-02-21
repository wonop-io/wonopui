//! Accordion component for WonopUI.
//!
//! A collapsible content panel for showing/hiding content.

use wonopui_core::*;

/// Default CSS classes for accordion styling.
pub mod classes {
    /// Container styles for accordion.
    pub const CONTAINER: &str =
        "[&:not(:last-child)]:border-b border-zinc-200 dark:border-zinc-700 dark:text-zinc-100";

    /// Header styles for accordion.
    pub const HEADER: &str = "flex justify-between items-center py-4 cursor-pointer";

    /// Title styles for accordion.
    pub const TITLE: &str = "text-lg font-medium text-zinc-800 dark:text-zinc-100";

    /// Content styles for accordion.
    pub const CONTENT: &str = "py-2 text-zinc-600 dark:text-zinc-300 mb-8";
}

/// Properties for the Accordion component.
#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    /// The title shown in the accordion header.
    pub title: String,

    /// Content to show when expanded.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Whether the accordion is initially open.
    #[prop_or_default]
    pub default_open: bool,
}

/// A collapsible content panel for showing/hiding content.
///
/// # Example
///
/// ```rust
/// use wonopui_accordion::Accordion;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Accordion title="Click to expand">
///             {"Hidden content here"}
///         </Accordion>
///     }
/// }
/// ```
#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let is_open = use_state(|| props.default_open);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class={classes!(classes::CONTAINER, props.class.clone())}>
            <div class={classes::HEADER} {onclick}>
                <h2 class={classes::TITLE}>{ &props.title }</h2>
                <svg
                    class={format!("w-5 h-5 transition-transform {}", if *is_open { "rotate-180" } else { "" })}
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
            </div>
            if *is_open {
                <div class={classes::CONTENT}>
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}
