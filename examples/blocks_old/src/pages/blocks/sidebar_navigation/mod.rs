use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(SidebarNavigation)]
pub fn sidebar_navigation() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Sidebar Navigation"} />
            </Breadcrumb>
            <ExampleBlock title={"Sidebar Navigation 1"}>
                <SidebarNavigation1 />
            </ExampleBlock>
            <ExampleBlock title={"Sidebar Navigation 2"}>
                <SidebarNavigation2 />
            </ExampleBlock>
            <ExampleBlock title={"Sidebar Navigation 3"}>
                <SidebarNavigation3 />
            </ExampleBlock>
            <ExampleBlock title={"Sidebar Navigation 4"}>
                <SidebarNavigation4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(SidebarNavigation1)]
pub fn sidebar_navigation1() -> Html {
    html! {
        <LayoutProvider>
            <Layout sidebar={html!{
                <Sidebar>
                    <ul class="space-y-2">
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Home"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Profile"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Settings"}</a></li>
                    </ul>
                </Sidebar>
            }}>
                <MainContent>
                    <p>{"Content for Sidebar Navigation 1"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(SidebarNavigation2)]
pub fn sidebar_navigation2() -> Html {
    html! {
        <LayoutProvider>
            <Layout sidebar={html!{
                <Sidebar>
                    <ul class="space-y-2">
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Dashboard"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Messages"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Notifications"}</a></li>
                    </ul>
                </Sidebar>
            }}>
                <MainContent>
                    <p>{"Content for Sidebar Navigation 2"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(SidebarNavigation3)]
pub fn sidebar_navigation3() -> Html {
    html! {
        <LayoutProvider>
            <Layout sidebar={html!{
                <Sidebar>
                    <ul class="space-y-2">
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Overview"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Reports"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Analytics"}</a></li>
                    </ul>
                </Sidebar>
            }}>
                <MainContent>
                    <p>{"Content for Sidebar Navigation 3"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(SidebarNavigation4)]
pub fn sidebar_navigation4() -> Html {
    html! {
        <LayoutProvider>
            <Layout sidebar={html!{
                <Sidebar>
                    <ul class="space-y-2">
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Projects"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Tasks"}</a></li>
                        <li><a href="#" class="block p-2 rounded hover:bg-gray-200">{"Calendar"}</a></li>
                    </ul>
                </Sidebar>
            }}>
                <MainContent>
                    <p>{"Content for Sidebar Navigation 4"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}
