use crate::base_components::container::{Container, ContainerVariant};
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::ClassesStr;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub expanding: bool,
    #[prop_or(true)]
    pub padding_x: bool,
    #[prop_or(true)]
    pub padding_y: bool,
    #[prop_or_default]
    pub aside: Option<Html>,
}

#[function_component(MainContent)]
pub fn main_content(props: &ContentProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let base_container = html! {
        <Container key="main" tag="main" variant={ContainerVariant::None} expanding={props.expanding} padding_x={props.padding_x} padding_y={props.padding_y} class={classes!(props.class.clone(), if props.aside.is_some() { brandguide.content_with_aside.clone() } else { ClassesStr::empty() })}>
            {props.children.clone()}
        </Container>
    };
    html! {
        <>
            {base_container}
            {if let Some(aside) = &props.aside {
                html! {
                    <aside class={classes!(&brandguide.content_aside)}>
                        <Container class={classes!(&brandguide.content_aside_container)}>
                            {aside.clone()}
                        </Container>
                    </aside>
                }
            } else {
                html! {}
            }}
        </>
    }
}

// Snippets to update brandguide:
// ("content_with_aside".to_string(), "md:mr-96".to_string()),
// ("content_aside".to_string(), "fixed top-0 right-0 h-full w-96 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 overflow-y-auto hidden md:block".to_string()),
// ("content_aside_container".to_string(), "h-full".to_string()),
//
// pub content_with_aside: ClassesContainer<T>,
// pub content_aside: ClassesContainer<T>,
// pub content_aside_container: ClassesContainer<T>,
//
// content_with_aside: self.content_with_aside.to_owned(),
// content_aside: self.content_aside.to_owned(),
// content_aside_container: self.content_aside_container.to_owned(),
//
// content_with_aside: default_config_hm
// .get("content_with_aside")
// .expect("Template parameter missing")
// .clone(),
// content_aside: default_config_hm
// .get("content_aside")
// .expect("Template parameter missing")
// .clone(),
// content_aside_container: default_config_hm
// .get("content_aside_container")
// .expect("Template parameter missing")
// .clone(),
