use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(CarouselDocumentation)]
pub fn carousel_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Carousel Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Carousel component is a versatile container that can hold multiple items and cycle through them at a specified interval. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Carousel interval={3000}>
                        <CarouselItem>
                            <p>{"Item 1"}</p>
                        </CarouselItem>
                        <CarouselItem>
                            <p>{"Item 2"}</p>
                        </CarouselItem>
                        <CarouselItem>
                            <p>{"Item 3"}</p>
                        </CarouselItem>
                    </Carousel>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Carousel interval={3000}>
        <CarouselItem>
            <p>{"Item 1"}</p>
        </CarouselItem>
        <CarouselItem>
            <p>{"Item 2"}</p>
        </CarouselItem>
        <CarouselItem>
            <p>{"Item 3"}</p>
        </CarouselItem>
    </Carousel>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Carousel" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the carousel component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the carousel component." }</li>
                <li>{ "interval: u32 - The interval in milliseconds for cycling through the items." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "CarouselItem" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "An individual item within the carousel." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the carousel item." }</li>
            </ul>
        </div>
    }
}
