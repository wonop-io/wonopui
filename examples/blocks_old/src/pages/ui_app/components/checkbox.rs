use super::example_code::ExampleCode;
use wonopui::BRANDGUIDE;
use wonopui::*;
use yew::prelude::*;

#[function_component(CheckboxDocumentation)]
pub fn checkbox_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Checkbox Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Checkbox component is a versatile input element that allows users to toggle between checked and unchecked states. It can be used in forms, settings, and anywhere a binary choice is needed." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class={"flex items-center space-x-2"}>
                        <Checkbox id="terms" checked={false} on_toggle={Callback::from(|_| {})} />
                        <label
                            for="terms"
                            class={BRANDGUIDE.checkbox_label}
                        >
                            { "Accept terms and conditions" }
                        </label>
                    </div>
                }}
                code={r#"
<div class="flex items-center space-x-2">
    <Checkbox id="terms" checked={false} on_toggle={Callback::from(|_| {})} />
    <label for="terms" class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
        { "Accept terms and conditions" }
    </label>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Checkbox" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the checkbox component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "id: String - The unique identifier for the checkbox." }</li>
                <li>{ "checked: bool - The checked state of the checkbox." }</li>
                <li>{ "on_toggle: Callback<MouseEvent> - The callback to be called when the checkbox is toggled." }</li>
                <li>{ "disabled: bool - Whether the checkbox is disabled." }</li>
            </ul>
        </div>
    }
}
