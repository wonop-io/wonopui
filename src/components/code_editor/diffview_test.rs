#[cfg(test)]
mod tests {
    use super::super::diffview::*;
    use super::super::diff_types::*;
    
    #[test]
    fn test_diff_computation() {
        let old_text = "line1\nline2\nline3";
        let new_text = "line1\nline2 modified\nline3\nline4";
        
        let hunks = compute_diff(old_text, new_text, 1);
        
        assert!(!hunks.is_empty());
        
        // Check that we have the expected changes
        let has_modified = hunks.iter()
            .flat_map(|h| &h.lines)
            .any(|l| matches!(l.change_type, ChangeType::Removed | ChangeType::Added));
        
        assert!(has_modified, "Should detect modifications");
    }
    
    #[test]
    fn test_identical_texts() {
        let text = "same\ntext\nhere";
        let hunks = compute_diff(text, text, 3);
        
        // When texts are identical, there should be no hunks (no changes to show)
        // or if there are hunks, all lines should be unchanged
        if !hunks.is_empty() {
            let all_unchanged = hunks.iter()
                .flat_map(|h| &h.lines)
                .all(|l| matches!(l.change_type, ChangeType::Unchanged));
            assert!(all_unchanged, "All lines should be unchanged when texts are identical");
        }
    }
    
    #[test]
    fn test_empty_diff() {
        let hunks = compute_diff("", "", 3);
        assert!(hunks.is_empty() || hunks[0].lines.is_empty());
    }
    
    #[test]
    fn test_word_diff() {
        let old = "The quick brown fox";
        let new = "The fast brown fox";
        
        let word_changes = compute_word_diff(old, new);
        
        // Should detect word-level changes
        assert!(!word_changes.is_empty(), "Should detect word changes");
        
        // Should have both removals and additions
        let has_removed = word_changes.iter().any(|w| w.change_type == ChangeType::Removed);
        let has_added = word_changes.iter().any(|w| w.change_type == ChangeType::Added);
        
        assert!(has_removed, "Should detect removed words");
        assert!(has_added, "Should detect added words");
    }
    
    #[test]
    fn test_tokenize_line() {
        let text = "hello_world(123, 'test')";
        let tokens = tokenize_line(text);
        
        // Should tokenize into meaningful parts
        assert!(!tokens.is_empty());
        assert!(tokens.iter().any(|t| t.contains("hello_world")));
        assert!(tokens.iter().any(|t| t.contains("123")));
    }
    
    #[test]
    fn test_compute_diff_with_word_option() {
        let old = "function calculate(a, b) { return a + b; }";
        let new = "function compute(x, y) { return x + y; }";
        
        let hunks = compute_diff_with_options(old, new, 0, true);
        
        assert!(!hunks.is_empty(), "Should produce hunks for changes");
    }
    
    #[test]
    fn test_context_lines() {
        let old = "1\n2\n3\n4\n5";
        let new = "1\n2\nX\n4\n5";
        
        let hunks_no_ctx = compute_diff(old, new, 0);
        let hunks_with_ctx = compute_diff(old, new, 2);
        
        assert!(!hunks_no_ctx.is_empty());
        assert!(!hunks_with_ctx.is_empty());
        
        // With context should include surrounding unchanged lines
        let lines_with_ctx: usize = hunks_with_ctx.iter().map(|h| h.lines.len()).sum();
        let lines_no_ctx: usize = hunks_no_ctx.iter().map(|h| h.lines.len()).sum();
        
        assert!(lines_with_ctx >= lines_no_ctx, "Context should include more lines");
    }
    
    #[test]
    fn test_hunk_line_numbers() {
        let old = "line1\nline2\nline3";
        let new = "line1\nmodified\nline3\nadded";
        
        let hunks = compute_diff(old, new, 0);
        
        assert!(!hunks.is_empty());
        
        // Check line numbers are properly set
        let first_hunk = &hunks[0];
        assert!(first_hunk.old_start > 0);
        assert!(first_hunk.new_start > 0);
        
        // Check that lines have appropriate line numbers
        for line in &first_hunk.lines {
            match line.change_type {
                ChangeType::Removed => assert!(line.old_line_no.is_some()),
                ChangeType::Added => assert!(line.new_line_no.is_some()),
                ChangeType::Unchanged => {
                    assert!(line.old_line_no.is_some());
                    assert!(line.new_line_no.is_some());
                }
                _ => {}
            }
        }
    }
}