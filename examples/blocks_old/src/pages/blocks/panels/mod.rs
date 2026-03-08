use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use panel1::Panel1;
use panel2::Panel2;
use panel3::Panel3;
use panel4::Panel4;

#[function_component(Panels)]
pub fn panels() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Panels"} />
            </Breadcrumb>
            <ExampleBlock title={"Panel 1"}>
                <Panel1 />
            </ExampleBlock>
            <ExampleBlock title={"Panel 2"}>
                <Panel2 />
            </ExampleBlock>
            <ExampleBlock title={"Panel 3"}>
                <Panel3 />
            </ExampleBlock>
            <ExampleBlock title={"Panel 4"}>
                <Panel4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod panel1 {
    use super::*;

    #[function_component(Panel1)]
    pub fn panel1() -> Html {
        html! {
            <div class="p-4 bg-gray-100 border border-gray-300 rounded">
                <h2 class="text-lg font-bold">{"Panel 1"}</h2>
                <p>{"This is the content of Panel 1."}</p>
            </div>
        }
    }
}

pub mod panel2 {
    use super::*;

    #[function_component(Panel2)]
    pub fn panel2() -> Html {
        html! {
            <div class="p-4 bg-blue-100 border border-blue-300 rounded">
                <h2 class="text-lg font-bold">{"Panel 2"}</h2>
                <p>{"This is the content of Panel 2."}</p>
            </div>
        }
    }
}

pub mod panel3 {
    use super::*;

    #[function_component(Panel3)]
    pub fn panel3() -> Html {
        html! {
            <div class="p-4 bg-green-100 border border-green-300 rounded">
                <h2 class="text-lg font-bold">{"Panel 3"}</h2>
                <p>{"This is the content of Panel 3."}</p>
            </div>
        }
    }
}

pub mod panel4 {
    use super::*;

    #[function_component(Panel4)]
    pub fn panel4() -> Html {
        html! {
            <div class="p-4 bg-red-100 border border-red-300 rounded">
                <h2 class="text-lg font-bold">{"Panel 4"}</h2>
                <p>{"This is the content of Panel 4."}</p>
            </div>
        }
    }
}
