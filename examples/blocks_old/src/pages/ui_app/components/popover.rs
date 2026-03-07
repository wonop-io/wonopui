use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(PopoverDocumentation)]
pub fn popover_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Popover Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Popover component is a versatile UI element that displays content in a floating container. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Popover>
                        <PopoverTrigger>
                            <Button>{"Open Popover"}</Button>
                        </PopoverTrigger>
                        <PopoverContent>
                            {"Popover Content"}
                        </PopoverContent>
                    </Popover>
                }}
                code={r#"
<div class="mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow">
    <Popover>
        <PopoverTrigger>
            <Button>{"Open Popover"}</Button>
        </PopoverTrigger>
        <PopoverContent>
            {"Popover Content"}
        </PopoverContent>
    </Popover>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Popover" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the popover component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the popover component." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "PopoverTrigger" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The element that triggers the opening and closing of the popover." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the trigger element." }</li>
                <li>{ "as_child: bool - Whether the trigger should be rendered as a child element." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "PopoverContent" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The container for the popover content." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the popover content." }</li>
                <li>{ "class: Option<String> - Additional CSS classes for styling the content container." }</li>
            </ul>
        </div>
    }
}
