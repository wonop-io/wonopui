//! Avatar component for WonopUI.
//!
//! A component for displaying user profile images or initials.

use wonopui_core::*;

/// Default CSS classes for avatar styling.
pub mod classes {
    /// Base avatar styles.
    pub const BASE: &str = "rounded-full object-cover border-2 border-white dark:border-zinc-800 shadow-sm";
    
    /// Small avatar size.
    pub const SIZE_SMALL: &str = "w-8 h-8";
    
    /// Medium avatar size (default).
    pub const SIZE_MEDIUM: &str = "w-12 h-12";
    
    /// Large avatar size.
    pub const SIZE_LARGE: &str = "w-16 h-16";
    
    /// Fallback container for initials.
    pub const FALLBACK: &str = "flex items-center justify-center bg-zinc-200 dark:bg-zinc-700";
    
    /// Initials text styling.
    pub const INITIALS: &str = "text-zinc-700 dark:text-zinc-200 font-medium";
}

/// Avatar size options.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AvatarSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Properties for the Avatar component.
#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    /// Image source URL.
    #[prop_or_default]
    pub src: Option<String>,
    
    /// Alt text, also used to generate initials fallback.
    #[prop_or_default]
    pub alt: String,
    
    /// Avatar size.
    #[prop_or_default]
    pub size: AvatarSize,
    
    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// A component for displaying user profile images or initials.
///
/// # Example
///
/// ```rust
/// use wonopui_avatar::{Avatar, AvatarSize};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <>
///             <Avatar src="https://example.com/photo.jpg" alt="John Doe" />
///             <Avatar alt="Jane Smith" size={AvatarSize::Large} />
///         </>
///     }
/// }
/// ```
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let size_class = match props.size {
        AvatarSize::Small => classes::SIZE_SMALL,
        AvatarSize::Medium => classes::SIZE_MEDIUM,
        AvatarSize::Large => classes::SIZE_LARGE,
    };

    match &props.src {
        Some(src) => html! {
            <img 
                class={classes!(classes::BASE, size_class, props.class.clone())} 
                src={src.clone()} 
                alt={props.alt.clone()} 
            />
        },
        None => {
            let initials = props
                .alt
                .split_whitespace()
                .take(2)
                .filter_map(|word| word.chars().next())
                .map(|c| c.to_uppercase().to_string())
                .collect::<Vec<_>>()
                .join("");

            let initials = if initials.is_empty() {
                "?".to_string()
            } else {
                initials
            };

            html! {
                <div class={classes!(classes::BASE, classes::FALLBACK, size_class, props.class.clone())}>
                    <span class={classes::INITIALS}>{initials}</span>
                </div>
            }
        }
    }
}
