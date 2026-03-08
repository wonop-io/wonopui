use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(DialogDocumentation)]
pub fn dialog_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Dialog Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Dialog component is a versatile UI element that displays content in a modal dialog. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <DialogProvider>
                        <DialogTrigger id="example-dialog">
                            <Button>{"Open Dialog"}</Button>
                        </DialogTrigger>
                        <Dialog id="example-dialog">
                            <DialogHeader>
                                <DialogTitle>{"Dialog Title"}</DialogTitle>
                            </DialogHeader>
                            <DialogBody>{"This is a description inside the dialog."}</DialogBody>
                            <DialogFooter>
                                <DialogClose>
                                    <Button>{"Close"}</Button>
                                </DialogClose>
                            </DialogFooter>
                        </Dialog>
                    </DialogProvider>
                }}
                code={r#"
<DialogProvider>
    <DialogTrigger id="example-dialog">
        <Button>{"Open Dialog"}</Button>
    </DialogTrigger>
    <Dialog id="example-dialog">
        <DialogHeader>
            <DialogTitle>{"Dialog Title"}</DialogTitle>
        </DialogHeader>
        <DialogBody>{"This is a description inside the dialog."}</DialogBody>
        <DialogFooter>
            <DialogClose>
                <Button>{"Close"}</Button>
            </DialogClose>
        </DialogFooter>
    </Dialog>
</DialogProvider>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Dialog" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the dialog component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the dialog component." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DialogTrigger" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The element that triggers the opening and closing of the dialog." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the trigger element." }</li>
                <li>{ "id: String - A unique identifier for the dialog." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Dialog" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The container for the dialog content." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the dialog content." }</li>
                <li>{ "id: String - A unique identifier for the dialog, matching the DialogTrigger id." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DialogHeader" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The header section of the dialog." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the header." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DialogTitle" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The title of the dialog." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the title." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DialogBody" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The body section of the dialog." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the body." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DialogFooter" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The footer section of the dialog." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the footer." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "DialogClose" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The element that triggers the closing of the dialog." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the close element." }</li>
            </ul>
        </div>
    }
}
