use gloo_console as console;
use std::rc::Rc;
use yew::html::ChildrenProps;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NotificationContext {
    pub show_notification: Callback<NotificationProps>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationProps {
    pub title: String,
    pub description: String,
    pub action: Option<Html>,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    console::log!("Rendering notification");
    html! {
        <div class="fixed bottom-4 right-4 z-50 max-w-sm w-full bg-white shadow-lg rounded-lg p-4">
            <div class="flex justify-between items-center">
                <div>
                    <h2 class="text-lg font-semibold text-gray-900">{ &props.title }</h2>
                    <p class="text-sm text-gray-600">{ &props.description }</p>
                </div>
                { if let Some(action) = &props.action {
                    html! { <div>{ action.clone() }</div> }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationProviderProps {
    pub children: Children,
}

#[function_component(NotificationProvider)]
pub fn notification_provider(props: &NotificationProviderProps) -> Html {
    let notification_state = use_state(|| None);

    let show_notification = {
        let notification_state = notification_state.clone();
        Callback::from(move |notification: NotificationProps| {
            notification_state.set(Some(notification));
        })
    };

    let context = Rc::new(NotificationContext {
        show_notification: show_notification.clone(),
    });

    html! {
        <ContextProvider<Rc<NotificationContext>> context={context}>
            { for props.children.iter() }
            { if let Some(notification) = &*notification_state {
                html! { <Notification title={notification.title.clone()} description={notification.description.clone()} action={notification.action.clone()} /> }
            } else {
                html! {}
            }}
        </ContextProvider<Rc<NotificationContext>>>
    }
}

// New entries in the brand guide (to be added in config.rs):
// notification_container: "fixed bottom-4 right-4 z-50 max-w-sm w-full bg-white shadow-lg rounded-lg p-4",
// notification_title: "text-lg font-semibold text-gray-900",
// notification_description: "text-sm text-gray-600",
// notification_button: "border border-gray-300 rounded px-4 py-2",
// notification_action: "text-blue-500"
