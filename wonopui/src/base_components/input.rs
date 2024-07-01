use crate::config::BRANDGUIDE;
use yew::prelude::*;
//use web_sys::InputData;

#[derive(Properties, PartialEq)]
pub struct InputProps {
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
    #[prop_or("text".to_string())]
    pub kind: String,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input
            type={props.kind.clone()}
            class={classes!(BRANDGUIDE.input_base, props.class.clone())}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            placeholder={props.placeholder.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
        />
    }
}

// input_base: "border rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-600"
