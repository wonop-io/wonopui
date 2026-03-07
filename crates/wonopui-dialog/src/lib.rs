//! Dialog component for WonopUI.
//!
//! A modal dialog component with provider pattern for state management.

use std::rc::Rc;
use wonopui_core::*;

/// Default CSS classes for dialog styling.
pub mod classes {
    /// Dialog overlay/backdrop container - shadcn v4 style.
    pub const OVERLAY: &str = "fixed inset-0 z-50 bg-black/50 backdrop-blur-[2px] data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0";

    /// Dialog container for centering content.
    pub const CONTAINER: &str = "fixed inset-0 z-50 flex items-center justify-center overflow-auto";

    /// Dialog content container - shadcn v4 style with animations.
    pub const CONTENT: &str = "relative bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 grid w-full max-w-[calc(100%-2rem)] sm:max-w-lg gap-4 rounded-lg border border-zinc-200 dark:border-zinc-800 p-6 shadow-lg outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 duration-200";

    /// Dialog close button - shadcn v4 style.
    pub const CLOSE_BUTTON: &str = "absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:outline-none focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 text-zinc-500 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-50";

    /// Dialog header - shadcn v4 style.
    pub const HEADER: &str = "flex flex-col gap-2 text-center sm:text-left";

    /// Dialog title - shadcn v4 style.
    pub const TITLE: &str = "text-lg leading-none font-semibold text-zinc-950 dark:text-zinc-50";

    /// Dialog description/body - shadcn v4 style.
    pub const DESCRIPTION: &str = "text-sm text-zinc-500 dark:text-zinc-400";

    /// Dialog footer - shadcn v4 style.
    pub const FOOTER: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";
}

/// Context for managing dialog state.
#[derive(Clone, PartialEq)]
pub struct DialogContext {
    pub toggle: Callback<Vec<String>>,
    pub open_id: Vec<String>,
}

/// Hook to manually open and close dialogs programmatically.
#[hook]
pub fn use_dialog() -> (Callback<String>, Callback<()>, Option<String>) {
    let context = use_context::<Rc<DialogContext>>()
        .expect("DialogContext not found. Wrap your component with DialogProvider");

    let open_dialog = {
        let toggle = context.toggle.clone();
        let open_id = context.open_id.clone();
        Callback::from(move |id: String| {
            let mut new_open_id = open_id.clone();
            new_open_id.push(id);
            toggle.emit(new_open_id);
        })
    };

    let close_dialog = {
        let toggle = context.toggle.clone();
        let open_id = context.open_id.clone();
        Callback::from(move |_| {
            let mut new_open_id = open_id.clone();
            if !new_open_id.is_empty() {
                new_open_id.pop();
            }
            toggle.emit(new_open_id);
        })
    };

    let currently_open = context.open_id.last().cloned();

    (open_dialog, close_dialog, currently_open)
}

#[derive(Properties, PartialEq)]
pub struct DialogProviderProps {
    pub children: Children,
}

/// Provider component for dialog state management.
#[function_component(DialogProvider)]
pub fn dialog_provider(props: &DialogProviderProps) -> Html {
    let open_id = use_state(Vec::new);
    let toggle = {
        let open_id = open_id.clone();
        Callback::from(move |v| {
            open_id.set(v);
        })
    };

    let context = Rc::new(DialogContext {
        toggle: toggle.clone(),
        open_id: (*open_id).clone(),
    });

    html! {
        <ContextProvider<Rc<DialogContext>> context={context}>
            { for props.children.iter() }
        </ContextProvider<Rc<DialogContext>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogTriggerProps {
    pub children: Children,
    pub id: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
}

/// Button/element that triggers a dialog to open.
#[function_component(DialogTrigger)]
pub fn dialog_trigger(props: &DialogTriggerProps) -> Html {
    let context = use_context::<Rc<DialogContext>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        let id = props.id.clone();
        let open_id = context.open_id.clone();
        Callback::from(move |_| {
            let mut new_open_id = open_id.clone();
            if new_open_id.is_empty() || new_open_id.last() != Some(&id) {
                new_open_id.push(id.clone());
            } else {
                new_open_id.pop();
            }
            toggle.emit(new_open_id)
        })
    };

    html! {
        <@{props.tag.clone()} class={props.class.clone()} {onclick}>
            { for props.children.iter() }
        </@>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    pub children: Children,
    pub id: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// The dialog modal component.
#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    let context = use_context::<Rc<DialogContext>>().expect("no context found");

    let is_open = !context.open_id.is_empty() && context.open_id.last() == Some(&props.id);
    let state = if is_open { "open" } else { "closed" };

    if !is_open {
        return html! {};
    }

    html! {
        <>
            <div data-slot="dialog-overlay" data-state={state} class={classes::OVERLAY} />
            <div data-slot="dialog-portal" class={classes::CONTAINER}>
                <div data-slot="dialog-content" data-state={state} class={classes::CONTENT} ref={props.node_ref.clone()}>
                    { for props.children.iter() }
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogHeaderProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DialogHeader)]
pub fn dialog_header(props: &DialogHeaderProps) -> Html {
    html! {
        <div data-slot="dialog-header" class={classes!(classes::HEADER, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogTitleProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DialogTitle)]
pub fn dialog_title(props: &DialogTitleProps) -> Html {
    html! {
        <h2 data-slot="dialog-title" class={classes!(classes::TITLE, props.class.clone())}>
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogBodyProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DialogBody)]
pub fn dialog_body(props: &DialogBodyProps) -> Html {
    html! {
        <p data-slot="dialog-description" class={classes!(classes::DESCRIPTION, props.class.clone())}>
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogFooterProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DialogFooter)]
pub fn dialog_footer(props: &DialogFooterProps) -> Html {
    html! {
        <div data-slot="dialog-footer" class={classes!(classes::FOOTER, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogCloseProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// Button that closes the currently open dialog.
#[function_component(DialogClose)]
pub fn dialog_close(props: &DialogCloseProps) -> Html {
    let context = use_context::<Rc<DialogContext>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        let open_id = context.open_id.clone();
        Callback::from(move |_| {
            let mut new_open_id = open_id.clone();
            new_open_id.pop();
            toggle.emit(new_open_id)
        })
    };

    html! {
        <button data-slot="dialog-close" type="button" class={classes!(classes::CLOSE_BUTTON, props.class.clone())} {onclick}>
            { for props.children.iter() }
        </button>
    }
}
