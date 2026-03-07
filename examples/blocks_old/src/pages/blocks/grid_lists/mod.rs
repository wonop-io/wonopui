use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use grid_list1::GridList1;
use grid_list2::GridList2;
use grid_list3::GridList3;
use grid_list4::GridList4;

#[function_component(GridLists)]
pub fn grid_lists() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Grid Lists"} />
            </Breadcrumb>
            <ExampleBlock title={"Grid List 1"}>
                <GridList1 />
            </ExampleBlock>
            <ExampleBlock title={"Grid List 2"}>
                <GridList2 />
            </ExampleBlock>
            <ExampleBlock title={"Grid List 3"}>
                <GridList3 />
            </ExampleBlock>
            <ExampleBlock title={"Grid List 4"}>
                <GridList4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod grid_list1 {
    use super::*;

    #[function_component(GridList1)]
    pub fn grid_list1() -> Html {
        html! {
            <div class="grid grid-cols-3 gap-4 p-4">
                <div class="p-4 border rounded shadow">{"Item 1"}</div>
                <div class="p-4 border rounded shadow">{"Item 2"}</div>
                <div class="p-4 border rounded shadow">{"Item 3"}</div>
                <div class="p-4 border rounded shadow">{"Item 4"}</div>
                <div class="p-4 border rounded shadow">{"Item 5"}</div>
                <div class="p-4 border rounded shadow">{"Item 6"}</div>
            </div>
        }
    }
}

pub mod grid_list2 {
    use super::*;

    #[function_component(GridList2)]
    pub fn grid_list2() -> Html {
        html! {
            <div class="grid grid-cols-4 gap-4 p-4">
                <div class="p-4 border rounded shadow">{"Item 1"}</div>
                <div class="p-4 border rounded shadow">{"Item 2"}</div>
                <div class="p-4 border rounded shadow">{"Item 3"}</div>
                <div class="p-4 border rounded shadow">{"Item 4"}</div>
                <div class="p-4 border rounded shadow">{"Item 5"}</div>
                <div class="p-4 border rounded shadow">{"Item 6"}</div>
                <div class="p-4 border rounded shadow">{"Item 7"}</div>
                <div class="p-4 border rounded shadow">{"Item 8"}</div>
            </div>
        }
    }
}

pub mod grid_list3 {
    use super::*;

    #[function_component(GridList3)]
    pub fn grid_list3() -> Html {
        html! {
            <div class="grid grid-cols-2 gap-4 p-4">
                <div class="p-4 border rounded shadow">{"Item 1"}</div>
                <div class="p-4 border rounded shadow">{"Item 2"}</div>
                <div class="p-4 border rounded shadow">{"Item 3"}</div>
                <div class="p-4 border rounded shadow">{"Item 4"}</div>
            </div>
        }
    }
}

pub mod grid_list4 {
    use super::*;

    #[function_component(GridList4)]
    pub fn grid_list4() -> Html {
        html! {
            <div class="grid grid-cols-1 gap-4 p-4">
                <div class="p-4 border rounded shadow">{"Item 1"}</div>
                <div class="p-4 border rounded shadow">{"Item 2"}</div>
                <div class="p-4 border rounded shadow">{"Item 3"}</div>
            </div>
        }
    }
}
