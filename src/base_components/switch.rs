#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
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

// Snippets to update brandguide:
// ("switch_base".to_string(), "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2".to_string()),
// ("switch_thumb".to_string(), "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out".to_string()),
// ("switch_checked".to_string(), "bg-indigo-600".to_string()),
// ("switch_unchecked".to_string(), "bg-gray-200".to_string()),
// ("switch_translate_checked".to_string(), "translate-x-5".to_string()),
// ("switch_translate_unchecked".to_string(), "translate-x-0".to_string()),
// ("switch_disabled".to_string(), "disabled:cursor-not-allowed disabled:opacity-50".to_string()),
// ("switch_label".to_string(), "sr-only".to_string()),
//
// pub switch_base: ClassesContainer<T>,
// pub switch_thumb: ClassesContainer<T>,
// pub switch_checked: ClassesContainer<T>,
// pub switch_unchecked: ClassesContainer<T>,
// pub switch_translate_checked: ClassesContainer<T>,
// pub switch_translate_unchecked: ClassesContainer<T>,
// pub switch_disabled: ClassesContainer<T>,
// pub switch_label: ClassesContainer<T>,
//
// switch_base: self.switch_base.to_owned(),
// switch_thumb: self.switch_thumb.to_owned(),
// switch_checked: self.switch_checked.to_owned(),
// switch_unchecked: self.switch_unchecked.to_owned(),
// switch_translate_checked: self.switch_translate_checked.to_owned(),
// switch_translate_unchecked: self.switch_translate_unchecked.to_owned(),
// switch_disabled: self.switch_disabled.to_owned(),
// switch_label: self.switch_label.to_owned(),
//
// switch_base: default_config_hm
// .get("switch_base")
// .expect("Template parameter missing")
// .clone(),
// switch_thumb: default_config_hm
// .get("switch_thumb")
// .expect("Template parameter missing")
// .clone(),
// switch_checked: default_config_hm
// .get("switch_checked")
// .expect("Template parameter missing")
// .clone(),
// switch_unchecked: default_config_hm
// .get("switch_unchecked")
// .expect("Template parameter missing")
// .clone(),
// switch_translate_checked: default_config_hm
// .get("switch_translate_checked")
// .expect("Template parameter missing")
// .clone(),
// switch_translate_unchecked: default_config_hm
// .get("switch_translate_unchecked")
// .expect("Template parameter missing")
// .clone(),
// switch_disabled: default_config_hm
// .get("switch_disabled")
// .expect("Template parameter missing")
// .clone(),
// switch_label: default_config_hm
// .get("switch_label")
// .expect("Template parameter missing")
// .clone(),
