use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(ToggleDemo)]
pub fn toggle_demo() -> Html {
    let checked = use_state(|| false);

    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    html! {
        <div class={"flex items-center space-x-2"}>
            <Toggle id="bold-toggle" checked={*checked} on_toggle={on_toggle}>
                <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
                    <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
                </svg>
            </Toggle>
            <label
                for="bold-toggle"
                class={BRANDGUIDE.toggle_label}
            >
                { "Toggle bold" }
            </label>
        </div>
    }
}

#[function_component(ToggleDocumentation)]
pub fn toggle_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Toggle Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Toggle component is a versatile switch component that allows users to toggle between two states. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <ToggleDemo />
                }}
                code={r#"
let checked = use_state(|| false);

let on_toggle = {
    let checked = checked.clone();
    Callback::from(move |_| {
        checked.set(!*checked);
    })
};

html! {
    <div class={"flex items-center space-x-2"}>
        <Toggle id="bold-toggle" checked={*checked} on_toggle={on_toggle}>
            <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
                <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
            </svg>
        </Toggle>
        <label
            for="bold-toggle"
            class={BRANDGUIDE.toggle_label}
        >
            { "Toggle bold" }
        </label>
    </div>
}"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Toggle" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the toggle component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "id: String - The id of the toggle component." }</li>
                <li>{ "checked: bool - The initial checked state of the toggle." }</li>
                <li>{ "on_toggle: Callback<MouseEvent> - The callback to be called when the toggle is clicked." }</li>
                <li>{ "disabled: bool - Whether the toggle is disabled." }</li>
                <li>{ "children: Children - The child elements to be rendered inside the toggle component." }</li>
            </ul>
        </div>
    }
}
