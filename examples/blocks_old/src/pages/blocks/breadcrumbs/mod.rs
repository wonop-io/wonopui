use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(BreadcrumbExample1)]
pub fn breadcrumb_example1() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbItem label={"Home"} href="/" />
            <BreadcrumbItem label={"Library"} href="/library" />
            <BreadcrumbItem label={"Data"} />
        </Breadcrumb>
    }
}

#[function_component(BreadcrumbExample2)]
pub fn breadcrumb_example2() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbItem label={"Dashboard"} href="/" />
            <BreadcrumbItem label={"Settings"} href="/settings" />
            <BreadcrumbItem label={"Profile"} />
        </Breadcrumb>
    }
}

#[function_component(BreadcrumbExample3)]
pub fn breadcrumb_example3() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbItem label={"Shop"} href="/" />
            <BreadcrumbItem label={"Electronics"} href="/electronics" />
            <BreadcrumbItem label={"Mobile Phones"} />
        </Breadcrumb>
    }
}

#[function_component(BreadcrumbExample4)]
pub fn breadcrumb_example4() -> Html {
    html! {
        <Breadcrumb>
            <BreadcrumbItem label={"Projects"} href="/" />
            <BreadcrumbItem label={"Project A"} href="/project-a" />
            <BreadcrumbItem label={"Details"} />
        </Breadcrumb>
    }
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Breadcrumbs"} />
            </Breadcrumb>
            <ExampleBlock title={"Breadcrumb Example 1"} isolate={true}>
                <BreadcrumbExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Breadcrumb Example 2"} isolate={true}>
                <BreadcrumbExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Breadcrumb Example 3"} isolate={true}>
                <BreadcrumbExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Breadcrumb Example 4"} isolate={true}>
                <BreadcrumbExample4 />
            </ExampleBlock>
        </MainContent>
    }
}
