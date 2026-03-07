use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(AccordionDocumentation)]
pub fn accordion_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Accordion Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Accordion component is a collapsible section of content. It allows users to toggle the visibility of content sections." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Accordion title="Accordion example">
                        <p>{ "Hello world" }</p>
                    </Accordion>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Accordion title="Accordion example">
        <p>{ "Hello world" }</p>
    </Accordion>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Accordion" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the accordion component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "title: String - The title of the accordion section." }</li>
                <li>{ "children: Children - The child elements to be rendered inside the accordion content." }</li>
            </ul>
        </div>
    }
}
