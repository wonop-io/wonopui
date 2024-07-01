use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    #[prop_or_default]
    pub badge_type: BadgeType,
}

#[derive(PartialEq)]
pub enum BadgeType {
    Success,
    Warning,
    Error,
    Info,
    Default,
}
// inline-flex items-center rounded-md bg-gray-50 px-2 py-1 text-xs
// font-medium text-gray-600 ring-1 ring-inset ring-gray-500/10
impl Default for BadgeType {
    fn default() -> Self {
        BadgeType::Default
    }
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let badge_class = match props.badge_type {
        BadgeType::Success => BRANDGUIDE.badge_success,
        BadgeType::Warning => BRANDGUIDE.badge_warning,
        BadgeType::Error => BRANDGUIDE.badge_error, // TODO: Unconsistent with Button
        BadgeType::Info => BRANDGUIDE.badge_info,
        BadgeType::Default => BRANDGUIDE.badge_default,
    };

    html! {
        <span class={classes!(BRANDGUIDE.badge_base, badge_class)}>
            { &props.label }
        </span>
    }
}
