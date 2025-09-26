// Utility components

#[cfg(feature = "BrowserProvider")]
pub mod browser_provider;
#[cfg(feature = "Calendar")]
pub mod calendar;
#[cfg(feature = "DarkModeProvider")]
pub mod dark_mode_provider;
#[cfg(feature = "MediaQuery")]
pub mod media_query;
#[cfg(feature = "WindowProvider")]
pub mod window_provider;
// Re-enable when DatePicker feature is fully implemented
// #[cfg(feature = "DatePicker")]
// pub mod date_picker;
#[cfg(feature = "CopyButton")]
pub mod copy_button;
#[cfg(feature = "DragPoint")]
pub mod drag_point;
#[cfg(feature = "Iframe")]
pub mod iframe;
#[cfg(feature = "PaintCanvas")]
pub mod paint_canvas;

#[cfg(feature = "Calendar")]
pub use calendar::Calendar;
#[cfg(feature = "DarkModeProvider")]
pub use dark_mode_provider::{use_dark_mode, DarkModeColor, DarkModeProvider};
#[cfg(feature = "MediaQuery")]
pub use media_query::use_media_query;
#[cfg(feature = "WindowProvider")]
pub use window_provider::{use_window as use_window_legacy, WindowProvider};
// Re-enable when DatePicker feature is fully implemented
// #[cfg(feature = "DatePicker")]
// pub use date_picker::DatePicker;
#[cfg(feature = "CopyButton")]
pub use copy_button::CopyButton;
#[cfg(feature = "DragPoint")]
pub use drag_point::DragPoint;
#[cfg(feature = "Iframe")]
pub use iframe::Iframe;
#[cfg(feature = "PaintCanvas")]
pub use paint_canvas::PaintCanvas;

#[cfg(all(feature = "BrowserProvider", not(feature = "WindowProvider")))]
pub use browser_provider::{
    use_clipboard, use_document, use_local_storage, use_local_storage_state, use_location,
    use_navigation, use_session_storage, use_viewport, use_window, use_window_navigator,
    BrowserContext, BrowserProvider, ClipboardOps, NavigationOps, ViewportInfo,
};

#[cfg(all(feature = "BrowserProvider", feature = "WindowProvider"))]
pub use browser_provider::{
    use_clipboard, use_document, use_local_storage, use_local_storage_state, use_location,
    use_navigation, use_session_storage, use_viewport, use_window as use_browser_window,
    use_window_navigator, BrowserContext, BrowserProvider, ClipboardOps, NavigationOps,
    ViewportInfo,
};
