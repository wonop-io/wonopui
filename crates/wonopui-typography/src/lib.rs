//! Typography components for WonopUI.
//!
//! Provides heading and paragraph components with consistent styling.

use wonopui_core::*;

/// Default CSS classes for typography styling.
pub mod classes {
    /// H1 heading styles.
    pub const H1: &str = "mt-6 mb-10 text-zinc-800 dark:text-zinc-100 text-4xl font-bold tracking-tight";
    
    /// H2 heading styles.
    pub const H2: &str = "mt-5 mb-8 text-zinc-800 dark:text-zinc-100 text-3xl font-semibold tracking-tight";
    
    /// H3 heading styles.
    pub const H3: &str = "mt-4 mb-6 text-zinc-800 dark:text-zinc-100 text-2xl font-semibold tracking-tight";
    
    /// H4 heading styles.
    pub const H4: &str = "mt-3 mb-4 text-zinc-800 dark:text-zinc-100 text-xl font-semibold";
    
    /// H5 heading styles.
    pub const H5: &str = "mt-2 mb-3 text-zinc-800 dark:text-zinc-100 text-lg font-medium";
    
    /// H6 heading styles.
    pub const H6: &str = "mt-2 mb-2 text-zinc-800 dark:text-zinc-100 text-base font-medium";
    
    /// Paragraph styles.
    pub const P: &str = "my-2 text-zinc-800 dark:text-zinc-100 text-base font-normal mb-4 leading-relaxed";
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

// Convenience type aliases
pub type H1 = Heading;
pub type H2 = Heading;
pub type H3 = Heading;
pub type H4 = Heading;
pub type H5 = Heading;
pub type H6 = Heading;
pub type P = Paragraph;
