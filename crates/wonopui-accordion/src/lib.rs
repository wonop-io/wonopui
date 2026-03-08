//! Accordion component for WonopUI.
//!
//! A collapsible content panel for showing/hiding content.

use wonopui_core::*;

/// Default CSS classes for accordion styling (shadcn v4 style).
pub mod classes {
    /// Container/item styles - shadcn v4 AccordionItem with last-child border.
    pub const CONTAINER: &str = "border-b border-zinc-200 dark:border-zinc-800 last:border-b-0";

    /// Header wrapper for proper flex layout.
    pub const HEADER_WRAPPER: &str = "flex";

    /// Header/trigger styles - shadcn v4 AccordionTrigger with proper focus ring.
    pub const HEADER: &str = "flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline cursor-pointer focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180";

    /// Title styles - shadcn v4.
    pub const TITLE: &str = "text-zinc-950 dark:text-zinc-50";

    /// Icon styles - shadcn v4 with proper size and transition.
    pub const ICON: &str = "size-4 shrink-0 text-zinc-500 dark:text-zinc-400 transition-transform duration-200 translate-y-0.5 pointer-events-none";

    /// Content wrapper styles - shadcn v4 AccordionContent with animations.
    pub const CONTENT_WRAPPER: &str = "overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down";

    /// Content inner styles - shadcn v4.
    pub const CONTENT: &str = "pt-0 pb-4 text-zinc-500 dark:text-zinc-400";
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
    let data_state = if *is_open { "open" } else { "closed" };
    
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div data-slot="accordion-item" class={classes!(classes::CONTAINER, props.class.clone())}>
            <div class={classes::HEADER_WRAPPER}>
                <button 
                    data-slot="accordion-trigger" 
                    data-state={data_state} 
                    type="button"
                    class={classes::HEADER} 
                    {onclick}
                    aria-expanded={(*is_open).to_string()}
                >
                    <span class={classes::TITLE}>{ &props.title }</span>
                    <svg
                        class={classes!(classes::ICON, if *is_open { "rotate-180" } else { "" })}
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                    </svg>
                </button>
            </div>
            if *is_open {
                <div data-slot="accordion-content" data-state={data_state} class={classes::CONTENT_WRAPPER}>
                    <div class={classes::CONTENT}>
                        { for props.children.iter() }
                    </div>
                </div>
            }
        </div>
    }
}
