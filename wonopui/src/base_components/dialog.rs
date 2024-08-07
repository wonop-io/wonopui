use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogContext {
    pub is_open: bool,
    pub toggle: Callback<Vec<String>>,
    pub open_id: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    pub children: Children,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
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
            // Here we would need to update the context's open_id, but since it's in an Rc,
            // we can't mutate it directly. In a real implementation, you might want to use
            // a state management solution that allows updating the context.
            toggle.emit(new_open_id)
        })
    };

    html! {
        <div {onclick}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogContentProps {
    pub children: Children,
    pub id: String,
}

#[function_component(DialogContent)]
pub fn dialog_content(props: &DialogContentProps) -> Html {
    let context = use_context::<Rc<DialogContext>>().expect("no context found");

    if !context.is_open || context.open_id.last() != Some(&props.id) {
        return html! {};
    }

    html! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75">
            <div class="bg-white rounded-lg shadow-lg max-w-md w-full">
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
    html! {
        <div class="p-4 border-b border-gray-200">
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
    html! {
        <h2 class="text-lg font-semibold text-gray-900">
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
    html! {
        <p class="text-sm text-gray-600 p-4">
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
    html! {
        <div class="p-4 border-t border-gray-200 flex flex-row">
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

// New entries in the brand guide (to be added in config.rs):
// dialog_container: "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75",
// dialog_content: "bg-white rounded-lg shadow-lg max-w-md w-full",
// dialog_header: "p-4 border-b border-gray-200",
// dialog_title: "text-lg font-semibold text-gray-900",
// dialog_description: "text-sm text-gray-600",
// dialog_footer: "p-4 border-t border-gray-200"
