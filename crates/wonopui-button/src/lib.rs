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

    /// Icon button variant (square icon-only).
    pub const ICON: &str = "hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

    /// Toolbar button variant (flat with smaller padding).
    pub const TOOLBAR: &str = "hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-zinc-50";

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
    /// Square icon-only button.
    Icon,
    /// Flat toolbar button (smaller padding).
    Toolbar,
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
        ButtonVariant::Outline => classes::OUTLINE,
        ButtonVariant::Success => classes::SUCCESS,
        ButtonVariant::Warning => classes::WARNING,
        ButtonVariant::Danger => classes::DANGER,
        ButtonVariant::Ghost => classes::GHOST,
        ButtonVariant::Link => classes::LINK,
        ButtonVariant::Default => classes::DEFAULT,
        ButtonVariant::Icon => classes::ICON,
        ButtonVariant::Toolbar => classes::TOOLBAR,
    };

    let size_class = if is_toolbar_variant {
        classes::SIZE_TOOLBAR
    } else if is_icon_variant {
        match props.size {
            ButtonSize::Small | ButtonSize::IconSmall => classes::SIZE_ICON_SM,
            ButtonSize::Large | ButtonSize::IconLarge => classes::SIZE_ICON_LG,
            _ => classes::SIZE_ICON,
        }
    } else {
        match props.size {
            ButtonSize::XSmall => classes::SIZE_XS,
            ButtonSize::Small => classes::SIZE_SMALL,
            ButtonSize::Medium => classes::SIZE_MEDIUM,
            ButtonSize::Large => classes::SIZE_LARGE,
            ButtonSize::Icon => classes::SIZE_ICON,
            ButtonSize::IconSmall => classes::SIZE_ICON_SM,
            ButtonSize::IconLarge => classes::SIZE_ICON_LG,
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
            data-slot="button"
            data-variant={format!("{:?}", props.variant).to_lowercase()}
            data-size={format!("{:?}", props.size).to_lowercase()}
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

        // New variants
        assert!(classes::OUTLINE.contains("border"));

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