//! Carousel component for WonopUI.
//!
//! A carousel/slideshow component that cycles through content.

use gloo_timers::callback::Interval;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Carousel component
pub mod classes {
    pub const CONTAINER: &str = "relative overflow-hidden rounded-lg";
    pub const INNER: &str = "relative w-full";
    pub const ITEM: &str = "hidden";
    pub const ITEM_ACTIVE: &str = "block";
    pub const CONTROLS: &str = "absolute inset-0 flex items-center justify-between p-4";
    pub const CONTROL_BUTTON: &str = "inline-flex items-center justify-center rounded-full bg-white/80 dark:bg-gray-800/80 p-2 text-gray-800 dark:text-gray-200 hover:bg-white dark:hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary transition-colors";
    pub const INDICATORS: &str = "absolute bottom-4 left-1/2 -translate-x-1/2 flex space-x-2";
    pub const INDICATOR: &str =
        "w-2 h-2 rounded-full bg-white/50 hover:bg-white/75 transition-colors cursor-pointer";
    pub const INDICATOR_ACTIVE: &str =
        "w-2 h-2 rounded-full bg-white transition-colors cursor-pointer";
}

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    #[prop_or_default]
    pub children: Children,
    /// Auto-advance interval in milliseconds. 0 or unset disables auto-advance.
    #[prop_or(5000)]
    pub interval: u32,
    #[prop_or_default]
    pub class: Classes,
    /// Custom next button content
    #[prop_or_default]
    pub next: Option<Html>,
    /// Custom previous button content
    #[prop_or_default]
    pub prev: Option<Html>,
    /// Show indicator dots
    #[prop_or(true)]
    pub show_indicators: bool,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let current_index = use_state(|| 0usize);
    let total_items = props.children.len();
    let interval = props.interval;

    // Auto-advance effect
    {
        let current_index = current_index.clone();
        use_effect_with((interval, total_items), move |(interval, total_items)| {
            let interval = *interval;
            let total_items = *total_items;

            let interval_handle: Option<Interval> = if interval == 0 || total_items == 0 {
                None
            } else {
                let current_index = current_index.clone();
                Some(Interval::new(interval, move || {
                    current_index.set((*current_index + 1) % total_items);
                }))
            };

            move || drop(interval_handle)
        });
    }

    let on_prev = {
        let current_index = current_index.clone();
        Callback::from(move |_: MouseEvent| {
            if total_items > 0 {
                current_index.set((*current_index + total_items - 1) % total_items);
            }
        })
    };

    let on_next = {
        let current_index = current_index.clone();
        Callback::from(move |_: MouseEvent| {
            if total_items > 0 {
                current_index.set((*current_index + 1) % total_items);
            }
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            <div class={classes::INNER}>
                { for props.children.iter().enumerate().map(|(index, child)| {
                    let item_class = if index == *current_index {
                        classes::ITEM_ACTIVE
                    } else {
                        classes::ITEM
                    };
                    html! {
                        <div class={item_class}>
                            { child }
                        </div>
                    }
                }) }
            </div>

            // Controls
            <div class={classes::CONTROLS}>
                if let Some(prev) = &props.prev {
                    <button class={classes::CONTROL_BUTTON} onclick={on_prev.clone()}>
                        { prev.clone() }
                    </button>
                } else {
                    <button class={classes::CONTROL_BUTTON} onclick={on_prev.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="m15 18-6-6 6-6"/>
                        </svg>
                    </button>
                }

                if let Some(next) = &props.next {
                    <button class={classes::CONTROL_BUTTON} onclick={on_next.clone()}>
                        { next.clone() }
                    </button>
                } else {
                    <button class={classes::CONTROL_BUTTON} onclick={on_next.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="m9 18 6-6-6-6"/>
                        </svg>
                    </button>
                }
            </div>

            // Indicators
            if props.show_indicators && total_items > 1 {
                <div class={classes::INDICATORS}>
                    { for (0..total_items).map(|index| {
                        let current_index_clone = current_index.clone();
                        let onclick = Callback::from(move |_: MouseEvent| {
                            current_index_clone.set(index);
                        });
                        let indicator_class = if index == *current_index {
                            classes::INDICATOR_ACTIVE
                        } else {
                            classes::INDICATOR
                        };
                        html! {
                            <button class={indicator_class} onclick={onclick} />
                        }
                    }) }
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CarouselItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CarouselItem)]
pub fn carousel_item(props: &CarouselItemProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            { for props.children.iter() }
        </div>
    }
}
