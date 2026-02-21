/// History management for undo/redo operations in the markdown editor.
/// 
/// This module provides a generic history stack that can track changes to blocks
/// and allow users to undo/redo their actions.

use std::collections::VecDeque;

/// Maximum number of history entries to keep
const MAX_HISTORY_SIZE: usize = 100;

/// Represents a snapshot of the editor state at a point in time
#[derive(Clone, Debug)]
pub struct HistoryEntry<T: Clone> {
    /// The blocks state at this point
    pub blocks: Vec<T>,
    /// The active block index
    pub active_index: usize,
    /// Description of the action that led to this state
    pub description: String,
}

/// Manages undo/redo history for the editor
#[derive(Clone, Debug)]
pub struct EditorHistory<T: Clone> {
    /// Past states (undo stack)
    undo_stack: VecDeque<HistoryEntry<T>>,
    /// Future states (redo stack)
    redo_stack: Vec<HistoryEntry<T>>,
    /// Whether we're currently applying an undo/redo operation
    is_applying: bool,
}

impl<T: Clone> Default for EditorHistory<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> EditorHistory<T> {
    /// Create a new empty history
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: Vec::new(),
            is_applying: false,
        }
    }

    /// Push a new state onto the history
    /// 
    /// This clears the redo stack (you can't redo after making new changes)
    pub fn push(&mut self, blocks: Vec<T>, active_index: usize, description: impl Into<String>) {
        // Don't record history while applying undo/redo
        if self.is_applying {
            return;
        }

        let entry = HistoryEntry {
            blocks,
            active_index,
            description: description.into(),
        };

        self.undo_stack.push_back(entry);

        // Clear redo stack when new changes are made
        self.redo_stack.clear();

        // Limit history size
        while self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.undo_stack.len() > 1
    }

    /// Check if redo is available  
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Undo the last action
    /// 
    /// Returns the previous state if available
    pub fn undo(&mut self) -> Option<HistoryEntry<T>> {
        if !self.can_undo() {
            return None;
        }

        self.is_applying = true;

        // Move current state to redo stack
        if let Some(current) = self.undo_stack.pop_back() {
            self.redo_stack.push(current);
        }

        // Return the previous state (without removing it)
        let result = self.undo_stack.back().cloned();
        
        self.is_applying = false;
        result
    }

    /// Redo the last undone action
    /// 
    /// Returns the restored state if available
    pub fn redo(&mut self) -> Option<HistoryEntry<T>> {
        if !self.can_redo() {
            return None;
        }

        self.is_applying = true;

        // Move state from redo stack back to undo stack
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push_back(entry.clone());
            self.is_applying = false;
            return Some(entry);
        }

        self.is_applying = false;
        None
    }

    /// Get the number of undo steps available
    pub fn undo_count(&self) -> usize {
        if self.undo_stack.len() > 0 {
            self.undo_stack.len() - 1
        } else {
            0
        }
    }

    /// Get the number of redo steps available
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Initialize history with the starting state
    pub fn initialize(&mut self, blocks: Vec<T>, active_index: usize) {
        self.clear();
        self.push(blocks, active_index, "Initial state");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_basic() {
        let mut history = EditorHistory::<String>::new();
        
        // Initialize with first state
        history.push(vec!["block1".to_string()], 0, "Initial");
        
        // Add second state
        history.push(vec!["block1".to_string(), "block2".to_string()], 1, "Add block");
        
        assert!(history.can_undo());
        assert!(!history.can_redo());
        
        // Undo
        let prev = history.undo();
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().blocks.len(), 1);
        
        assert!(history.can_redo());
        
        // Redo
        let next = history.redo();
        assert!(next.is_some());
        assert_eq!(next.unwrap().blocks.len(), 2);
    }

    #[test]
    fn test_history_clear_redo_on_new_change() {
        let mut history = EditorHistory::<String>::new();
        
        history.push(vec!["a".to_string()], 0, "First");
        history.push(vec!["b".to_string()], 0, "Second");
        history.push(vec!["c".to_string()], 0, "Third");
        
        // Undo twice
        history.undo();
        history.undo();
        
        assert!(history.can_redo());
        
        // Make new change - should clear redo stack
        history.push(vec!["d".to_string()], 0, "New change");
        
        assert!(!history.can_redo());
    }
}
