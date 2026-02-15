//! Col (Column) component for wonopui
//!
//! A flex column layout component.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const COL_CONTAINER: &str = "flex flex-col";
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColGap {
    None,
    #[default]
    Default,
    Sm,
    Md,
    Lg,
    Xl,
}

impl ColGap {
    pub fn to_class(&self) -> &'static str {
        match self {
            ColGap::None => "",
            ColGap::Default => "gap-4",
            ColGap::Sm => "gap-2",
            ColGap::Md => "gap-4",
            ColGap::Lg => "gap-6",
            ColGap::Xl => "gap-8",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColAlign {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl ColAlign {
    pub fn to_class(&self) -> &'static str {
        match self {
            ColAlign::Start => "items-start",
            ColAlign::Center => "items-center",
            ColAlign::End => "items-end",
            ColAlign::Stretch => "items-stretch",
            ColAlign::Baseline => "items-baseline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

impl ColJustify {
    pub fn to_class(&self) -> &'static str {
        match self {
            ColJustify::Start => "justify-start",
            ColJustify::Center => "justify-center",
            ColJustify::End => "justify-end",
            ColJustify::Between => "justify-between",
            ColJustify::Around => "justify-around",
            ColJustify::Evenly => "justify-evenly",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ColProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
    #[prop_or_default]
    pub gap: ColGap,
    #[prop_or_default]
    pub align: ColAlign,
    #[prop_or_default]
    pub justify: ColJustify,
}

#[function_component(Col)]
pub fn col(props: &ColProps) -> Html {
    let class = merge_classes(&[
        classes::COL_CONTAINER,
        props.gap.to_class(),
        props.align.to_class(),
        props.justify.to_class(),
        &props.class.to_string(),
    ]);

    html!(
        <@{props.tag.clone()} class={class}>
            { props.children.clone() }
        </@>
    )
}

/// A Row component (flex row layout) - complementary to Col
pub mod row {
    use super::*;

    pub mod classes {
        pub const ROW_CONTAINER: &str = "flex flex-row";
    }

    #[derive(Properties, PartialEq)]
    pub struct RowProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub class: Classes,
        #[prop_or("div".to_string())]
        pub tag: String,
        #[prop_or_default]
        pub gap: ColGap,
        #[prop_or_default]
        pub align: ColAlign,
        #[prop_or_default]
        pub justify: ColJustify,
        #[prop_or_default]
        pub wrap: bool,
    }

    #[function_component(Row)]
    pub fn row(props: &RowProps) -> Html {
        let class = merge_classes(&[
            classes::ROW_CONTAINER,
            props.gap.to_class(),
            props.align.to_class(),
            props.justify.to_class(),
            if props.wrap { "flex-wrap" } else { "" },
            &props.class.to_string(),
        ]);

        html!(
            <@{props.tag.clone()} class={class}>
                { props.children.clone() }
            </@>
        )
    }
}

pub use row::Row;
