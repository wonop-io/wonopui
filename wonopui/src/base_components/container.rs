#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::ClassesStr;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum ContainerVariant {
    Small,
    Narrow,
    Large,
    Responsive,
    None,
}

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
    #[prop_or(true)]
    pub expanding: bool,
    #[prop_or(true)]
    pub padding_x: bool,
    #[prop_or(true)]
    pub padding_y: bool,
    #[prop_or(ContainerVariant::Responsive)]
    pub variant: ContainerVariant,
    #[prop_or_default]
    pub style: Option<String>,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let padding_x = if props.padding_x {
        brandguide.container_padding_x.clone()
    } else {
        ClassesStr::empty()
    };
    let padding_y = if props.padding_y {
        brandguide.container_padding_y.clone()
    } else {
        ClassesStr::empty()
    };

    let expanding = if props.expanding {
        brandguide.container_expanding.clone()
    } else {
        ClassesStr::empty()
    };

    let variant = match props.variant {
        ContainerVariant::Small => brandguide.container_small.clone(),
        ContainerVariant::Narrow => brandguide.container_narrow.clone(),
        ContainerVariant::Large => brandguide.container_large.clone(),
        ContainerVariant::Responsive => brandguide.container_responsive.clone(),
        ContainerVariant::None => ClassesStr::empty(),
    };

    let container_class = classes!(padding_x, padding_y, expanding, variant);

    html!(
        <@{props.tag.clone()} class={classes!(container_class, props.class.clone())} style={props.style.clone()}>{props.children.clone()}</@>
    )
}
