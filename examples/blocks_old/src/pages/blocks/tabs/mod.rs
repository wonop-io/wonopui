use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(TabsExample)]
pub fn tabs_example() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Tabs"} />
            </Breadcrumb>
            <ExampleBlock title={"Tabs Example 1"}>
                <TabsExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Tabs Example 2"}>
                <TabsExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Tabs Example 3"}>
                <TabsExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Tabs Example 4"}>
                <TabsExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(TabsExample1)]
pub fn tabs_example1() -> Html {
    html! {
        <Tabs>
            <TabsList>
                <TabsTrigger>{"Tab 1"}</TabsTrigger>
                <TabsTrigger>{"Tab 2"}</TabsTrigger>
                <TabsTrigger>{"Tab 3"}</TabsTrigger>
            </TabsList>
            <TabsContent>
                <div>{"Content for Tab 1"}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Content for Tab 2"}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Content for Tab 3"}</div>
            </TabsContent>
        </Tabs>
    }
}

#[function_component(TabsExample2)]
pub fn tabs_example2() -> Html {
    html! {
        <Tabs>
            <TabsList>
                <TabsTrigger>{"Overview"}</TabsTrigger>
                <TabsTrigger>{"Details"}</TabsTrigger>
                <TabsTrigger>{"Reviews"}</TabsTrigger>
            </TabsList>
            <TabsContent>
                <div>{"Overview content goes here."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Details content goes here."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Reviews content goes here."}</div>
            </TabsContent>
        </Tabs>
    }
}

#[function_component(TabsExample3)]
pub fn tabs_example3() -> Html {
    html! {
        <Tabs>
            <TabsList>
                <TabsTrigger>{"Home"}</TabsTrigger>
                <TabsTrigger>{"Profile"}</TabsTrigger>
                <TabsTrigger>{"Settings"}</TabsTrigger>
            </TabsList>
            <TabsContent>
                <div>{"Home content goes here."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Profile content goes here."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Settings content goes here."}</div>
            </TabsContent>
        </Tabs>
    }
}

#[function_component(TabsExample4)]
pub fn tabs_example4() -> Html {
    html! {
        <Tabs>
            <TabsList>
                <TabsTrigger>{"First"}</TabsTrigger>
                <TabsTrigger>{"Second"}</TabsTrigger>
                <TabsTrigger>{"Third"}</TabsTrigger>
                <TabsTrigger>{"Fourth"}</TabsTrigger>
            </TabsList>
            <TabsContent>
                <div>{"First tab content."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Second tab content."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Third tab content."}</div>
            </TabsContent>
            <TabsContent>
                <div>{"Fourth tab content."}</div>
            </TabsContent>
        </Tabs>
    }
}
