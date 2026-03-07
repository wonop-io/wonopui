use crate::pages::example_block::ExampleBlock;
use gloo_console as console;
use wonopui::*;
use yew::prelude::*;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    html! {
        <SidebarColumn header={html!{
            <SidebarHeaderWithLogo />
        }}
        >
        <SidebarHeading title={"Discover"}  />
        <SidebarMenu>
            <SidebarItem label={"Listen Now"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <circle cx="12" cy="12" r="10"></circle>
                    <polygon points="10 8 16 12 10 16 10 8"></polygon>
                </svg>
            }} />
            <SidebarItem label={"Browse"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <rect width="7" height="7" x="3" y="3" rx="1"></rect>
                    <rect width="7" height="7" x="14" y="3" rx="1"></rect>
                    <rect width="7" height="7" x="14" y="14" rx="1"></rect>
                    <rect width="7" height="7" x="3" y="14" rx="1"></rect>
                </svg>
            }} />
            <SidebarItem label={"Radio"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9"></path>
                    <path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5"></path>
                    <circle cx="12" cy="12" r="2"></circle>
                    <path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5"></path>
                    <path d="M19.1 4.9C23 8.8 23 15.1 19.1 19"></path>
                </svg>
            }} />
        </SidebarMenu>
        <SidebarHeading title={"Library"}  />
        <SidebarMenu>
            <SidebarItem label={"Playlists"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <path d="M21 15V6"></path>
                    <path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z"></path>
                    <path d="M12 12H3"></path>
                    <path d="M16 6H3"></path>
                    <path d="M12 18H3"></path>
                </svg>
            }} />
            <SidebarItem label={"Songs"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <circle cx="8" cy="18" r="4"></circle>
                    <path d="M12 18V2l7 4"></path>
                </svg>
            }} />
            <SidebarItem label={"Made for You"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                    <circle cx="12" cy="7" r="4"></circle>
                </svg>
            }} />
            <SidebarItem label={"Artists"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <path d="m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12"></path>
                    <circle cx="17" cy="7" r="5"></circle>
                </svg>
            }} />
            <SidebarItem label={"Albums"} icon={html!{
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                    <path d="m16 6 4 14"></path>
                    <path d="M12 6v14"></path>
                    <path d="M8 8v12"></path>
                    <path d="M4 4v16"></path>
                </svg>
            }} />
        </SidebarMenu>
    </SidebarColumn>
    }
}

#[function_component(DesignChoiceMenu)]
pub fn design_choice_menu() -> Html {
    html! {
        <SidebarColumn hide_when_folded={true}>
            <Container class="flex flex-col items-strecth h-full">
            <GroupButton default_value="layout" class="inline-block">
                <GroupButtonTrigger value="layout">
                    {"Layout"}
                </GroupButtonTrigger>
                <GroupButtonTrigger value="icon">
                    {"Icon"}
                </GroupButtonTrigger>
            </GroupButton>
            <div class="flex-1 flex-grow overflow-y-auto">
                <div class="flex flex-col space-y-4">
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                <Card class="w-full h-48 bg-zinc-700 cursor-pointer">
                </Card>
                </div>
            </div>
            </Container>
        </SidebarColumn>
    }
}

#[function_component(SidebarHeaderWithLogo)]
pub fn sidebar_header_with_logo() -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded; // Use

    match folded {
        true => html! {
            <SidebarHeader>
                <div class="flex items-center justify-center px-4 py-2 w-full h-16 text-center">
                    <H1>{"A"}</H1>
                </div>
            </SidebarHeader>
        },
        false => html! {
            <SidebarHeader>
            <div class="flex items-center justify-between px-4 py-2 w-full h-16">
                <h1 class="text-xl font-bold">{"App Header"}</h1>
            </div>
        </SidebarHeader>
        },
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
        <MultiColumnSidebar
            curtain_content={html!{
                <div class="absolute top-2 left-2 cursor-pointer" onclick={open_mobile_menu}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                    </svg>
                </div>
            }}>
            <MainMenu />
            <DesignChoiceMenu />

        </MultiColumnSidebar>
    }
}
#[derive(Properties, PartialEq)]
pub struct Layout1Props {
    pub initial_state: LayoutState,
}

#[function_component(Layout1)]
pub fn layout1(props: &Layout1Props) -> Html {
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
        <LayoutProvider initial_state={props.initial_state.clone()}>
            <Layout sidebar={html! { <AppSidebar /> }} topbar={html!{
                <Topbar>
                    <Container class="flex justify-between items-center space-x-4 lg:space-x-0 py-2 dark:bg-zinc-900" padding_y={false}>
                        <div class="flex flex space-x-2 items-center justify-start block lg:hidden">
                            <MobileMenuButton>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 dark:stroke-zinc-300">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                                </svg>
                            </MobileMenuButton>
                        </div>

                        <div class="flex-grow flex rounded px-2 lg:px-0 py-1 space-x-2 items-center justify-start">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4 dark:stroke-zinc-300">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
                            </svg>
                            <input class="outline-none w-full bg-transparent dark:text-zinc-300" placeholder="Search..."/>
                        </div>

                        <div class="flex space-x-2">
                            <Dropdown items={user_menu} position={PopoverPosition::SouthEnd}>
                                <Avatar src="/assets/profile_lowres.png" size={AvatarSize::Small}/>
                            </Dropdown>

                        </div>
                    </Container>
                </Topbar>
            }}>
                <MainContent class="bg-white dark:bg-zinc-900">

                    <div class="relative overflow-hidden rounded-md border h-[100%] text-zinc-700 dark:text-zinc-300 dark:border-zinc-300 flex justify-center items-center">
                        <svg class="absolute inset-0 h-full w-full stroke-gray-900/10 dark:stroke-zinc-200/10" fill="none">
                        <defs>
                            <pattern id="dash" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
                                <line x1="0" y1="0" x2="0" y2="10" />
                            </pattern>
                        </defs>
                        <rect stroke="none" fill="url(#dash)" width="100%" height="100%"></rect>
                        </svg>
                        <p class="p-2 z-10 bg-white dark:bg-zinc-800 rounded-md">{"Content placeholder"}</p>
                    </div>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}
