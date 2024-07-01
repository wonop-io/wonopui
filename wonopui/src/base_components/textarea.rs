use crate::config::BRANDGUIDE;
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
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    html! {
        <textarea
            class={classes!(BRANDGUIDE.textarea_base, props.class.clone())}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            placeholder={props.placeholder.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
        />
    }
}

// textarea_base: "border rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-600"
