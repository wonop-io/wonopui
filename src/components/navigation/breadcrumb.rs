#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};
use yew_router::Routable;

#[derive(Properties, PartialEq)]
pub struct BreadcrumbProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub separator_icon: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbItemProps {
    pub label: String,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbRouteItemProps<R: Routable + 'static> {
    pub label: String,
    pub to: R,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn BreadcrumbRouteItem<R: Routable + 'static>(props: &BreadcrumbRouteItemProps<R>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let combined_class = classes!(
        "transition-colors",
        "hover:text-foreground",
        props.class.clone()
    );

    html! {
        <li class={&brandguide.breadcrumb_item}>
            <Link<R> to={props.to.clone()} classes={combined_class}>
                { &props.label }
            </Link<R>>
        </li>
    }
}

#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let link_class = classes!(
        "transition-colors",
        "hover:text-foreground",
        props.class.clone()
    );
    let span_class = classes!("font-normal", "text-foreground", props.class.clone());

    html! {
        <li class={&brandguide.breadcrumb_item}>
            {
                if let Some(href) = &props.href {
                    html! { <a class={link_class} href={href.clone()}>{ &props.label }</a> }
                } else {
                    html! { <span role="link" aria-disabled="true" aria-current="page" class={span_class}>{ &props.label }</span> }
                }
            }
        </li>
    }
}

#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let separator_icon = props.separator_icon.clone().unwrap_or_else(|| html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right">
            <path d="m9 18 6-6-6-6"></path>
        </svg>
    });

    let nav_class = classes!(&brandguide.breadcrumb_nav, props.class.clone());

    html! {
        <nav aria-label="breadcrumb" class={nav_class}>
            <ol class={&brandguide.breadcrumb_list}>
                { for props.children.iter().enumerate().map(|(index, child)| {
                    html! {
                        <>
                            { child }
                            { if index < props.children.len() - 1 {
                                html! {
                                    <li role="presentation" aria-hidden="true" class={&brandguide.breadcrumb_separator}>
                                        { separator_icon.clone() }
                                    </li>
                                }
                            } else {
                                html! {}
                            }}
                        </>
                    }
                }) }
            </ol>
        </nav>
    }
}
