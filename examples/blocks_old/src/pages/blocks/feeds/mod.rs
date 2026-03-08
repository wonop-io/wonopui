use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use feed1::Feed1;
use feed2::Feed2;
use feed3::Feed3;
use feed4::Feed4;

#[function_component(Feeds)]
pub fn feeds() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Feeds"} />
            </Breadcrumb>
            <ExampleBlock title={"Feed 1"}>
                <Feed1 />
            </ExampleBlock>
            <ExampleBlock title={"Feed 2"}>
                <Feed2 />
            </ExampleBlock>
            <ExampleBlock title={"Feed 3"}>
                <Feed3 />
            </ExampleBlock>
            <ExampleBlock title={"Feed 4"}>
                <Feed4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod feed1 {
    use super::*;

    #[function_component(Feed1)]
    pub fn feed1() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Feed 1"}</h2>
                <p>{"This is the first feed example."}</p>
                <div class="border-t my-4"></div>
                <p>{"Content of the first feed."}</p>
            </div>
        }
    }
}

pub mod feed2 {
    use super::*;

    #[function_component(Feed2)]
    pub fn feed2() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Feed 2"}</h2>
                <p>{"This is the second feed example."}</p>
                <div class="border-t my-4"></div>
                <p>{"Content of the second feed."}</p>
            </div>
        }
    }
}

pub mod feed3 {
    use super::*;

    #[function_component(Feed3)]
    pub fn feed3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Feed 3"}</h2>
                <p>{"This is the third feed example."}</p>
                <div class="border-t my-4"></div>
                <p>{"Content of the third feed."}</p>
            </div>
        }
    }
}

pub mod feed4 {
    use super::*;

    #[function_component(Feed4)]
    pub fn feed4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Feed 4"}</h2>
                <p>{"This is the fourth feed example."}</p>
                <div class="border-t my-4"></div>
                <p>{"Content of the fourth feed."}</p>
            </div>
        }
    }
}
