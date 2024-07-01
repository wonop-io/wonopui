use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SwitchButtonProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub on_toggle: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_icon: Option<Html>,
    #[prop_or_default]
    pub off_icon: Option<Html>,
}

#[function_component(SwitchButton)]
pub fn switch_button(props: &SwitchButtonProps) -> Html {
    let checked = use_state(|| props.checked);

    let state_class = if *checked {
        BRANDGUIDE.switch_checked
    } else {
        BRANDGUIDE.switch_unchecked
    };

    let translate_class = if *checked {
        "translate-x-5"
    } else {
        "translate-x-0"
    };

    let on_click = {
        let checked = checked.clone();
        let on_toggle = props.on_toggle.clone();
        let disabled = props.disabled;
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                checked.set(!*checked);
                on_toggle.emit(e);
            }
        })
    };

    html! {
        <button
            type="button"
            role="switch"
            aria-checked={checked.to_string()}
            class={classes!(
                "relative",
                "inline-flex",
                "h-6",
                "w-11",
                BRANDGUIDE.switch_base,
                state_class,
                if props.disabled { BRANDGUIDE.switch_disabled } else { "" },
            )}
            id={props.id.clone()}
            onclick={on_click}
            disabled={props.disabled}
        >
            <span class="sr-only">{ "Use setting" }</span>
            <span
                aria-hidden="true"
                class={classes!(
                    BRANDGUIDE.switch_thumb,
                    translate_class,
                )}
            >
                { if *checked {
                    props.on_icon.clone().unwrap_or_default()
                } else {
                    props.off_icon.clone().unwrap_or_default()
                }}
            </span>
        </button>
    }
}

#[function_component(SwitchDemo)]
pub fn switch_demo() -> Html {
    let checked = use_state(|| false);

    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    html! {
        <div class={"flex items-center space-x-2"}>
            <SwitchButton
                id="bold-switch"
                checked={*checked}
                on_toggle={on_toggle}
                on_icon={Some(html! { <i class="fas fa-check"></i> })}
                off_icon={Some(html! { <i class="fas fa-times"></i> })}
            />
            <label
                for="bold-switch"
                class={BRANDGUIDE.switch_label}
            >
                { "Toggle bold" }
            </label>
        </div>
    }
}

// New entries in the brand guide:
// switch_base: "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2",
// switch_thumb: "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out",
// switch_checked: "bg-indigo-600",
// switch_unchecked: "bg-gray-200",
// switch_translate_checked: "translate-x-5",
// switch_translate_unchecked: "translate-x-0",
// switch_disabled: "disabled:cursor-not-allowed disabled:opacity-50",
// switch_label: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
