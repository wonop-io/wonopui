use crate::config::BRANDGUIDE;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub ontext: Callback<String>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub onkeypress: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeyup: Callback<KeyboardEvent>,
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
    #[prop_or_default]
    pub maxlength: Option<i32>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub min: Option<String>,
    #[prop_or_default]
    pub max: Option<String>,
    #[prop_or_default]
    pub step: Option<String>,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let ontext = use_callback(
        (props.ontext.clone(), props.oninput.clone()),
        |e: InputEvent, (ontext, oninput)| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ontext.emit(input.value());
            oninput.emit(e);
        },
    );

    let onchange = use_callback(props.onchange.clone(), |e, onchange| {
        onchange.emit(e);
    });

    html! {
        <input
            type={props.kind.clone()}
            class={classes!(BRANDGUIDE.input_base, props.class.clone())}
            value={props.value.clone()}
            oninput={ontext}
            onchange={onchange}
            onkeypress={props.onkeypress.clone()}
            onkeydown={props.onkeydown.clone()}
            onkeyup={props.onkeyup.clone()}
            placeholder={props.placeholder.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
            maxlength={match props.maxlength {
                Some(v) => v.to_string(),
                None => "".to_string()
            }}
            readonly={props.readonly}
            min={props.min.clone()}
            max={props.max.clone()}
            step={props.step.clone()}
            ref={props.node_ref.clone()}
        />
    }
}

// input_base: "border rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-600"
