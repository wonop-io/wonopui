#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use std::rc::Rc;
use yew::function_component;
use yew::html;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum TabsDirection {
    Row,
    Column,
    Auto,
}

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub children: Children,
    pub default_value: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(TabsDirection::Auto)]
    pub direction: TabsDirection,
}

#[derive(Clone, PartialEq)]
pub struct TabsState {
    pub active_tab: String,
    pub set_active_tab: Callback<String>,
    pub direction: TabsDirection,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let active_tab = use_state(|| props.default_value.clone());

    let set_active_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |new_tab: String| {
            active_tab.set(new_tab);
        })
    };

    let state = Rc::new(TabsState {
        active_tab: (*active_tab).clone(),
        set_active_tab,
        direction: props.direction.clone(),
    });

    html! {
        <ContextProvider<Rc<TabsState>> context={state}>
            <div class={classes!(props.class.clone(), &brandguide.tabs_container)}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<TabsState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsListProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TabsList)]
pub fn tabs_list(props: &TabsListProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");
    let class = match state.direction {
        TabsDirection::Auto | TabsDirection::Row => &brandguide.tabs_list_row,
        TabsDirection::Column => &brandguide.tabs_list_column,
    };
    html! {
        <div class={classes!(props.class.clone(), class, &brandguide.tabs_list)}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsTriggerProps {
    pub value: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TabsTrigger)]
pub fn tabs_trigger(props: &TabsTriggerProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");

    let onclick = {
        let set_active_tab = state.set_active_tab.clone();
        let value = props.value.clone();
        Callback::from(move |_| set_active_tab.emit(value.clone()))
    };

    let is_active = state.active_tab == props.value;

    html! {
        <button
            type="button"
            role="tab"
            onclick={onclick}
            class={classes!(
                props.class.clone(),
                if is_active { &brandguide.tabs_trigger_active } else { &brandguide.tabs_trigger_inactive },
                &brandguide.tabs_trigger
            )}
        >
            { for props.children.iter() }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsContentProps {
    pub value: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TabsContent)]
pub fn tabs_content(props: &TabsContentProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");

    if state.active_tab != props.value {
        return html! {};
    }

    html! {
        <div class={classes!(props.class.clone(), &brandguide.tabs_content)}>
            { for props.children.iter() }
        </div>
    }
}
