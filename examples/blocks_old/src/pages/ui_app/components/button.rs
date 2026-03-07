use super::example_code::ExampleCode;
use gloo_console as console;
use wonopui::*;
use yew::prelude::*;

#[function_component(ButtonDocumentation)]
pub fn button_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Button Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Button component is a versatile and customizable button element that can be used in various parts of your application. It supports different types such as Primary, Secondary, Danger, and Default, and can handle click events." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| console::log!("Button clicked!"))}>
                        {"Click Me"}
                    </Button>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| console::log!("Button clicked!"))}>
        {"Click Me"}
    </Button>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Button" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main button component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "onclick: Callback<MouseEvent> - The callback to be executed when the button is clicked." }</li>
                <li>{ "variant: ButtonVariant - The type of the button, which can be Primary, Secondary, Danger, or Default." }</li>
                <li>{ "children: Children - The child elements to be rendered inside the button." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "ButtonVariant" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "An enum representing the different types of buttons." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "Primary - A primary button, usually used for main actions." }</li>
                <li>{ "Secondary - A secondary button, usually used for less important actions." }</li>
                <li>{ "Danger - A danger button, usually used for destructive actions." }</li>
                <li>{ "Default - A default button, used for general purposes." }</li>
            </ul>
        </div>
    }
}
