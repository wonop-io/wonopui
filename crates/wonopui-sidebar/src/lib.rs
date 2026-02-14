//! Sidebar component for wonopui
//!
//! A navigation sidebar that can be positioned on the left or right,
//! and can be folded/expanded.

use wonopui_core::merge_classes;
use wonopui_layout::{LayoutContext, LayoutAction};
use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator};
use yew_router::Routable;

pub mod classes {
    pub const SIDEBAR_CONTAINER: &str = "fixed inset-y-0 z-50 flex flex-col bg-white dark:bg-zinc-900 border-r border-zinc-200 dark:border-zinc-800 transition-all duration-300";
    pub const SIDEBAR_LEFT: &str = "left-0";
    pub const SIDEBAR_RIGHT: &str = "right-0 border-r-0 border-l";
    pub const SIDEBAR_STANDARD: &str = "w-72";
    pub const SIDEBAR_FOLDED: &str = "w-18";
    pub const SIDEBAR_HIDDEN: &str = "hidden lg:flex";
    
    pub const SIDEBAR_HEADER: &str = "flex h-16 shrink-0 items-center px-6 border-b border-zinc-200 dark:border-zinc-800";
    pub const SIDEBAR_CONTENT: &str = "flex-1 flex flex-col gap-y-5 overflow-y-auto px-6 py-4";
    pub const SIDEBAR_FOOTER: &str = "flex shrink-0 items-center px-6 py-4 border-t border-zinc-200 dark:border-zinc-800";
    
    pub const SIDEBAR_HEADING: &str = "text-xs font-semibold leading-6 text-gray-400 dark:text-zinc-500 uppercase tracking-wider";
    pub const SIDEBAR_NAV: &str = "flex flex-col gap-y-1";
    pub const SIDEBAR_MENU: &str = "flex flex-col gap-y-1";
    pub const SIDEBAR_ITEM: &str = "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-zinc-300 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-gray-50 dark:hover:bg-zinc-800";
    pub const SIDEBAR_ITEM_ACTIVE: &str = "bg-gray-50 dark:bg-zinc-800 text-blue-600 dark:text-blue-400";
    pub const SIDEBAR_ITEM_ICON: &str = "h-6 w-6 shrink-0";
    pub const SIDEBAR_LINK: &str = "group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-700 dark:text-zinc-300 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-gray-50 dark:hover:bg-zinc-800 cursor-pointer";
}

#[derive(Clone, PartialEq, Default)]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
}

impl SidebarPosition {
    pub fn to_class(&self) -> &'static str {
        match self {
            SidebarPosition::Left => classes::SIDEBAR_LEFT,
            SidebarPosition::Right => classes::SIDEBAR_RIGHT,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub position: SidebarPosition,
    #[prop_or_default]
    pub folded: bool,
    #[prop_or_default]
    pub hidden_on_mobile: bool,
    /// Optional header content
    #[prop_or_default]
    pub header: Option<Html>,
    /// Optional footer content
    #[prop_or_default]
    pub footer: Option<Html>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let size_class = if props.folded {
        classes::SIDEBAR_FOLDED
    } else {
        classes::SIDEBAR_STANDARD
    };

    let class = merge_classes(&[
        classes::SIDEBAR_CONTAINER,
        props.position.to_class(),
        size_class,
        if props.hidden_on_mobile { classes::SIDEBAR_HIDDEN } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            if let Some(header) = &props.header {
                { header.clone() }
            }
            <nav class={classes::SIDEBAR_CONTENT}>
                { for props.children.iter() }
            </nav>
            if let Some(footer) = &props.footer {
                { footer.clone() }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarHeader)]
pub fn sidebar_header(props: &SidebarHeaderProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_HEADER,
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarContent)]
pub fn sidebar_content(props: &SidebarContentProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_CONTENT,
        &props.class.to_string(),
    ]);

    html! {
        <nav class={class}>
            { for props.children.iter() }
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarFooter)]
pub fn sidebar_footer(props: &SidebarFooterProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_FOOTER,
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarHeadingProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarHeading)]
pub fn sidebar_heading(props: &SidebarHeadingProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_HEADING,
        &props.class.to_string(),
    ]);

    html! {
        <h2 class={class}>
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarNavProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarNav)]
pub fn sidebar_nav(props: &SidebarNavProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_NAV,
        &props.class.to_string(),
    ]);

    html! {
        <ul class={class} role="list">
            { for props.children.iter() }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub icon: Option<Html>,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub href: Option<String>,
}

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_ITEM,
        if props.active { classes::SIDEBAR_ITEM_ACTIVE } else { "" },
        &props.class.to_string(),
    ]);

    let content = html! {
        <>
            if let Some(icon) = &props.icon {
                <span class={classes::SIDEBAR_ITEM_ICON}>
                    { icon.clone() }
                </span>
            }
            { for props.children.iter() }
        </>
    };

    if let Some(href) = &props.href {
        html! {
            <li>
                <a href={href.clone()} class={class}>
                    { content }
                </a>
            </li>
        }
    } else {
        html! {
            <li>
                <button class={class} onclick={props.onclick.clone()}>
                    { content }
                </button>
            </li>
        }
    }
}

// SidebarMenu component
#[derive(Properties, PartialEq)]
pub struct SidebarMenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarMenu)]
pub fn sidebar_menu(props: &SidebarMenuProps) -> Html {
    let class = merge_classes(&[
        classes::SIDEBAR_MENU,
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { for props.children.iter() }
        </div>
    }
}

// SidebarLink component - generic over router types
#[derive(Properties, PartialEq)]
pub struct SidebarLinkProps<R: Routable + 'static> {
    pub label: String,
    #[prop_or_default]
    pub icon: Html,
    pub to: R,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn SidebarLink<R: Routable + 'static>(props: &SidebarLinkProps<R>) -> Html {
    let layout_context = use_context::<LayoutContext>();
    let folded = layout_context.as_ref().map(|ctx| ctx.sidebar_folded).unwrap_or(false);
    
    let justify = if folded {
        "justify-center"
    } else {
        "justify-start"
    };

    let navigator = use_navigator().unwrap();
    let onclick = {
        let layout_context = layout_context.clone();
        let to = props.to.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ref ctx) = layout_context {
                ctx.dispatch(LayoutAction::SetMobileMenuOpen(false));
            }
            navigator.push(&to);
        })
    };

    let location = use_location().expect("Failed to get location");
    let is_active = location.path() == props.to.to_path();

    let class = merge_classes(&[
        classes::SIDEBAR_LINK,
        justify,
        if is_active { classes::SIDEBAR_ITEM_ACTIVE } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <button class={class} onclick={onclick}>
            { props.icon.clone() }
            if !folded {
                <span>{ &props.label }</span>
            }
        </button>
    }
}
