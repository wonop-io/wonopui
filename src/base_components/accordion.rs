#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::{use_brandguide, use_set_brandguide};
use crate::config::{BrandGuideType, ClassesStr};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class={classes!(&brandguide.accordion_container, props.class.clone())}>
            <div class={classes!(&brandguide.accordion_header)} {onclick}>
                <h2 class={&brandguide.accordion_title}>{ &props.title }</h2>
            </div>
            if *is_open {
                <div class={&brandguide.accordion_content}>
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}

// New entries in the brand guide:
// ("accordion_container".to_string(), "border-b border-gray-200 dark:border-gray-700".to_string()),
// ("accordion_header".to_string(), "flex justify-between items-center py-4 cursor-pointer".to_string()),
// ("accordion_title".to_string(), "text-lg font-medium".to_string()),
// ("accordion_content".to_string(), "py-2".to_string()),
