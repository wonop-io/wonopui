//! Container component for WonopUI.
//!
//! A layout component for constraining content width with responsive padding.

use wonopui_core::*;

/// Default CSS classes for container styling.
pub mod classes {
    /// Horizontal padding styles.
    pub const PADDING_X: &str = "px-4 sm:px-6 lg:px-8";

    /// Vertical padding styles.
    pub const PADDING_Y: &str = "py-4 sm:py-6 lg:py-8";

    /// Expanding container (flex-grow).
    pub const EXPANDING: &str = "grow-1";

    /// Small container width.
    pub const SMALL: &str = "mx-auto w-full max-w-96";

    /// Narrow container width.
    pub const NARROW: &str = "mx-auto w-full max-w-3xl";

    /// Large container width.
    pub const LARGE: &str = "mx-auto w-full max-w-7xl";

    /// Responsive container width.
    pub const RESPONSIVE: &str = "mx-auto container";
}

/// Container size variants.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ContainerVariant {
    /// Small container (max-w-96).
    Small,
    /// Narrow container (max-w-3xl).
    Narrow,
    /// Large container (max-w-7xl).
    Large,
    /// Responsive container (uses Tailwind container).
    #[default]
    Responsive,
    /// No width constraint.
    None,
}

/// Properties for the Container component.
#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    /// Container content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// HTML tag to use (default: "div").
    #[prop_or("div".to_string())]
    pub tag: String,

    /// Whether the container should expand to fill available space.
    #[prop_or(true)]
    pub expanding: bool,

    /// Whether to apply horizontal padding.
    #[prop_or(true)]
    pub padding_x: bool,

    /// Whether to apply vertical padding.
    #[prop_or(true)]
    pub padding_y: bool,

    /// Container width variant.
    #[prop_or_default]
    pub variant: ContainerVariant,

    /// Inline style attribute.
    #[prop_or_default]
    pub style: Option<String>,
}

/// A layout component for constraining content width with responsive padding.
///
/// # Example
///
/// ```rust
/// use wonopui_container::{Container, ContainerVariant};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Container variant={ContainerVariant::Large}>
///             {"Content with max-width constraint"}
///         </Container>
///     }
/// }
/// ```
#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let padding_x = if props.padding_x {
        classes::PADDING_X
    } else {
        ""
    };
    let padding_y = if props.padding_y {
        classes::PADDING_Y
    } else {
        ""
    };
    let expanding = if props.expanding {
        classes::EXPANDING
    } else {
        ""
    };

    let variant = match props.variant {
        ContainerVariant::Small => classes::SMALL,
        ContainerVariant::Narrow => classes::NARROW,
        ContainerVariant::Large => classes::LARGE,
        ContainerVariant::Responsive => classes::RESPONSIVE,
        ContainerVariant::None => "",
    };

    let container_class = classes!(
        padding_x,
        padding_y,
        expanding,
        variant,
        props.class.clone()
    );

    html!(
        <@{props.tag.clone()} class={container_class} style={props.style.clone()}>
            { for props.children.iter() }
        </@>
    )
}
