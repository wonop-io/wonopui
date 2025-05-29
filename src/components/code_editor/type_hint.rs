use yew::prelude::*;

/// Represents a type hint in the code editor
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TypeHint {
    /// Line number where the type hint appears (1-indexed)
    pub line_number: usize,
    
    /// Optional column position in the line (0-indexed)
    pub column: Option<usize>,
    
    /// The hint text to display
    pub hint: String,
    
    /// Optional width constraint for the hint in pixels
    #[prop_or(0)]
    pub max_width: u32,
    
    /// Whether to show the hint inline or as a tooltip
    #[prop_or(true)]
    pub inline: bool,
    
    /// CSS classes to apply to the hint
    #[prop_or_default]
    pub class: Classes,
}

impl TypeHint {
    /// Create a new type hint
    pub fn new(line_number: usize, hint: impl Into<String>) -> Self {
        Self {
            line_number,
            column: None,
            hint: hint.into(),
            max_width: 0,
            inline: true,
            class: Classes::new(),
        }
    }
    
    /// Add a column position to this hint
    pub fn at_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }
    
    /// Set a maximum width for this hint
    pub fn with_max_width(mut self, width: u32) -> Self {
        self.max_width = width;
        self
    }
    
    /// Make this hint appear as a tooltip instead of inline
    pub fn as_tooltip(mut self) -> Self {
        self.inline = false;
        self
    }
    
    /// Add CSS classes to this hint
    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class = class.into();
        self
    }
}