//! Utility functions for DOM access.

use web_sys::{Document, Window};

/// Get the global window object
pub fn window() -> Window {
    web_sys::window().expect("no global window exists")
}

/// Get the document object
pub fn document() -> Document {
    window().document().expect("no document exists")
}
