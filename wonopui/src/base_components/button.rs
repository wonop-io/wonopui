use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub kind: Option<String>,
}

#[derive(PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
    Default,
}

#[derive(PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Default
    }
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Medium
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let button_class = match props.variant {
        ButtonVariant::Primary => BRANDGUIDE.button_primary,
        ButtonVariant::Secondary => BRANDGUIDE.button_secondary,
        ButtonVariant::Success => BRANDGUIDE.button_success,
        ButtonVariant::Warning => BRANDGUIDE.button_warning,
        ButtonVariant::Danger => BRANDGUIDE.button_danger,
        ButtonVariant::Ghost => BRANDGUIDE.button_ghost,
        ButtonVariant::Default => BRANDGUIDE.button_default,
    };

    let size_class = match props.size {
        ButtonSize::Small => "text-xs",//BRANDGUIDE.button_small,
        ButtonSize::Medium => "",//BRANDGUIDE.button_medium,
        ButtonSize::Large => "text-lg",//BRANDGUIDE.button_large,
    };

    html! {
        <button
            class={classes!(BRANDGUIDE.button_base, button_class, size_class, props.class.clone())}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            type={props.kind.clone().unwrap_or_else(|| "button".to_string())}
        >
            { for props.children.iter() }
        </button>
    }
}
