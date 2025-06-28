use crate::config::get_brandguide;
use crate::config::ClassesStr;
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
    let brandguide = get_brandguide();
    let checked = use_state(|| props.checked);

    let state_class = if *checked {
        &brandguide.toggle_checked
    } else {
        &brandguide.toggle_unchecked
    };

    let disabled_class = if props.disabled {
        brandguide.toggle_disabled.clone()
    } else {
        ClassesStr::empty()
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
            class={classes!(
                &brandguide.toggle_container,
                &brandguide.toggle_base,
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
