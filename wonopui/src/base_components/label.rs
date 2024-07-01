use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub for_id: String,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    html! {
        <label
            class={classes!(BRANDGUIDE.label_base, props.class.clone())}
            for={props.for_id.clone()}
        >
            {props.children.clone()}
        </label>
    }
}

// label_base: "px-4 py-2"
