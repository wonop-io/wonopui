mod campaign_list;
mod dashboard;
mod layout1;
mod product_list;

use crate::pages::example_block::ExampleBlock;
use campaign_list::CampaignList;
use dashboard::Dashboard;
use gloo_console as console;
use layout1::AppLayout;
use product_list::ProductList;
use wonopui::*;
use yew::prelude::*;

#[function_component(HomeScreens)]
pub fn home_screens() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Sidebar Layouts"} />
            </Breadcrumb>
            <ExampleBlock title={"Dashboard"}>
                <Dashboard />
            </ExampleBlock>
            <ExampleBlock title={"Product Lists"}>
                <ProductList />
            </ExampleBlock>
            <ExampleBlock title={"Campaign List"}>
                <CampaignList />
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
