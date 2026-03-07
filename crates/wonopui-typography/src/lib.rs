//! Typography components for WonopUI.
//!
//! Provides heading and paragraph components with consistent styling.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for typography styling.
/// Based on shadcn/ui v4 typography patterns.
pub mod classes {
    /// H1 heading styles - Page title, very large and bold.
    pub const H1: &str = "scroll-m-20 text-4xl font-extrabold tracking-tight text-zinc-950 dark:text-zinc-50 lg:text-5xl";

    /// H2 heading styles - Section headings, border-bottom optional.
    pub const H2: &str = "scroll-m-20 border-b border-zinc-200 dark:border-zinc-800 pb-2 text-3xl font-semibold tracking-tight text-zinc-950 dark:text-zinc-50 first:mt-0";

    /// H3 heading styles - Subsection headings.
    pub const H3: &str = "scroll-m-20 text-2xl font-semibold tracking-tight text-zinc-950 dark:text-zinc-50";

    /// H4 heading styles - Minor headings.
    pub const H4: &str = "scroll-m-20 text-xl font-semibold tracking-tight text-zinc-950 dark:text-zinc-50";

    /// H5 heading styles - Smaller headings.
    pub const H5: &str = "scroll-m-20 text-lg font-semibold tracking-tight text-zinc-950 dark:text-zinc-50";

    /// H6 heading styles - Smallest headings.
    pub const H6: &str = "scroll-m-20 text-base font-semibold tracking-tight text-zinc-950 dark:text-zinc-50";

    /// Paragraph/body text styles.
    pub const P: &str = "leading-7 text-zinc-700 dark:text-zinc-300 [&:not(:first-child)]:mt-6";

    /// Large text variant (shadcn: large).
    pub const LARGE: &str = "text-lg font-semibold text-zinc-950 dark:text-zinc-50";

    /// Small text variant (shadcn: small).
    pub const SMALL: &str = "text-sm font-medium leading-none text-zinc-950 dark:text-zinc-50";

    /// Muted text variant (shadcn: muted).
    pub const MUTED: &str = "text-sm text-zinc-500 dark:text-zinc-400";

    /// Lead text variant (intro paragraphs).
    pub const LEAD: &str = "text-xl text-zinc-500 dark:text-zinc-400";

    /// Blockquote styles.
    pub const BLOCKQUOTE: &str = "mt-6 border-l-2 border-zinc-300 dark:border-zinc-700 pl-6 italic text-zinc-800 dark:text-zinc-200";

    /// Inline code styles.
    pub const CODE: &str = "relative rounded bg-zinc-100 dark:bg-zinc-800 px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold text-zinc-900 dark:text-zinc-100";

    /// Unordered list styles.
    pub const UL: &str = "my-6 ml-6 list-disc text-zinc-700 dark:text-zinc-300 [&>li]:mt-2";

    /// Ordered list styles.
    pub const OL: &str = "my-6 ml-6 list-decimal text-zinc-700 dark:text-zinc-300 [&>li]:mt-2";
}

/// Heading level for typography components.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// Properties for the Heading component.
#[derive(Properties, PartialEq)]
pub struct HeadingProps {
    /// Heading level.
    #[prop_or_default]
    pub level: HeadingLevel,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Heading content (children).
    #[prop_or_default]
    pub children: Children,
}

/// A heading component that renders the appropriate HTML heading element.
#[function_component(Heading)]
pub fn heading(props: &HeadingProps) -> Html {
    let (tag, class) = match props.level {
        HeadingLevel::H1 => ("h1", classes::H1),
        HeadingLevel::H2 => ("h2", classes::H2),
        HeadingLevel::H3 => ("h3", classes::H3),
        HeadingLevel::H4 => ("h4", classes::H4),
        HeadingLevel::H5 => ("h5", classes::H5),
        HeadingLevel::H6 => ("h6", classes::H6),
    };

    let classes = classes!(class, props.class.clone());

    html! {
        <@{tag} class={classes}>
            { for props.children.iter() }
        </@>
    }
}

/// Properties for the Paragraph component.
#[derive(Properties, PartialEq)]
pub struct ParagraphProps {
    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Paragraph content (children).
    #[prop_or_default]
    pub children: Children,
}

/// A paragraph component with consistent styling.
#[function_component(Paragraph)]
pub fn paragraph(props: &ParagraphProps) -> Html {
    html! {
        <p class={classes!(classes::P, props.class.clone())}>
            { for props.children.iter() }
        </p>
    }
}

// Convenience component wrappers for each heading level

/// H1 Heading component - largest heading for page titles.
#[function_component(H1)]
pub fn h1(props: &HeadingProps) -> Html {
    html! {
        <Heading level={HeadingLevel::H1} class={props.class.clone()}>
            { for props.children.iter() }
        </Heading>
    }
}

/// H2 Heading component - section headings.
#[function_component(H2)]
pub fn h2(props: &HeadingProps) -> Html {
    html! {
        <Heading level={HeadingLevel::H2} class={props.class.clone()}>
            { for props.children.iter() }
        </Heading>
    }
}

/// H3 Heading component - subsection headings.
#[function_component(H3)]
pub fn h3(props: &HeadingProps) -> Html {
    html! {
        <Heading level={HeadingLevel::H3} class={props.class.clone()}>
            { for props.children.iter() }
        </Heading>
    }
}

/// H4 Heading component - minor headings.
#[function_component(H4)]
pub fn h4(props: &HeadingProps) -> Html {
    html! {
        <Heading level={HeadingLevel::H4} class={props.class.clone()}>
            { for props.children.iter() }
        </Heading>
    }
}

/// H5 Heading component - smaller headings.
#[function_component(H5)]
pub fn h5(props: &HeadingProps) -> Html {
    html! {
        <Heading level={HeadingLevel::H5} class={props.class.clone()}>
            { for props.children.iter() }
        </Heading>
    }
}

/// H6 Heading component - smallest headings.
#[function_component(H6)]
pub fn h6(props: &HeadingProps) -> Html {
    html! {
        <Heading level={HeadingLevel::H6} class={props.class.clone()}>
            { for props.children.iter() }
        </Heading>
    }
}

/// Alias for Paragraph component.
pub type P = Paragraph;
