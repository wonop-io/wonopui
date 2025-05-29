use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(NotificationThemeEditor)]
pub fn notification_theme_editor() -> Html {
    let fields = vec![
        (
            "notification_container".to_string(),
            "Notification Container".to_string(),
        ),
        (
            "notification_content".to_string(),
            "Notification Content".to_string(),
        ),
        (
            "notification_title".to_string(),
            "Notification Title".to_string(),
        ),
        (
            "notification_description".to_string(),
            "Notification Description".to_string(),
        ),
        (
            "notification_timestamp".to_string(),
            "Notification Timestamp".to_string(),
        ),
        (
            "notification_close_button".to_string(),
            "Close Button".to_string(),
        ),
        (
            "notification_close_icon".to_string(),
            "Close Icon".to_string(),
        ),
        (
            "notification_action_container".to_string(),
            "Action Container".to_string(),
        ),
        (
            "notification_list_container".to_string(),
            "Notification List Container".to_string(),
        ),
    ];

    let preview = html! {
        <NotificationProvider>
            <NotificationDemo />
        </NotificationProvider>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(NotificationDemo)]
fn notification_demo() -> Html {
    let show_notification = use_notify();

    let on_click_simple = {
        let show_notification = show_notification.clone();
        Callback::from(move |_| {
            show_notification.emit((
                "Simple Notification".to_string(),
                "This is a simple notification without an action.".to_string(),
                None,
            ));
        })
    };

    let on_click_with_action = {
        let show_notification = show_notification.clone();
        Callback::from(move |_| {
            show_notification.emit((
                "Notification with Action".to_string(),
                "This notification includes an action button.".to_string(),
                Some(html! {
                    <button class="text-blue-500 hover:text-blue-700">{"Take Action"}</button>
                }),
            ));
        })
    };

    let on_click_long = {
        let show_notification = show_notification.clone();
        Callback::from(move |_| {
            show_notification.emit((
                "Long Notification".to_string(),
                "This is a longer notification to demonstrate how the component handles more content. It might wrap to multiple lines depending on the width of the notification.".to_string(),
                Some(html! {
                    <button class="text-green-500 hover:text-green-700">{"Dismiss"}</button>
                }),
            ));
        })
    };

    html! {
        <div class="space-x-4">
            <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click_simple}>
                { "Show Simple Notification" }
            </button>
            <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click_with_action}>
                { "Show Notification with Action" }
            </button>
            <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click_long}>
                { "Show Long Notification" }
            </button>
        </div>
    }
}

#[function_component(NotificationDocumentation)]
pub fn notification_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Notification Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Notification component is a versatile UI element that displays notifications with a title, description, and optional action. It provides a flexible and customizable experience for showing various types of notifications to users." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <NotificationProvider>
                        <NotificationDemo />
                    </NotificationProvider>
                }}
                customize={html! {
                    <NotificationThemeEditor />
                }}
                code={r#"
use wonopui::*;
use yew::prelude::*;

#[function_component(NotificationDemo)]
fn notification_demo() -> Html {
    let show_notification = use_notify();

    let on_click_simple = {
        let show_notification = show_notification.clone();
        Callback::from(move |_| {
            show_notification.emit((
                "Simple Notification".to_string(),
                "This is a simple notification without an action.".to_string(),
                None,
            ));
        })
    };

    let on_click_with_action = {
        let show_notification = show_notification.clone();
        Callback::from(move |_| {
            show_notification.emit((
                "Notification with Action".to_string(),
                "This notification includes an action button.".to_string(),
                Some(html! {
                    <button class="text-blue-500 hover:text-blue-700">{"Take Action"}</button>
                }),
            ));
        })
    };

    html! {
        <div class="space-x-4">
            <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click_simple}>
                { "Show Simple Notification" }
            </button>
            <button class="border border-gray-300 rounded px-4 py-2" onclick={on_click_with_action}>
                { "Show Notification with Action" }
            </button>
        </div>
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

            <Features features={vec!["Notification", "NotificationProvider", "use_notify"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="NotificationProvider"
                description="The provider component that manages the notification state and context."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the provider."),
                ]}
            />

            <ApiSection
                title="use_notify"
                description="A hook that provides access to the show_notification function."
                props={vec![
                    ("Returns", "Callback<(String, String, Option<Html>)>", "A callback to show a notification."),
                    ("Parameters", "(title: String, description: String, action: Option<Html>)", "The parameters for the notification."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Wrap your application or a part of it with the NotificationProvider to enable notifications.".to_string(),
                    "Use the use_notify hook in child components to access the show_notification function.".to_string(),
                    "Notifications are displayed in a stack, with the most recent notification at the top.".to_string(),
                    "Each notification has a close button to dismiss it manually.".to_string(),
                    "The action parameter allows you to add custom interactive elements to the notification.".to_string(),
                    "Notifications display a human-readable timestamp showing how long ago they were created.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Notification".to_string()}
                class_descriptions={vec![
                    ("notification_container".to_string(), "The main container for each individual notification".to_string()),
                    ("notification_content".to_string(), "Container for the notification's content (title, description, timestamp)".to_string()),
                    ("notification_title".to_string(), "Styles for the notification title".to_string()),
                    ("notification_description".to_string(), "Styles for the notification description".to_string()),
                    ("notification_timestamp".to_string(), "Styles for the notification timestamp".to_string()),
                    ("notification_close_button".to_string(), "Styles for the close button".to_string()),
                    ("notification_close_icon".to_string(), "Styles for the close icon within the close button".to_string()),
                    ("notification_action_container".to_string(), "Container for the optional action element".to_string()),
                    ("notification_list_container".to_string(), "The container for the entire list of notifications".to_string()),
                ]}
            />
        </Container>
    }
}
