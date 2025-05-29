use std::fmt;
use yew::prelude::*;

/// Represents a type of annotation in the code editor
#[derive(Clone, Debug, PartialEq)]
pub enum AnnotationType {
    /// Error annotation (red)
    Error,
    /// Warning annotation (yellow)
    Warning,
    /// Information annotation (blue)
    Info,
    /// Success annotation (green)
    Success
}

impl fmt::Display for AnnotationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnnotationType::Error => write!(f, "error"),
            AnnotationType::Warning => write!(f, "warning"),
            AnnotationType::Info => write!(f, "info"),
            AnnotationType::Success => write!(f, "success"),
        }
    }
}

/// Represents an annotation in the code editor
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Annotation {
    /// Line number where the annotation appears (1-indexed)
    pub line_number: usize,
    
    /// Type of this annotation
    pub annotation_type: AnnotationType,
    
    /// The message to display for this annotation
    pub message: String,
    
    /// Optional column range where this annotation applies
    /// Format is (start_column, end_column)
    #[prop_or_default]
    pub column_range: Option<(usize, usize)>,
    
    /// Whether this annotation should be displayed inline or as a popup
    #[prop_or(false)]
    pub inline: bool,
}

impl Annotation {
    /// Create a new error annotation
    pub fn error(line_number: usize, message: impl Into<String>) -> Self {
        Self {
            line_number,
            annotation_type: AnnotationType::Error,
            message: message.into(),
            column_range: None,
            inline: false,
        }
    }
    
    /// Create a new warning annotation
    pub fn warning(line_number: usize, message: impl Into<String>) -> Self {
        Self {
            line_number,
            annotation_type: AnnotationType::Warning,
            message: message.into(),
            column_range: None,
            inline: false,
        }
    }
    
    /// Create a new info annotation
    pub fn info(line_number: usize, message: impl Into<String>) -> Self {
        Self {
            line_number,
            annotation_type: AnnotationType::Info,
            message: message.into(),
            column_range: None,
            inline: false,
        }
    }
    
    /// Create a new success annotation
    pub fn success(line_number: usize, message: impl Into<String>) -> Self {
        Self {
            line_number,
            annotation_type: AnnotationType::Success,
            message: message.into(),
            column_range: None,
            inline: false,
        }
    }
    
    /// Make this annotation inline
    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }
    
    /// Add a column range to this annotation
    pub fn with_column_range(mut self, start: usize, end: usize) -> Self {
        self.column_range = Some((start, end));
        self
    }
}