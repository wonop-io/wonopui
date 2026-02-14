//! Notification component for wonopui
//!
//! Toast-style notifications that appear in a corner of the screen.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const NOTIFICATION_LIST_CONTAINER: &str = "fixed bottom-4 right-4 z-50 flex flex-col gap-2 max-w-sm";
    pub const NOTIFICATION_CONTAINER: &str = "bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-600 rounded-lg shadow-lg p-4";
    pub const NOTIFICATION_CONTENT: &str = "flex items-start justify-between gap-4";
    pub const NOTIFICATION_TITLE: &str = "text-sm font-semibold text-gray-900 dark:text-zinc-100";
    pub const NOTIFICATION_DESCRIPTION: &str = "text-sm text-gray-600 dark:text-zinc-400 mt-1";
    pub const NOTIFICATION_CLOSE_BUTTON: &str = "text-gray-400 hover:text-gray-600 dark:text-zinc-500 dark:hover:text-zinc-300";
    pub const NOTIFICATION_CLOSE_ICON: &str = "w-5 h-5";
    pub const NOTIFICATION_ACTION_CONTAINER: &str = "mt-3 pt-3 border-t border-gray-100 dark:border-zinc-700";
}

#[derive(Clone, PartialEq)]
pub struct NotificationContext {
    pub show_notification: Callback<(String, String, Option<Html>)>,
    pub remove_notification: Callback<usize>,
}

#[derive(Clone, PartialEq)]
pub struct NotificationData {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub action: Option<Html>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationProps {
    pub id: usize,
    pub title: String,
    pub description: String,
    #[prop_or_default]
    pub action: Option<Html>,
    pub on_close: Callback<usize>,
    #[prop_or_default]
    pub class: Classes,
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

    let container_class = merge_classes(&[
        classes::NOTIFICATION_CONTAINER,
        &props.class.to_string(),
    ]);

    html! {
        <div class={container_class}>
            <div class={classes::NOTIFICATION_CONTENT}>
                <div>
                    <h2 class={classes::NOTIFICATION_TITLE}>{ &props.title }</h2>
                    <p class={classes::NOTIFICATION_DESCRIPTION}>{ &props.description }</p>
                </div>
                <button onclick={on_close} class={classes::NOTIFICATION_CLOSE_BUTTON}>
                    <svg xmlns="http://www.w3.org/2000/svg" class={classes::NOTIFICATION_CLOSE_ICON} viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
            { if let Some(action) = &props.action {
                html! { <div class={classes::NOTIFICATION_ACTION_CONTAINER}>{ action.clone() }</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NotificationProviderProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(NotificationProvider)]
pub fn notification_provider(props: &NotificationProviderProps) -> Html {
    let notifications = use_state(Vec::<NotificationData>::new);
    let next_id = use_state(|| 0usize);

    let show_notification = {
        let notifications = notifications.clone();
        let next_id = next_id.clone();
        Callback::from(move |args: (String, String, Option<Html>)| {
            let (title, description, action) = args;
            let id = *next_id;
            next_id.set(id + 1);
            let notification = NotificationData {
                id,
                title,
                description,
                action,
            };
            notifications.set(
                (*notifications)
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(notification))
                    .collect(),
            );
        })
    };

    let remove_notification = {
        let notifications = notifications.clone();
        Callback::from(move |id: usize| {
            notifications.set(
                (*notifications)
                    .clone()
                    .into_iter()
                    .filter(|n| n.id != id)
                    .collect(),
            );
        })
    };

    let context = Rc::new(NotificationContext {
        show_notification: show_notification.clone(),
        remove_notification: remove_notification.clone(),
    });

    let list_class = merge_classes(&[
        classes::NOTIFICATION_LIST_CONTAINER,
        &props.class.to_string(),
    ]);

    html! {
        <ContextProvider<Rc<NotificationContext>> context={context}>
            { for props.children.iter() }
            <div class={list_class}>
                { for notifications.iter().rev().map(|notification| {
                    let on_close = remove_notification.clone();
                    html! { 
                        <Notification
                            id={notification.id}
                            title={notification.title.clone()}
                            description={notification.description.clone()}
                            action={notification.action.clone()}
                            on_close={on_close}
                        /> 
                    }
                })}
            </div>
        </ContextProvider<Rc<NotificationContext>>>
    }
}

/// Hook to get access to the notification API
#[hook]
pub fn use_notify() -> Callback<(String, String, Option<Html>)> {
    use_context::<Rc<NotificationContext>>()
        .expect("NotificationContext not found")
        .show_notification
        .clone()
}

/// Hook to get the full notification context
#[hook]
pub fn use_notification_context() -> Rc<NotificationContext> {
    use_context::<Rc<NotificationContext>>()
        .expect("NotificationContext not found")
}
