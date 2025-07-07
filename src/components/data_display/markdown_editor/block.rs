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
    fn render(
        &self,
        arguments: String,
        update_block: Callback<Self>,
        onkeydown: Callback<KeyboardEvent>,
        onfocus: Callback<FocusEvent>,
        onblur: Callback<FocusEvent>,
        has_focus: bool,
    ) -> Html;
    fn can_delete(&self) -> bool {
        true // Default implementation returns true
    }

    // Define command triggers for this block type (default is "/")
    fn command_triggers() -> Vec<String> {
        vec!["/".to_string()]
    }

    // Only search is a static method, as it needs to return multiple instances
    fn search(query: Option<String>) -> Vec<Self>;

    // Create a new block of this type
    fn new_block() -> Self;
}

// Block model representing a single block in the editor
#[derive(Clone, PartialEq)]
pub struct Block<T: BlockTrait> {
    pub id: String,
    pub content: String,
    pub block_type: T,
}

impl<T: BlockTrait> Block<T> {
    pub fn new(block_type: T) -> Self {
        Self {
            id: format!("block-{}", Utc::now().timestamp_millis()),
            content: String::new(),
            block_type,
        }
    }
}
