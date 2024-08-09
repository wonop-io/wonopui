use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div 
            class={classes!(BRANDGUIDE.card_container, props.class.clone())}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    html! {
        <div class={classes!(BRANDGUIDE.card_header, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardTitleProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardTitle)]
pub fn card_title(props: &CardTitleProps) -> Html {
    html! {
        <h2 class={BRANDGUIDE.card_title}>
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardContent)]
pub fn card_content(props: &CardContentProps) -> Html {
    html! {
        <div class={BRANDGUIDE.card_body}>
            { for props.children.iter() }
        </div>
    }
}
