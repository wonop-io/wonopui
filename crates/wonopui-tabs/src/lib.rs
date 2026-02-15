//! Tabs component for WonopUI.
//!
//! A tabbed interface component with provider pattern for state management.

use std::rc::Rc;
use wonopui_core::*;

/// Default CSS classes for tabs styling.
pub mod classes {
    /// Tabs container.
    pub const CONTAINER: &str = "";

    /// Tabs list base styles.
    pub const LIST: &str = "inline-flex h-10 items-center justify-center rounded-md bg-zinc-100 dark:bg-zinc-800 p-1 text-zinc-600 dark:text-zinc-400";

    /// Tabs list row direction.
    pub const LIST_ROW: &str = "flex-row";

    /// Tabs list column direction.
    pub const LIST_COLUMN: &str = "flex-col h-auto";

    /// Tab trigger base styles.
    pub const TRIGGER: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-white transition-all focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-zinc-950 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 dark:ring-offset-zinc-950 dark:focus-visible:ring-zinc-300";

    /// Active tab trigger styles.
    pub const TRIGGER_ACTIVE: &str =
        "bg-white dark:bg-zinc-950 text-zinc-900 dark:text-zinc-100 shadow-sm";

    /// Inactive tab trigger styles.
    pub const TRIGGER_INACTIVE: &str =
        "text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100";

    /// Disabled tab trigger styles.
    pub const TRIGGER_DISABLED: &str = "opacity-50 cursor-not-allowed";

    /// Tab content container.
    pub const CONTENT: &str = "mt-2";
}

/// Direction for tabs layout.
#[derive(Clone, PartialEq, Default)]
pub enum TabsDirection {
    Row,
    Column,
    #[default]
    Auto,
}

/// Internal state for tabs.
#[derive(Clone, PartialEq)]
pub struct TabsState {
    pub active_tab: String,
    pub set_active_tab: Callback<String>,
    pub direction: TabsDirection,
}

#[derive(Properties, PartialEq)]
pub struct TabsProviderProps {
    pub children: Children,
    pub default_value: String,
    #[prop_or_default]
    pub direction: TabsDirection,
}

/// Provider component for tabs state management.
#[function_component(TabsProvider)]
pub fn tabs_provider(props: &TabsProviderProps) -> Html {
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
            { for props.children.iter() }
        </ContextProvider<Rc<TabsState>>>
    }
}

/// Hook to access the tabs state.
///
/// Returns a tuple containing:
/// - The currently active tab value
/// - A callback to set the active tab
#[hook]
pub fn use_tabs() -> (String, Callback<String>) {
    let state =
        use_context::<Rc<TabsState>>().expect("use_tabs must be used within a TabsProvider");
    (state.active_tab.clone(), state.set_active_tab.clone())
}

#[derive(Properties, PartialEq)]
pub struct TabsLayoutProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TabsLayout)]
pub fn tabs_layout(props: &TabsLayoutProps) -> Html {
    html! {
        <div class={classes!(classes::CONTAINER, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub children: Children,
    pub default_value: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub direction: TabsDirection,
}

/// Main Tabs component that combines provider and layout.
#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    html! {
        <TabsProvider default_value={props.default_value.clone()} direction={props.direction.clone()}>
            <TabsLayout class={props.class.clone()}>
                { for props.children.iter() }
            </TabsLayout>
        </TabsProvider>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsListProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// Container for tab triggers.
#[function_component(TabsList)]
pub fn tabs_list(props: &TabsListProps) -> Html {
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");
    let direction_class = match state.direction {
        TabsDirection::Auto | TabsDirection::Row => classes::LIST_ROW,
        TabsDirection::Column => classes::LIST_COLUMN,
    };

    html! {
        <div class={classes!(classes::LIST, direction_class, props.class.clone())}>
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
    #[prop_or_default]
    pub disabled: bool,
}

/// Individual tab trigger button.
#[function_component(TabsTrigger)]
pub fn tabs_trigger(props: &TabsTriggerProps) -> Html {
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");

    let onclick = {
        let set_active_tab = state.set_active_tab.clone();
        let value = props.value.clone();
        let disabled = props.disabled;
        Callback::from(move |_| {
            if !disabled {
                set_active_tab.emit(value.clone())
            }
        })
    };

    let is_active = state.active_tab == props.value;
    let state_class = if is_active {
        classes::TRIGGER_ACTIVE
    } else {
        classes::TRIGGER_INACTIVE
    };
    let disabled_class = if props.disabled {
        classes::TRIGGER_DISABLED
    } else {
        ""
    };

    html! {
        <button
            type="button"
            role="tab"
            onclick={onclick}
            disabled={props.disabled}
            class={classes!(
                classes::TRIGGER,
                state_class,
                disabled_class,
                props.class.clone(),
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

/// Content panel for a specific tab.
#[function_component(TabsContent)]
pub fn tabs_content(props: &TabsContentProps) -> Html {
    let state = use_context::<Rc<TabsState>>().expect("no context found for TabsState");

    if state.active_tab != props.value {
        return html! {};
    }

    html! {
        <div class={classes!(classes::CONTENT, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}
