//! Types for diff view functionality

/// View mode for the diff viewer
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DiffViewMode {
    /// Side-by-side view (two columns)
    #[default]
    SideBySide,
    /// Unified/inline view (single column)
    Inline,
    /// Alias for SideBySide
    Split,
    /// Alias for Inline
    Unified,
}

/// Information about a line in the diff
#[derive(Clone, Debug, PartialEq)]
pub struct DiffLineInfo {
    pub line_number: usize,
    pub side: DiffSide,
    pub content: String,
    pub change_type: ChangeType,
}

/// Which side of the diff a line is on
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiffSide {
    /// Left side (old text)
    Left,
    /// Right side (new text)
    Right,
    /// Both sides (unchanged line, in inline mode)
    Both,
}

/// Type of change for a line
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
    Unchanged,
}

/// A hunk (group of changes) in a diff
#[derive(Clone, Debug)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

/// A single line in a diff
#[derive(Clone, Debug)]
pub struct DiffLine {
    pub old_line_no: Option<usize>,
    pub new_line_no: Option<usize>,
    pub change_type: ChangeType,
    pub content: String,
    pub word_changes: Option<Vec<WordChange>>,
}

/// A word-level change within a line
#[derive(Clone, Debug)]
pub struct WordChange {
    pub start: usize,
    pub end: usize,
    pub change_type: ChangeType,
}
