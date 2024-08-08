use crate::components::multicol_sidebar::{MultiColumnSidebar, SidebarColumn};
use crate::{components::layout_context::LayoutContext, LayoutAction};
use yew::prelude::*; // Import LayoutContext and SidebarPosition
use yew_router::prelude::Link;
use yew_router::Routable;
use yew_router::prelude::use_location;
use gloo_console as console;

#[derive(Properties, PartialEq)]
pub struct SidebarHeadingProps {
    pub title: String,
}

#[function_component(SidebarHeading)]
pub fn sidebar_heading(props: &SidebarHeadingProps) -> Html {
    let SidebarHeadingProps { title } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded; // Use sidebar_folded from LayoutContext
    if folded {
        return html! {};
    }
    html! {
        <h2 class="mt-4 mb-1 px-4 text-lg font-semibold tracking-tight text-zinc-700 dark:text-zinc-300">{title}</h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarLinkProps<R: Routable+'static> {
    pub label: String,
    #[prop_or_default]
    pub icon: Html,
    pub to: R,
}

#[function_component]
pub fn SidebarLink<R: Routable+'static>(props: &SidebarLinkProps<R>) -> Html {
    let SidebarLinkProps { label, icon, to } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;
    let justify = if folded { "justify-center" } else { "justify-start" };
    
    let location = use_location().expect("Failed to get location");
    let is_active = location.path() == to.to_path();

    let mut class = classes!(
        "flex-grow", "mx-3", "px-2", "hover:bg-zinc-200", "hover:dark:bg-zinc-700",
        "inline-flex", "space-x-2", "items-center", "whitespace-nowrap", "rounded-md",
        "text-sm", "font-medium", "transition-colors", "focus-visible:outline-none",
        "focus-visible:ring-1", "focus-visible:ring-ring", "disabled:pointer-events-none",
        "disabled:opacity-50", "hover:bg-accent", "hover:text-accent-foreground",
        "h-9", "px-1.5", "py-1", "text-zinc-700", "dark:text-zinc-300", justify
    );

    if is_active {
        class.push("bg-zinc-200 dark:bg-zinc-700");
    }

    html! {
        <Link<R> to={to.clone()} classes={class}>
            {icon.clone()}
            if !folded {
                <span>{label}</span>
            }
        </Link<R>>
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
    let SidebarItemProps { label, icon, href, onclick, active, children } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;
    let justify = if folded { "justify-center" } else { "justify-start" };
    let has_action = label.is_some() || onclick.is_some();
    let mut class = classes!(
        "flex-grow", "mx-3", "px-2", 
        "inline-flex", "space-x-2", "items-center", "whitespace-nowrap", "rounded-md",
        "text-sm", "font-medium", "transition-colors", "focus-visible:outline-none",
        "focus-visible:ring-1", "focus-visible:ring-ring", "disabled:pointer-events-none",
        "disabled:opacity-50", 
        "h-9", "px-1.5", "py-1", "text-zinc-700", "dark:text-zinc-300", justify
    );

    if has_action {
        class.push("hover:bg-zinc-200");
        class.push("hover:dark:bg-zinc-700");
        class.push("hover:bg-accent");
        class.push("hover:text-accent-foreground");
    }

    if *active {
        class.push("bg-zinc-200 dark:bg-zinc-700");
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
    html! {
        <div class="space-y-1 my-1 flex flex-col items-stretch">
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
}

#[function_component(SidebarHeader)]
pub fn sidebar_header(props: &SidebarHeaderProps) -> Html {
    html! {
        <div class="h-16 shrink-0 border-b border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900">
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
    html! {
        <div class="border-t border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900">
            {for props.children.iter()}
        </div>
    }
}
