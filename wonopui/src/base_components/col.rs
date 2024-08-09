use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ColProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
}

#[function_component(Col)]
pub fn col(props: &ColProps) -> Html {
    // TODO:
    let col_classes = classes!("flex","flex-col");
    
    html!(
        <@{props.tag.clone()} class={classes!(col_classes, props.class.clone())}>{props.children.clone()}</@>
    )
}
