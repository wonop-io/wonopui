// block.rs
use yew::prelude::*;

/// Generate a UUID v4 using JavaScript's crypto API
fn uuid_v4() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Math;
        // Simple UUID v4 generation
        let mut bytes = [0u8; 16];
        for byte in bytes.iter_mut() {
            *byte = (Math::random() * 256.0) as u8;
        }
        // Set version (4) and variant bits
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Fallback for non-wasm targets (e.g., tests)
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        format!("uuid-{}-{}", duration.as_secs(), duration.subsec_nanos())
    }
}

// Trait that an enum should implement to be used as a block type
pub trait BlockTrait: Clone + PartialEq + 'static {
    /// Get a unique identifier for this block.
    /// Default implementation generates a UUID using js_sys.
    fn id(&self) -> String {
        // Default: generate a new UUID each time (not ideal for persistence)
        // Implementations should override this with their own ID tracking
        uuid_v4()
    }

    /// Create a block with a specific ID.
    /// Default implementation ignores the ID (implementations should override).
    fn with_id(self, _id: String) -> Self {
        self
    }

    fn icon(&self) -> Html;
    fn name(&self) -> String;

    // Get the text content of this block
    fn get_content(&self) -> String;

    // Create a new block of the same type with updated content
    fn set_content(&self, content: String) -> Self;

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

    // Create a new block with initial content
    fn new_block_with_content(content: String) -> Self {
        Self::new_block().set_content(content)
    }

    // Method to handle command search - moved from main component
    fn search(query: Option<String>) -> Vec<Self>;

    // Can this block be deleted?
    fn can_delete(&self) -> bool {
        self.get_content().is_empty()
    }

    // Check if cursor is at the start of content (for backspace handling)
    fn is_cursor_at_start(&self) -> bool {
        self.get_content().is_empty()
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

    // Convert block to markdown representation
    fn to_markdown(&self) -> String {
        // Default implementation - blocks should override this
        self.get_content()
    }
}
