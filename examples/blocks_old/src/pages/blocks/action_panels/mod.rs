use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use action_panel1::ActionPanel1;
use action_panel2::ActionPanel2;
use action_panel3::ActionPanel3;
use action_panel4::ActionPanel4;

#[function_component(ActionPanels)]
pub fn action_panels() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Action Panels"} />
            </Breadcrumb>
            <ExampleBlock title={"Action Panel 1"}>
                <ActionPanel1 />
            </ExampleBlock>
            <ExampleBlock title={"Action Panel 2"}>
                <ActionPanel2 />
            </ExampleBlock>
            <ExampleBlock title={"Action Panel 3"}>
                <ActionPanel3 />
            </ExampleBlock>
            <ExampleBlock title={"Action Panel 4"}>
                <ActionPanel4 />
            </ExampleBlock>
        </MainContent>
    }
}

mod action_panel1 {
    use wonopui::*;
    use yew::prelude::*;

    #[function_component(ActionPanel1)]
    pub fn action_panel1() -> Html {
        html! {
            <Card>
                <CardContent>
                    <p>{"This is Action Panel 1"}</p>
                </CardContent>
            </Card>
        }
    }
}

mod action_panel2 {
    use wonopui::*;
    use yew::prelude::*;

    #[function_component(ActionPanel2)]
    pub fn action_panel2() -> Html {
        html! {
            <Card>
                <CardContent>
                    <p>{"This is Action Panel 2"}</p>
                </CardContent>
            </Card>
        }
    }
}

mod action_panel3 {
    use wonopui::*;
    use yew::prelude::*;

    #[function_component(ActionPanel3)]
    pub fn action_panel3() -> Html {
        html! {
            <Card>
                <CardContent>
                    <p>{"This is Action Panel 3"}</p>
                </CardContent>
            </Card>
        }
    }
}

mod action_panel4 {
    use wonopui::*;
    use yew::prelude::*;

    #[function_component(ActionPanel4)]
    pub fn action_panel4() -> Html {
        html! {
            <Card>
                <CardContent>
                    <p>{"This is Action Panel 4"}</p>
                </CardContent>
            </Card>
        }
    }
}
