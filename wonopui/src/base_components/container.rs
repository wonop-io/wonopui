use yew::prelude::*;

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
    #[prop_or(false)]
    pub use_break_points: bool,
    #[prop_or(true)]
    pub padding_x: bool,
    #[prop_or(true)]
    pub padding_y: bool,
    #[prop_or(false)]
    pub narrow: bool,
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
    let breakpoints = if props.use_break_points {
        classes!("container", "mx-auto")
    } else {
        classes!("mx-auto", "max-w-7xl")
    };
    let expanding = if props.expanding {
        classes!("w-full")
    } else {
        classes!("")
    };
    let narrow = if props.narrow {
        classes!("max-w-3xl", "mx-auto")
    } else {
        breakpoints
    };
    let container_class = classes!(padding_x, padding_y, expanding, narrow);

    html!(
        <@{props.tag.clone()} class={classes!(container_class, props.class.clone())}>{props.children.clone()}</@>
    )
}
