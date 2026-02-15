//! Button component for WonopUI.
//!
//! A versatile button component with multiple variants and sizes.

use wonopui_core::*;

/// Default CSS classes for button styling.
pub mod classes {
    /// Base button styles applied to all buttons.
    pub const BASE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-white transition-colors focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-zinc-950 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 dark:ring-offset-zinc-950 dark:focus-visible:ring-zinc-300";

    /// Primary button variant.
    pub const PRIMARY: &str = "bg-zinc-900 text-zinc-50 hover:bg-zinc-900/90 dark:bg-zinc-50 dark:text-zinc-900 dark:hover:bg-zinc-50/90";

    /// Secondary button variant.
    pub const SECONDARY: &str = "border border-zinc-200 dark:border-zinc-700 bg-white hover:bg-zinc-100 hover:text-zinc-900 dark:bg-zinc-950 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Danger button variant.
    pub const DANGER: &str = "bg-red-500 text-white hover:bg-red-600 dark:bg-red-900 dark:text-red-50 dark:hover:bg-red-800";

    /// Success button variant.
    pub const SUCCESS: &str = "bg-emerald-500 text-white hover:bg-emerald-600 dark:bg-emerald-900 dark:text-emerald-50 dark:hover:bg-emerald-800";

    /// Warning button variant.
    pub const WARNING: &str = "bg-amber-500 text-white hover:bg-amber-600 dark:bg-amber-900 dark:text-amber-50 dark:hover:bg-amber-800";

    /// Ghost button variant (transparent background).
    pub const GHOST: &str =
        "hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Default button variant.
    pub const DEFAULT: &str = "border border-zinc-200 dark:border-zinc-700 bg-white hover:bg-zinc-100 hover:text-zinc-900 dark:bg-zinc-950 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Small button size.
    pub const SIZE_SMALL: &str = "h-9 rounded-md px-3";

    /// Medium button size (default).
    pub const SIZE_MEDIUM: &str = "h-10 py-2 px-4";

    /// Large button size.
    pub const SIZE_LARGE: &str = "h-11 rounded-md px-8";
}

/// Button variant determines the visual style.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
}

/// Button size determines the dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Properties for the Button component.
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// Click handler callback.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Visual variant of the button.
    #[prop_or_default]
    pub variant: ButtonVariant,

    /// Size of the button.
    #[prop_or_default]
    pub size: ButtonSize,

    /// Additional CSS classes to apply.
    #[prop_or_default]
    pub class: Classes,

    /// Button content (children).
    #[prop_or_default]
    pub children: Children,

    /// Whether the button is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Button type attribute (e.g., "button", "submit", "reset").
    #[prop_or_default]
    pub kind: Option<String>,
}

/// A versatile button component with multiple variants and sizes.
///
/// # Example
///
/// ```rust
/// use wonopui_button::{Button, ButtonVariant, ButtonSize};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let onclick = Callback::from(|_| log::info!("Button clicked!"));
///
///     html! {
///         <Button
///             variant={ButtonVariant::Primary}
///             size={ButtonSize::Medium}
///             onclick={onclick}
///         >
///             {"Click me"}
///         </Button>
///     }
/// }
/// ```
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_class = match props.variant {
        ButtonVariant::Primary => classes::PRIMARY,
        ButtonVariant::Secondary => classes::SECONDARY,
        ButtonVariant::Success => classes::SUCCESS,
        ButtonVariant::Warning => classes::WARNING,
        ButtonVariant::Danger => classes::DANGER,
        ButtonVariant::Ghost => classes::GHOST,
        ButtonVariant::Default => classes::DEFAULT,
    };

    let size_class = match props.size {
        ButtonSize::Small => classes::SIZE_SMALL,
        ButtonSize::Medium => classes::SIZE_MEDIUM,
        ButtonSize::Large => classes::SIZE_LARGE,
    };

    html! {
        <button
            class={classes!(classes::BASE, variant_class, size_class, props.class.clone())}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            type={props.kind.clone().unwrap_or_else(|| "button".to_string())}
        >
            { for props.children.iter() }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_default() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Default);
    }

    #[test]
    fn test_button_size_default() {
        assert_eq!(ButtonSize::default(), ButtonSize::Medium);
    }

    #[test]
    fn test_button_variant_equality() {
        assert_eq!(ButtonVariant::Primary, ButtonVariant::Primary);
        assert_ne!(ButtonVariant::Primary, ButtonVariant::Secondary);
    }

    #[test]
    fn test_button_size_equality() {
        assert_eq!(ButtonSize::Small, ButtonSize::Small);
        assert_ne!(ButtonSize::Small, ButtonSize::Large);
    }

    #[test]
    fn test_button_variant_clone() {
        let v = ButtonVariant::Danger;
        let v2 = v;
        assert_eq!(v, v2);
    }

    #[test]
    fn test_button_size_clone() {
        let s = ButtonSize::Large;
        let s2 = s;
        assert_eq!(s, s2);
    }

    #[test]
    fn test_classes_constants_contain_expected_classes() {
        // Base should contain common button styling
        assert!(classes::BASE.contains("inline-flex"));
        assert!(classes::BASE.contains("rounded"));

        // Variants should have their color styling
        assert!(classes::PRIMARY.contains("bg-zinc"));
        assert!(classes::DANGER.contains("bg-red"));
        assert!(classes::SUCCESS.contains("bg-emerald"));
        assert!(classes::WARNING.contains("bg-amber"));

        // Sizes should have height classes
        assert!(classes::SIZE_SMALL.contains("h-9"));
        assert!(classes::SIZE_MEDIUM.contains("h-10"));
        assert!(classes::SIZE_LARGE.contains("h-11"));
    }
}
