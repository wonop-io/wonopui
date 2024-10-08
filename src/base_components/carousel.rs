#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub interval: u32,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub next: Option<Html>,
    #[prop_or_default]
    pub prev: Option<Html>,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let current_index = use_state(|| 0);
    let total_items = props.children.len();
    let interval = props.interval;

    {
        let current_index = current_index.clone();
        use_effect_with(
            (current_index, total_items, interval),
            move |(current_index, total_items, interval)| {
                let current_index = current_index.clone();
                let total_items = total_items.clone();
                let interval = interval.clone();
                let interval_id = gloo::timers::callback::Interval::new(interval, move || {
                    current_index.set((*current_index + 1) % total_items);
                });
                || drop(interval_id)
            },
        );
    }

    html! {
        <div class={classes!(brandguide.carousel_container.clone(), props.class.clone())}>
            <div class={&brandguide.carousel_inner}>
                { for props.children.iter().enumerate().map(|(index, child)| {
                    let class = if index == *current_index {
                        &brandguide.carousel_item_active
                    } else {
                        &brandguide.carousel_item
                    };
                    html! {
                        <div class={class}>
                            { child }
                        </div>
                    }
                }) }
            </div>
            <div class={&brandguide.carousel_controls}>
                if let Some(prev) = &props.prev {
                    <button onclick={{
                        let current_index = current_index.clone();
                        Callback::from(move |_| current_index.set((*current_index + total_items - 1) % total_items))
                    }}>
                        {prev.clone()}
                    </button>
                }
                if let Some(next) = &props.next {
                    <button onclick={{
                        let current_index = current_index.clone();
                        Callback::from(move |_| current_index.set((*current_index + 1) % total_items))
                    }}>
                        { next.clone() }
                    </button>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CarouselItemProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CarouselItem)]
pub fn carousel_item(props: &CarouselItemProps) -> Html {
    html! {
        <>
            { for props.children.iter() }
        </>
    }
}
