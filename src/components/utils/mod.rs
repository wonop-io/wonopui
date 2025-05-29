// Utility components

#[cfg(feature = "WindowProvider")]
pub mod window_provider;
#[cfg(feature = "MediaQuery")]
pub mod media_query;
#[cfg(feature = "DarkModeProvider")]
pub mod dark_mode_provider;
#[cfg(feature = "Calendar")]
pub mod calendar;
// Re-enable when DatePicker feature is fully implemented
// #[cfg(feature = "DatePicker")]
// pub mod date_picker;
#[cfg(feature = "PaintCanvas")]
pub mod paint_canvas;
#[cfg(feature = "Iframe")]
pub mod iframe;
#[cfg(feature = "DragPoint")]
pub mod drag_point;
#[cfg(feature = "CopyButton")]
pub mod copy_button;

#[cfg(feature = "WindowProvider")]
pub use window_provider::{use_window, WindowProvider};
#[cfg(feature = "MediaQuery")]
pub use media_query::use_media_query;
#[cfg(feature = "DarkModeProvider")]
pub use dark_mode_provider::{use_dark_mode, DarkModeColor, DarkModeProvider};
#[cfg(feature = "Calendar")]
pub use calendar::Calendar;
// Re-enable when DatePicker feature is fully implemented
// #[cfg(feature = "DatePicker")]
// pub use date_picker::DatePicker;
#[cfg(feature = "PaintCanvas")]
pub use paint_canvas::PaintCanvas;
#[cfg(feature = "Iframe")]
pub use iframe::Iframe;
#[cfg(feature = "DragPoint")]
pub use drag_point::DragPoint;
#[cfg(feature = "CopyButton")]
pub use copy_button::CopyButton;