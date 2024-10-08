#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaceholderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("Content placeholder".to_string())]
    pub text: String,
}

#[function_component(Placeholder)]
pub fn placeholder(props: &PlaceholderProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    html! {
        <div class={classes!(&brandguide.placeholder_container, props.class.clone())}>
            <svg class={classes!(&brandguide.placeholder_svg)} fill="none">
            <defs>
                <pattern id="dash" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
                    <line x1="0" y1="0" x2="0" y2="10" />
                </pattern>
            </defs>
            <rect stroke="none" fill="url(#dash)" width="100%" height="100%"></rect>
            </svg>
            <p class={classes!(&brandguide.placeholder_text)}>{&props.text}</p>
        </div>
    }
}

// Snippets to update brandguide:
// ("placeholder_container".to_string(), "relative overflow-hidden rounded-md border w-full h-[100%] text-zinc-7000 dark:text-zinc-300 dark:border-zinc-300 flex justify-center items-center".to_string()),
// ("placeholder_svg".to_string(), "absolute inset-0 h-full w-full stroke-gray-900/10 dark:stroke-zinc-200/10".to_string()),
// ("placeholder_text".to_string(), "p-2 z-10 bg-white dark:bg-zinc-800 rounded-md".to_string()),
//
// pub placeholder_container: ClassesContainer<T>,
// pub placeholder_svg: ClassesContainer<T>,
// pub placeholder_text: ClassesContainer<T>,
//
// placeholder_container: self.placeholder_container.to_owned(),
// placeholder_svg: self.placeholder_svg.to_owned(),
// placeholder_text: self.placeholder_text.to_owned(),
//
// placeholder_container: default_config_hm
// .get("placeholder_container")
// .expect("Template parameter missing")
// .clone(),
// placeholder_svg: default_config_hm
// .get("placeholder_svg")
// .expect("Template parameter missing")
// .clone(),
// placeholder_text: default_config_hm
// .get("placeholder_text")
// .expect("Template parameter missing")
// .clone(),
