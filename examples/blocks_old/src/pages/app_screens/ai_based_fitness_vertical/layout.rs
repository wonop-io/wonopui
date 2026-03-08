use wonopui::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AIFitnessLayoutProps {
    #[prop_or_default]
    pub initial_state: LayoutState,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AIFitnessSidebar)]
pub fn ai_fitness_sidebar() -> Html {
    html! {
        <Sidebar
            header={html!{
                <SidebarHeader>
                    <div class="flex items-center justify-center px-4 py-2 w-full h-16 text-center">
                        <H1>{"AI Fitness"}</H1>
                    </div>
                </SidebarHeader>
            }}
            footer={html!{
                <SidebarFooter>
                    <div class="flex items-center space-x-4 items-center justify-center px-4 py-2 hover:bg-zinc-200 hover:dark:bg-zinc-700 w-full">
                        <Avatar size={AvatarSize::Small} src="/assets/profile_lowres.png" alt="User" />
                        <div class="flex flex-grow flex-col space-y-1">
                            <span>{"John Doe"}</span>
                            <span class="text-xs text-zinc-500">{"john.doe@example.com"}</span>
                        </div>
                    </div>
                </SidebarFooter>
            }}>

            <SidebarHeading title={"Program"} />
            <SidebarMenu>
                <SidebarItem label={"Daily Program"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M12 2v20M2 12h20"></path>
                    </svg>
                }} />
                <SidebarItem label={"Workout Timer"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <circle cx="12" cy="12" r="10"></circle>
                        <path d="M12 6v6l4 2"></path>
                    </svg>
                }} />
                <SidebarItem label={"Progress Tracking"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M3 12h18M3 6h18M3 18h18"></path>
                    </svg>
                }} />
                <SidebarItem label={"Exercise Library"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M4 4h16v16H4z"></path>
                    </svg>
                }} />
                <SidebarItem label={"Nutrition Plan"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M12 2v20M2 12h20"></path>
                    </svg>
                }} />
                <SidebarItem label={"Goal Setting"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M12 2v20M2 12h20"></path>
                    </svg>
                }} />
                <SidebarItem label={"Activity Log"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M12 2v20M2 12h20"></path>
                    </svg>
                }} />
                <SidebarItem label={"Community"} icon={html!{
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                        <path d="M12 2v20M2 12h20"></path>
                    </svg>
                }} />
            </SidebarMenu>
        </Sidebar>
    }
}

#[function_component(AIFitnessLayout)]
pub fn ai_fitness_layout(props: &AIFitnessLayoutProps) -> Html {
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
            <Layout sidebar={html! { <AIFitnessSidebar /> }} topbar={html!{
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
