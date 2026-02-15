//! Media query hook for wonopui
//!
//! Provides a hook to reactively respond to CSS media query changes.

use gloo_events::EventListener;
use yew::prelude::*;

/// Hook to check if a CSS media query matches.
///
/// Returns `true` if the media query matches, `false` otherwise.
/// The value updates automatically when the media query match status changes.
///
/// # Example
///
/// ```rust,ignore
/// use wonopui_media_query::use_media_query;
///
/// #[function_component(ResponsiveComponent)]
/// fn responsive_component() -> Html {
///     let is_mobile = use_media_query("(max-width: 768px)");
///     let prefers_dark = use_media_query("(prefers-color-scheme: dark)");
///
///     html! {
///         <div>
///             if is_mobile {
///                 <p>{"Mobile view"}</p>
///             } else {
///                 <p>{"Desktop view"}</p>
///             }
///             if prefers_dark {
///                 <p>{"Dark mode preferred"}</p>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let query_string = query.to_string();
    let state = use_state_eq(|| false);

    {
        let state = state.clone();
        let query = query_string.clone();
        use_effect_with(query, move |query| {
            let mut listener_option: Option<EventListener> = None;

            if let Some(window) = web_sys::window() {
                if let Ok(Some(media_query_list)) = window.match_media(query) {
                    // Set initial state
                    state.set(media_query_list.matches());

                    let state_clone = state.clone();
                    let media_query_list_clone = media_query_list.clone();

                    let listener = EventListener::new(&media_query_list, "change", move |_event| {
                        state_clone.set(media_query_list_clone.matches());
                    });

                    listener_option = Some(listener);
                }
            }

            move || {
                drop(listener_option);
            }
        });
    }

    *state
}

/// Common media query breakpoints
pub mod breakpoints {
    /// Mobile devices (max-width: 640px)
    pub const SM: &str = "(max-width: 640px)";
    /// Tablets (max-width: 768px)
    pub const MD: &str = "(max-width: 768px)";
    /// Laptops (max-width: 1024px)
    pub const LG: &str = "(max-width: 1024px)";
    /// Desktops (max-width: 1280px)
    pub const XL: &str = "(max-width: 1280px)";
    /// Large desktops (max-width: 1536px)
    pub const XXL: &str = "(max-width: 1536px)";

    /// Minimum width breakpoints (Tailwind-style)
    pub const MIN_SM: &str = "(min-width: 640px)";
    pub const MIN_MD: &str = "(min-width: 768px)";
    pub const MIN_LG: &str = "(min-width: 1024px)";
    pub const MIN_XL: &str = "(min-width: 1280px)";
    pub const MIN_XXL: &str = "(min-width: 1536px)";

    /// Color scheme preferences
    pub const PREFERS_DARK: &str = "(prefers-color-scheme: dark)";
    pub const PREFERS_LIGHT: &str = "(prefers-color-scheme: light)";

    /// Motion preferences
    pub const PREFERS_REDUCED_MOTION: &str = "(prefers-reduced-motion: reduce)";
}
