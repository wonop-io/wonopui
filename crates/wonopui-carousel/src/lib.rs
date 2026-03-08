//! Carousel component for WonopUI.
//!
//! A carousel/slideshow component that cycles through content.

use gloo_timers::callback::Interval;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Carousel component (shadcn v4)
pub mod classes {
    /// Root container with relative positioning
    pub const CONTAINER: &str = "relative";
    /// Content wrapper that handles overflow and flex layout
    pub const CONTENT: &str = "overflow-hidden";
    /// Inner container for slide items
    pub const INNER: &str = "-ml-4 flex";
    /// Inner container for vertical orientation
    pub const INNER_VERTICAL: &str = "-mt-4 flex-col";
    /// Individual carousel item
    pub const ITEM: &str = "min-w-0 shrink-0 grow-0 basis-full pl-4 hidden";
    /// Active carousel item
    pub const ITEM_ACTIVE: &str = "min-w-0 shrink-0 grow-0 basis-full pl-4 block";
    /// Vertical item spacing
    pub const ITEM_VERTICAL: &str = "pt-4 pl-0";
    /// Controls wrapper - positioned at sides
    pub const CONTROLS: &str = "absolute inset-0 flex items-center justify-between pointer-events-none";
    /// Navigation button base - premium rounded button with shadow
    pub const CONTROL_BUTTON: &str = "pointer-events-auto inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 border border-zinc-200 bg-white/90 backdrop-blur-sm shadow-md hover:bg-white hover:shadow-lg dark:border-zinc-800 dark:bg-zinc-950/90 dark:hover:bg-zinc-950 size-8 rounded-full focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none text-zinc-900 dark:text-zinc-50";
    /// Previous button positioning
    pub const CONTROL_BUTTON_PREVIOUS: &str = "-left-12 absolute";
    /// Next button positioning
    pub const CONTROL_BUTTON_NEXT: &str = "-right-12 absolute";
    /// Indicators container
    pub const INDICATORS: &str = "absolute bottom-4 left-1/2 -translate-x-1/2 flex items-center gap-1.5";
    /// Indicator dot base
    pub const INDICATOR: &str = "size-2 rounded-full bg-zinc-950/20 dark:bg-zinc-50/20 hover:bg-zinc-950/40 dark:hover:bg-zinc-50/40 transition-all duration-200 cursor-pointer";
    /// Active indicator dot
    pub const INDICATOR_ACTIVE: &str = "size-2 rounded-full bg-zinc-950 dark:bg-zinc-50 transition-all duration-200 cursor-pointer";
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
        <div data-slot="carousel" class={container_class} role="region" aria-roledescription="carousel">
            <div data-slot="carousel-content" class={classes::CONTENT}>
                <div data-slot="carousel-inner" class={classes::INNER}>
                    { for props.children.iter().enumerate().map(|(index, child)| {
                        let is_active = index == *current_index;
                        let item_class = if is_active {
                            classes::ITEM_ACTIVE
                        } else {
                            classes::ITEM
                        };
                        html! {
                            <div
                                data-slot="carousel-item"
                                data-active={is_active.then_some("true")}
                                class={item_class}
                                role="group"
                                aria-roledescription="slide"
                                aria-label={format!("Slide {} of {}", index + 1, total_items)}
                            >
                                { child }
                            </div>
                        }
                    }) }
                </div>
            </div>

            // Controls
            <div data-slot="carousel-controls" class={classes::CONTROLS}>
                if let Some(prev) = &props.prev {
                    <button
                        data-slot="carousel-previous"
                        class={classes::CONTROL_BUTTON}
                        onclick={on_prev.clone()}
                        type="button"
                        aria-label="Previous slide"
                    >
                        { prev.clone() }
                    </button>
                } else {
                    <button
                        data-slot="carousel-previous"
                        class={classes::CONTROL_BUTTON}
                        onclick={on_prev.clone()}
                        type="button"
                        aria-label="Previous slide"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                            <path d="m15 18-6-6 6-6"/>
                        </svg>
                        <span class="sr-only">{"Previous slide"}</span>
                    </button>
                }

                if let Some(next) = &props.next {
                    <button
                        data-slot="carousel-next"
                        class={classes::CONTROL_BUTTON}
                        onclick={on_next.clone()}
                        type="button"
                        aria-label="Next slide"
                    >
                        { next.clone() }
                    </button>
                } else {
                    <button
                        data-slot="carousel-next"
                        class={classes::CONTROL_BUTTON}
                        onclick={on_next.clone()}
                        type="button"
                        aria-label="Next slide"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                            <path d="m9 18 6-6-6-6"/>
                        </svg>
                        <span class="sr-only">{"Next slide"}</span>
                    </button>
                }
            </div>

            // Indicators/Dots
            if props.show_indicators && total_items > 1 {
                <div data-slot="carousel-dots" class={classes::INDICATORS} role="tablist" aria-label="Slides">
                    { for (0..total_items).map(|index| {
                        let current_index_clone = current_index.clone();
                        let is_active = index == *current_index;
                        let onclick = Callback::from(move |_: MouseEvent| {
                            current_index_clone.set(index);
                        });
                        let indicator_class = if is_active {
                            classes::INDICATOR_ACTIVE
                        } else {
                            classes::INDICATOR
                        };
                        html! {
                            <button
                                data-slot="carousel-dot"
                                data-active={is_active.then_some("true")}
                                class={indicator_class}
                                onclick={onclick}
                                type="button"
                                role="tab"
                                aria-selected={is_active.to_string()}
                                aria-label={format!("Go to slide {}", index + 1)}
                            />
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
