use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use pagination1::Pagination1;
use pagination2::Pagination2;
use pagination3::Pagination3;
use pagination4::Pagination4;

#[function_component(Paginations)]
pub fn paginations() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Paginations"} />
            </Breadcrumb>
            <ExampleBlock title={"Pagination 1"}>
                <Pagination1 />
            </ExampleBlock>
            <ExampleBlock title={"Pagination 2"}>
                <Pagination2 />
            </ExampleBlock>
            <ExampleBlock title={"Pagination 3"}>
                <Pagination3 />
            </ExampleBlock>
            <ExampleBlock title={"Pagination 4"}>
                <Pagination4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod pagination1 {
    use super::*;

    #[function_component(Pagination1)]
    pub fn pagination1() -> Html {
        html! {
            <div class="flex justify-center space-x-2">
                <Button>{"Previous"}</Button>
                <Button>{"1"}</Button>
                <Button>{"2"}</Button>
                <Button>{"3"}</Button>
                <Button>{"Next"}</Button>
            </div>
        }
    }
}

pub mod pagination2 {
    use super::*;

    #[function_component(Pagination2)]
    pub fn pagination2() -> Html {
        html! {
            <div class="flex justify-center space-x-2">
                <Button>{"<<"}</Button>
                <Button>{"<"}</Button>
                <Button>{"1"}</Button>
                <Button>{"2"}</Button>
                <Button>{"3"}</Button>
                <Button>{">"}</Button>
                <Button>{">>"}</Button>
            </div>
        }
    }
}

pub mod pagination3 {
    use super::*;

    #[function_component(Pagination3)]
    pub fn pagination3() -> Html {
        html! {
            <div class="flex justify-center space-x-2">
                <Button>{"First"}</Button>
                <Button>{"Previous"}</Button>
                <Button>{"1"}</Button>
                <Button>{"2"}</Button>
                <Button>{"3"}</Button>
                <Button>{"Next"}</Button>
                <Button>{"Last"}</Button>
            </div>
        }
    }
}

pub mod pagination4 {
    use super::*;

    #[function_component(Pagination4)]
    pub fn pagination4() -> Html {
        html! {
            <div class="flex justify-center space-x-2">
                <Button>{"1"}</Button>
                <Button>{"2"}</Button>
                <Button>{"3"}</Button>
                <Button>{"4"}</Button>
                <Button>{"5"}</Button>
            </div>
        }
    }
}
