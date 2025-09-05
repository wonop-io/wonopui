
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiffViewMode {
    SideBySide,
    Inline,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiffLineInfo {
    pub line_number: usize,
    pub side: DiffSide,
    pub content: String,
    pub change_type: ChangeType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiffSide {
    Left,   // Old text
    Right,  // New text
    Both,   // Unchanged line (in inline mode)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
    Unchanged,
}

#[derive(Clone, Debug)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Clone, Debug)]
pub struct DiffLine {
    pub old_line_no: Option<usize>,
    pub new_line_no: Option<usize>,
    pub change_type: ChangeType,
    pub content: String,
    pub word_changes: Option<Vec<WordChange>>,
}

#[derive(Clone, Debug)]
pub struct WordChange {
    pub start: usize,
    pub end: usize,
    pub change_type: ChangeType,
}

impl Default for DiffViewMode {
    fn default() -> Self {
        DiffViewMode::SideBySide
    }
}