use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class={BRANDGUIDE.default_separator}>
            <div class="cursor-pointer py-4" {onclick}>
                <h2 class={BRANDGUIDE.typography_h2}>{ &props.title }</h2>
            </div>
            if *is_open {
                <div class="py-2">
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}
