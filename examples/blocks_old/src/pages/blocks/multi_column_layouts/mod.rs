mod layout1;

use crate::pages::example_block::ExampleBlock;
use gloo_console as console;
use wonopui::*;
use yew::prelude::*;

use layout1::Layout1;

#[function_component(MultiColumnLayouts)]
pub fn multi_column_layouts() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Sidebar Layouts"} />
            </Breadcrumb>
            <ExampleBlock title={"Layout 1"}>
                <Layout1 initial_state={LayoutState { standard_menu_size:550,  sidebar_folded: false, ..LayoutState::new() }} />
            </ExampleBlock>
            <ExampleBlock title={"Layout 2"}>
                <Layout1 initial_state={LayoutState { standard_menu_size:550,sidebar_folded: true, ..LayoutState::new() }} />
            </ExampleBlock>
            <ExampleBlock title={"Layout 3"}>
                <Layout1 initial_state={LayoutState { standard_menu_size:550, sidebar_position: SidebarPosition::Right, ..LayoutState::new() }} />
            </ExampleBlock>

        </MainContent>
    }
}

#[function_component(Layout2)]
pub fn layout2() -> Html {
    html! {
        <LayoutProvider>
            <Layout sidebar={html!{
                <Sidebar>
                    <p>{"Hello World"}</p>
                </Sidebar>
            }} topbar={html!{
                <Topbar>
                    <div class="w-full flex justify-between space-x-2">
                        <Breadcrumb>
                            <BreadcrumbItem label={"Dashboard"} href="/" / >
                            <BreadcrumbItem label={"Users"} / >

                        </Breadcrumb>
                        <div class="flex flex-0 space-x-2">
                        <input class="border rounded p-2 flex-grow outline-none" placeholder="Search for ..."/>
                        <Button>
                            {"Login"}
                        </Button>
                        </div>
                    </div>
                </Topbar>
            }}>
                <MainContent>
                    <p>{"Hello World"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}

#[function_component(Layout3)]
pub fn layout3() -> Html {
    html! {
        <LayoutProvider>
            <Layout sidebar={html!{
                <Sidebar>
                    <p>{"Hello World"}</p>
                </Sidebar>
            }}>
                <MainContent>
                    <p>{"Hello World"}</p>
                </MainContent>
            </Layout>
        </LayoutProvider>
    }
}
