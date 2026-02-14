//! Core utilities and types for WonopUI components.
//!
//! This crate provides shared functionality used across all WonopUI components.

/// A utility for merging CSS class strings.
/// 
/// This function takes an array of class strings and combines them
/// into a single space-separated string, filtering out empty strings.
/// 
/// # Example
/// ```
/// use wonopui_core::merge_classes;
/// let classes = merge_classes(&["btn", "btn-primary", ""]);
/// assert_eq!(classes, "btn btn-primary");
/// ```
pub fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}

/// A utility for merging a base class with optional additional classes.
/// 
/// This is a convenience function for the common case of having a base class
/// and optionally adding more classes.
pub fn merge_classes_opt(base: &str, additional: Option<&str>) -> String {
    match additional {
        Some(extra) if !extra.is_empty() => format!("{} {}", base, extra),
        _ => base.to_string(),
    }
}

/// Common size variants used across components.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

/// Common variant types for component styling.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
}

/// Common status types for feedback components.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Status {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Direction for layout components.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

/// Position variants for popovers and tooltips.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Position {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
    BottomStart,
    BottomEnd,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
}

// Re-export commonly used yew types for convenience
pub use yew::prelude::*;
