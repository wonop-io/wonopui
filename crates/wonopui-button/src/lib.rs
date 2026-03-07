//! Button component for WonopUI.
//!
//! A versatile button component with multiple variants and sizes.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for button styling.
/// Based on shadcn/ui v4 button component.
pub mod classes {
    /// Base button styles applied to all buttons.
    /// Uses shadcn v4 focus pattern: focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    pub const BASE: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all duration-200 shrink-0 outline-none focus-visible:border-zinc-950 focus-visible:ring-zinc-950/50 focus-visible:ring-[3px] dark:focus-visible:border-zinc-300 dark:focus-visible:ring-zinc-300/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

    /// Primary/Default button variant (shadcn: default).
    /// Dark background with light text.
    pub const PRIMARY: &str = "bg-zinc-900 text-zinc-50 shadow-sm hover:bg-zinc-800 active:bg-zinc-950 dark:bg-zinc-50 dark:text-zinc-900 dark:hover:bg-zinc-200 dark:active:bg-zinc-100";

    /// Secondary button variant (shadcn: secondary).
    /// Muted background with dark text.
    pub const SECONDARY: &str = "bg-zinc-100 text-zinc-900 shadow-sm hover:bg-zinc-200 active:bg-zinc-300 dark:bg-zinc-800 dark:text-zinc-50 dark:hover:bg-zinc-700 dark:active:bg-zinc-600";

    /// Outline button variant (shadcn: outline).
    /// Border with transparent background.
    pub const OUTLINE: &str = "border border-zinc-200 bg-white shadow-xs hover:bg-zinc-100 hover:text-zinc-900 active:bg-zinc-200 dark:border-zinc-800 dark:bg-zinc-950 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 dark:active:bg-zinc-700";

    /// Danger/Destructive button variant (shadcn: destructive).
    /// Red background for dangerous actions.
    pub const DANGER: &str = "bg-red-500 text-white shadow-sm hover:bg-red-600 active:bg-red-700 focus-visible:ring-red-500/50 dark:bg-red-600 dark:hover:bg-red-500 dark:active:bg-red-700 dark:focus-visible:ring-red-400/50";

    /// Success button variant.
    /// Green background for positive actions.
    pub const SUCCESS: &str = "bg-emerald-500 text-white shadow-sm hover:bg-emerald-600 active:bg-emerald-700 focus-visible:ring-emerald-500/50 dark:bg-emerald-600 dark:hover:bg-emerald-500 dark:active:bg-emerald-700 dark:focus-visible:ring-emerald-400/50";

    /// Warning button variant.
    /// Amber background for cautionary actions.
    pub const WARNING: &str = "bg-amber-500 text-white shadow-sm hover:bg-amber-600 active:bg-amber-700 focus-visible:ring-amber-500/50 dark:bg-amber-600 dark:hover:bg-amber-500 dark:active:bg-amber-700 dark:focus-visible:ring-amber-400/50";

    /// Ghost button variant (shadcn: ghost).
    /// Transparent background with hover state.
    pub const GHOST: &str = "hover:bg-zinc-100 hover:text-zinc-900 active:bg-zinc-200 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 dark:active:bg-zinc-700";

    /// Link button variant (shadcn: link).
    /// Text only with underline on hover.
    pub const LINK: &str = "text-zinc-900 underline-offset-4 hover:underline dark:text-zinc-50";

    /// Default button variant (alias for OUTLINE).
    pub const DEFAULT: &str = OUTLINE;

    /// Extra small button size (shadcn: xs).
    pub const SIZE_XS: &str = "h-6 gap-1 rounded-md px-2 text-xs [&_svg:not([class*='size-'])]:size-3";

    /// Small button size (shadcn: sm).
    pub const SIZE_SMALL: &str = "h-8 gap-1.5 rounded-md px-3 [&_svg:not([class*='size-'])]:size-3.5";

    /// Medium button size (default).
    pub const SIZE_MEDIUM: &str = "h-9 px-4 py-2";

    /// Large button size (shadcn: lg).
    pub const SIZE_LARGE: &str = "h-10 rounded-md px-6";

    /// Icon button size (square, for icon-only buttons).
    pub const SIZE_ICON: &str = "size-9";

    /// Small icon button size.
    pub const SIZE_ICON_SM: &str = "size-8 rounded-md";

    /// Large icon button size.
    pub const SIZE_ICON_LG: &str = "size-10";
}

/// Button variant determines the visual style.
/// Matches shadcn/ui v4 button variants.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonVariant {
    /// Default outline style with border
    #[default]
    Default,
    /// Primary filled style (dark bg)
    Primary,
    /// Secondary muted style
    Secondary,
    /// Outline with border (alias for Default)
    Outline,
    /// Success green style
    Success,
    /// Warning amber style
    Warning,
    /// Danger/Destructive red style
    Danger,
    /// Ghost transparent style
    Ghost,
    /// Link text-only style
    Link,
}

/// Button size determines the dimensions.
/// Matches shadcn/ui v4 button sizes.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonSize {
    /// Extra small: h-6
    XSmall,
    /// Small: h-8
    Small,
    /// Default: h-9
    #[default]
    Medium,
    /// Large: h-10
    Large,
    /// Icon square: size-9
    Icon,
    /// Small icon: size-8
    IconSmall,
    /// Large icon: size-10
    IconLarge,
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
/// ```rust,ignore
/// use wonopui_button::{Button, ButtonVariant, ButtonSize};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let onclick = Callback::from(|_| {
///         // Handle button click
///     });
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
        ButtonVariant::Outline => classes::OUTLINE,
        ButtonVariant::Success => classes::SUCCESS,
        ButtonVariant::Warning => classes::WARNING,
        ButtonVariant::Danger => classes::DANGER,
        ButtonVariant::Ghost => classes::GHOST,
        ButtonVariant::Link => classes::LINK,
        ButtonVariant::Default => classes::DEFAULT,
    };

    let size_class = match props.size {
        ButtonSize::XSmall => classes::SIZE_XS,
        ButtonSize::Small => classes::SIZE_SMALL,
        ButtonSize::Medium => classes::SIZE_MEDIUM,
        ButtonSize::Large => classes::SIZE_LARGE,
        ButtonSize::Icon => classes::SIZE_ICON,
        ButtonSize::IconSmall => classes::SIZE_ICON_SM,
        ButtonSize::IconLarge => classes::SIZE_ICON_LG,
    };

    html! {
        <button
            data-slot="button"
            data-variant={format!("{:?}", props.variant).to_lowercase()}
            data-size={format!("{:?}", props.size).to_lowercase()}
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
        // Base should contain common button styling (shadcn v4 patterns)
        assert!(classes::BASE.contains("inline-flex"));
        assert!(classes::BASE.contains("rounded"));
        assert!(classes::BASE.contains("gap-2")); // shadcn v4 adds gap
        assert!(classes::BASE.contains("transition-all")); // smooth transitions
        assert!(classes::BASE.contains("focus-visible:ring")); // focus ring

        // Variants should have their color styling
        assert!(classes::PRIMARY.contains("bg-zinc"));
        assert!(classes::DANGER.contains("bg-red"));
        assert!(classes::SUCCESS.contains("bg-emerald"));
        assert!(classes::WARNING.contains("bg-amber"));
        assert!(classes::GHOST.contains("hover:bg-zinc"));
        assert!(classes::LINK.contains("underline"));

        // Variants should have shadow (premium feel)
        assert!(classes::PRIMARY.contains("shadow"));
        assert!(classes::SECONDARY.contains("shadow"));

        // Sizes should have correct height classes (shadcn v4)
        assert!(classes::SIZE_XS.contains("h-6"));
        assert!(classes::SIZE_SMALL.contains("h-8"));
        assert!(classes::SIZE_MEDIUM.contains("h-9"));
        assert!(classes::SIZE_LARGE.contains("h-10"));
        assert!(classes::SIZE_ICON.contains("size-9"));
    }
}
