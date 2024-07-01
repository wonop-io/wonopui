use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BreadcrumbProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub separator_icon: Option<Html>,
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbItemProps {
    pub label: String,
    #[prop_or_default]
    pub href: Option<String>,
}

#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProps) -> Html {
    html! {
        <li class={BRANDGUIDE.breadcrumb_item}>
            {
                if let Some(href) = &props.href {
                    html! { <a class="transition-colors hover:text-foreground" href={href.clone()}>{ &props.label }</a> }
                } else {
                    html! { <span role="link" aria-disabled="true" aria-current="page" class="font-normal text-foreground">{ &props.label }</span> }
                }
            }
        </li>
    }
}

#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    let separator_icon = props.separator_icon.clone().unwrap_or_else(|| html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right">
            <path d="m9 18 6-6-6-6"></path>
        </svg>
    });

    html! {
        <nav aria-label="breadcrumb" class={BRANDGUIDE.breadcrumb_nav}>
            <ol class={BRANDGUIDE.breadcrumb_list}>
                { for props.children.iter().enumerate().map(|(index, child)| {
                    html! {
                        <>
                            { child }
                            { if index < props.children.len() - 1 {
                                html! {
                                    <li role="presentation" aria-hidden="true" class={BRANDGUIDE.breadcrumb_separator}>
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
