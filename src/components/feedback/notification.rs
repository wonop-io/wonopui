use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
#[cfg(not(feature = "ssr"))]
use gloo_console as console;
use std::rc::Rc;
use yew::prelude::*;

#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;

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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let on_close = {
        let id = props.id;
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(id);
        })
    };

    let human_time = HumanTime::from(props.timestamp);

    html! {
        <div class={classes!(&brandguide.notification_container)}>
            <div class={classes!(&brandguide.notification_content)}>
                <div>
                    <h2 class={classes!(&brandguide.notification_title)}>{ &props.title }</h2>
                    <p class={classes!(&brandguide.notification_description)}>{ &props.description }</p>
                    <p class={classes!(&brandguide.notification_timestamp)}>{ human_time.to_string() }</p>
                </div>
                <button onclick={on_close} class={classes!(&brandguide.notification_close_button)}>
                    <svg xmlns="http://www.w3.org/2000/svg" class={classes!(&brandguide.notification_close_icon)} viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
            { if let Some(action) = &props.action {
                html! { <div class={classes!(&brandguide.notification_action_container)}>{ action.clone() }</div> }
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

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

    html! {
        <ContextProvider<Rc<NotificationContext>> context={context}>
            { for props.children.iter() }
            <div class={classes!(&brandguide.notification_list_container)}>
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

#[hook]
pub fn use_notify() -> Callback<(String, String, Option<Html>)> {
    use_context::<Rc<NotificationContext>>()
        .expect("NotificationContext not found")
        .show_notification
        .clone()
}
