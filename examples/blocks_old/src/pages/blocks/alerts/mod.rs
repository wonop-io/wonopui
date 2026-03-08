use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(AlertExample1)]
pub fn alert_example1() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Alert alert_type={AlertType::Success}>
                {"This is a success alert!"}
            </Alert>
        </div>
    }
}

#[function_component(AlertExample2)]
pub fn alert_example2() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Alert alert_type={AlertType::Error}>
                {"This is an error alert!"}
            </Alert>
        </div>
    }
}

#[function_component(AlertExample3)]
pub fn alert_example3() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Alert alert_type={AlertType::Warning}>
                {"This is a warning alert!"}
            </Alert>
        </div>
    }
}

#[function_component(AlertExample4)]
pub fn alert_example4() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Alert alert_type={AlertType::Info}>
                {"This is an info alert!"}
            </Alert>
        </div>
    }
}

#[function_component(Alerts)]
pub fn alerts() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Alerts"} />
            </Breadcrumb>
            <ExampleBlock title={"Success Alert"}>
                <AlertExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Error Alert"}>
                <AlertExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Warning Alert"}>
                <AlertExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Info Alert"}>
                <AlertExample4 />
            </ExampleBlock>
        </MainContent>
    }
}
