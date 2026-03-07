use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use description_list1::DescriptionList1;
use description_list2::DescriptionList2;
use description_list3::DescriptionList3;
use description_list4::DescriptionList4;

#[function_component(DescriptionLists)]
pub fn description_lists() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Description Lists"} />
            </Breadcrumb>
            <ExampleBlock title={"Description List 1"}>
                <DescriptionList1 />
            </ExampleBlock>
            <ExampleBlock title={"Description List 2"}>
                <DescriptionList2 />
            </ExampleBlock>
            <ExampleBlock title={"Description List 3"}>
                <DescriptionList3 />
            </ExampleBlock>
            <ExampleBlock title={"Description List 4"}>
                <DescriptionList4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod description_list1 {
    use super::*;

    #[function_component(DescriptionList1)]
    pub fn description_list1() -> Html {
        html! {
            <dl class="p-4 border rounded shadow">
                <dt class="text-lg font-bold">{"Term 1"}</dt>
                <dd class="ml-4">{"Description for term 1."}</dd>
                <dt class="text-lg font-bold mt-2">{"Term 2"}</dt>
                <dd class="ml-4">{"Description for term 2."}</dd>
                <dt class="text-lg font-bold mt-2">{"Term 3"}</dt>
                <dd class="ml-4">{"Description for term 3."}</dd>
            </dl>
        }
    }
}

pub mod description_list2 {
    use super::*;

    #[function_component(DescriptionList2)]
    pub fn description_list2() -> Html {
        html! {
            <dl class="p-4 border rounded shadow">
                <dt class="text-lg font-bold">{"Feature A"}</dt>
                <dd class="ml-4">{"Details about feature A."}</dd>
                <dt class="text-lg font-bold mt-2">{"Feature B"}</dt>
                <dd class="ml-4">{"Details about feature B."}</dd>
                <dt class="text-lg font-bold mt-2">{"Feature C"}</dt>
                <dd class="ml-4">{"Details about feature C."}</dd>
            </dl>
        }
    }
}

pub mod description_list3 {
    use super::*;

    #[function_component(DescriptionList3)]
    pub fn description_list3() -> Html {
        html! {
            <dl class="p-4 border rounded shadow">
                <dt class="text-lg font-bold">{"Item 1"}</dt>
                <dd class="ml-4">{"Information about item 1."}</dd>
                <dt class="text-lg font-bold mt-2">{"Item 2"}</dt>
                <dd class="ml-4">{"Information about item 2."}</dd>
                <dt class="text-lg font-bold mt-2">{"Item 3"}</dt>
                <dd class="ml-4">{"Information about item 3."}</dd>
            </dl>
        }
    }
}

pub mod description_list4 {
    use super::*;

    #[function_component(DescriptionList4)]
    pub fn description_list4() -> Html {
        html! {
            <dl class="p-4 border rounded shadow">
                <dt class="text-lg font-bold">{"Concept X"}</dt>
                <dd class="ml-4">{"Explanation of concept X."}</dd>
                <dt class="text-lg font-bold mt-2">{"Concept Y"}</dt>
                <dd class="ml-4">{"Explanation of concept Y."}</dd>
                <dt class="text-lg font-bold mt-2">{"Concept Z"}</dt>
                <dd class="ml-4">{"Explanation of concept Z."}</dd>
            </dl>
        }
    }
}
