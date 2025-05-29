#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TypographyProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(H1)]
pub fn h1(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h1 class={classes!(&brandguide.typography_h1, props.class.clone())}>
            { for props.children.iter() }
        </h1>
    }
}

#[function_component(H2)]
pub fn h2(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h2 class={classes!(&brandguide.typography_h2, props.class.clone())}>
            { for props.children.iter() }
        </h2>
    }
}

#[function_component(H3)]
pub fn h3(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h3 class={classes!(&brandguide.typography_h3, props.class.clone())}>
            { for props.children.iter() }
        </h3>
    }
}

#[function_component(H4)]
pub fn h4(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h4 class={classes!(&brandguide.typography_h4, props.class.clone())}>
            { for props.children.iter() }
        </h4>
    }
}

#[function_component(H5)]
pub fn h5(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h5 class={classes!(&brandguide.typography_h5, props.class.clone())}>
            { for props.children.iter() }
        </h5>
    }
}

#[function_component(H6)]
pub fn h6(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h6 class={classes!(&brandguide.typography_h6, props.class.clone())}>
            { for props.children.iter() }
        </h6>
    }
}

#[function_component(Paragraph)]
pub fn paragraph(props: &TypographyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <p class={classes!(&brandguide.typography_p, props.class.clone())}>
            { for props.children.iter() }
        </p>
    }
}
