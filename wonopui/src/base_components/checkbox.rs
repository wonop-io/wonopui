use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub on_toggle: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let state_class = if props.checked {
        BRANDGUIDE.checkbox_checked
    } else {
        BRANDGUIDE.checkbox_unchecked
    };

    let disabled_class = if props.disabled {
        BRANDGUIDE.checkbox_disabled
    } else {
        ""
    };

    html! {
        <button
            type="button"
            role="checkbox"
            aria-checked={props.checked.to_string()}
            data-state={state_class}
            value="on"
            class={classes!(
                "peer",
                BRANDGUIDE.checkbox_base,
                state_class,
                disabled_class,
            )}
            id={props.id.clone()}
            onclick={props.on_toggle.clone()}
            disabled={props.disabled}
        />
    }
}

#[function_component(CheckboxDemo)]
pub fn checkbox_demo() -> Html {
    let checked = use_state(|| false);

    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    html! {
        <div class={"flex items-center space-x-2"}>
            <Checkbox id="terms" checked={*checked} on_toggle={on_toggle} />
            <label
                for="terms"
                class={BRANDGUIDE.checkbox_label}
            >
                { "Accept terms and conditions" }
            </label>
        </div>
    }
}

// New entries in the brand guide:
// checkbox_base: "h-4 w-4 shrink-0 rounded-sm border ring-offset-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2",
// checkbox_checked: "bg-blue-500 text-white", // Standard colors
// checkbox_unchecked: "border-blue-500", // Standard color
// checkbox_disabled: "disabled:cursor-not-allowed disabled:opacity-50",
// checkbox_label: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
