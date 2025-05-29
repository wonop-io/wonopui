#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::ClassesStr;
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let state_class = if props.checked {
        &brandguide.checkbox_checked
    } else {
        &brandguide.checkbox_unchecked
    };

    let disabled_class = if props.disabled {
        brandguide.checkbox_disabled.clone()
    } else {
        ClassesStr::empty()
    };

    html! {
        <button
            type="button"
            role="checkbox"
            aria-checked={props.checked.to_string()}
            value="on"
            class={classes!(
                "peer",
                &brandguide.checkbox_base,
                state_class,
                &disabled_class,
            )}
            id={props.id.clone()}
            onclick={props.on_toggle.clone()}
            disabled={props.disabled}
        />
    }
}
