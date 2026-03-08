use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(CommandDocumentation)]
pub fn command_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Command Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Command component is a versatile input component that allows users to select an option from a list. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Command options={vec![
                        ("1".to_string(), "Option 1".to_string(), None),
                        ("2".to_string(), "Option 2".to_string(), None),
                        ("3".to_string(), "Option 3".to_string(), None)
                    ]} />
                }}
                code={r#"
<div class="mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow">
    <Command options={vec![
        ("1".to_string(), "Option 1".to_string(), None),
        ("2".to_string(), "Option 2".to_string(), None),
        ("3".to_string(), "Option 3".to_string(), None)
    ]} />
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Command" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the command component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "placeholder: String - The placeholder text to be displayed in the input field." }</li>
                <li>{ "options: Vec<(String, String, Option<Html>)> - A list of options where each option is a tuple of (value, label, icon)." }</li>
                <li>{ "on_select: Callback<String> - The callback to be called when an option is selected." }</li>
            </ul>
        </div>
    }
}
