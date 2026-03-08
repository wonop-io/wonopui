use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use list_container1::ListContainer1;
use list_container2::ListContainer2;
use list_container3::ListContainer3;
use list_container4::ListContainer4;

#[function_component(ListContainers)]
pub fn list_containers() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"List Containers"} />
            </Breadcrumb>
            <ExampleBlock title={"List Container 1"}>
                <ListContainer1 />
            </ExampleBlock>
            <ExampleBlock title={"List Container 2"}>
                <ListContainer2 />
            </ExampleBlock>
            <ExampleBlock title={"List Container 3"}>
                <ListContainer3 />
            </ExampleBlock>
            <ExampleBlock title={"List Container 4"}>
                <ListContainer4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod list_container1 {
    use super::*;

    #[function_component(ListContainer1)]
    pub fn list_container1() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"List Container 1"}</h2>
                <ul class="list-disc pl-5 space-y-2">
                    <li>{"Item 1"}</li>
                    <li>{"Item 2"}</li>
                    <li>{"Item 3"}</li>
                    <li>{"Item 4"}</li>
                </ul>
            </div>
        }
    }
}

pub mod list_container2 {
    use super::*;

    #[function_component(ListContainer2)]
    pub fn list_container2() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"List Container 2"}</h2>
                <ol class="list-decimal pl-5 space-y-2">
                    <li>{"First item"}</li>
                    <li>{"Second item"}</li>
                    <li>{"Third item"}</li>
                    <li>{"Fourth item"}</li>
                </ol>
            </div>
        }
    }
}

pub mod list_container3 {
    use super::*;

    #[function_component(ListContainer3)]
    pub fn list_container3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"List Container 3"}</h2>
                <ul class="list-disc pl-5 space-y-2">
                    <li class="flex items-center">
                        <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 3" />
                        <span class="ml-2">{"User 1"}</span>
                    </li>
                    <li class="flex items-center">
                        <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 3" />
                        <span class="ml-2">{"User 2"}</span>
                    </li>
                    <li class="flex items-center">
                        <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 3" />
                        <span class="ml-2">{"User 3"}</span>
                    </li>
                    <li class="flex items-center">
                        <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 3" />
                        <span class="ml-2">{"User 4"}</span>
                    </li>
                </ul>
            </div>
        }
    }
}

pub mod list_container4 {
    use super::*;

    #[function_component(ListContainer4)]
    pub fn list_container4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"List Container 4"}</h2>
                <ul class="list-disc pl-5 space-y-2">
                    <li class="flex items-center">
                        <Checkbox />
                        <span class="ml-2">{"Task 1"}</span>
                    </li>
                    <li class="flex items-center">
                        <Checkbox />
                        <span class="ml-2">{"Task 2"}</span>
                    </li>
                    <li class="flex items-center">
                        <Checkbox />
                        <span class="ml-2">{"Task 3"}</span>
                    </li>
                    <li class="flex items-center">
                        <Checkbox />
                        <span class="ml-2">{"Task 4"}</span>
                    </li>
                </ul>
            </div>
        }
    }
}
