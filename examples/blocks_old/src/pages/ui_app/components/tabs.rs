use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(TabsDocumentation)]
pub fn tabs_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Tabs Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Tabs component is a versatile UI element that allows users to switch between different views or sections within the same page. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Tabs default_value="tab1">
                        <TabsList>
                            <TabsTrigger value="tab1">{ "Tab 1" }</TabsTrigger>
                            <TabsTrigger value="tab2">{ "Tab 2" }</TabsTrigger>
                            <TabsTrigger value="tab3">{ "Tab 3" }</TabsTrigger>
                        </TabsList>
                        <TabsContent value="tab1">
                            { "Tab 1 Content" }
                        </TabsContent>
                        <TabsContent value="tab2">
                            { "Tab 2 Content" }
                        </TabsContent>
                        <TabsContent value="tab3">
                            { "Tab 3 Content" }
                        </TabsContent>
                    </Tabs>
                }}
                code={r#"
<Tabs default_value="tab1">
    <TabsList>
        <TabsTrigger value="tab1">{ "Tab 1" }</TabsTrigger>
        <TabsTrigger value="tab2">{ "Tab 2" }</TabsTrigger>
        <TabsTrigger value="tab3">{ "Tab 3" }</TabsTrigger>
    </TabsList>
    <TabsContent value="tab1">
        { "Tab 1 Content" }
    </TabsContent>
    <TabsContent value="tab2">
        { "Tab 2 Content" }
    </TabsContent>
    <TabsContent value="tab3">
        { "Tab 3 Content" }
    </TabsContent>
</Tabs>
                "#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Tabs" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the tabs component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the tabs component." }</li>
                <li>{ "default_value: String - The default active tab value." }</li>
                <li>{ "class: Classes - Additional CSS classes for styling the tabs container." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "TabsList" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The container for the list of tab triggers." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the tabs list." }</li>
                <li>{ "class: Classes - Additional CSS classes for styling the tabs list." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "TabsTrigger" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The button that triggers the switching of tabs." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "value: String - The value of the tab to be activated when the trigger is clicked." }</li>
                <li>{ "children: Children - The child elements to be rendered inside the tabs trigger." }</li>
                <li>{ "class: Classes - Additional CSS classes for styling the tabs trigger." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "TabsContent" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The container for the content of a specific tab." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "value: String - The value of the tab that this content belongs to." }</li>
                <li>{ "children: Children - The child elements to be rendered inside the tabs content." }</li>
                <li>{ "class: Classes - Additional CSS classes for styling the tabs content." }</li>
            </ul>
        </div>
    }
}
