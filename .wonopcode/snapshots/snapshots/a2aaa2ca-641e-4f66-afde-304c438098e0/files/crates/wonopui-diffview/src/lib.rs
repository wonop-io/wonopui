//! Diff View component for WonopUI.
//!
//! A component for displaying text diffs with line-by-line comparison.

use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the DiffView component
pub mod classes {
    pub const CONTAINER: &str = "font-mono text-sm overflow-auto border rounded";
    pub const LINE: &str = "flex";
    pub const LINE_NUMBER: &str = "w-12 text-right pr-2 text-muted-foreground select-none border-r";
    pub const LINE_CONTENT: &str = "pl-2 flex-1 whitespace-pre";
    pub const LINE_ADDED: &str = "bg-green-50 dark:bg-green-900/20";
    pub const LINE_REMOVED: &str = "bg-red-50 dark:bg-red-900/20";
    pub const LINE_UNCHANGED: &str = "";
    pub const LINE_NUMBER_ADDED: &str = "w-12 text-right pr-2 text-green-600 dark:text-green-400 select-none border-r bg-green-50 dark:bg-green-900/20";
    pub const LINE_NUMBER_REMOVED: &str = "w-12 text-right pr-2 text-red-600 dark:text-red-400 select-none border-r bg-red-50 dark:bg-red-900/20";
    pub const CONTENT_ADDED: &str = "pl-2 flex-1 whitespace-pre bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-200";
    pub const CONTENT_REMOVED: &str = "pl-2 flex-1 whitespace-pre bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-200";
}

/// Represents a line change type
#[derive(Clone, PartialEq)]
pub enum LineType {
    Added,
    Removed,
    Unchanged,
}

/// Represents a single line in the diff
#[derive(Clone, PartialEq)]
pub struct DiffLine {
    pub line_type: LineType,
    pub old_line_number: Option<usize>,
    pub new_line_number: Option<usize>,
    pub content: String,
}

#[derive(Properties, PartialEq)]
pub struct DiffViewProps {
    /// The lines to display
    pub lines: Vec<DiffLine>,
    #[prop_or_default]
    pub class: Classes,
    /// Show unified view (single column) vs split view (two columns)
    #[prop_or(true)]
    pub unified: bool,
}

#[function_component(DiffView)]
pub fn diff_view(props: &DiffViewProps) -> Html {
    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            { for props.lines.iter().map(|line| {
                let (line_number_class, content_class) = match line.line_type {
                    LineType::Added => (classes::LINE_NUMBER_ADDED, classes::CONTENT_ADDED),
                    LineType::Removed => (classes::LINE_NUMBER_REMOVED, classes::CONTENT_REMOVED),
                    LineType::Unchanged => (classes::LINE_NUMBER, classes::LINE_CONTENT),
                };
                
                let prefix = match line.line_type {
                    LineType::Added => "+",
                    LineType::Removed => "-",
                    LineType::Unchanged => " ",
                };
                
                let line_num = line.new_line_number
                    .or(line.old_line_number)
                    .map(|n| n.to_string())
                    .unwrap_or_default();
                
                html! {
                    <div class={classes::LINE}>
                        <div class={line_number_class}>{ line_num }</div>
                        <div class={content_class}>
                            { prefix }{ &line.content }
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}

/// Helper function to create diff lines from two text strings
/// Note: This is a simple line-by-line diff. For a proper diff,
/// consider using a diff algorithm library.
pub fn simple_diff(old_text: &str, new_text: &str) -> Vec<DiffLine> {
    let old_lines: Vec<&str> = old_text.lines().collect();
    let new_lines: Vec<&str> = new_text.lines().collect();
    
    let mut result = Vec::new();
    let mut old_idx = 0;
    let mut new_idx = 0;
    
    while old_idx < old_lines.len() || new_idx < new_lines.len() {
        match (old_lines.get(old_idx), new_lines.get(new_idx)) {
            (Some(old), Some(new)) if old == new => {
                result.push(DiffLine {
                    line_type: LineType::Unchanged,
                    old_line_number: Some(old_idx + 1),
                    new_line_number: Some(new_idx + 1),
                    content: old.to_string(),
                });
                old_idx += 1;
                new_idx += 1;
            }
            (Some(old), _) => {
                result.push(DiffLine {
                    line_type: LineType::Removed,
                    old_line_number: Some(old_idx + 1),
                    new_line_number: None,
                    content: old.to_string(),
                });
                old_idx += 1;
            }
            (None, Some(new)) => {
                result.push(DiffLine {
                    line_type: LineType::Added,
                    old_line_number: None,
                    new_line_number: Some(new_idx + 1),
                    content: new.to_string(),
                });
                new_idx += 1;
            }
            (None, None) => break,
        }
    }
    
    result
}
