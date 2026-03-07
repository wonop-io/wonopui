use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(NotificationDemo)]
pub fn notification_demo() -> Html {
    let show_notification = use_notify();

    let on_click = Callback::from(move |_| {
        show_notification.emit((
            "Scheduled: Catch up".to_string(),
            "Friday, February 10, 2023 at 5:57 PM".to_string(),
            Some(html! {
                <button class="text-blue-500">{"Undo"}</button>
            }),
        ));
    });

    html! {
        <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click}>
            { "Add to calendar" }
        </button>
    }
}

#[function_component(NotificationDocumentation)]
pub fn notification_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Notification Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Notification component is a versatile UI element that displays notifications with a title, description, and optional action. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <NotificationProvider>
                        <NotificationDemo />
                    </NotificationProvider>
                }}
                code={r#"
use wonopui::*;
use yew::prelude::*;

#[function_component(NotificationDemo)]
pub fn notification_demo() -> Html {
    let show_notification = use_notify();

    let on_click = Callback::from(move |_| {
        show_notification.emit((
            "Scheduled: Catch up".to_string(),
            "Friday, February 10, 2023 at 5:57 PM".to_string(),
            Some(html! {
                <button class="text-blue-500">{"Undo"}</button>
            })
        ));
    });

    html! {
        <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click}>
            { "Add to calendar" }
        </button>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <NotificationProvider>
            <NotificationDemo />
        </NotificationProvider>
    }
}
                "#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "NotificationProvider" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The provider component that manages the notification state and context." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the provider." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "use_notify" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "A hook that provides access to the show_notification function." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "Returns: Callback<(String, String, Option<Html>)> - A callback to show a notification." }</li>
                <li>{ "Parameters: (title: String, description: String, action: Option<Html>)" }</li>
            </ul>
        </div>
    }
}
