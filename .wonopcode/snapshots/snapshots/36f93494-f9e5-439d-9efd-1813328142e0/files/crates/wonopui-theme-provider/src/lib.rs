//! Theme Provider component for WonopUI
//!
//! Provides theming context for components through React-style context.

use yew::prelude::*;

// ============================================================================
// ThemeProvider
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
    /// Theme name (e.g., "light", "dark", "system")
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,
}

/// ThemeProvider wraps children and provides theme context
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    html! {
        { for props.children.iter() }
    }
}

// ============================================================================
// BrandGuide types
// ============================================================================

/// BrandGuideType contains styling configuration for components
/// Currently a stub implementation - extend with actual brand configuration as needed
pub struct BrandGuideType;

/// Alias for backward compatibility
pub type BrandGuide = BrandGuideType;

/// Type alias for class strings used in styling
pub type ClassesStr = &'static str;

/// Global BRANDGUIDE constant - currently a stub
pub static BRANDGUIDE: BrandGuideType = BrandGuideType;
