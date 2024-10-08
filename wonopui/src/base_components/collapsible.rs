#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let is_open = use_state(|| props.open);

    let toggle_open = {
        let is_open = is_open.clone();
        let on_open_change = props.on_open_change.clone();
        Callback::from(move |_: MouseEvent| {
            let new_state = !*is_open;
            is_open.set(new_state);
            on_open_change.emit(new_state);
        })
    };

    html! {
        <div class={classes!(&brandguide.collapsible_container, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTriggerProps {
    #[prop_or_default]
    pub as_child: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(CollapsibleTrigger)]
pub fn collapsible_trigger(props: &CollapsibleTriggerProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div onclick={&props.onclick} class={&brandguide.collapsible_button}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleContentProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub is_open: bool,
}

#[function_component(CollapsibleContent)]
pub fn collapsible_content(props: &CollapsibleContentProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        if props.is_open {
            <div class={classes!(&brandguide.collapsible_content, props.class.clone())}>
                { for props.children.iter() }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleHeaderProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleHeader)]
pub fn collapsible_header(props: &CollapsibleHeaderProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={classes!(&brandguide.collapsible_header, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTitleProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleTitle)]
pub fn collapsible_title(props: &CollapsibleTitleProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h4 class={classes!(&brandguide.collapsible_title, props.class.clone())}>
            { for props.children.iter() }
        </h4>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleItemProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleItem)]
pub fn collapsible_item(props: &CollapsibleItemProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={classes!(&brandguide.collapsible_item, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}
