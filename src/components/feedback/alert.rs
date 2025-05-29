#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    #[prop_or(AlertType::Info)]
    pub alert_type: AlertType,
    #[prop_or_default]
    pub icon: Option<Html>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(PartialEq)]
pub enum AlertType {
    Success,
    Warning,
    Error,
    Info,
}

impl Default for AlertType {
    fn default() -> Self {
        AlertType::Info
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertTitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(AlertTitle)]
pub fn alert_title(props: &AlertTitleProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    html! {
        <div class={classes!(brandguide.alert_title.clone(), props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertDescriptionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(AlertDescription)]
pub fn alert_description(props: &AlertDescriptionProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    html! {
        <div class={classes!(brandguide.alert_description.clone(), props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let alert_class = match props.alert_type {
        AlertType::Success => &brandguide.alert_success,
        AlertType::Warning => &brandguide.alert_warning,
        AlertType::Error => &brandguide.alert_error,
        AlertType::Info => &brandguide.alert_info,
    };

    html! {
        <div class={classes!(format!("{} {}", brandguide.alert_base, alert_class), props.class.clone())}>
            { if let Some(icon) = &props.icon { html! { <span>{icon.clone()}</span> } } else { html! {} } }
            { for props.children.iter() }
        </div>
    }
}
