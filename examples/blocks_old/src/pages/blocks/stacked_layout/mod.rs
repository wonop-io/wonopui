use crate::pages::example_block::ExampleBlock;
use gloo_console as console;
use wonopui::*;
use yew::prelude::*;

#[function_component(StackedLayouts)]
pub fn stacked_layouts() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Stacked Layouts"} />
            </Breadcrumb>
            <ExampleBlock title={"Stacked Layout 1"}>
                <StackedLayout1 />
            </ExampleBlock>
            <ExampleBlock title={"Stacked Layout 2"}>
                <StackedLayout2 />
            </ExampleBlock>
            <ExampleBlock title={"Stacked Layout 3"}>
                <StackedLayout3 />
            </ExampleBlock>
            <ExampleBlock title={"Stacked Layout 4"}>
                <StackedLayout4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(AppSidebar)]
pub fn app_sidebar() -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let mobile_menu_open = layout_context.mobile_menu_open; // Use sidebar_folded from LayoutContext
    let open_mobile_menu = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            println!("Mobile menu open");
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(!mobile_menu_open));
        })
    };
    html! {
        <Sidebar curtain_content={html!{
            <div class="absolute top-2 left-2 cursor-pointer" onclick={open_mobile_menu}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                </svg>
            </div>
        }}>

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
    }
}

#[function_component(StackedLayout1)]
pub fn stacked_layout1() -> Html {
    let user_menu = vec![
        DropdownItemProps {
            label: "Item A".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Item B".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Item C".to_string(),
            ..Default::default()
        },
    ];
    html! {
        <LayoutProvider initial_state={LayoutState { mobile_menu_only: true, ..LayoutState::new() }}>
            <Layout sidebar={html! { <AppSidebar /> }} topbar={html!{
                <Topbar>
                    <Container class="flex justify-between items-center">
                        <div class="flex space-x-2 items-center justify-start">
                            <img class="block h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
                            <MobileMenuButton>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                                </svg>
                            </MobileMenuButton>
                        </div>
                        <div class="hidden lg:block">
                            {"Menu"}
                        </div>
                        <div>
                            <Dropdown items={user_menu} >
                                <button type="button" class="relative flex max-w-xs items-center rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" id="user-menu-button" aria-expanded="false" aria-haspopup="true">
                                    <span class="absolute -inset-1.5"></span>
                                    <span class="sr-only">{"Open user menu"}</span>
                                    <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
                                </button>
                            </Dropdown>
                        </div>
                    </Container>
                </Topbar>
            }}>
                <MainContent>
                    <H1>{"Dashboard"}</H1>
                    <Paragraph>{"Content for Stacked Layout 1"}</Paragraph>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(StackedLayout2)]
pub fn stacked_layout2() -> Html {
    html! {
        <LayoutProvider>
            <Layout topbar={html!{
                <Topbar>
                    <Container class="flex justify-between items-stretch space-x-2 text-sm">
                        <Breadcrumb>
                            <BreadcrumbItem label={"Home"} href="/" />
                            <BreadcrumbItem label={"Page 2"} />
                        </Breadcrumb>
                        <div class="flex flex-0 space-x-2">
                            <input class="border rounded p-2 flex-grow outline-none" placeholder="Search..."/>
                            <Button>
                                {"Sign Up"}
                            </Button>
                        </div>
                    </Container>
                </Topbar>
            }}>
                <MainContent>
                    <p>{"Content for Stacked Layout 2"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(StackedLayout3)]
pub fn stacked_layout3() -> Html {
    html! {
        <LayoutProvider>
            <Layout topbar={html!{
                <Topbar>
                    <Container class="flex justify-between items-stretch space-x-2 py-2" padding_y={false}>
                        <Breadcrumb>
                            <BreadcrumbItem label={"Home"} href="/" />
                            <BreadcrumbItem label={"Page 2"} />
                        </Breadcrumb>
                        <div class="flex flex-0 space-x-2">
                            <input class="border rounded p-2 flex-grow outline-none" placeholder="Search..."/>
                            <Button>
                                {"Sign Up"}
                            </Button>
                        </div>
                    </Container>
                </Topbar>
            }}>
                <MainContent>
                    <p>{"Content for Stacked Layout 2"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(StackedLayout4)]
pub fn stacked_layout4() -> Html {
    html! {
        <LayoutProvider>
            <Layout topbar={html!{
                <Topbar>
                    <div class="w-full flex justify-between space-x-2">
                        <Breadcrumb>
                            <BreadcrumbItem label={"Home"} href="/" />
                            <BreadcrumbItem label={"Page 4"} />
                        </Breadcrumb>
                        <div class="flex flex-0 space-x-2">
                            <input class="border rounded p-2 flex-grow outline-none" placeholder="Search..."/>
                            <Button>
                                {"Help"}
                            </Button>
                        </div>
                    </div>
                </Topbar>
            }}>
                <MainContent>
                    <p>{"Content for Stacked Layout 4"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}
