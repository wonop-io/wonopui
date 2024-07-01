use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub on_toggle: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let checked = use_state(|| props.checked);

    let state_class = if *checked {
        BRANDGUIDE.toggle_checked
    } else {
        BRANDGUIDE.toggle_unchecked
    };

    let disabled_class = if props.disabled {
        BRANDGUIDE.toggle_disabled
    } else {
        ""
    };

    let on_click = {
        let checked = checked.clone();
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |e: MouseEvent| {
            checked.set(!*checked);
            on_toggle.emit(e);
        })
    };

    html! {
        <button
            type="button"
            role="switch"
            aria-checked={checked.to_string()}
            data-state={state_class}
            class={classes!(
                "inline-flex","items-center","justify-center",
                BRANDGUIDE.toggle_base,
                state_class,
                disabled_class,
            )}
            id={props.id.clone()}
            onclick={on_click}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </button>
    }
}

#[function_component(ToggleDemo)]
pub fn toggle_demo() -> Html {
    let checked = use_state(|| false);

    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    html! {
        <div class={"flex items-center space-x-2"}>
            <Toggle id="bold-toggle" checked={*checked} on_toggle={on_toggle}>
                <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
                    <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
                </svg>
            </Toggle>
            <label
                for="bold-toggle"
                class={BRANDGUIDE.toggle_label}
            >
                { "Toggle bold" }
            </label>
        </div>
    }
}

// New entries in the brand guide:
// toggle_base: "h-8 w-8 rounded-full border ring-offset-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2",
// toggle_checked: "bg-blue-500 text-white", // Standard colors
// toggle_unchecked: "border-blue-500", // Standard color
// toggle_disabled: "disabled:cursor-not-allowed disabled:opacity-50",
// toggle_label: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
