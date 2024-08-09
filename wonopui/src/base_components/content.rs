use crate::base_components::container::{Container, ContainerVariant};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub expanding: bool,
    #[prop_or(true)]
    pub padding_x: bool,
    #[prop_or(true)]
    pub padding_y: bool,
    #[prop_or_default]
    pub aside: Option<Html>        
}

#[function_component(MainContent)]
pub fn main_content(props: &ContentProps) -> Html {
    let base_container = html! {
        <Container key="main" tag="main" variant={ContainerVariant::None} expanding={props.expanding} padding_x={props.padding_x} padding_y={props.padding_y} class={classes!(props.class.clone(), if props.aside.is_some() { "md:mr-96" } else { "" })}>
            {props.children.clone()}
        </Container>
    };
    html! {
        <>
            {base_container}
            {if let Some(aside) = &props.aside {
                html! {
                    <aside class="fixed top-0 right-0 h-full w-96 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 overflow-y-auto hidden md:block">
                        <Container class="h-full">
                            {aside.clone()}
                        </Container>
                    </aside>
                }
            } else {
                html! {}
            }}
        </>
    }
}
