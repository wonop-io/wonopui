//! Notification component for wonopui
//!
//! Toast-style notifications that appear in a corner of the screen.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for notification/toast styling (shadcn v4 style).
pub mod classes {
    /// List container styles - shadcn v4 Toaster with more spacing.
    pub const NOTIFICATION_LIST_CONTAINER: &str = "fixed bottom-6 right-6 z-50 flex flex-col gap-4 w-full max-w-[420px] pointer-events-none";
    
    /// Individual notification container - premium styling with more padding.
    pub const NOTIFICATION_CONTAINER: &str = "group pointer-events-auto relative flex w-full items-center justify-between gap-5 overflow-hidden rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 p-5 pr-12 shadow-xl transition-all duration-300 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-80 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-right-full data-[state=open]:slide-in-from-right-full";
    
    /// Success variant.
    pub const NOTIFICATION_SUCCESS: &str = "border-green-500/50 bg-green-50 dark:bg-green-950/50 text-green-900 dark:text-green-100";
    
    /// Error variant.
    pub const NOTIFICATION_ERROR: &str = "border-red-500/50 bg-red-50 dark:bg-red-950/50 text-red-900 dark:text-red-100";
    
    /// Warning variant.
    pub const NOTIFICATION_WARNING: &str = "border-yellow-500/50 bg-yellow-50 dark:bg-yellow-950/50 text-yellow-900 dark:text-yellow-100";
    
    /// Content wrapper styles with more gap.
    pub const NOTIFICATION_CONTENT: &str = "flex flex-col gap-1.5";
    
    /// Title styles - premium with better line-height.
    pub const NOTIFICATION_TITLE: &str = "text-sm font-semibold leading-5 tracking-tight";
    
    /// Description styles - better line-height for readability.
    pub const NOTIFICATION_DESCRIPTION: &str = "text-sm text-zinc-500 dark:text-zinc-400 leading-relaxed";
    
    /// Close button styles - premium with better positioning.
    pub const NOTIFICATION_CLOSE_BUTTON: &str = "absolute right-4 top-4 rounded-md p-1 text-zinc-400 dark:text-zinc-500 opacity-0 transition-all duration-200 hover:text-zinc-900 dark:hover:text-zinc-50 hover:bg-zinc-100 dark:hover:bg-zinc-800 focus:opacity-100 focus:outline-none focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-2 group-hover:opacity-100 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
    
    /// Close icon styles.
    pub const NOTIFICATION_CLOSE_ICON: &str = "size-4";
    
    /// Action container styles - shadcn v4 ToastAction.
    pub const NOTIFICATION_ACTION_CONTAINER: &str = "shrink-0";
    
    /// Action button styles - premium with better padding.
    pub const NOTIFICATION_ACTION_BUTTON: &str = "inline-flex items-center justify-center rounded-lg text-sm font-medium px-4 py-2 border border-zinc-200 dark:border-zinc-800 bg-transparent hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-all duration-200 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-2 outline-none";
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

    let container_class =
        merge_classes(&[classes::NOTIFICATION_CONTAINER, &props.class.to_string()]);

    html! {
        <div data-slot="toast" data-state="open" class={container_class} role="alert">
            <div class={classes::NOTIFICATION_CONTENT}>
                <h2 data-slot="toast-title" class={classes::NOTIFICATION_TITLE}>{ &props.title }</h2>
                <p data-slot="toast-description" class={classes::NOTIFICATION_DESCRIPTION}>{ &props.description }</p>
            </div>
            { if let Some(action) = &props.action {
                html! { <div data-slot="toast-action" class={classes::NOTIFICATION_ACTION_CONTAINER}>{ action.clone() }</div> }
            } else {
                html! {}
            }}
            <button data-slot="toast-close" type="button" onclick={on_close} class={classes::NOTIFICATION_CLOSE_BUTTON}>
                <svg xmlns="http://www.w3.org/2000/svg" class={classes::NOTIFICATION_CLOSE_ICON} viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
            </button>
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
            <div data-slot="toaster" class={list_class}>
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
    use_context::<Rc<NotificationContext>>().expect("NotificationContext not found")
}
