use crate::pages::example_block::ExampleBlock;

use wonopui::*;
use yew::prelude::*;

use divider1::Divider1;
use divider2::Divider2;
use divider3::Divider3;
use divider4::Divider4;

#[function_component(Dividers)]
pub fn dividers() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Dividers"} />
            </Breadcrumb>
            <ExampleBlock title={"Divider 1"}>
                <Divider1 />
            </ExampleBlock>
            <ExampleBlock title={"Divider 2"}>
                <Divider2 />
            </ExampleBlock>
            <ExampleBlock title={"Divider 3"}>
                <Divider3 />
            </ExampleBlock>
            <ExampleBlock title={"Divider 4"}>
                <Divider4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod divider1 {
    use super::*;

    #[function_component(Divider1)]
    pub fn divider1() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Divider 1"}</h2>
                <p>{"This is the first divider example."}</p>
                <div class="border-t my-4"></div>
                <p>{"Content after the divider."}</p>
            </div>
        }
    }
}

pub mod divider2 {
    use super::*;

    #[function_component(Divider2)]
    pub fn divider2() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Divider 2"}</h2>
                <p>{"This is the second divider example."}</p>
                <div class="border-b my-4"></div>
                <p>{"Content after the divider."}</p>
            </div>
        }
    }
}

pub mod divider3 {
    use super::*;

    #[function_component(Divider3)]
    pub fn divider3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Divider 3"}</h2>
                <p>{"This is the third divider example."}</p>
                <div class="border-l my-4 h-32"></div>
                <p>{"Content after the divider."}</p>
            </div>
        }
    }
}

pub mod divider4 {
    use super::*;

    #[function_component(Divider4)]
    pub fn divider4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Divider 4"}</h2>
                <p>{"This is the fourth divider example."}</p>
                <div class="border-r my-4 h-32"></div>
                <p>{"Content after the divider."}</p>
            </div>
        }
    }
}
