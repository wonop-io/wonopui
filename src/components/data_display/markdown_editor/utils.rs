// utils.rs
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Window};

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

pub fn get_cursor_position(element: &HtmlElement) -> Option<usize> {
    if let Some(selection) = window().get_selection().ok().flatten() {
        if selection.range_count() > 0 {
            if let Ok(range) = selection.get_range_at(0) {
                if let Ok(offset) = range.start_offset() {
                    return Some(offset as usize);
                }
            }
        }
    }
    None
}

pub fn set_cursor_position(element: &HtmlElement, position: usize) -> bool {
    if let Some(selection) = window().get_selection().ok().flatten() {
        if let Ok(range) = document().create_range() {
            if let Some(first_child) = element.first_child() {
                if let Ok(_) = range.set_start(&first_child, position as u32) {
                    range.collapse_with_to_start(true);
                    selection.remove_all_ranges().ok();
                    selection.add_range(&range).ok();
                    return true;
                }
            }
        }
    }
    false
}

pub fn split_text_at_cursor(element: &HtmlElement) -> (String, String) {
    let content = element.inner_text();
    if let Some(pos) = get_cursor_position(element) {
        if pos <= content.len() {
            return (content[..pos].to_string(), content[pos..].to_string());
        }
    }
    (content, String::new())
}
