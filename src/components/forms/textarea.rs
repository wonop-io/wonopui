use crate::config::get_brandguide;
use crate::config::ClassesStr;
use yew::prelude::*;

//use web_sys::InputData;

#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let brandguide = get_brandguide();

    let disabled_class = if props.disabled {
        brandguide.textarea_disabled.clone()
    } else {
        ClassesStr::empty()
    };

    html! {
        <textarea
            class={classes!(&brandguide.textarea_base, disabled_class, props.class.clone())}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            placeholder={props.placeholder.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
            disabled={props.disabled}
        />
    }
}

// textarea_base: "border rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-600"
// textarea_disabled: "opacity-50 cursor-not-allowed"
