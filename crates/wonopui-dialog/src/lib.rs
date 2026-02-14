//! Dialog component for WonopUI.
//!
//! A modal dialog component with provider pattern for state management.

use std::rc::Rc;
use wonopui_core::*;

/// Default CSS classes for dialog styling.
pub mod classes {
    /// Dialog overlay/backdrop container.
    pub const CONTAINER: &str = "fixed inset-0 z-50 flex items-center justify-center bg-zinc-900/80 dark:bg-zinc-950/90 backdrop-blur-sm overflow-auto pointer-events-auto";
    
    /// Dialog content container.
    pub const CONTENT: &str = "bg-white dark:bg-zinc-800 rounded-md shadow-md max-w-md w-full border border-zinc-200 dark:border-zinc-700 transition-all duration-300 ease-out transform";
    
    /// Dialog header.
    pub const HEADER: &str = "p-4 border-b border-zinc-200 dark:border-zinc-700 flex items-center justify-between";
    
    /// Dialog title.
    pub const TITLE: &str = "text-lg font-semibold text-zinc-900 dark:text-zinc-100";
    
    /// Dialog description/body.
    pub const DESCRIPTION: &str = "text-sm text-zinc-600 dark:text-zinc-400 p-4";
    
    /// Dialog footer.
    pub const FOOTER: &str = "p-4 border-t border-zinc-200 dark:border-zinc-700 flex justify-end space-x-2";
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

    let extra_classes = if context.open_id.is_empty() || context.open_id.last() != Some(&props.id) {
        "hidden"
    } else {
        ""
    };

    html! {
        <div class={classes!(classes::CONTAINER, extra_classes)}>
            <div class={classes::CONTENT} ref={props.node_ref.clone()}>
                { for props.children.iter() }
            </div>
        </div>
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
        <div class={classes!(classes::HEADER, props.class.clone())}>
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
        <h2 class={classes!(classes::TITLE, props.class.clone())}>
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
        <p class={classes!(classes::DESCRIPTION, props.class.clone())}>
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
        <div class={classes!(classes::FOOTER, props.class.clone())}>
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
        <div class={props.class.clone()} {onclick}>
            { for props.children.iter() }
        </div>
    }
}
