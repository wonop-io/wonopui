// Re-export types from various component categories

// Core components types
#[cfg(feature = "Button")]
pub use crate::components::core::ButtonVariant;
#[cfg(feature = "Button")]
pub use crate::components::core::ButtonSize;
#[cfg(feature = "Badge")]
pub use crate::components::core::BadgeType;

// Data display types
#[cfg(feature = "Avatar")]
pub use crate::components::data_display::AvatarSize;

// Feedback types
#[cfg(feature = "Alert")]
pub use crate::components::feedback::AlertType;

// Layout types
#[cfg(feature = "Container")]
pub use crate::components::layout::ContainerVariant;
#[cfg(feature = "Layout")]
pub use crate::components::layout::LayoutDirection;
#[cfg(feature = "Layout")]
pub use crate::components::layout::SidebarPosition;

// Navigation types
#[cfg(feature = "Tabs")]
pub use crate::components::navigation::TabsDirection;

// Overlay types
#[cfg(feature = "Popover")]
pub use crate::components::overlays::PopoverPosition;
#[cfg(feature = "Drawer")]
pub use crate::components::overlays::DrawerSide;

// Utility types
#[cfg(feature = "DarkModeProvider")]
pub use crate::components::utils::DarkModeColor;

// Also re-export any shared enums and types
pub use crate::properties::FlexDirection;

// Theme-related types
#[cfg(feature = "ThemeProvider")]
pub use crate::theme::ClassesStr;
#[cfg(feature = "ThemeProvider")]
pub use crate::theme::BrandGuideType;