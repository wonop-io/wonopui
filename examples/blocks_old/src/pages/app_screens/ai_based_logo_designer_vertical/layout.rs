use wonopui::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AILogoDesignerLayoutProps {
    #[prop_or_default]
    pub initial_state: LayoutState,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(SidebarHeaderWithLogo)]
pub fn sidebar_header_with_logo() -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded; // Use

    match folded {
        true => html! {
            <SidebarHeader>
                <div class="flex items-center justify-center px-4 py-2 w-full h-16 text-center">
                    <H1>{"L"}</H1>
                </div>
            </SidebarHeader>
        },
        false => html! {
            <SidebarHeader>
                <div class="flex items-center justify-center px-4 py-2 w-full h-16">
                    <H1>{"Logo M8"}</H1>
                </div>
            </SidebarHeader>
        },
    }
}

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    html! {
        <SidebarColumn
            header={html!{
                <SidebarHeaderWithLogo />
            }}
            footer={html!{
                <SidebarFooter>
                    <div class="flex items-center space-x-4 items-center justify-center px-4 py-2 hover:bg-zinc-200 hover:dark:bg-zinc-700 w-full cursor-pointer h-16">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                        </svg>
                        <div class="flex flex-grow flex-col space-y-1">
                            <span>{"Settings"}</span>
                        </div>
                    </div>
                </SidebarFooter>
            }}>

            <SidebarMenu>
                <SidebarItem label={"Preview"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M9.316,4.52a.67.67,0,0,1,0,.922C8.586,6.23,6.928,7.75,5,7.75S1.414,6.23.684,5.442a.67.67,0,0,1,0-.922A6.485,6.485,0,0,1,5,2.25,6.485,6.485,0,0,1,9.316,4.52Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M3.750 5.000 A1.250 1.250 0 1 0 6.250 5.000 A1.250 1.250 0 1 0 3.750 5.000 Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Suggested"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M7.75,5.75A2.75,2.75,0,1,0,4,8.312V9a.5.5,0,0,0,.5.5h1A.5.5,0,0,0,6,9V8.312A2.751,2.751,0,0,0,7.75,5.75Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M5 0.5L5 1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M2.022 1L2.75 1.81" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M0.5 3.5L1.5 3.5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M7.978 1L7.25 1.81" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M9.5 3.5L8.5 3.5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Layout"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M4.681,4.929.73,3.114a.388.388,0,0,1,0-.7L4.67.572a.775.775,0,0,1,.649,0L9.274,2.386a.389.389,0,0,1,0,.705L5.33,4.928A.775.775,0,0,1,4.681,4.929Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M9.495,5.245,5.425,7.139a1,1,0,0,1-.839,0L.5,5.264" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M9.495,7.512l-4.07,1.9a1,1,0,0,1-.839,0L.5,7.531" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Palette"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M8,6.5a3,3,0,0,1-6,0c0-1.657,3-6,3-6S8,4.843,8,6.5Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Background"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M0.5 9.5L5.028 4.972" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M8.984,1.361,8.362,3.092a.27.27,0,0,0,.034.254L9.448,4.775a.288.288,0,0,1-.236.449L7.4,5.243a.293.293,0,0,0-.233.122L6.083,6.871a.274.274,0,0,1-.49-.067l-.5-1.72a.26.26,0,0,0-.179-.179l-1.72-.5a.274.274,0,0,1-.067-.49L4.635,2.835A.293.293,0,0,0,4.757,2.6L4.776.788A.288.288,0,0,1,5.225.552L6.654,1.6a.27.27,0,0,0,.254.034l1.731-.622A.267.267,0,0,1,8.984,1.361Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Name"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M2.5,7.5,4.538,2.608a.5.5,0,0,1,.924,0L7.5,7.5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M3.387 5.37L6.613 5.37" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M0.500 0.500 L9.500 0.500 L9.500 9.500 L0.500 9.500 Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Slogan"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M0.5 8.5L3.5 1.5 5 5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M1.66 5.793L4.05 5.793" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M9.5 8.5L7.5 4 5.5 8.5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M5.944 7.5L9.056 7.5" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Symbol"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M5.34.909l1.1,2.224a.379.379,0,0,0,.286.208L9.175,3.7a.379.379,0,0,1,.21.647L7.61,6.077a.381.381,0,0,0-.109.336L7.92,8.858a.379.379,0,0,1-.551.4L5.177,8.1a.382.382,0,0,0-.354,0L2.631,9.258a.379.379,0,0,1-.551-.4L2.5,6.413a.381.381,0,0,0-.109-.336L.615,4.345A.379.379,0,0,1,.825,3.7l2.452-.357a.379.379,0,0,0,.286-.208L4.66.909A.379.379,0,0,1,5.34.909Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Container"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M0.500 5.000 A4.500 4.500 0 1 0 9.500 5.000 A4.500 4.500 0 1 0 0.500 5.000 Z" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"History"} icon={html!{
                    <svg viewBox="0 0 14 14" width="14" height="14">
                        <g transform="matrix(1.4,0,0,1.4,0,0)">
                            <path d="M5.25,1A4,4,0,1,1,2.481,2.114" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M0.75 2.5L2.481 2.114 2.868 3.661" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M5.25 3.5L5.25 5 6.25 6" fill="none" stroke-linecap="round" stroke-linejoin="round"></path>
                        </g>
                    </svg>
                }} />
                <SidebarItem label={"Your Logos"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" width="15px" height="13px" viewBox="0 0 15 13" version="1.1">
                        <g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round">
                            <g id="Artboard-Copy-40" transform="translate(-24.000000, -260.000000)" stroke="#none" stroke-width="1.2">
                                <g id="Group-8" transform="translate(11.000000, 73.000000)">
                                    <g id="Group" transform="translate(14.000000, 188.000000)">
                                        <path d="M6.49957077,10.8995818 L0.980570772,5.85258179 C-2.00042923,2.87158179 2.38657077,-2.90841821 6.49957077,1.75258179 C10.6125708,-2.90841821 14.9995708,2.87158179 12.0185708,5.85258179 L6.49957077,10.8995818 Z" id="Path"></path>
                                    </g>
                                </g>
                            </g>
                        </g>
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

#[function_component(AILogoDesignerSidebar)]
pub fn ai_logo_designer_sidebar() -> Html {
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

#[function_component(AILogoDesignerLayout)]
pub fn ai_logo_designer_layout(props: &AILogoDesignerLayoutProps) -> Html {
    let user_menu = vec![
        DropdownItemProps {
            label: "Profile".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Settings".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Logout".to_string(),
            ..Default::default()
        },
    ];
    html! {
        <LayoutProvider initial_state={props.initial_state.clone()}>
            <Layout sidebar={html! { <AILogoDesignerSidebar /> }} topbar={html!{
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
                <MainContent class="bg-white dark:bg-zinc-900 overflow-y-auto h-full">
                    {props.children.clone()}
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}
