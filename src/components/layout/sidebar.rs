use crate::components::layout::layout_context::{LayoutAction, LayoutContext};
use crate::components::layout::multicol_sidebar::{MultiColumnSidebar, SidebarColumn};
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*; // Import LayoutContext and SidebarPosition
use yew_router::prelude::use_location;
use yew_router::prelude::use_navigator;
use yew_router::prelude::Link;
use yew_router::Routable;

#[derive(Properties, PartialEq)]
pub struct SidebarHeadingProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(SidebarHeading)]
pub fn sidebar_heading(props: &SidebarHeadingProps) -> Html {
    let SidebarHeadingProps { children } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded; // Use sidebar_folded from LayoutContext
    if folded {
        return html! {};
    }
    
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    
    html! {
        <h2 class={brandguide.sidebar_heading.to_string()}>
            {children}
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarLinkProps<R: Routable + 'static> {
    pub label: String,
    #[prop_or_default]
    pub icon: Html,
    pub to: R,
}

#[function_component]
pub fn SidebarLink<R: Routable + 'static>(props: &SidebarLinkProps<R>) -> Html {
    let SidebarLinkProps { label, icon, to } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;
    let justify = if folded {
        "justify-center"
    } else {
        "justify-start"
    };

    let navigator = use_navigator().unwrap();
    let onclick = {
        let layout_context = layout_context.clone();
        let to = to.clone();
        Callback::from(move |e: MouseEvent| {
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(false));
            navigator.push(&to)
        })
    };

    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    
    let location = use_location().expect("Failed to get location");
    let is_active = location.path() == to.to_path();

    let mut class = classes!(
        &brandguide.sidebar_link_base,
        justify
    );

    if is_active {
        class.push(&brandguide.sidebar_link_active);
    }

    html! {
        <button class={class} onclick={onclick.clone()}>
            {icon.clone()}
            if !folded {
                <span>{label}</span>
            }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub icon: Html,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let SidebarItemProps {
        label,
        icon,
        href,
        onclick,
        active,
        children,
    } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;
    let justify = if folded {
        "justify-center"
    } else {
        "justify-start"
    };
    let has_action = label.is_some() || onclick.is_some();

    let orig_onclick = onclick.clone();
    let onclick = {
        let layout_context = layout_context.clone();
        Callback::from(move |e: MouseEvent| {
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(false));
            if let Some(onclick) = &orig_onclick {
                onclick.emit(e);
            }
        })
    };
    
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    
    let mut class = classes!(
        &brandguide.sidebar_item_base,
        justify
    );

    if has_action {
        class.push(&brandguide.sidebar_item_hover);
    }

    if *active {
        class.push(&brandguide.sidebar_item_active);
    }

    let content = html! {
        <>
            {icon.clone()}
            if !folded {
                if let Some(label) = label {
                    <span>
                        {label}
                    </span>
                }
                {children}
            }
        </>
    };

    match href {
        Some(href) => html! {
            <a href={href.clone()} class={class} onclick={onclick.clone()}>
                {content}
            </a>
        },
        None => html! {
            <button class={class} onclick={onclick.clone()}>
                {content}
            </button>
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarMenuProps {
    pub children: Children,
}

#[function_component(SidebarMenu)]
pub fn sidebar_menu(props: &SidebarMenuProps) -> Html {
    let SidebarMenuProps { children } = props;
    
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    
    html! {
        <div class={brandguide.sidebar_menu.to_string()}>
            {children}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub curtain_content: Html,
    #[prop_or_default]
    pub header: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    html! {
      <MultiColumnSidebar curtain_content={props.curtain_content.clone()}>
        <SidebarColumn header={props.header.clone()} footer={props.footer.clone()}>
          {props.children.clone()}
        </SidebarColumn>
      </MultiColumnSidebar>
    }
}

#[derive(Properties, PartialEq)]
pub struct MobileMenuButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(MobileMenuButton)]
pub fn mobile_menu_button(props: &MobileMenuButtonProps) -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let mobile_menu_open = layout_context.mobile_menu_open; // Use sidebar_folded from LayoutContext
    let open_mobile_menu = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(!mobile_menu_open));
        })
    };
    // TODO: Use Button instead
    html! {
        <button class={classes!("lg:hidden", props.class.clone())} onclick={open_mobile_menu}>
            {props.children.clone()}
        </button>
    }
}
#[derive(Properties, PartialEq)]
pub struct SidebarHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarHeader)]
pub fn sidebar_header(props: &SidebarHeaderProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    
    let default_classes = classes!(
        &brandguide.sidebar_header
    );

    let combined_classes = classes!(default_classes, props.class.clone());

    html! {
        <div
            class={combined_classes}
            onclick={props.onclick.clone()}
        >
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarFooterProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(SidebarFooter)]
pub fn sidebar_footer(props: &SidebarFooterProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    
    html! {
        <div class={brandguide.sidebar_footer.to_string()}>
            {for props.children.iter()}
        </div>
    }
}
