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

    /// Outline button variant (transparent with border).
    pub const OUTLINE: &str = "border border-zinc-200 dark:border-zinc-700 bg-transparent hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Link button variant (text link style).
    pub const LINK: &str = "text-zinc-900 underline-offset-4 hover:underline dark:text-zinc-50";

    /// Icon button variant (square icon-only).
    pub const ICON: &str = "hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Toolbar button variant (flat with smaller padding).
    pub const TOOLBAR: &str = "hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Small button size.
    pub const SIZE_SMALL: &str = "h-9 rounded-md px-3";

    /// Medium button size (default).
    pub const SIZE_MEDIUM: &str = "h-10 py-2 px-4";

    /// Large button size.
    pub const SIZE_LARGE: &str = "h-11 rounded-md px-8";

    /// Icon-only small button size (square).
    pub const SIZE_ICON_SMALL: &str = "h-7 w-7";

    /// Icon-only medium button size (square).
    pub const SIZE_ICON_MEDIUM: &str = "h-8 w-8";

    /// Icon-only large button size (square).
    pub const SIZE_ICON_LARGE: &str = "h-10 w-10";

    /// Toolbar button size.
    pub const SIZE_TOOLBAR: &str = "p-1.5";

    /// Active/selected state.
    pub const ACTIVE: &str = "bg-zinc-100 dark:bg-zinc-800";

    /// Full width button.
    pub const FULL_WIDTH: &str = "w-full";

    /// Spinner animation.
    pub const SPINNER: &str = "animate-spin h-4 w-4";

    /// Gap between icon and text.
    pub const ICON_GAP: &str = "gap-2";
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
    /// Transparent with border.
    Outline,
    /// Text link style (underline on hover).
    Link,
    /// Square icon-only button.
    Icon,
    /// Flat toolbar button (smaller padding).
    Toolbar,
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
    /// Click handler callback. Optional to allow disabled buttons without handlers.
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

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

    /// Icon to display before the text.
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Icon to display after the text.
    #[prop_or_default]
    pub icon_after: Option<Html>,

    /// Show loading spinner and disable the button.
    #[prop_or_default]
    pub loading: bool,

    /// Text to display while loading (default: "Loading...").
    #[prop_or_default]
    pub loading_text: Option<String>,

    /// Toggle/selected state.
    #[prop_or_default]
    pub active: bool,

    /// Make button full width (w-full).
    #[prop_or_default]
    pub full_width: bool,

    /// HTML title tooltip.
    #[prop_or_default]
    pub title: Option<String>,
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
    let is_icon_variant = matches!(props.variant, ButtonVariant::Icon);
    let is_toolbar_variant = matches!(props.variant, ButtonVariant::Toolbar);
    let is_disabled = props.disabled || props.loading;

    let variant_class = match props.variant {
        ButtonVariant::Primary => classes::PRIMARY,
        ButtonVariant::Secondary => classes::SECONDARY,
        ButtonVariant::Success => classes::SUCCESS,
        ButtonVariant::Warning => classes::WARNING,
        ButtonVariant::Danger => classes::DANGER,
        ButtonVariant::Ghost => classes::GHOST,
        ButtonVariant::Default => classes::DEFAULT,
        ButtonVariant::Outline => classes::OUTLINE,
        ButtonVariant::Link => classes::LINK,
        ButtonVariant::Icon => classes::ICON,
        ButtonVariant::Toolbar => classes::TOOLBAR,
    };

    let size_class = if is_icon_variant {
        match props.size {
            ButtonSize::Small => classes::SIZE_ICON_SMALL,
            ButtonSize::Medium => classes::SIZE_ICON_MEDIUM,
            ButtonSize::Large => classes::SIZE_ICON_LARGE,
        }
    } else if is_toolbar_variant {
        classes::SIZE_TOOLBAR
    } else {
        match props.size {
            ButtonSize::Small => classes::SIZE_SMALL,
            ButtonSize::Medium => classes::SIZE_MEDIUM,
            ButtonSize::Large => classes::SIZE_LARGE,
        }
    };

    let has_icon_or_children = props.icon.is_some() || props.icon_after.is_some() || !props.children.is_empty();

    let onclick = props.onclick.clone().map(|cb| {
        Callback::from(move |e: MouseEvent| {
            cb.emit(e);
        })
    });

    // Spinner SVG for loading state
    let spinner = html! {
        <svg class={classes::SPINNER} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
    };

    html! {
        <button
            class={classes!(
                classes::BASE,
                variant_class,
                size_class,
                if props.active { classes::ACTIVE } else { "" },
                if props.full_width { classes::FULL_WIDTH } else { "" },
                if has_icon_or_children && !is_icon_variant { classes::ICON_GAP } else { "" },
                props.class.clone()
            )}
            onclick={onclick}
            disabled={is_disabled}
            type={props.kind.clone().unwrap_or_else(|| "button".to_string())}
            title={props.title.clone()}
        >
            if props.loading {
                { spinner }
                { props.loading_text.clone().unwrap_or_else(|| "Loading...".to_string()) }
            } else {
                if let Some(icon) = &props.icon {
                    { icon.clone() }
                }
                { for props.children.iter() }
                if let Some(icon_after) = &props.icon_after {
                    { icon_after.clone() }
                }
            }
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
    fn test_new_variants_exist() {
        let _ = ButtonVariant::Outline;
        let _ = ButtonVariant::Link;
        let _ = ButtonVariant::Icon;
        let _ = ButtonVariant::Toolbar;
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

        // New variants
        assert!(classes::OUTLINE.contains("border"));
        assert!(classes::LINK.contains("underline"));

        // Sizes should have height classes
        assert!(classes::SIZE_SMALL.contains("h-9"));
        assert!(classes::SIZE_MEDIUM.contains("h-10"));
        assert!(classes::SIZE_LARGE.contains("h-11"));

        // Icon sizes should be square
        assert!(classes::SIZE_ICON_SMALL.contains("w-7"));
        assert!(classes::SIZE_ICON_MEDIUM.contains("w-8"));
        assert!(classes::SIZE_ICON_LARGE.contains("w-10"));
    }
}
