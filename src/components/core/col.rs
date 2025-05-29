#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ColProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
}

#[function_component(Col)]
pub fn col(props: &ColProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    html!(
        <@{props.tag.clone()} class={classes!(&brandguide.col_container, props.class.clone())}>{props.children.clone()}</@>
    )
}

// Snippets to update brandguide:
// ("col_container".to_string(), "flex flex-col".to_string()),
//
// pub col_container: ClassesContainer<T>,
//
// col_container: self.col_container.to_owned(),
//
// col_container: default_config_hm
// .get("col_container")
// .expect("Template parameter missing")
// .clone(),
