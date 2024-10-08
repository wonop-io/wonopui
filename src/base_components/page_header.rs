#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    html! {
        <div class={classes!(&brandguide.page_header_container)}>
            <span class={classes!(&brandguide.page_header_title)}>{&props.title}</span>
            <div class={classes!(&brandguide.page_header_actions)}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

// Snippets to update brandguide:
// ("page_header_container".to_string(), "flex justify-between items-end mb-8".to_string()),
// ("page_header_title".to_string(), "text-2xl font-bold".to_string()),
// ("page_header_actions".to_string(), "flex space-x-2".to_string()),
//
// pub page_header_container: ClassesContainer<T>,
// pub page_header_title: ClassesContainer<T>,
// pub page_header_actions: ClassesContainer<T>,
//
// page_header_container: self.page_header_container.to_owned(),
// page_header_title: self.page_header_title.to_owned(),
// page_header_actions: self.page_header_actions.to_owned(),
//
// page_header_container: default_config_hm
// .get("page_header_container")
// .expect("Template parameter missing")
// .clone(),
// page_header_title: default_config_hm
// .get("page_header_title")
// .expect("Template parameter missing")
// .clone(),
// page_header_actions: default_config_hm
// .get("page_header_actions")
// .expect("Template parameter missing")
// .clone(),
