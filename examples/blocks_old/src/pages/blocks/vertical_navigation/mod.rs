use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(VerticalNavigation)]
pub fn vertical_navigation() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Vertical Navigation"} />
            </Breadcrumb>
            <ExampleBlock title={"Vertical Navigation Example 1"}>
                <VerticalNavigationExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Vertical Navigation Example 2"}>
                <VerticalNavigationExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Vertical Navigation Example 3"}>
                <VerticalNavigationExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Vertical Navigation Example 4"}>
                <VerticalNavigationExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(VerticalNavigationExample1)]
pub fn vertical_navigation_example1() -> Html {
    html! {
        <div class="flex flex-col w-64 h-full bg-gray-800 text-white">
            <div class="p-4">
                <h2 class="text-lg font-semibold">{"Navigation 1"}</h2>
            </div>
            <nav class="flex-1 px-2 space-y-1">
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Dashboard"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Settings"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Profile"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Help"}</a>
            </nav>
        </div>
    }
}

#[function_component(VerticalNavigationExample2)]
pub fn vertical_navigation_example2() -> Html {
    html! {
        <div class="flex flex-col w-64 h-full bg-gray-900 text-white">
            <div class="p-4">
                <h2 class="text-lg font-semibold">{"Navigation 2"}</h2>
            </div>
            <nav class="flex-1 px-2 space-y-1">
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Home"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"About"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Services"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-700">{"Contact"}</a>
            </nav>
        </div>
    }
}

#[function_component(VerticalNavigationExample3)]
pub fn vertical_navigation_example3() -> Html {
    html! {
        <div class="flex flex-col w-64 h-full bg-gray-700 text-white">
            <div class="p-4">
                <h2 class="text-lg font-semibold">{"Navigation 3"}</h2>
            </div>
            <nav class="flex-1 px-2 space-y-1">
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-600">{"Overview"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-600">{"Features"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-600">{"Pricing"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-600">{"FAQ"}</a>
            </nav>
        </div>
    }
}

#[function_component(VerticalNavigationExample4)]
pub fn vertical_navigation_example4() -> Html {
    html! {
        <div class="flex flex-col w-64 h-full bg-gray-600 text-white">
            <div class="p-4">
                <h2 class="text-lg font-semibold">{"Navigation 4"}</h2>
            </div>
            <nav class="flex-1 px-2 space-y-1">
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-500">{"Section 1"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-500">{"Section 2"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-500">{"Section 3"}</a>
                <a href="#" class="block px-4 py-2 text-sm font-medium rounded hover:bg-gray-500">{"Section 4"}</a>
            </nav>
        </div>
    }
}
