use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub interval: u32,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let current_index = use_state(|| 0);
    let total_items = props.children.len();
    let interval = props.interval;

    {
        let current_index = current_index.clone();
        use_effect_with((), move |_| {
            let interval_id = gloo::timers::callback::Interval::new(interval, move || {
                current_index.set((*current_index + 1) % total_items);
            });
            || drop(interval_id)
        });
    }

    html! {
        <div class={BRANDGUIDE.carousel_container}>
            <div class={BRANDGUIDE.carousel_inner}>
                { for props.children.iter().enumerate().map(|(index, child)| {
                    let class = if index == *current_index {
                        BRANDGUIDE.carousel_item_active
                    } else {
                        BRANDGUIDE.carousel_item
                    };
                    html! {
                        <div class={class}>
                            { child }
                        </div>
                    }
                }) }
            </div>
            <div class={BRANDGUIDE.carousel_controls}>
                <button onclick={{
                    let current_index = current_index.clone();
                    Callback::from(move |_| current_index.set((*current_index + total_items - 1) % total_items))
                }}>
                    { "Previous" }
                </button>
                <button onclick={{
                    let current_index = current_index.clone();
                    Callback::from(move |_| current_index.set((*current_index + 1) % total_items))
                }}>
                    { "Next" }
                </button>
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
        <div class={BRANDGUIDE.carousel_item}>
            { for props.children.iter() }
        </div>
    }
}
