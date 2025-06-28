use crate::config::get_brandguide;
use crate::config::ClassesStr;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SwitchButtonProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub default_value: bool,
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
    let brandguide = get_brandguide();
    let checked = use_state(|| props.checked.unwrap_or(props.default_value));

    use_effect_with(
        (checked.clone(), props.checked),
        |(checked, prop_checked)| {
            if let Some(prop_checked) = prop_checked {
                if (**checked) != *prop_checked {
                    checked.set(*prop_checked);
                }
            }
        },
    );

    let state_class = if *checked {
        &brandguide.switch_checked
    } else {
        &brandguide.switch_unchecked
    };

    let translate_class = if *checked {
        &brandguide.switch_translate_checked
    } else {
        &brandguide.switch_translate_unchecked
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
                &brandguide.switch_base,
                state_class,
                if props.disabled { brandguide.switch_disabled.clone() } else { ClassesStr::empty() },
            )}
            id={props.id.clone()}
            onclick={on_click}
            disabled={props.disabled}
        >
            /* <span class={&brandguide.switch_label}>{ "Use setting" }</span> */
            <span
                aria-hidden="true"
                class={classes!(
                    &brandguide.switch_thumb,
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
