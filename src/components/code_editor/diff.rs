use yew::prelude::*;

/// Represents a type of diff change
#[derive(Clone, Debug, PartialEq)]
pub enum DiffType {
    /// Line was added (green highlight)
    Added,
    /// Line was removed (red highlight)
    Removed,
    /// Line was modified (yellow highlight)
    Modified
}

/// Represents a diff in the code editor
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Diff {
    /// Line number where the diff appears (1-indexed)
    pub line_number: usize,
    
    /// Type of change this diff represents
    pub diff_type: DiffType,
    
    /// Optional column range for partial line diffs
    /// Format is (start_column, end_column)
    #[prop_or_default]
    pub column_range: Option<(usize, usize)>,
    
    /// Optional message to show when hovering over the diff
    #[prop_or_default]
    pub message: Option<String>,
}

impl Diff {
    /// Create a new diff for an added line
    pub fn added(line_number: usize) -> Self {
        Self {
            line_number,
            diff_type: DiffType::Added,
            column_range: None,
            message: None,
        }
    }
    
    /// Create a new diff for a removed line
    pub fn removed(line_number: usize) -> Self {
        Self {
            line_number,
            diff_type: DiffType::Removed,
            column_range: None,
            message: None,
        }
    }
    
    /// Create a new diff for a modified line
    pub fn modified(line_number: usize) -> Self {
        Self {
            line_number,
            diff_type: DiffType::Modified,
            column_range: None,
            message: None,
        }
    }
    
    /// Add a column range to this diff
    pub fn with_column_range(mut self, start: usize, end: usize) -> Self {
        self.column_range = Some((start, end));
        self
    }
    
    /// Add a message to this diff
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}