//! Avatar component for WonopUI.
//!
//! A component for displaying user profile images or initials.
//! Styled to match shadcn/ui v4 design system.

use wonopui_core::*;

/// Default CSS classes for avatar styling.
/// Based on shadcn/ui v4 avatar component.
pub mod classes {
    /// Base avatar styles - matches shadcn v4 Avatar.
    pub const BASE: &str = "group/avatar relative flex shrink-0 overflow-hidden rounded-full select-none";

    /// Small avatar size (shadcn: sm).
    pub const SIZE_SMALL: &str = "size-6";

    /// Medium avatar size (default).
    pub const SIZE_MEDIUM: &str = "size-8";

    /// Large avatar size (shadcn: lg).
    pub const SIZE_LARGE: &str = "size-10";

    /// Image styles - matches shadcn v4 AvatarImage with object-cover
    pub const IMAGE: &str = "aspect-square size-full object-cover";

    /// Fallback container for initials - matches shadcn v4 AvatarFallback.
    pub const FALLBACK: &str = "bg-zinc-100 dark:bg-zinc-800 text-zinc-500 dark:text-zinc-400 flex size-full items-center justify-center rounded-full text-sm group-data-[size=sm]/avatar:text-xs";

    /// Avatar badge styles (for status indicators).
    pub const BADGE: &str = "bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 ring-white dark:ring-zinc-950 absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full ring-2 select-none";

    /// Avatar group container.
    pub const GROUP: &str = "group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-white dark:*:data-[slot=avatar]:ring-zinc-950 *:data-[slot=avatar]:ring-2";

    /// Avatar group count indicator.
    pub const GROUP_COUNT: &str = "bg-zinc-100 dark:bg-zinc-800 text-zinc-500 dark:text-zinc-400 ring-white dark:ring-zinc-950 relative flex size-8 shrink-0 items-center justify-center rounded-full text-sm ring-2";
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
    let (size_class, size_attr) = match props.size {
        AvatarSize::Small => (classes::SIZE_SMALL, "sm"),
        AvatarSize::Medium => (classes::SIZE_MEDIUM, "default"),
        AvatarSize::Large => (classes::SIZE_LARGE, "lg"),
    };

    match &props.src {
        Some(src) => html! {
            <span
                data-slot="avatar"
                data-size={size_attr}
                class={classes!(classes::BASE, size_class, props.class.clone())}
            >
                <img
                    data-slot="avatar-image"
                    class={classes::IMAGE}
                    src={src.clone()}
                    alt={props.alt.clone()}
                />
            </span>
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
                <span
                    data-slot="avatar"
                    data-size={size_attr}
                    class={classes!(classes::BASE, size_class, props.class.clone())}
                >
                    <span data-slot="avatar-fallback" class={classes::FALLBACK}>
                        {initials}
                    </span>
                </span>
            }
        }
    }
}
