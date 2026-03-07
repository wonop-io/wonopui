//! Card component for WonopUI.
//!
//! A container component with header, title, and content sections.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for card styling.
/// Based on shadcn/ui v4 card component.
pub mod classes {
    /// Card container styles - matches shadcn v4 Card component.
    pub const CONTAINER: &str = "bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 flex flex-col gap-6 rounded-xl border border-zinc-200 dark:border-zinc-800 py-6 shadow-sm";

    /// Card header styles - matches shadcn v4 CardHeader.
    pub const HEADER: &str = "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6";

    /// Card title styles - matches shadcn v4 CardTitle.
    pub const TITLE: &str = "leading-none font-semibold";

    /// Card description styles - matches shadcn v4 CardDescription.
    pub const DESCRIPTION: &str = "text-zinc-500 dark:text-zinc-400 text-sm";

    /// Card action slot (for buttons in header).
    pub const ACTION: &str = "col-start-2 row-span-2 row-start-1 self-start justify-self-end";

    /// Card body/content styles - matches shadcn v4 CardContent.
    pub const BODY: &str = "px-6";

    /// Card footer styles - matches shadcn v4 CardFooter.
    pub const FOOTER: &str = "flex items-center px-6 [.border-t]:pt-6";
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
            data-slot="card"
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
        <div data-slot="card-header" class={classes!(classes::HEADER, props.class.clone())}>
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
        <div data-slot="card-title" class={classes!(classes::TITLE, props.class.clone())}>
            { for props.children.iter() }
        </div>
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
        <div data-slot="card-content" class={classes!(classes::BODY, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

/// Properties for CardDescription.
#[derive(Properties, PartialEq)]
pub struct CardDescriptionProps {
    /// Description content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Card description component.
#[function_component(CardDescription)]
pub fn card_description(props: &CardDescriptionProps) -> Html {
    html! {
        <div data-slot="card-description" class={classes!(classes::DESCRIPTION, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

/// Properties for CardFooter.
#[derive(Properties, PartialEq)]
pub struct CardFooterProps {
    /// Footer content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Card footer section.
#[function_component(CardFooter)]
pub fn card_footer(props: &CardFooterProps) -> Html {
    html! {
        <div data-slot="card-footer" class={classes!(classes::FOOTER, props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}
