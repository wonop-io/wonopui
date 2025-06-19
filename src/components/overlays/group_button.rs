#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::properties::FlexDirection;
use std::rc::Rc;
use yew::function_component;
use yew::html;
use yew::prelude::*;

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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    #[cfg(not(feature = "ssr"))]
    let active_button = use_state(|| props.default_value.clone());
    #[cfg(feature = "ssr")]
    let active_button_value = props.default_value.clone();

    #[cfg(not(feature = "ssr"))]
    let set_active_button = {
        let active_button = active_button.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |new_button: String| {
            active_button.set(new_button.clone());
            on_change.emit(new_button);
        })
    };
    #[cfg(feature = "ssr")]
    let set_active_button = {
        let on_change = props.on_change.clone();
        Callback::from(move |new_button: String| {
            on_change.emit(new_button);
        })
    };

    let state = Rc::new(GroupButtonState {
        #[cfg(not(feature = "ssr"))]
        active_button: (*active_button).clone(),
        #[cfg(feature = "ssr")]
        active_button: active_button_value,
        set_active_button,
    });

    html! {
        <ContextProvider<Rc<GroupButtonState>> context={state}>
            <div role="group" class={classes!(props.class.clone(), &brandguide.group_button_container, props.direction.to_string(), &brandguide.group_button_list)}>
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
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
                &brandguide.group_button_trigger,
                if is_active { &brandguide.group_button_trigger_active } else { &brandguide.group_button_trigger_inactive },
            )}
        >
            { for props.children.iter() }
        </button>
    }
}

// Snippets to update brandguide:
// ("group_button_container".to_string(), "flex w-full".to_string()),
// ("group_button_list".to_string(), "h-10 items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600".to_string()),
// ("group_button_trigger".to_string(), "inline-flex items-center justify-center whitespace-nowrap px-3 py-1.5 text-sm font-medium transition-all ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50".to_string()),
// ("group_button_trigger_active".to_string(), "bg-white text-black shadow-sm".to_string()),
// ("group_button_trigger_inactive".to_string(), "bg-gray-200 text-gray-600".to_string()),
// ("group_button_demo".to_string(), "w-[400px]".to_string()),
//
// pub group_button_container: ClassesContainer<T>,
// pub group_button_list: ClassesContainer<T>,
// pub group_button_trigger: ClassesContainer<T>,
// pub group_button_trigger_active: ClassesContainer<T>,
// pub group_button_trigger_inactive: ClassesContainer<T>,
// pub group_button_demo: ClassesContainer<T>,
//
// group_button_container: self.group_button_container.to_owned(),
// group_button_list: self.group_button_list.to_owned(),
// group_button_trigger: self.group_button_trigger.to_owned(),
// group_button_trigger_active: self.group_button_trigger_active.to_owned(),
// group_button_trigger_inactive: self.group_button_trigger_inactive.to_owned(),
// group_button_demo: self.group_button_demo.to_owned(),
//
// group_button_container: default_config_hm
// .get("group_button_container")
// .expect("Template parameter missing")
// .clone(),
// group_button_list: default_config_hm
// .get("group_button_list")
// .expect("Template parameter missing")
// .clone(),
// group_button_trigger: default_config_hm
// .get("group_button_trigger")
// .expect("Template parameter missing")
// .clone(),
// group_button_trigger_active: default_config_hm
// .get("group_button_trigger_active")
// .expect("Template parameter missing")
// .clone(),
// group_button_trigger_inactive: default_config_hm
// .get("group_button_trigger_inactive")
// .expect("Template parameter missing")
// .clone(),
// group_button_demo: default_config_hm
// .get("group_button_demo")
// .expect("Template parameter missing")
// .clone(),
