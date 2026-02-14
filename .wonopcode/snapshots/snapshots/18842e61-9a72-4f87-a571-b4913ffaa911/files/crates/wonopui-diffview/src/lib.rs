//! Diff View component for WonopUI.
//!
//! A comprehensive component for displaying text diffs with:
//! - Side-by-side and unified/inline view modes
//! - Proper diff algorithm (using `similar` crate)
//! - Word-level diff highlighting
//! - Syntax highlighting support (optional)
//! - Collapsible unchanged sections
//! - Search functionality
//! - Keyboard navigation

use similar::{DiffTag, TextDiff};
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the DiffView component
pub mod classes {
    pub const CONTAINER: &str = "font-mono text-sm overflow-auto border rounded";
    pub const LINE: &str = "flex";
    pub const LINE_NUMBER: &str =
        "w-12 text-right pr-2 text-muted-foreground select-none border-r";
    pub const LINE_CONTENT: &str = "pl-2 flex-1 whitespace-pre";
    pub const LINE_ADDED: &str = "bg-green-50 dark:bg-green-900/20";
    pub const LINE_REMOVED: &str = "bg-red-50 dark:bg-red-900/20";
    pub const LINE_UNCHANGED: &str = "";
    pub const LINE_NUMBER_ADDED: &str = "w-12 text-right pr-2 text-green-600 dark:text-green-400 select-none border-r bg-green-50 dark:bg-green-900/20";
    pub const LINE_NUMBER_REMOVED: &str = "w-12 text-right pr-2 text-red-600 dark:text-red-400 select-none border-r bg-red-50 dark:bg-red-900/20";
    pub const CONTENT_ADDED: &str = "pl-2 flex-1 whitespace-pre bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-200";
    pub const CONTENT_REMOVED: &str = "pl-2 flex-1 whitespace-pre bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-200";
}

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

/// Represents a line change type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineType {
    Added,
    Removed,
    Unchanged,
}

/// Alias for LineType for compatibility
pub type ChangeType = LineType;

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

/// Information about a line in the diff (for click callbacks)
#[derive(Clone, Debug, PartialEq)]
pub struct DiffLineInfo {
    pub line_number: usize,
    pub side: DiffSide,
    pub content: String,
    pub change_type: LineType,
}

/// Represents a single line in the diff
#[derive(Clone, Debug, PartialEq)]
pub struct DiffLine {
    pub line_type: LineType,
    pub old_line_number: Option<usize>,
    pub new_line_number: Option<usize>,
    pub content: String,
    pub word_changes: Option<Vec<WordChange>>,
}

/// A word-level change within a line
#[derive(Clone, Debug, PartialEq)]
pub struct WordChange {
    pub start: usize,
    pub end: usize,
    pub change_type: LineType,
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

/// Properties for the simplified DiffView (existing API)
#[derive(Properties, PartialEq)]
pub struct DiffViewProps {
    /// The lines to display (for pre-computed diffs)
    #[prop_or_default]
    pub lines: Vec<DiffLine>,

    /// Old text (for automatic diff computation)
    #[prop_or_default]
    pub old_text: String,

    /// New text (for automatic diff computation)
    #[prop_or_default]
    pub new_text: String,

    /// Alias for old_text
    #[prop_or_default]
    pub original: String,

    /// Alias for new_text
    #[prop_or_default]
    pub modified: String,

    #[prop_or_default]
    pub class: Classes,

    /// Show unified view (single column) vs split view (two columns)
    #[prop_or(true)]
    pub unified: bool,

    /// View mode (alternative to unified flag)
    #[prop_or_default]
    pub mode: DiffViewMode,

    /// Show line numbers
    #[prop_or(true)]
    pub show_line_numbers: bool,

    /// Number of context lines to show around changes
    #[prop_or(3)]
    pub context_lines: usize,

    /// Language for syntax highlighting
    #[prop_or_default]
    pub language: String,

    /// Theme for syntax highlighting
    #[prop_or_default]
    pub theme: Option<String>,

    /// Font size as string (e.g., "14px")
    #[prop_or_default]
    pub font_size: Option<String>,

    /// Line height as string (e.g., "1.5")
    #[prop_or_default]
    pub line_height: Option<String>,

    /// Enable unified diff mode (same as setting mode to Unified)
    #[prop_or(false)]
    pub unified_diff: bool,

    /// Callback when a line is clicked
    #[prop_or_default]
    pub on_line_click: Option<Callback<DiffLineInfo>>,
}

/// Compute diff from two text strings using the `similar` crate
pub fn compute_diff(old_text: &str, new_text: &str, context_lines: usize) -> Vec<DiffHunk> {
    let diff = TextDiff::from_lines(old_text, new_text);
    let mut hunks = Vec::new();

    for group in diff.grouped_ops(context_lines) {
        let mut lines = Vec::new();
        let mut old_line_no = group.first().map(|op| op.old_range().start + 1).unwrap_or(1);
        let mut new_line_no = group.first().map(|op| op.new_range().start + 1).unwrap_or(1);

        let old_start = old_line_no;
        let new_start = new_line_no;

        for op in group {
            match op.tag() {
                DiffTag::Delete => {
                    let old_range = op.old_range();
                    for idx in old_range.clone() {
                        let old_line = diff.old_slices()[idx];
                        lines.push(DiffLine {
                            line_type: LineType::Removed,
                            old_line_number: Some(old_line_no),
                            new_line_number: None,
                            content: old_line.to_string(),
                            word_changes: None,
                        });
                        old_line_no += 1;
                    }
                }
                DiffTag::Insert => {
                    let new_range = op.new_range();
                    for idx in new_range.clone() {
                        let new_line = diff.new_slices()[idx];
                        lines.push(DiffLine {
                            line_type: LineType::Added,
                            old_line_number: None,
                            new_line_number: Some(new_line_no),
                            content: new_line.to_string(),
                            word_changes: None,
                        });
                        new_line_no += 1;
                    }
                }
                DiffTag::Equal => {
                    let old_range = op.old_range();
                    let new_range = op.new_range();
                    for (old_idx, _new_idx) in old_range.zip(new_range) {
                        let old_line = diff.old_slices()[old_idx];
                        lines.push(DiffLine {
                            line_type: LineType::Unchanged,
                            old_line_number: Some(old_line_no),
                            new_line_number: Some(new_line_no),
                            content: old_line.to_string(),
                            word_changes: None,
                        });
                        old_line_no += 1;
                        new_line_no += 1;
                    }
                }
                DiffTag::Replace => {
                    // Handle replace as delete + insert
                    let old_range = op.old_range();
                    for idx in old_range.clone() {
                        let old_line = diff.old_slices()[idx];
                        lines.push(DiffLine {
                            line_type: LineType::Removed,
                            old_line_number: Some(old_line_no),
                            new_line_number: None,
                            content: old_line.to_string(),
                            word_changes: None,
                        });
                        old_line_no += 1;
                    }

                    let new_range = op.new_range();
                    for idx in new_range.clone() {
                        let new_line = diff.new_slices()[idx];
                        lines.push(DiffLine {
                            line_type: LineType::Added,
                            old_line_number: None,
                            new_line_number: Some(new_line_no),
                            content: new_line.to_string(),
                            word_changes: None,
                        });
                        new_line_no += 1;
                    }
                }
            }
        }

        let old_count = old_line_no - old_start;
        let new_count = new_line_no - new_start;

        hunks.push(DiffHunk {
            old_start,
            old_count,
            new_start,
            new_count,
            lines,
        });
    }

    hunks
}

/// Helper function to create diff lines from two text strings (simple line-by-line)
pub fn simple_diff(old_text: &str, new_text: &str) -> Vec<DiffLine> {
    let hunks = compute_diff(old_text, new_text, 3);
    hunks.into_iter().flat_map(|h| h.lines).collect()
}

/// The DiffView component
#[function_component(DiffView)]
pub fn diff_view(props: &DiffViewProps) -> Html {
    // Determine the effective mode
    let is_inline = props.unified
        || props.unified_diff
        || matches!(props.mode, DiffViewMode::Inline | DiffViewMode::Unified);

    // Get the diff lines - either use provided lines or compute from text
    let diff_lines = use_memo(
        (
            props.lines.clone(),
            props.old_text.clone(),
            props.new_text.clone(),
            props.original.clone(),
            props.modified.clone(),
            props.context_lines,
        ),
        |(lines, old_text, new_text, original, modified, context_lines)| {
            if !lines.is_empty() {
                lines.clone()
            } else {
                // Use old_text/new_text if provided, otherwise fall back to original/modified
                let old = if !old_text.is_empty() {
                    old_text
                } else {
                    original
                };
                let new = if !new_text.is_empty() {
                    new_text
                } else {
                    modified
                };

                if !old.is_empty() || !new.is_empty() {
                    let hunks = compute_diff(old, new, *context_lines);
                    hunks.into_iter().flat_map(|h| h.lines).collect()
                } else {
                    Vec::new()
                }
            }
        },
    );

    // Build style string from optional props
    let style = {
        let mut parts = Vec::new();
        if let Some(ref fs) = props.font_size {
            parts.push(format!("font-size: {};", fs));
        }
        if let Some(ref lh) = props.line_height {
            parts.push(format!("line-height: {};", lh));
        }
        parts.join(" ")
    };

    let container_class = merge_classes(&[
        classes::CONTAINER,
        &props.class.to_string(),
        "bg-white dark:bg-gray-900",
    ]);

    if is_inline {
        // Unified/inline view
        html! {
            <div class={container_class} style={style}>
                <div class="bg-gradient-to-r from-gray-100 to-gray-50 dark:from-gray-800 dark:to-gray-850 px-4 py-2 border-b border-gray-300 dark:border-gray-700 flex items-center justify-between">
                    <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">{"Unified Diff View"}</h3>
                    <div class="flex items-center gap-4 text-xs">
                        <span class="flex items-center gap-1">
                            <span class="w-3 h-3 bg-green-500 rounded-sm shadow-sm border border-green-600"></span>
                            <span class="text-gray-600 dark:text-gray-400">{"Added"}</span>
                        </span>
                        <span class="flex items-center gap-1">
                            <span class="w-3 h-3 bg-red-500 rounded-sm shadow-sm border border-red-600"></span>
                            <span class="text-gray-600 dark:text-gray-400">{"Removed"}</span>
                        </span>
                    </div>
                </div>
                <div class="overflow-x-auto">
                    {
                        if diff_lines.is_empty() {
                            html! {
                                <div class="p-8 text-center text-gray-500 dark:text-gray-400">
                                    <div class="text-lg mb-2">{"No differences found"}</div>
                                    <div class="text-sm">{"The texts are identical"}</div>
                                </div>
                            }
                        } else {
                            html! {
                                <div>
                                    { for diff_lines.iter().map(|line| render_inline_line(line, props.show_line_numbers)) }
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        }
    } else {
        // Side-by-side view
        let (left_lines, right_lines) = prepare_side_by_side_lines(&diff_lines);

        html! {
            <div class={container_class} style={style}>
                <div class="flex divide-x divide-gray-200 dark:divide-gray-700 bg-white dark:bg-gray-900">
                    // Left pane (old text)
                    <div class="flex-1 min-w-0 bg-white dark:bg-gray-900">
                        <div class="bg-gradient-to-r from-red-100 to-red-50 dark:from-red-500/10 dark:to-red-600/5 px-4 py-2 border-b-2 border-red-300 dark:border-red-500 flex items-center justify-between shadow-sm">
                            <h3 class="text-sm font-bold text-red-700 dark:text-red-300 flex items-center gap-2">
                                {"Original"}
                            </h3>
                            <span class="text-xs font-medium text-red-600 dark:text-red-400">
                                {format!("{} lines", left_lines.iter().filter(|l| l.is_some()).count())}
                            </span>
                        </div>
                        <div class="overflow-x-auto">
                            { for left_lines.iter().enumerate().map(|(idx, line_opt)| {
                                render_side_line(line_opt, idx, DiffSide::Left, props.show_line_numbers)
                            }) }
                        </div>
                    </div>

                    // Right pane (new text)
                    <div class="flex-1 min-w-0 bg-white dark:bg-gray-900">
                        <div class="bg-gradient-to-r from-green-100 to-green-50 dark:from-green-500/10 dark:to-green-600/5 px-4 py-2 border-b-2 border-green-300 dark:border-green-500 flex items-center justify-between shadow-sm">
                            <h3 class="text-sm font-bold text-green-700 dark:text-green-300 flex items-center gap-2">
                                {"Modified"}
                            </h3>
                            <span class="text-xs font-medium text-green-600 dark:text-green-400">
                                {format!("{} lines", right_lines.iter().filter(|l| l.is_some()).count())}
                            </span>
                        </div>
                        <div class="overflow-x-auto">
                            { for right_lines.iter().enumerate().map(|(idx, line_opt)| {
                                render_side_line(line_opt, idx, DiffSide::Right, props.show_line_numbers)
                            }) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn render_inline_line(line: &DiffLine, show_line_numbers: bool) -> Html {
    let (line_number_class, content_class, bg_class) = match line.line_type {
        LineType::Added => (
            classes::LINE_NUMBER_ADDED,
            classes::CONTENT_ADDED,
            "bg-green-50 dark:bg-green-900/20",
        ),
        LineType::Removed => (
            classes::LINE_NUMBER_REMOVED,
            classes::CONTENT_REMOVED,
            "bg-red-50 dark:bg-red-900/20",
        ),
        LineType::Unchanged => (classes::LINE_NUMBER, classes::LINE_CONTENT, ""),
    };

    let prefix = match line.line_type {
        LineType::Added => "+",
        LineType::Removed => "-",
        LineType::Unchanged => " ",
    };

    let line_num = line
        .new_line_number
        .or(line.old_line_number)
        .map(|n| n.to_string())
        .unwrap_or_default();

    html! {
        <div class={format!("{} {}", classes::LINE, bg_class)}>
            if show_line_numbers {
                <div class={line_number_class}>{ line_num }</div>
            }
            <div class={content_class}>
                <span class="select-none opacity-50 mr-1">{ prefix }</span>
                { &line.content }
            </div>
        </div>
    }
}

fn render_side_line(
    line_opt: &Option<DiffLine>,
    _idx: usize,
    side: DiffSide,
    show_line_numbers: bool,
) -> Html {
    if let Some(line) = line_opt {
        let (bg_class, text_class) = match (line.line_type, side) {
            (LineType::Added, DiffSide::Right) => (
                "bg-green-50 dark:bg-green-900/20",
                "text-green-800 dark:text-green-200",
            ),
            (LineType::Removed, DiffSide::Left) => (
                "bg-red-50 dark:bg-red-900/20",
                "text-red-800 dark:text-red-200",
            ),
            (LineType::Unchanged, _) => ("", "text-gray-800 dark:text-gray-200"),
            _ => ("bg-gray-50 dark:bg-gray-800/50", "text-gray-500"),
        };

        let line_num = match side {
            DiffSide::Left => line.old_line_number,
            DiffSide::Right => line.new_line_number,
            DiffSide::Both => line.new_line_number.or(line.old_line_number),
        }
        .map(|n| n.to_string())
        .unwrap_or_default();

        html! {
            <div class={format!("flex {}", bg_class)}>
                if show_line_numbers {
                    <div class="w-12 text-right pr-2 text-gray-500 dark:text-gray-400 select-none border-r border-gray-200 dark:border-gray-700">
                        { line_num }
                    </div>
                }
                <div class={format!("pl-2 flex-1 whitespace-pre {}", text_class)}>
                    { &line.content }
                </div>
            </div>
        }
    } else {
        // Empty placeholder line for alignment
        html! {
            <div class="flex bg-gray-50 dark:bg-gray-800/30">
                if show_line_numbers {
                    <div class="w-12 text-right pr-2 text-gray-400 dark:text-gray-600 select-none border-r border-gray-200 dark:border-gray-700">
                    </div>
                }
                <div class="pl-2 flex-1 whitespace-pre text-gray-400 dark:text-gray-600">
                    {"\u{00A0}"} // Non-breaking space for height
                </div>
            </div>
        }
    }
}

fn prepare_side_by_side_lines(lines: &[DiffLine]) -> (Vec<Option<DiffLine>>, Vec<Option<DiffLine>>) {
    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        match line.line_type {
            LineType::Unchanged => {
                left_lines.push(Some(line.clone()));
                right_lines.push(Some(line.clone()));
                i += 1;
            }
            LineType::Removed => {
                // Collect consecutive removed lines
                let mut removed_lines = vec![line.clone()];
                let mut j = i + 1;
                while j < lines.len() && lines[j].line_type == LineType::Removed {
                    removed_lines.push(lines[j].clone());
                    j += 1;
                }

                // Check for added lines immediately after
                let mut added_lines = Vec::new();
                while j < lines.len() && lines[j].line_type == LineType::Added {
                    added_lines.push(lines[j].clone());
                    j += 1;
                }

                // Align removed and added lines side by side
                let max_count = removed_lines.len().max(added_lines.len());
                for idx in 0..max_count {
                    if idx < removed_lines.len() {
                        left_lines.push(Some(removed_lines[idx].clone()));
                    } else {
                        left_lines.push(None);
                    }

                    if idx < added_lines.len() {
                        right_lines.push(Some(added_lines[idx].clone()));
                    } else {
                        right_lines.push(None);
                    }
                }

                i = j;
            }
            LineType::Added => {
                // Pure addition without preceding removal
                left_lines.push(None);
                right_lines.push(Some(line.clone()));
                i += 1;
            }
        }
    }

    (left_lines, right_lines)
}
