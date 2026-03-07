use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Stats)]
pub fn stats() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Stats"} />
            </Breadcrumb>
            <ExampleBlock title={"Stats Example 1"}>
                <StatsExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Stats Example 2"}>
                <StatsExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Stats Example 3"}>
                <StatsExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Stats Example 4"}>
                <StatsExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(StatsExample1)]
pub fn stats_example1() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <h2 class="text-xl font-bold mb-4">{"Stats Example 1"}</h2>
            <div class="grid grid-cols-2 gap-4">
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Users"}</h3>
                    <p class="text-2xl">{"1,234"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Revenue"}</h3>
                    <p class="text-2xl">{"$12,345"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Orders"}</h3>
                    <p class="text-2xl">{"567"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Feedback"}</h3>
                    <p class="text-2xl">{"89%"}</p>
                </div>
            </div>
        </div>
    }
}

#[function_component(StatsExample2)]
pub fn stats_example2() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <h2 class="text-xl font-bold mb-4">{"Stats Example 2"}</h2>
            <div class="grid grid-cols-1 gap-4">
                <div class="p-4 border rounded flex justify-between items-center">
                    <h3 class="text-lg font-semibold">{"New Signups"}</h3>
                    <p class="text-2xl">{"345"}</p>
                </div>
                <div class="p-4 border rounded flex justify-between items-center">
                    <h3 class="text-lg font-semibold">{"Active Users"}</h3>
                    <p class="text-2xl">{"1,234"}</p>
                </div>
                <div class="p-4 border rounded flex justify-between items-center">
                    <h3 class="text-lg font-semibold">{"Churn Rate"}</h3>
                    <p class="text-2xl">{"5%"}</p>
                </div>
                <div class="p-4 border rounded flex justify-between items-center">
                    <h3 class="text-lg font-semibold">{"Net Promoter Score"}</h3>
                    <p class="text-2xl">{"75"}</p>
                </div>
            </div>
        </div>
    }
}

#[function_component(StatsExample3)]
pub fn stats_example3() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <h2 class="text-xl font-bold mb-4">{"Stats Example 3"}</h2>
            <div class="grid grid-cols-3 gap-4">
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Page Views"}</h3>
                    <p class="text-2xl">{"8,765"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Unique Visitors"}</h3>
                    <p class="text-2xl">{"4,321"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Bounce Rate"}</h3>
                    <p class="text-2xl">{"32%"}</p>
                </div>
            </div>
        </div>
    }
}

#[function_component(StatsExample4)]
pub fn stats_example4() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <h2 class="text-xl font-bold mb-4">{"Stats Example 4"}</h2>
            <div class="grid grid-cols-4 gap-4">
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Sales"}</h3>
                    <p class="text-2xl">{"$23,456"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Expenses"}</h3>
                    <p class="text-2xl">{"$12,345"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"Profit"}</h3>
                    <p class="text-2xl">{"$11,111"}</p>
                </div>
                <div class="p-4 border rounded">
                    <h3 class="text-lg font-semibold">{"ROI"}</h3>
                    <p class="text-2xl">{"90%"}</p>
                </div>
            </div>
        </div>
    }
}
