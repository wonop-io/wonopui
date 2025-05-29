#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let button_class = match props.variant {
        ButtonVariant::Primary => &brandguide.button_primary,
        ButtonVariant::Secondary => &brandguide.button_secondary,
        ButtonVariant::Success => &brandguide.button_success,
        ButtonVariant::Warning => &brandguide.button_warning,
        ButtonVariant::Danger => &brandguide.button_danger,
        ButtonVariant::Ghost => &brandguide.button_ghost,
        ButtonVariant::Default => &brandguide.button_default,
    };

    let size_class = match props.size {
        ButtonSize::Small => &brandguide.button_small,
        ButtonSize::Medium => &brandguide.button_medium,
        ButtonSize::Large => &brandguide.button_large,
    };

    html! {
        <button
            class={classes!(&brandguide.button_base, button_class, size_class, props.class.clone())}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            type={props.kind.clone().unwrap_or_else(|| "button".to_string())}
        >
            { for props.children.iter() }
        </button>
    }
}

// Snippets to update brandguide:
// ("button_base".to_string(), "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background".to_string()),
// ("button_primary".to_string(), "bg-primary text-primary-foreground hover:bg-primary/90".to_string()),
// ("button_secondary".to_string(), "bg-secondary text-secondary-foreground hover:bg-secondary/80".to_string()),
// ("button_success".to_string(), "bg-green-500 text-white hover:bg-green-600".to_string()),
// ("button_warning".to_string(), "bg-yellow-500 text-white hover:bg-yellow-600".to_string()),
// ("button_danger".to_string(), "bg-destructive text-destructive-foreground hover:bg-destructive/90".to_string()),
// ("button_ghost".to_string(), "hover:bg-accent hover:text-accent-foreground".to_string()),
// ("button_default".to_string(), "bg-primary text-primary-foreground hover:bg-primary/90".to_string()),
// ("button_small".to_string(), "h-9 px-3 rounded-md".to_string()),
// ("button_medium".to_string(), "h-10 py-2 px-4".to_string()),
// ("button_large".to_string(), "h-11 px-8 rounded-md".to_string()),
//
// button_base: self.button_base.to_owned(),
// button_primary: self.button_primary.to_owned(),
// button_secondary: self.button_secondary.to_owned(),
// button_success: self.button_success.to_owned(),
// button_warning: self.button_warning.to_owned(),
// button_danger: self.button_danger.to_owned(),
// button_ghost: self.button_ghost.to_owned(),
// button_default: self.button_default.to_owned(),
// button_small: self.button_small.to_owned(),
// button_medium: self.button_medium.to_owned(),
// button_large: self.button_large.to_owned(),
//
// button_base: default_config_hm
// .get("button_base")
// .expect("Template parameter missing")
// .clone(),
// button_primary: default_config_hm
// .get("button_primary")
// .expect("Template parameter missing")
// .clone(),
// button_secondary: default_config_hm
// .get("button_secondary")
// .expect("Template parameter missing")
// .clone(),
// button_success: default_config_hm
// .get("button_success")
// .expect("Template parameter missing")
// .clone(),
// button_warning: default_config_hm
// .get("button_warning")
// .expect("Template parameter missing")
// .clone(),
// button_danger: default_config_hm
// .get("button_danger")
// .expect("Template parameter missing")
// .clone(),
// button_ghost: default_config_hm
// .get("button_ghost")
// .expect("Template parameter missing")
// .clone(),
// button_default: default_config_hm
// .get("button_default")
// .expect("Template parameter missing")
// .clone(),
// button_small: default_config_hm
// .get("button_small")
// .expect("Template parameter missing")
// .clone(),
// button_medium: default_config_hm
// .get("button_medium")
// .expect("Template parameter missing")
// .clone(),
// button_large: default_config_hm
// .get("button_large")
// .expect("Template parameter missing")
// .clone(),
