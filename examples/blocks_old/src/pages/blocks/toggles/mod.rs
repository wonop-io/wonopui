use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Toggles)]
pub fn toggles() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Toggles"} />
            </Breadcrumb>
            <ExampleBlock title={"Toggle Example 1"}>
                <ToggleExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Toggle Example 2"}>
                <ToggleExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Toggle Example 3"}>
                <ToggleExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Toggle Example 4"}>
                <ToggleExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(ToggleExample1)]
pub fn toggle_example1() -> Html {
    html! {
        <div class="p-4">
            <label for="toggle1" class="block text-sm font-medium text-gray-700">{"Example 1"}</label>
            <Toggle id="toggle1" name="toggle1" />
        </div>
    }
}

#[function_component(ToggleExample2)]
pub fn toggle_example2() -> Html {
    html! {
        <div class="p-4">
            <label for="toggle2" class="block text-sm font-medium text-gray-700">{"Example 2"}</label>
            <Toggle id="toggle2" name="toggle2" />
        </div>
    }
}

#[function_component(ToggleExample3)]
pub fn toggle_example3() -> Html {
    html! {
        <div class="p-4">
            <label for="toggle3" class="block text-sm font-medium text-gray-700">{"Example 3"}</label>
            <Toggle id="toggle3" name="toggle3" />
        </div>
    }
}

#[function_component(ToggleExample4)]
pub fn toggle_example4() -> Html {
    html! {
        <div class="p-4">
            <label for="toggle4" class="block text-sm font-medium text-gray-700">{"Example 4"}</label>
            <Toggle id="toggle4" name="toggle4" />
        </div>
    }
}
