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
pub struct TabsProps {
    pub children: Children,
    pub default_value: String,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Clone, PartialEq)]
pub struct TabsState {
    pub active_tab: String,
    pub set_active_tab: Callback<String>,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
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
    });

    html! {
        <ContextProvider<Rc<TabsState>> context={state}>
            <div class={classes!(props.class.clone(), "flex", "flex-col", BRANDGUIDE.tabs_container)}>
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
    #[prop_or(FlexDirection::Row)]
    pub direction: FlexDirection,
}

#[function_component(TabsList)]
pub fn tabs_list(props: &TabsListProps) -> Html {
    html! {
        // <div>
            <div role="tablist" class={classes!(props.class.clone(), "inline-flex",  props.direction.to_string(), BRANDGUIDE.tabs_list)}>
                { for props.children.iter() }
            </div>
        // </div>
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
            //aria-selected={is_active}
            onclick={onclick}
            class={classes!(
                props.class.clone(),
                if is_active { BRANDGUIDE.tabs_trigger_active } else { BRANDGUIDE.tabs_trigger_inactive },
                BRANDGUIDE.tabs_trigger
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
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");

    if state.active_tab != props.value {
        return html! {};
    }

    html! {
        <div class={classes!(props.class.clone(), BRANDGUIDE.tabs_content)}>
            { for props.children.iter() }
        </div>
    }
}

// New entries in the brand guide:
// tabs_list: "h-10 items-center justify-center rounded-md bg-gray-100 p-1 text-gray-600",
// tabs_trigger: "ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
// tabs_content: "ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-500 focus-visible:ring-offset-2"

#[function_component(TabsDemo)]
pub fn tabs_demo() -> Html {
    html! {
        <Tabs default_value="account" class="w-[400px]">
            <TabsList class="flex w-full"  direction={FlexDirection::Row}>
                <TabsTrigger value="account">{"Account"}</TabsTrigger>
                <TabsTrigger value="password">{"Password"}</TabsTrigger>
            </TabsList>
            <TabsContent value="account">
                <div class="rounded-lg border bg-white text-black shadow-sm">
                    <div class="flex flex-col space-y-1.5 p-6">
                        <h3 class="text-2xl font-semibold leading-none tracking-tight">{"Account"}</h3>
                        <p class="text-sm text-gray-600">{"Make changes to your account here. Click save when you're done."}</p>
                    </div>
                    <div class="p-6 pt-0 space-y-2">
                        <div class="space-y-1">
                            <label class="text-sm font-medium leading-none" for="name">{"Name"}</label>
                            <input class="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm" id="name" value="Pedro Duarte" />
                        </div>
                        <div class="space-y-1">
                            <label class="text-sm font-medium leading-none" for="username">{"Username"}</label>
                            <input class="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm" id="username" value="@peduarte" />
                        </div>
                    </div>
                    <div class="flex items-center p-6 pt-0">
                        <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors bg-blue-500 text-white hover:bg-blue-600 h-10 px-4 py-2">{"Save changes"}</button>
                    </div>
                </div>
            </TabsContent>
            <TabsContent value="password">
                <div class="rounded-lg border bg-white text-black shadow-sm">
                    <div class="flex flex-col space-y-1.5 p-6">
                        <h3 class="text-2xl font-semibold leading-none tracking-tight">{"Password"}</h3>
                        <p class="text-sm text-gray-600">{"Change your password here. After saving, you'll be logged out."}</p>
                    </div>
                    <div class="p-6 pt-0 space-y-2">
                        <div class="space-y-1">
                            <label class="text-sm font-medium leading-none" for="current">{"Current password"}</label>
                            <input class="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm" id="current" type="password" />
                        </div>
                        <div class="space-y-1">
                            <label class="text-sm font-medium leading-none" for="new">{"New password"}</label>
                            <input class="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm" id="new" type="password" />
                        </div>
                    </div>
                    <div class="flex items-center p-6 pt-0">
                        <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors bg-blue-500 text-white hover:bg-blue-600 h-10 px-4 py-2">{"Save password"}</button>
                    </div>
                </div>
            </TabsContent>
        </Tabs>
    }
}
