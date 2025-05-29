// New organization
mod components;
mod config;
mod properties;

#[cfg(feature = "ThemeProvider")]
mod component_editor;

// Prelude for backward compatibility
pub mod prelude;

// Public exports
pub use components::*;
pub use config::BRANDGUIDE;
pub use properties::*;

// Re-export hooks for backward compatibility
#[cfg(feature = "MediaQuery")]
pub use components::utils::media_query::use_media_query;
#[cfg(feature = "WindowProvider")]
pub use components::utils::window_provider::use_window;
#[cfg(feature = "DarkModeProvider")]
pub use components::utils::dark_mode_provider::use_dark_mode;

// Re-export components for backward compatibility
// Layout components
#[cfg(feature = "Layout")]
pub use components::layout::layout::Layout;
#[cfg(feature = "Layout")]
pub use components::layout::layout_context::{LayoutAction, LayoutContext};
#[cfg(feature = "Layout")]
pub use components::layout::LayoutProvider;
#[cfg(feature = "Sidebar")]
pub use components::layout::sidebar::{Sidebar, SidebarHeader, SidebarHeading, SidebarLink, SidebarMenu};
#[cfg(feature = "MulticolSidebar")]
pub use components::layout::multicol_sidebar::SidebarColumn;
#[cfg(feature = "Topbar")]
pub use components::layout::topbar::Topbar;

// Core components
#[cfg(feature = "Button")]
pub use components::core::button::Button;
#[cfg(feature = "Command")]
pub use components::Command;
#[cfg(feature = "Switch")]
pub use components::forms::switch::SwitchButton;

// Form components
#[cfg(feature = "Input")]
pub use components::forms::input::Input;

// Layout components
#[cfg(feature = "Container")]
pub use components::layout::container::{Container, ContainerVariant};

#[cfg(feature = "ThemeProvider")]
pub use component_editor::ComponentEditor;
#[cfg(feature = "ThemeProvider")]
pub use config::{use_brandguide, use_set_brandguide, BrandGuideType, ClassesStr, ThemeProvider};
