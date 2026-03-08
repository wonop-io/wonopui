use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(AlertDocumentation)]
pub fn alert_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Alert Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Alert component is used to display important messages to the user. It supports different types of alerts such as info, success, warning, and error." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <>
                        <Alert message="This is an info alert" alert_type={AlertType::Info} />
                        <Alert message="This is a success alert" alert_type={AlertType::Success} />
                        <Alert message="This is a warning alert" alert_type={AlertType::Warning} />
                        <Alert message="This is an error alert" alert_type={AlertType::Error} />
                    </>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Alert message="This is an info alert" alert_type={AlertType::Info} />
    <Alert message="This is a success alert" alert_type={AlertType::Success} />
    <Alert message="This is a warning alert" alert_type={AlertType::Warning} />
    <Alert message="This is an error alert" alert_type={AlertType::Error} />
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Alert" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the alert component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "message: String - The message to be displayed inside the alert." }</li>
                <li>{ "alert_type: AlertType - The type of alert to be displayed. It can be one of the following: Info, Success, Warning, Error." }</li>
            </ul>
        </div>
    }
}
