use crate::config::BRANDGUIDE;
use std::rc::Rc;
use yew::function_component;
use yew::html;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

impl ToString for FlexDirection {
    fn to_string(&self) -> String {
        match self {
            FlexDirection::Row => "flex-row".to_string(),
            FlexDirection::Column => "flex-col".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct GroupButtonProps {
    pub children: Children,
    pub default_value: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(FlexDirection::Row)]
    pub direction: FlexDirection,
    #[prop_or_default]
    pub on_change: Callback<String>,
}

#[derive(Clone, PartialEq)]
pub struct GroupButtonState {
    pub active_button: String,
    pub set_active_button: Callback<String>,
}

#[function_component(GroupButton)]
pub fn group_button(props: &GroupButtonProps) -> Html {
    let active_button = use_state(|| props.default_value.clone());

    let set_active_button = {
        let active_button = active_button.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |new_button: String| {
            active_button.set(new_button.clone());
            on_change.emit(new_button);
        })
    };

    let state = Rc::new(GroupButtonState {
        active_button: (*active_button).clone(),
        set_active_button,
    });

    html! {
        <ContextProvider<Rc<GroupButtonState>> context={state}>
            <div role="group" class={classes!(props.class.clone(), "flex", props.direction.to_string(), "w-full", BRANDGUIDE.group_button_list)}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<GroupButtonState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct GroupButtonTriggerProps {
    pub value: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(GroupButtonTrigger)]
pub fn group_button_trigger(props: &GroupButtonTriggerProps) -> Html {
    let state =
        use_context::<Rc<GroupButtonState>>().expect("no context found for GroupButtonState");

    let onclick = {
        let set_active_button = state.set_active_button.clone();
        let value = props.value.clone();
        let user_onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            set_active_button.emit(value.clone());
            user_onclick.emit(e);
        })
    };

    let is_active = state.active_button == props.value;

    html! {
        <button
            type="button"
            role="button"
            onclick={onclick}
            class={classes!(
                props.class.clone(),
                "inline-flex","items-center","justify-center","whitespace-nowrap","px-3","py-1.5","text-sm","font-medium","transition-all",
                if is_active { "bg-white text-black shadow-sm" } else { "bg-gray-200 text-gray-600" },
                BRANDGUIDE.group_button_trigger
            )}
        >
            { for props.children.iter() }
        </button>
    }
}

#[function_component(GroupButtonDemo)]
pub fn group_button_demo() -> Html {
    let on_change = Callback::from(|value: String| {
        // log::info!("Selected value: {}", value);
    });

    html! {
        <GroupButton default_value="option1" class="w-[400px]" direction={FlexDirection::Row} on_change={on_change}>
            <GroupButtonTrigger value="option1" onclick={Callback::noop()}>{"Option 1"}</GroupButtonTrigger>
            <GroupButtonTrigger value="option2" onclick={Callback::noop()}>{"Option 2"}</GroupButtonTrigger>
        </GroupButton>
    }
}

// New entries in the brand guide:
// group_button_list: "h-10 items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
// group_button_trigger: "ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
