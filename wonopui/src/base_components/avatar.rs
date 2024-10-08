#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub src: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or_default]
    pub size: AvatarSize,
}

#[derive(PartialEq)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

impl Default for AvatarSize {
    fn default() -> Self {
        AvatarSize::Medium
    }
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let size_class = match props.size {
        AvatarSize::Small => &brandguide.avatar_small,
        AvatarSize::Medium => &brandguide.avatar_medium,
        AvatarSize::Large => &brandguide.avatar_large,
    };

    html! {
        <img class={format!("{} {}", brandguide.avatar_base, size_class)} src={props.src.clone()} alt={props.alt.clone()} />
    }
}
