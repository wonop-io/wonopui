use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub message: String, // TODO: Deprecated
    #[prop_or(AlertType::Info)]
    pub alert_type: AlertType,
    #[prop_or_default]
    pub icon: Option<Html>,
    #[prop_or_default]
    pub children: Children,
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

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let alert_class = match props.alert_type {
        AlertType::Success => BRANDGUIDE.alert_success,
        AlertType::Warning => BRANDGUIDE.alert_warning,
        AlertType::Error => BRANDGUIDE.alert_error,
        AlertType::Info => BRANDGUIDE.alert_info,
    };

    html! {
        <div class={format!("{} {}", BRANDGUIDE.alert_base, alert_class)}>
            { if let Some(icon) = &props.icon { html! { <span>{icon.clone()}</span> } } else { html! {} } }
            if !props.message.is_empty() {
                <span>{ &props.message }</span>
            }
            { for props.children.iter() }
        </div>
    }
}
