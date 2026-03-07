use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(DrawerDocumentation)]
pub fn drawer_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Drawer Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Drawer component is a versatile UI element that slides in from the side of the screen. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
                        <Drawer<String>>
                            <DrawerHeader>
                                <DrawerTitle>{"Drawer Title"}</DrawerTitle>
                                <DrawerClose<String>>{"X"}</DrawerClose<String>>
                            </DrawerHeader>
                            <p>{"Drawer Content"}</p>
                            <DrawerFooter>
                                <DrawerClose<String>>
                                    <Button>{"Close"}</Button>
                                </DrawerClose<String>>
                            </DrawerFooter>
                        </Drawer<String>>
                    })}>
                        <DrawerTrigger<String> drawer={"example".to_string()}>
                            <Button>{"Open Drawer"}</Button>
                        </DrawerTrigger<String>>
                    </DrawerProvider<String>>
                }}
                code={r#"
<DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
    <Drawer<String>>
        <DrawerHeader>
            <DrawerTitle>{"Drawer Title"}</DrawerTitle>
            <DrawerClose<String>>{"X"}</DrawerClose<String>>
        </DrawerHeader>
        <p>{"Drawer Content"}</p>
        <DrawerFooter>
            <DrawerClose<String>>
                <Button>{"Close"}</Button>
            </DrawerClose<String>>
        </DrawerFooter>
    </Drawer<String>>
})}>
    <DrawerTrigger<String> drawer="example".to_string()>
        <Button>{"Open Drawer"}</Button>
    </DrawerTrigger<String>>
</DrawerProvider<String>>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Drawer" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the drawer component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the drawer component." }</li>
                <li>{ "side: DrawerSide - The side from which the drawer will slide in. Options are Left, Right, Top, Bottom. Default is Left." }</li>
                <li>{ "render: Callback<T, Html> - A callback to render the drawer content." }</li>
                <li>{ "curtain: bool - Whether to show a curtain behind the drawer. Default is false." }</li>
                <li>{ "curtain_content: Html - Content to show in the curtain. Default is empty." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DrawerTrigger" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The element that triggers the opening and closing of the drawer." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the trigger element." }</li>
                <li>{ "drawer: T - The identifier for the drawer to be opened." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Drawer" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The container for the drawer content." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the drawer content." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DrawerClose" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The element that triggers the closing of the drawer." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the close element." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DrawerHeader" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The header section of the drawer." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the header." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DrawerTitle" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The title of the drawer." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered as the title." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DrawerFooter" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The footer section of the drawer." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the footer." }</li>
            </ul>
        </div>
    }
}
