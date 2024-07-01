use crate::config::BRANDGUIDE;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectState {
    pub is_open: bool,
    pub toggle: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct SelectTriggerProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub placeholder: String,
}

#[derive(Properties, PartialEq)]
pub struct SelectContentProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct SelectGroupProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct SelectLabelProps {
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct SelectItemProps {
    pub value: String,
    pub label: String,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let is_open = use_state(|| false);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let state = Rc::new(SelectState {
        is_open: *is_open,
        toggle,
    });

    html! {
        <ContextProvider<Rc<SelectState>> context={state}>
            <div class={BRANDGUIDE.select_container.clone()}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<SelectState>>>
    }
}

#[function_component(SelectTrigger)]
pub fn select_trigger(props: &SelectTriggerProps) -> Html {
    let state = use_context::<Rc<SelectState>>().expect("no context found for SelectState");

    let onclick = {
        let toggle = state.toggle.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    html! {
        <button type="button" class={format!("{} {}", props.class, BRANDGUIDE.select_trigger.clone())} {onclick}>
            <span class={BRANDGUIDE.select_trigger_placeholder.clone()}>{ &props.placeholder }</span>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={BRANDGUIDE.select_trigger_icon.clone()} aria-hidden="true">
                <path d="m6 9 6 6 6-6"></path>
            </svg>
        </button>
    }
}

#[function_component(SelectContent)]
pub fn select_content(props: &SelectContentProps) -> Html {
    let state = use_context::<Rc<SelectState>>().expect("no context found for SelectState");

    if !state.is_open {
        return html! {};
    }

    html! {
        <div class={BRANDGUIDE.select_content_container.clone()}>
            <ul class={BRANDGUIDE.select_content_list.clone()}>
                { for props.children.iter() }
            </ul>
        </div>
    }
}

#[function_component(SelectGroup)]
pub fn select_group(props: &SelectGroupProps) -> Html {
    html! {
        <li class={BRANDGUIDE.select_group.clone()}>
            { for props.children.iter() }
        </li>
    }
}

#[function_component(SelectLabel)]
pub fn select_label(props: &SelectLabelProps) -> Html {
    html! {
        <div class={BRANDGUIDE.select_label.clone()}>
            { &props.label }
        </div>
    }
}

#[function_component(SelectItem)]
pub fn select_item(props: &SelectItemProps) -> Html {
    html! {
        <li class={BRANDGUIDE.select_item.clone()}>
            { &props.label }
        </li>
    }
}

// New component names to add to BRANDGUIDE:
// select_container: "relative inline-block text-left"
// select_trigger: "flex h-10 items-center justify-between rounded-md border border-gray-300 bg-white px-3 py-2 text-sm ring-offset-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-[180px]"
// select_trigger_placeholder: "pointer-events-none"
// select_trigger_icon: "lucide lucide-chevron-down h-4 w-4 opacity-50"
// select_content_container: "absolute mt-1 w-full rounded-md bg-white shadow-lg z-10"
// select_content_list: "max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm"
// select_group: "text-gray-900"
// select_label: "px-4 py-2 text-sm text-gray-700"
// select_item: "text-gray-900 cursor-pointer select-none relative py-2 pl-3 pr-9 hover:bg-gray-100"
