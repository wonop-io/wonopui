#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub badge_type: BadgeType,
    #[prop_or_default]
    pub children: Children,
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let badge_class = match props.badge_type {
        BadgeType::Success => &brandguide.badge_success,
        BadgeType::Warning => &brandguide.badge_warning,
        BadgeType::Error => &brandguide.badge_error, // TODO: Unconsistent with Button
        BadgeType::Info => &brandguide.badge_info,
        BadgeType::Default => &brandguide.badge_default,
    };

    html! {
        <span class={classes!(&brandguide.badge_base, badge_class)}>
            if let Some(label) = &props.label {
                { label }
            } else {
                { props.children.clone() }
            }
        </span>
    }
}
