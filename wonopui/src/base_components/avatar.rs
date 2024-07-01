use crate::config::BRANDGUIDE;
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
    let size_class = match props.size {
        AvatarSize::Small => BRANDGUIDE.avatar_small,
        AvatarSize::Medium => BRANDGUIDE.avatar_medium,
        AvatarSize::Large => BRANDGUIDE.avatar_large,
    };

    html! {
        <img class={format!("{} {}", BRANDGUIDE.avatar_base, size_class)} src={props.src.clone()} alt={props.alt.clone()} />
    }
}
