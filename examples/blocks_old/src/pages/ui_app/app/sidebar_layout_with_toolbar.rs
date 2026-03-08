use wonopui::{
    Layout, LayoutAction, LayoutContext, LayoutProvider, Sidebar, SidebarHeading, SidebarItem,
    SidebarMenu,
};
use yew::prelude::*;

#[function_component(SidebarExample)]
pub fn sidebar_example() -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded; // Use sidebar_folded from LayoutContext
    let fold = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            println!("Folded");
            layout_context.dispatch(LayoutAction::SetSidebarFolded(!folded));
        })
    };

    let mobile_menu_open = layout_context.mobile_menu_open; // Use sidebar_folded from LayoutContext
    let open_mobile_menu = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            println!("Mobile menu open");
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(!mobile_menu_open));
        })
    };

    let sidebar = html! {
        <Sidebar>
        <button onclick={fold}>{"Fold"}</button>
        <button class="absolute top-4 right-4 lg:hidden" onclick={open_mobile_menu.clone()}>{"X"}</button>
        <SidebarHeading title={"Discover"} />
        <SidebarMenu>
            <SidebarItem label={"Listen Now"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <circle cx="12" cy="12" r="10"></circle>
                    <polygon points="10 8 16 12 10 16 10 8"></polygon>
                </svg>
            }} />
            <SidebarItem label={"Browse"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <rect width="7" height="7" x="3" y="3" rx="1"></rect>
                    <rect width="7" height="7" x="14" y="3" rx="1"></rect>
                    <rect width="7" height="7" x="14" y="14" rx="1"></rect>
                    <rect width="7" height="7" x="3" y="14" rx="1"></rect>
                </svg>
            }} />
            <SidebarItem label={"Radio"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9"></path>
                    <path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5"></path>
                    <circle cx="12" cy="12" r="2"></circle>
                    <path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5"></path>
                    <path d="M19.1 4.9C23 8.8 23 15.1 19.1 19"></path>
                </svg>
            }} />
        </SidebarMenu>
        <SidebarHeading title={"Library"} />
        <SidebarMenu>
            <SidebarItem label={"Playlists"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <path d="M21 15V6"></path>
                    <path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z"></path>
                    <path d="M12 12H3"></path>
                    <path d="M16 6H3"></path>
                    <path d="M12 18H3"></path>
                </svg>
            }} />
            <SidebarItem label={"Songs"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <circle cx="8" cy="18" r="4"></circle>
                    <path d="M12 18V2l7 4"></path>
                </svg>
            }} />
            <SidebarItem label={"Made for You"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                    <circle cx="12" cy="7" r="4"></circle>
                </svg>
            }} />
            <SidebarItem label={"Artists"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <path d="m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12"></path>
                    <circle cx="17" cy="7" r="5"></circle>
                </svg>
            }} />
            <SidebarItem label={"Albums"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4">
                    <path d="m16 6 4 14"></path>
                    <path d="M12 6v14"></path>
                    <path d="M8 8v12"></path>
                    <path d="M4 4v16"></path>
                </svg>
            }} />
        </SidebarMenu>
        </Sidebar>
    };

    html! {
        <Layout sidebar={Some(sidebar)}>
            <button onclick={open_mobile_menu}>{"Open mobile menu"}</button>
            <h2 class="text-2xl">{"Hello x"}</h2>
        </Layout>
    }
}

#[function_component(SidebarLayoutWithToolbar)]
pub fn sidebar_layout_with_toolbar() -> Html {
    html! {
        <LayoutProvider>
            <SidebarExample />
        </LayoutProvider>
    }
}
