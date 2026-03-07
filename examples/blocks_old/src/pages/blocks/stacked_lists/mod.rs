use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(StackedLists)]
pub fn stacked_lists() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Stacked Lists"} />
            </Breadcrumb>
            <ExampleBlock title={"Stacked List 1"}>
                <StackedList1 />
            </ExampleBlock>
            <ExampleBlock title={"Stacked List 2"}>
                <StackedList2 />
            </ExampleBlock>
            <ExampleBlock title={"Stacked List 3"}>
                <StackedList3 />
            </ExampleBlock>
            <ExampleBlock title={"Stacked List 4"}>
                <StackedList4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(StackedList1)]
pub fn stacked_list1() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Stacked List 1"}</h2>
            <ul class="space-y-2">
                <li class="p-2 border rounded">{"Item 1"}</li>
                <li class="p-2 border rounded">{"Item 2"}</li>
                <li class="p-2 border rounded">{"Item 3"}</li>
                <li class="p-2 border rounded">{"Item 4"}</li>
            </ul>
        </div>
    }
}

#[function_component(StackedList2)]
pub fn stacked_list2() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Stacked List 2"}</h2>
            <ul class="space-y-2">
                <li class="p-2 border rounded flex justify-between">
                    <span>{"Item 1"}</span>
                    <Button>{"Action"}</Button>
                </li>
                <li class="p-2 border rounded flex justify-between">
                    <span>{"Item 2"}</span>
                    <Button>{"Action"}</Button>
                </li>
                <li class="p-2 border rounded flex justify-between">
                    <span>{"Item 3"}</span>
                    <Button>{"Action"}</Button>
                </li>
                <li class="p-2 border rounded flex justify-between">
                    <span>{"Item 4"}</span>
                    <Button>{"Action"}</Button>
                </li>
            </ul>
        </div>
    }
}

#[function_component(StackedList3)]
pub fn stacked_list3() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Stacked List 3"}</h2>
            <ul class="space-y-2">
                <li class="p-2 border rounded flex items-center space-x-2">
                    <Avatar size={AvatarSize::Small} />
                    <span>{"Item 1"}</span>
                </li>
                <li class="p-2 border rounded flex items-center space-x-2">
                    <Avatar size={AvatarSize::Small} />
                    <span>{"Item 2"}</span>
                </li>
                <li class="p-2 border rounded flex items-center space-x-2">
                    <Avatar size={AvatarSize::Small} />
                    <span>{"Item 3"}</span>
                </li>
                <li class="p-2 border rounded flex items-center space-x-2">
                    <Avatar size={AvatarSize::Small} />
                    <span>{"Item 4"}</span>
                </li>
            </ul>
        </div>
    }
}

#[function_component(StackedList4)]
pub fn stacked_list4() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Stacked List 4"}</h2>
            <ul class="space-y-2">
                <li class="p-2 border rounded flex justify-between items-center">
                    <span>{"Item 1"}</span>
                    <Toggle />
                </li>
                <li class="p-2 border rounded flex justify-between items-center">
                    <span>{"Item 2"}</span>
                    <Toggle />
                </li>
                <li class="p-2 border rounded flex justify-between items-center">
                    <span>{"Item 3"}</span>
                    <Toggle />
                </li>
                <li class="p-2 border rounded flex justify-between items-center">
                    <span>{"Item 4"}</span>
                    <Toggle />
                </li>
            </ul>
        </div>
    }
}
