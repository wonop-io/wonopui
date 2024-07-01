use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let is_open = use_state(|| props.open);

    let toggle_open = {
        let is_open = is_open.clone();
        let on_open_change = props.on_open_change.clone();
        Callback::from(move |_| {
            let new_state = !*is_open;
            is_open.set(new_state);
            on_open_change.emit(new_state);
        })
    };

    html! {
        <div onclick={toggle_open} class={classes!(props.class.clone(), if *is_open { "open" } else { "closed" })}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleTriggerProps {
    #[prop_or_default]
    pub as_child: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleTrigger)]
pub fn collapsible_trigger(props: &CollapsibleTriggerProps) -> Html {
    html! {
        <div>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CollapsibleContentProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleContent)]
pub fn collapsible_content(props: &CollapsibleContentProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(CollapsibleDemo)]
pub fn collapsible_demo() -> Html {
    let is_open = use_state(|| false);

    let on_open_change = {
        let is_open = is_open.clone();
        Callback::from(move |new_state: bool| {
            is_open.set(new_state);
        })
    };

    html! {
        <Collapsible open={*is_open} on_open_change={on_open_change} class="w-[350px] space-y-2">
            <div class="flex items-center justify-between space-x-4 px-4">
                <h4 class="text-sm font-semibold">{"@peduarte starred 3 repositories"}</h4>
                <CollapsibleTrigger as_child=true>
                    <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md w-9 p-0" onclick={on_open_change.reform(|_| !*is_open)}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevrons-up-down h-4 w-4">
                            <path d="m7 15 5 5 5-5"></path>
                            <path d="m7 9 5-5 5 5"></path>
                        </svg>
                        <span class="sr-only">{"Toggle"}</span>
                    </button>
                </CollapsibleTrigger>
            </div>
            <div class="rounded-md border px-4 py-3 font-mono text-sm">
                {"@radix-ui/primitives"}
            </div>
            <CollapsibleContent class="rounded-md border px-4 py-3 font-mono text-sm">
                <div class="rounded-md border px-4 py-3 font-mono text-sm">{"@radix-ui/colors"}</div>
                <div class="rounded-md border px-4 py-3 font-mono text-sm">{"@stitches/react"}</div>
            </CollapsibleContent>
        </Collapsible>
    }
}

// New entries in the brand guide:
// collapsible_container: "w-[350px] space-y-2",
// collapsible_header: "flex items-center justify-between space-x-4 px-4",
// collapsible_title: "text-sm font-semibold",
// collapsible_button: "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md w-9 p-0",
// collapsible_content: "rounded-md border px-4 py-3 font-mono text-sm",
// collapsible_item: "rounded-md border px-4 py-3 font-mono text-sm",
