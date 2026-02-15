//! Card component for WonopUI.
//!
//! A container component with header, title, and content sections.

use wonopui_core::*;

/// Default CSS classes for card styling.
pub mod classes {
    /// Card container styles.
    pub const CONTAINER: &str = "rounded-md border border-zinc-200 dark:border-zinc-700 shadow-md bg-white dark:bg-zinc-900 text-zinc-800 dark:text-zinc-100";

    /// Card header styles.
    pub const HEADER: &str = "p-6 border-b border-zinc-200 dark:border-zinc-700";

    /// Card title styles.
    pub const TITLE: &str = "text-xl font-semibold leading-none tracking-tight";

    /// Card body/content styles.
    pub const BODY: &str = "p-6";
}

/// Properties for the Card component.
#[derive(Properties, PartialEq)]
pub struct CardProps {
    /// Card content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Optional click handler.
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

/// A container component for displaying content in a card layout.
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div
            class={classes!(classes::CONTAINER, props.class.clone())}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Properties for CardHeader.
#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    /// Header content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Card header section.
#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    html! {
        <div class={classes!(classes::HEADER, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

/// Properties for CardTitle.
#[derive(Properties, PartialEq)]
pub struct CardTitleProps {
    /// Title content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Card title component.
#[function_component(CardTitle)]
pub fn card_title(props: &CardTitleProps) -> Html {
    html! {
        <h2 class={classes!(classes::TITLE, props.class.clone())}>
            { for props.children.iter() }
        </h2>
    }
}

/// Properties for CardContent.
#[derive(Properties, PartialEq)]
pub struct CardContentProps {
    /// Body content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Card body/content section.
#[function_component(CardContent)]
pub fn card_content(props: &CardContentProps) -> Html {
    html! {
        <div class={classes!(classes::BODY, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}
