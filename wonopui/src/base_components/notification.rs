use gloo_console as console;
use std::rc::Rc;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NotificationContext {
    pub show_notification: Callback<(String, String, Option<Html>)>,
    pub remove_notification: Callback<usize>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationProps {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub action: Option<Html>,
    pub timestamp: DateTime<Utc>,
    pub on_close: Callback<usize>,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let on_close = {
        let id = props.id;
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(id);
        })
    };

    let human_time = HumanTime::from(props.timestamp);

    html! {
        <div class="max-w-sm w-full bg-white shadow-lg rounded-lg p-3 sm:p-4 md:p-6 relative">
            <div class="flex justify-between items-start">
                <div>
                    <h2 class="text-lg font-semibold text-gray-900">{ &props.title }</h2>
                    <p class="text-sm text-gray-600">{ &props.description }</p>
                    <p class="text-xs text-gray-400">{ human_time.to_string() }</p>
                </div>
                <button onclick={on_close} class="absolute top-4 right-4 text-gray-500 hover:text-gray-700">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
            { if let Some(action) = &props.action {
                html! { <div class="mt-2">{ action.clone() }</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationProviderProps {
    pub children: Children,
}

#[function_component(NotificationProvider)]
pub fn notification_provider(props: &NotificationProviderProps) -> Html {
    let notifications = use_state(|| Vec::new());
    let next_id = use_state(|| 0usize);

    let show_notification = {
        let notifications = notifications.clone();
        let next_id = next_id.clone();
        Callback::from(move |args: (String, String, Option<Html>)| {
            let (title, description, action) = args;
            let id = *next_id;
            next_id.set(id + 1);
            let notification = NotificationProps {
                id,
                title,
                description,
                action,
                timestamp: Utc::now(),
                on_close: Callback::noop(), // This will be set later
            };
            notifications.set((*notifications).clone().into_iter().chain(std::iter::once(notification)).collect());
        })
    };

    let remove_notification = {
        let notifications = notifications.clone();
        Callback::from(move |id: usize| {
            notifications.set((*notifications).clone().into_iter().filter(|n| n.id != id).collect());
        })
    };

    let context = Rc::new(NotificationContext {
        show_notification: show_notification.clone(),
        remove_notification: remove_notification.clone(),
    });

    html! {
        <ContextProvider<Rc<NotificationContext>> context={context}>
            { for props.children.iter() }
            <div class="fixed bottom-4 right-4 z-50 space-y-4 flex flex-col">
                { for notifications.iter().rev().map(|notification| {
                    let on_close = remove_notification.clone();
                    html! { <Notification
                        id={notification.id}
                        title={notification.title.clone()}
                        description={notification.description.clone()}
                        action={notification.action.clone()}
                        timestamp={notification.timestamp}
                        on_close={on_close}
                    /> }
                })}
            </div>
        </ContextProvider<Rc<NotificationContext>>>
    }
}

// New entries in the brand guide (to be added in config.rs):
// notification_container: "fixed bottom-4 right-4 z-50 max-w-sm w-full bg-white shadow-lg rounded-lg p-4",
// notification_title: "text-lg font-semibold text-gray-900",
// notification_description: "text-sm text-gray-600",
// notification_timestamp: "text-xs text-gray-400",
// notification_button: "border border-gray-300 rounded px-4 py-2",
// notification_action: "text-blue-500"

#[hook]
pub fn use_notify() -> Callback<(String, String, Option<Html>)> {
    use_context::<Rc<NotificationContext>>()
        .expect("NotificationContext not found")
        .show_notification
        .clone()
}

/*
// Example usage:

#[function_component(ExampleComponent)]
pub fn example_component() -> Html {
    let show_notification = use_notify();

    let on_click = Callback::from(move |_| {
        show_notification.emit((
            "Example Notification".to_string(),
            "This is an example of using the notification system.".to_string(),
            Some(html! {
                <button class="text-blue-500">{"Action"}</button>
            })
        ));
    });

    html! {
        <div>
            <button onclick={on_click}>{"Show Notification"}</button>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <NotificationProvider>
            <ExampleComponent />
            // Other components...
        </NotificationProvider>
    }
}
*/