use crate::components::multicol_sidebar::{MultiColumnSidebar, SidebarColumn};
use crate::{components::layout_context::LayoutContext, LayoutAction};
use yew::prelude::*; // Import LayoutContext and SidebarPosition

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
        <h2 class="my-6 px-4 text-lg font-semibold tracking-tight text-zinc-700 dark:text-zinc-300">{title}</h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    pub label: String,
    #[prop_or_default]
    pub icon: Html,
}

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let SidebarItemProps { label, icon } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded; // Use sidebar_folded from LayoutContext
    let justify = if folded {
        "justify-center"
    } else {
        "justify-start"
    };
    html! {
        <button class={format!("{} flex-grow mx-3 px-2 hover:bg-zinc-200 hover:dark:bg-zinc-700  inline-flex space-x-2 items-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 px-1.5 py-1 text-zinc-700 dark:text-zinc-300", justify)}>
            {icon.clone()}
            if !folded {
                <span>{label}</span>
            }
        </button>
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
        <div class="space-y-1 my-6 flex flex-col items-stretch">
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
