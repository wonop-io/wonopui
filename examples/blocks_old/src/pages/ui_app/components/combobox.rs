use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(ComboboxDocumentation)]
pub fn combobox_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Combobox Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Combobox component is a versatile dropdown component that allows users to select an option from a list. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Combobox options={vec![("1".to_string(), "Option 1".to_string()), ("2".to_string(), "Option 2".to_string()), ("3".to_string(), "Option 3".to_string())]} />
                }}
                code={r#"
<div class="mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow">
    <Combobox options={vec![("1".to_string(), "Option 1".to_string()), ("2".to_string(), "Option 2".to_string()), ("3".to_string(), "Option 3".to_string())]} />
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Combobox" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the combobox component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "id: String - The unique identifier for the combobox." }</li>
                <li>{ "options: Vec<(String, String)> - A list of options where each option is a tuple of (value, label)." }</li>
                <li>{ "on_select: Callback<String> - The callback to be called when an option is selected." }</li>
                <li>{ "disabled: bool - Whether the combobox is disabled." }</li>
            </ul>
        </div>
    }
}
