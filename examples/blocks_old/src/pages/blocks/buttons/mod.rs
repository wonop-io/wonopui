use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(ButtonExample1)]
pub fn button_example1() -> Html {
    html! {
        <Button variant={ButtonVariant::Primary}>
            {"Primary Button"}
        </Button>
    }
}

#[function_component(ButtonExample2)]
pub fn button_example2() -> Html {
    html! {
        <Button variant={ButtonVariant::Secondary}>
            {"Secondary Button"}
        </Button>
    }
}

#[function_component(ButtonExample3)]
pub fn button_example3() -> Html {
    html! {
        <Button variant={ButtonVariant::Success}>
            {"Success Button"}
        </Button>
    }
}

#[function_component(ButtonExample4)]
pub fn button_example4() -> Html {
    html! {
        <Button variant={ButtonVariant::Danger}>
            {"Danger Button"}
        </Button>
    }
}

#[function_component(Buttons)]
pub fn buttons() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Buttons"} />
            </Breadcrumb>
            <ExampleBlock title={"Primary Button"} isolate={true}>
                <ButtonExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Secondary Button"} isolate={true}>
                <ButtonExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Success Button"} isolate={true}>
                <ButtonExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Danger Button"} isolate={true}>
                <ButtonExample4 />
            </ExampleBlock>
        </MainContent>
    }
}
