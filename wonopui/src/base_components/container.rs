use yew::prelude::*;

#[derive(PartialEq)]
pub enum ContainerVariant {
    Small,
    Narrow,
    Large,
    
    Responsive,
    None
}

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,

    #[prop_or(true)]
    pub expanding: bool,

    #[prop_or(true)]
    pub padding_x: bool,
    #[prop_or(true)]
    pub padding_y: bool,
    #[prop_or(ContainerVariant::Responsive)]
    pub variant: ContainerVariant,
    #[prop_or_default]
    pub style: Option<String>
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    // TODO Get classes from brand guide
    let padding_x = if props.padding_x {
        classes!("px-4", "sm:px-6", "lg:px-8")
    } else {
        classes!("")
    };
    let padding_y = if props.padding_y {
        classes!("py-4", "sm:py-4", "lg:py-6")
    } else {
        classes!("")
    };

    let expanding = if props.expanding {
        classes!("grow-1")
    } else {
        classes!("")
    };

    let variant = match props.variant {
        ContainerVariant::Small =>  classes!("mx-auto", "max-w-96"),
        ContainerVariant::Narrow =>  classes!("mx-auto", "max-w-3xl"),
        ContainerVariant::Large =>  classes!("mx-auto", "max-w-7xl"),
        ContainerVariant::Responsive =>  classes!("mx-auto", "container"),
        ContainerVariant::None => classes!("")
    };

    let container_class = classes!(padding_x, padding_y, expanding, variant);

    html!(
        <@{props.tag.clone()} class={classes!(container_class, props.class.clone())} style={props.style.clone()}>{props.children.clone()}</@>
    )
}
