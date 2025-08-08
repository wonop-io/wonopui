// block.rs
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use chrono::Utc;
use std::rc::Rc;
use yew::prelude::*;

// Trait that an enum should implement to be used as a block type
pub trait BlockTrait: Clone + PartialEq + 'static {
    fn icon(&self) -> Html;
    fn name(&self) -> String;

    // Simplified render function focusing only on display
    fn render(
        &self,
        update_block: Callback<Self>,
        onkeydown: Callback<KeyboardEvent>,
        onfocus: Callback<FocusEvent>,
        onblur: Callback<FocusEvent>,
        has_focus: bool,
    ) -> Html;

    // Method to get command triggers - moved from main component
    fn command_triggers() -> Vec<String> {
        vec!["/".to_string()] // Default trigger
    }

    // Method to create a new block of this type
    fn new_block() -> Self;

    // Method to handle command search - moved from main component
    fn search(query: Option<String>) -> Vec<Self>;

    // Can this block be deleted?
    fn can_delete(&self) -> bool {
        true
    }

    // Get available custom actions for this block type
    fn get_block_actions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("duplicate", "Duplicate"),
            ("delete", "Delete"),
            ("move-up", "Move Up"),
            ("move-down", "Move Down"),
        ]
    }

    // Handle custom block action
    fn handle_action(&self, action: &str) -> Option<Self> {
        match action {
            "duplicate" => Some(self.clone()),
            _ => None,
        }
    }
}
