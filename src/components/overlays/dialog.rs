#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogContext {
    pub is_open: bool,
    pub toggle: Callback<Vec<String>>,
    pub open_id: Vec<String>,
}

/// Hook to manually open and close dialogs programmatically
#[hook]
pub fn use_dialog() -> (Callback<String>, Callback<()>) {
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

    (open_dialog, close_dialog)
}

#[derive(Properties, PartialEq)]
pub struct DialogProviderProps {
    pub children: Children,
}

#[function_component(DialogProvider)]
pub fn dialog_provider(props: &DialogProviderProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let is_open = use_state(|| false);
    let open_id = use_state(|| Vec::new());
    let toggle = {
        let is_open = is_open.clone();
        let open_id = open_id.clone();
        Callback::from(move |v| {
            open_id.set(v);
            is_open.set(!*is_open)
        })
    };

    let context = Rc::new(DialogContext {
        is_open: *is_open,
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

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let context = use_context::<Rc<DialogContext>>().expect("no context found");

    if !context.is_open || context.open_id.last() != Some(&props.id) {
        return html! {};
    }

    html! {
        <div class={&brandguide.dialog_container}>
            <div class={&brandguide.dialog_content} ref={node_ref}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogHeaderProps {
    pub children: Children,
}

#[function_component(DialogHeader)]
pub fn dialog_header(props: &DialogHeaderProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={&brandguide.dialog_header}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogTitleProps {
    pub children: Children,
}

#[function_component(DialogTitle)]
pub fn dialog_title(props: &DialogTitleProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h2 class={&brandguide.dialog_title}>
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogBodyProps {
    pub children: Children,
}

#[function_component(DialogBody)]
pub fn dialog_body(props: &DialogBodyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <p class={&brandguide.dialog_description}>
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogFooterProps {
    pub children: Children,
}

#[function_component(DialogFooter)]
pub fn dialog_footer(props: &DialogFooterProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={&brandguide.dialog_footer}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogCloseProps {
    pub children: Children,
}

#[function_component(DialogClose)]
pub fn dialog_close(props: &DialogCloseProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
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
        <div {onclick}>
            { for props.children.iter() }
        </div>
    }
}
