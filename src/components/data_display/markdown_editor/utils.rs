// utils.rs
use wasm_bindgen::JsCast;
use web_sys::{Document, Window};

// Helper function to get the document
pub fn document() -> Document {
    web_sys::window()
        .expect("window should exist")
        .document()
        .expect("document should exist")
}

// Helper function to get the window
pub fn window() -> Window {
    web_sys::window().expect("window should exist")
}
