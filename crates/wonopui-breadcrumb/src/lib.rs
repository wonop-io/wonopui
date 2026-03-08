//! Breadcrumb component for WonopUI.
//!
//! Navigation component for showing hierarchical page location.

use wonopui_core::*;
use yew_router::prelude::{use_navigator, Link};
use yew_router::Routable;

/// Default CSS classes for breadcrumb styling (shadcn v4 style).
pub mod classes {
    /// Nav container styles - shadcn v4 Breadcrumb.
    pub const NAV: &str = "";

    /// List container styles - shadcn v4 BreadcrumbList.
    pub const LIST: &str = "flex flex-wrap items-center gap-1.5 break-words text-sm text-zinc-500 dark:text-zinc-400 sm:gap-2.5";

    /// Item styles - shadcn v4 BreadcrumbItem.
    pub const ITEM: &str = "inline-flex items-center gap-1.5";

    /// Link styles - shadcn v4 BreadcrumbLink.
    pub const LINK: &str = "transition-colors hover:text-zinc-950 dark:hover:text-zinc-50";

    /// Page styles (current item) - shadcn v4 BreadcrumbPage.
    pub const PAGE: &str = "font-normal text-zinc-950 dark:text-zinc-50";

    /// Separator styles - shadcn v4 BreadcrumbSeparator.
    pub const SEPARATOR: &str = "[&>svg]:size-3.5 text-zinc-500 dark:text-zinc-400";

    /// Ellipsis styles - shadcn v4 BreadcrumbEllipsis.
    pub const ELLIPSIS: &str = "flex size-9 items-center justify-center";
}

/// Properties for the Breadcrumb component.
#[derive(Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// Breadcrumb items as children.
    #[prop_or_default]
    pub children: Children,

    /// Custom separator icon.
    #[prop_or_default]
    pub separator_icon: Option<Html>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Properties for BreadcrumbItem.
#[derive(Properties, PartialEq)]
pub struct BreadcrumbItemProps {
    /// Display label.
    pub label: String,

    /// Optional href for linking.
    #[prop_or_default]
    pub href: Option<String>,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Properties for BreadcrumbRouteItem with yew-router support.
#[derive(Properties, PartialEq)]
pub struct BreadcrumbRouteItemProps<R: Routable + 'static> {
    /// Display label.
    pub label: String,

    /// Route to navigate to.
    pub to: R,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Properties for BreadcrumbLink with yew-router support.
#[derive(Properties, PartialEq)]
pub struct BreadcrumbLinkProps<R: Routable + 'static> {
    /// Link content.
    #[prop_or_default]
    pub children: Children,

    /// Route to navigate to.
    pub to: R,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Breadcrumb item with yew-router Link support.
#[function_component]
pub fn BreadcrumbRouteItem<R: Routable + 'static>(props: &BreadcrumbRouteItemProps<R>) -> Html {
    let combined_class = classes!(classes::LINK, props.class.clone());

    html! {
        <li data-slot="breadcrumb-item" class={classes::ITEM}>
            <Link<R> to={props.to.clone()} classes={combined_class}>
                { &props.label }
            </Link<R>>
        </li>
    }
}

/// Breadcrumb link with children and yew-router support.
#[function_component]
pub fn BreadcrumbLink<R: Routable + 'static>(props: &BreadcrumbLinkProps<R>) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let to = props.to.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&to);
        })
    };

    let combined_class = classes!(classes::LINK, props.class.clone());

    html! {
        <li data-slot="breadcrumb-item" class={classes::ITEM}>
            <a data-slot="breadcrumb-link" href={props.to.to_path()} class={combined_class} onclick={onclick}>
                { for props.children.iter() }
            </a>
        </li>
    }
}

/// Simple breadcrumb item (no router dependency).
#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProps) -> Html {
    let link_class = classes!(classes::LINK, props.class.clone());
    let span_class = classes!(classes::PAGE, props.class.clone());

    html! {
        <li data-slot="breadcrumb-item" class={classes::ITEM}>
            {
                if let Some(href) = &props.href {
                    html! { <a data-slot="breadcrumb-link" class={link_class} href={href.clone()}>{ &props.label }</a> }
                } else {
                    html! { <span data-slot="breadcrumb-page" role="link" aria-disabled="true" aria-current="page" class={span_class}>{ &props.label }</span> }
                }
            }
        </li>
    }
}

/// Navigation component for showing hierarchical page location.
///
/// # Example
///
/// ```rust
/// use wonopui_breadcrumb::{Breadcrumb, BreadcrumbItem};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Breadcrumb>
///             <BreadcrumbItem label="Home" href="/" />
///             <BreadcrumbItem label="Products" href="/products" />
///             <BreadcrumbItem label="Widget" />
///         </Breadcrumb>
///     }
/// }
/// ```
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    let separator_icon = props.separator_icon.clone().unwrap_or_else(|| html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m9 18 6-6-6-6"></path>
        </svg>
    });

    html! {
        <nav data-slot="breadcrumb" aria-label="breadcrumb" class={classes!(classes::NAV, props.class.clone())}>
            <ol data-slot="breadcrumb-list" class={classes::LIST}>
                { for props.children.iter().enumerate().map(|(index, child)| {
                    html! {
                        <>
                            { child }
                            { if index < props.children.len() - 1 {
                                html! {
                                    <li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" class={classes::SEPARATOR}>
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
