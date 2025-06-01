#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    #[prop_or_default]
    pub src: Option<String>,
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

    match &props.src {
        Some(src) => html! {
            <img class={format!("{} {}", brandguide.avatar_base, size_class)} src={src.clone()} alt={props.alt.clone()} />
        },
        None => {
            let initials = props
                .alt
                .split_whitespace()
                .take(2)
                .filter_map(|word| word.chars().next())
                .map(|c| c.to_uppercase().to_string())
                .collect::<Vec<_>>()
                .join("");

            let initials = if initials.is_empty() {
                "?".to_string()
            } else {
                initials
            };

            html! {
                <div class={format!("{} {} flex items-center justify-center bg-gray-200 dark:bg-gray-700",
                                   brandguide.avatar_base, size_class)}>
                    <span class="text-gray-700 dark:text-gray-200 font-medium">{initials}</span>
                </div>
            }
        }
    }
}
