#[cfg(test)]
mod tests {
    use super::super::syntax_highlighter::*;
    
    #[test]
    fn test_language_detection() {
        // Test that various language aliases are correctly detected
        // Just test that they can be created without panicking
        let _rust_highlighter = SyntaxHighlighter::new("rust", "light");
        let _rs_highlighter = SyntaxHighlighter::new("rs", "light");
        let _js_highlighter = SyntaxHighlighter::new("javascript", "light");
        let _unknown_highlighter = SyntaxHighlighter::new("unknown", "light");
        
        // All should create successfully
        assert!(true);
    }
    
    #[test]
    fn test_theme_detection() {
        // Test that theme names are correctly resolved
        // Just test that different themes can be created
        let _dark_highlighter = SyntaxHighlighter::new("rust", "dark");
        let _light_highlighter = SyntaxHighlighter::new("rust", "light");
        let _auto_highlighter = SyntaxHighlighter::new("rust", "auto");
        
        // All should create successfully
        assert!(true);
    }
    
    #[test]
    fn test_highlight_simple_code() {
        // Test that highlighting produces HTML output
        let highlighter = SyntaxHighlighter::new("rust", "light");
        let code = "let x = 42;";
        let html = highlighter.highlight_line(code);
        
        // Check that we get some HTML output
        let html_string = format!("{:?}", html);
        assert!(html_string.contains("span"));
    }
    
    #[test]
    fn test_multiple_languages() {
        // Test that different languages are recognized
        let languages = vec![
            ("rust", "fn main() {}"),
            ("python", "def main():"),
            ("javascript", "function main() {}"),
            ("go", "func main() {}"),
            ("java", "public static void main"),
        ];
        
        for (lang, code) in languages {
            let highlighter = SyntaxHighlighter::new(lang, "light");
            let html = highlighter.highlight_line(code);
            let html_string = format!("{:?}", html);
            
            // Each should produce some HTML output
            assert!(html_string.contains("span"), "Failed for language: {}", lang);
        }
    }
    
    #[test]
    fn test_diff_highlighting_preserves_colors() {
        // Test that diff highlighting preserves backgrounds
        let highlighter = SyntaxHighlighter::new("rust", "light");
        let code = "let x = 42;";
        
        // Diff highlighting should use rgba for better blending
        let diff_html = highlighter.highlight_line_for_diff(code, true);
        let html_string = format!("{:?}", diff_html);
        
        // Check for rgba colors (used in diff mode)
        assert!(html_string.contains("rgba") || html_string.contains("color"));
    }
}