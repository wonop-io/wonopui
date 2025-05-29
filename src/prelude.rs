//! The wonopui prelude.
//!
//! This module re-exports all components, enums, and types from the library
//! to provide a single import point for users.

// Re-export the entire components module and all its contents
pub use crate::components::*;

// Re-export core components and their types
#[cfg(feature = "Badge")]
pub use crate::components::core::badge::Badge;
#[cfg(feature = "Button")]
pub use crate::components::core::button::{Button, ButtonSize, ButtonVariant};
#[cfg(feature = "Col")]
pub use crate::components::core::col::Col;
#[cfg(feature = "Badge")]
pub use crate::components::core::BadgeType;
#[cfg(feature = "Command")]
pub use crate::components::Command;

// Re-export data display components and their types
#[cfg(feature = "Avatar")]
pub use crate::components::data_display::AvatarSize;

// Re-export feedback components and their types
#[cfg(feature = "Alert")]
pub use crate::components::feedback::AlertType;

// Re-export form components
#[cfg(feature = "Input")]
pub use crate::components::forms::input::Input;
#[cfg(feature = "Switch")]
pub use crate::components::forms::switch::SwitchButton;

// Re-export layout components and their types
#[cfg(feature = "Container")]
pub use crate::components::layout::container::{Container, ContainerVariant};
#[cfg(feature = "Layout")]
pub use crate::components::layout::layout::Layout;
#[cfg(feature = "Layout")]
pub use crate::components::layout::layout_context::{LayoutAction, LayoutContext};
#[cfg(feature = "MulticolSidebar")]
pub use crate::components::layout::multicol_sidebar::SidebarColumn;
#[cfg(feature = "Sidebar")]
pub use crate::components::layout::sidebar::{
    Sidebar, SidebarHeader, SidebarHeading, SidebarLink, SidebarMenu,
};
#[cfg(feature = "Topbar")]
pub use crate::components::layout::topbar::Topbar;
#[cfg(feature = "Layout")]
pub use crate::components::layout::LayoutDirection;
#[cfg(feature = "Layout")]
pub use crate::components::layout::LayoutProvider;
#[cfg(feature = "Layout")]
pub use crate::components::layout::SidebarPosition;

// Re-export navigation components and their types
#[cfg(feature = "Tabs")]
pub use crate::components::navigation::TabsDirection;

// Re-export overlay components and their types
#[cfg(feature = "Drawer")]
pub use crate::components::overlays::DrawerSide;
#[cfg(feature = "Popover")]
pub use crate::components::overlays::PopoverPosition;
#[cfg(feature = "Dialog")]
pub use crate::components::overlays::{
    Dialog, DialogBody, DialogClose, DialogFooter, DialogHeader, DialogProvider, DialogTitle,
    DialogTrigger,
};

// Re-export code editor components and types
pub use crate::components::code_editor::{CodeEditor, Annotation, AnnotationType, Diff, DiffType, TypeHint};

// Re-export utility components and their types
#[cfg(feature = "CopyButton")]
pub use crate::components::utils::copy_button::CopyButton;
#[cfg(feature = "DarkModeProvider")]
pub use crate::components::utils::dark_mode_provider::use_dark_mode;
#[cfg(feature = "MediaQuery")]
pub use crate::components::utils::media_query::use_media_query;
#[cfg(feature = "WindowProvider")]
pub use crate::components::utils::window_provider::use_window;
#[cfg(feature = "DarkModeProvider")]
pub use crate::components::utils::DarkModeColor;

// Re-export properties
pub use crate::properties::FlexDirection;

// Re-export theme and config related items
#[cfg(feature = "ThemeProvider")]
pub use crate::component_editor::ComponentEditor;
#[cfg(feature = "ThemeProvider")]
pub use crate::config::{
    use_brandguide, use_set_brandguide, BrandGuideType, ClassesStr, ThemeProvider, BRANDGUIDE,
};
