use crate::base_components::container::Container;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub expanding: bool,
    #[prop_or(false)]
    pub use_break_points: bool,
    #[prop_or(true)]
    pub padding_x: bool,
    #[prop_or(true)]
    pub padding_y: bool,
}

#[function_component(MainContent)]
pub fn main_content(props: &ContentProps) -> Html {
    html!(
        <Container tag="main" expanding={props.expanding} use_break_points={props.use_break_points} padding_x={props.padding_x} padding_y={props.padding_y} class={props.class.clone()}>
            {props.children.clone()}
        </Container>
    )
}
