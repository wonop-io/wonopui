use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(CardDocumentation)]
pub fn card_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Card Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Card component is a versatile container that can hold various types of content. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Card Title"}</CardTitle>
                        </CardHeader>
                        <CardContent>
                            {"Hello"}
                        </CardContent>
                    </Card>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Card>
        <CardHeader>
            <CardTitle>{"Card Title"}</CardTitle>
        </CardHeader>
        <CardContent>
            {"Hello"}
        </CardContent>
    </Card>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Card" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the card component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the card component." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "CardHeader" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The header section of the card." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the card header." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "CardTitle" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The title of the card." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the card title." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "CardContent" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The content section of the card." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the card content." }</li>
            </ul>
        </div>
    }
}
