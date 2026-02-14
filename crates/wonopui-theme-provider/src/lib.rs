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
/// Provides consistent styling classes across the library
pub struct BrandGuideType {
    // Form elements
    pub checkbox_label: &'static str,
    pub toggle_label: &'static str,
    pub label_base: &'static str,
    pub input_base: &'static str,
    pub textarea_base: &'static str,
    
    // Typography
    pub typography_h1: &'static str,
    pub typography_h2: &'static str,
    pub typography_h3: &'static str,
    pub typography_h4: &'static str,
    pub typography_h5: &'static str,
    pub typography_h6: &'static str,
    pub typography_p: &'static str,
    
    // Alert
    pub alert_base: &'static str,
    pub alert_success: &'static str,
    pub alert_warning: &'static str,
    pub alert_error: &'static str,
    pub alert_info: &'static str,
    pub alert_title: &'static str,
    pub alert_description: &'static str,
    
    // Button
    pub button_base: &'static str,
    pub button_primary: &'static str,
    pub button_secondary: &'static str,
    pub button_danger: &'static str,
    pub button_success: &'static str,
    pub button_warning: &'static str,
    pub button_ghost: &'static str,
    pub button_default: &'static str,
    
    // Badge
    pub badge_base: &'static str,
    pub badge_success: &'static str,
    pub badge_warning: &'static str,
    pub badge_error: &'static str,
    pub badge_info: &'static str,
    pub badge_default: &'static str,
}

/// Alias for backward compatibility
pub type BrandGuide = BrandGuideType;

/// Type alias for class strings used in styling
pub type ClassesStr = &'static str;

/// Global BRANDGUIDE constant with default Tailwind CSS classes
pub static BRANDGUIDE: BrandGuideType = BrandGuideType {
    // Form elements
    checkbox_label: "ml-2 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
    toggle_label: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-zinc-700 dark:text-zinc-300",
    label_base: "block text-sm font-medium text-zinc-800 dark:text-zinc-100 mb-1.5",
    input_base: "rounded-md border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 w-full px-3.5 py-2.5 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out",
    textarea_base: "rounded-md border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 w-full px-3.5 py-2.5 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out",
    
    // Typography
    typography_h1: "mt-6 mb-10 text-zinc-800 dark:text-zinc-100 text-4xl font-bold tracking-tight",
    typography_h2: "mt-5 mb-8 text-zinc-800 dark:text-zinc-100 text-3xl font-semibold tracking-tight",
    typography_h3: "mt-4 mb-6 text-zinc-800 dark:text-zinc-100 text-2xl font-semibold tracking-tight",
    typography_h4: "mt-3 mb-4 text-zinc-800 dark:text-zinc-100 text-xl font-semibold",
    typography_h5: "mt-2 mb-3 text-zinc-800 dark:text-zinc-100 text-lg font-medium",
    typography_h6: "mt-2 mb-2 text-zinc-800 dark:text-zinc-100 text-base font-medium",
    typography_p: "my-2 text-zinc-800 dark:text-zinc-100 text-base font-normal mb-4 leading-relaxed",
    
    // Alert
    alert_base: "mx-auto max-w-4xl w-full p-4 rounded-md bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700",
    alert_success: "mx-auto max-w-4xl w-full p-4 rounded-md bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700 text-zinc-800 dark:text-zinc-100 border-l-emerald-500 dark:border-l-emerald-500",
    alert_warning: "mx-auto max-w-4xl w-full p-4 rounded-md bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700 text-zinc-800 dark:text-zinc-100 border-l-amber-500 dark:border-l-amber-500",
    alert_error: "mx-auto max-w-4xl w-full p-4 rounded-md bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700 text-zinc-800 dark:text-zinc-100 border-l-red-500 dark:border-l-red-500",
    alert_info: "mx-auto max-w-4xl w-full p-4 rounded-md bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700 text-zinc-800 dark:text-zinc-100 border-l-indigo-500 dark:border-l-indigo-500",
    alert_title: "font-semibold text-lg mb-2",
    alert_description: "text-sm",
    
    // Button
    button_base: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800",
    button_primary: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 bg-indigo-500/90 dark:bg-indigo-600 hover:bg-indigo-700 text-white border border-indigo-500 dark:border-indigo-600 focus:ring-indigo-600",
    button_secondary: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 bg-zinc-500/90 dark:bg-zinc-600 hover:bg-zinc-700 text-white border border-zinc-500 dark:border-zinc-600 focus:ring-zinc-500",
    button_danger: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 bg-red-500/90 dark:bg-red-600 hover:bg-red-700 text-white border border-red-500 dark:border-red-600 focus:ring-red-500",
    button_success: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 bg-emerald-500/90 dark:bg-emerald-600 hover:bg-emerald-700 text-white border border-emerald-500 dark:border-emerald-600 focus:ring-emerald-500",
    button_warning: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 bg-amber-500/90 dark:bg-amber-600 hover:bg-amber-700 text-white border border-amber-500 dark:border-amber-600 focus:ring-amber-500",
    button_ghost: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 text-zinc-800 dark:text-zinc-100 border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-100 dark:hover:bg-zinc-800 focus:ring-zinc-400",
    button_default: "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 bg-zinc-300 dark:bg-zinc-700 hover:bg-zinc-200 dark:hover:bg-zinc-800 text-zinc-700 dark:text-zinc-300 border border-zinc-300 dark:border-zinc-600 focus:ring-zinc-400",
    
    // Badge
    badge_base: "font-medium px-2.5 py-1.5 inline-flex items-center rounded text-xs",
    badge_success: "font-medium px-2.5 py-1.5 inline-flex items-center rounded text-xs bg-emerald-500/90 dark:bg-emerald-600 text-white border border-emerald-500 dark:border-emerald-600",
    badge_warning: "font-medium px-2.5 py-1.5 inline-flex items-center rounded text-xs bg-amber-500/90 dark:bg-amber-600 text-white border border-amber-500 dark:border-amber-600",
    badge_error: "font-medium px-2.5 py-1.5 inline-flex items-center rounded text-xs bg-red-500/90 dark:bg-red-600 text-white border border-red-500 dark:border-red-600",
    badge_info: "font-medium px-2.5 py-1.5 inline-flex items-center rounded text-xs bg-indigo-500/90 dark:bg-indigo-600 text-white border border-indigo-500 dark:border-indigo-600",
    badge_default: "font-medium px-2.5 py-1.5 inline-flex items-center rounded text-xs bg-zinc-300 dark:bg-zinc-700 text-zinc-700 dark:text-zinc-300 border border-zinc-300 dark:border-zinc-600",
};
