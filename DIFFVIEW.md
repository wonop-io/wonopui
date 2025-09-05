# DiffView Component - Comprehensive Implementation Plan

## Overview
The DiffView component will be a powerful, feature-rich diff visualization component for the WonopUI library. It will compute differences between two text strings and display them with syntax highlighting in either side-by-side or inline mode.

## Architecture Overview

### Component Location
- **Main Component**: `src/components/code_editor/diffview.rs`
- **Gallery Example**: `examples/gallery/src/components/diffview.rs`
- **Supporting Types**: `src/components/code_editor/diff_types.rs`

### Key Dependencies
- **similar**: A diff library for Rust that provides robust text diffing algorithms (Myers, Patience, LCS)
- **syntect**: Already in use for syntax highlighting
- **yew**: Core framework
- **web-sys**: For DOM manipulation

## Detailed Component Structure

### 1. Core DiffView Component (`src/components/code_editor/diffview.rs`)

```rust
#[derive(Properties, PartialEq, Clone)]
pub struct DiffViewProps {
    // Required props
    pub old_text: String,           // Original text
    pub new_text: String,           // Modified text
    
    // Display options
    #[prop_or(DiffViewMode::SideBySide)]
    pub mode: DiffViewMode,         // Side-by-side or inline
    
    #[prop_or(true)]
    pub show_line_numbers: bool,    // Show line numbers
    
    #[prop_or(3)]
    pub context_lines: usize,       // Lines of context around changes
    
    // Styling
    #[prop_or_else(|| "rust".to_string())]
    pub language: String,           // Language for syntax highlighting
    
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,              // Color theme
    
    #[prop_or(14)]
    pub font_size: u8,              
    
    #[prop_or_else(|| "JetBrains Mono, monospace".to_string())]
    pub font_family: String,
    
    #[prop_or(1.5)]
    pub line_height: f32,
    
    // Features
    #[prop_or(false)]
    pub unified_diff: bool,         // Show unified diff header
    
    #[prop_or(false)]
    pub word_diff: bool,            // Highlight word-level changes
    
    #[prop_or(false)]
    pub ignore_whitespace: bool,    // Ignore whitespace changes
    
    #[prop_or(false)]
    pub collapsible_unchanged: bool, // Collapse unchanged sections
    
    // Callbacks
    #[prop_or_default]
    pub on_line_click: Option<Callback<DiffLineInfo>>,
    
    // Styling classes
    #[prop_or_default]
    pub class: Classes,
}
```

### 2. Supporting Types (`src/components/code_editor/diff_types.rs`)

```rust
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum DiffSide {
    Left,   // Old text
    Right,  // New text
    Both,   // Unchanged line (in inline mode)
}

#[derive(Clone, Debug, PartialEq)]
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
    pub word_changes: Option<Vec<WordChange>>, // For word-level diff
}

#[derive(Clone, Debug)]
pub struct WordChange {
    pub start: usize,
    pub end: usize,
    pub change_type: ChangeType,
}
```

## Implementation Steps

### Step 1: Add Dependencies (Cargo.toml)
```toml
[dependencies]
similar = { version = "2.4", features = ["text", "inline"] }
```

### Step 2: Core Diff Algorithm Implementation

#### 2.1 Text Diffing Function
```rust
fn compute_diff(old: &str, new: &str, options: &DiffOptions) -> Vec<DiffHunk> {
    use similar::{ChangeTag, TextDiff};
    
    let diff = TextDiff::from_lines(old, new);
    let mut hunks = Vec::new();
    
    // Process diff ops into hunks
    for group in diff.grouped_ops(options.context_lines) {
        let mut hunk_lines = Vec::new();
        
        for op in group {
            match op.tag() {
                ChangeTag::Delete => {
                    // Lines removed from old
                },
                ChangeTag::Insert => {
                    // Lines added to new
                },
                ChangeTag::Equal => {
                    // Unchanged lines
                }
            }
        }
        
        hunks.push(create_hunk(hunk_lines));
    }
    
    hunks
}
```

#### 2.2 Word-Level Diff (Optional Enhancement)
```rust
fn compute_word_diff(old_line: &str, new_line: &str) -> Vec<WordChange> {
    // Use similar's inline diff capabilities
    // to highlight changed words within modified lines
}
```

### Step 3: Rendering Implementation

#### 3.1 Side-by-Side View
```rust
fn render_side_by_side(&self) -> Html {
    html! {
        <div class="diff-view-container flex">
            // Left pane (old text)
            <div class="diff-pane diff-pane-old flex-1">
                <div class="diff-header">{"Original"}</div>
                { self.render_diff_lines(DiffSide::Left) }
            </div>
            
            // Center gutter (optional)
            <div class="diff-gutter">
                { self.render_change_indicators() }
            </div>
            
            // Right pane (new text)
            <div class="diff-pane diff-pane-new flex-1">
                <div class="diff-header">{"Modified"}</div>
                { self.render_diff_lines(DiffSide::Right) }
            </div>
        </div>
    }
}
```

#### 3.2 Inline View
```rust
fn render_inline(&self) -> Html {
    html! {
        <div class="diff-view-container">
            <div class="diff-header">{"Changes"}</div>
            { 
                for self.diff_hunks.iter().map(|hunk| {
                    self.render_inline_hunk(hunk)
                })
            }
        </div>
    }
}
```

### Step 4: Syntax Highlighting Integration

```rust
fn highlight_line(&self, content: &str, language: &str) -> Html {
    // Reuse existing syntax highlighting from CodeEditor
    // Apply diff styling on top of syntax highlighting
    
    let highlighted = self.apply_syntax_highlighting(content, language);
    html! {
        <span class="diff-line-content">
            { highlighted }
        </span>
    }
}
```

### Step 5: Styling Classes

#### Base Styles
```css
.diff-view-container {
    @apply border border-gray-300 dark:border-gray-700 rounded overflow-hidden;
}

.diff-line-added {
    @apply bg-emerald-50 dark:bg-emerald-950 text-emerald-900 dark:text-emerald-100;
}

.diff-line-removed {
    @apply bg-rose-50 dark:bg-rose-950 text-rose-900 dark:text-rose-100;
}

.diff-line-modified {
    @apply bg-amber-50 dark:bg-amber-950 text-amber-900 dark:text-amber-100;
}

.diff-word-added {
    @apply bg-emerald-200 dark:bg-emerald-800;
}

.diff-word-removed {
    @apply bg-rose-200 dark:bg-rose-800;
}
```

## Gallery Example Implementation

### Basic Example
```rust
#[function_component(DiffViewExample)]
pub fn diff_view_example() -> Html {
    let old_code = r#"fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = calculate_sum(5, 3);
    println!("Sum: {}", result);
}"#;

    let new_code = r#"fn calculate_sum(a: i32, b: i32) -> i32 {
    // Add logging for debugging
    println!("Calculating {} + {}", a, b);
    let sum = a + b;
    sum
}

fn calculate_product(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let sum_result = calculate_sum(5, 3);
    let product_result = calculate_product(5, 3);
    println!("Sum: {}", sum_result);
    println!("Product: {}", product_result);
}"#;

    html! {
        <ExampleCode
            preview={html! {
                <div class="space-y-4">
                    <h3>{"Side-by-Side View"}</h3>
                    <DiffView
                        old_text={old_code}
                        new_text={new_code}
                        mode={DiffViewMode::SideBySide}
                        language="rust"
                        show_line_numbers=true
                    />
                    
                    <h3>{"Inline View"}</h3>
                    <DiffView
                        old_text={old_code}
                        new_text={new_code}
                        mode={DiffViewMode::Inline}
                        language="rust"
                        show_line_numbers=true
                        word_diff=true
                    />
                </div>
            }}
            code={example_source_code}
        />
    }
}
```

### Advanced Features Example
```rust
// Example with:
// - Word-level diff highlighting
// - Collapsible unchanged sections
// - Custom themes
// - Whitespace handling options
// - Line click callbacks
```

## Component Features

### 1. Core Features
- [x] Compute diff between two strings
- [x] Side-by-side view mode
- [x] Inline view mode
- [x] Syntax highlighting support
- [x] Line numbers
- [x] Dark/light theme support

### 2. Advanced Features
- [ ] Word-level diff highlighting
- [ ] Collapsible unchanged sections
- [ ] Ignore whitespace option
- [ ] Copy button for individual hunks
- [ ] Export diff as unified format
- [ ] Search within diff
- [ ] Keyboard navigation

### 3. Interactive Features
- [ ] Click to copy line
- [ ] Hover for more context
- [ ] Expand/collapse hunks
- [ ] Toggle between modes
- [ ] Accept/reject changes (for editor integration)

## Testing Strategy

### Unit Tests
1. Test diff computation with various inputs
2. Test line number calculation
3. Test word diff algorithm
4. Test whitespace handling

### Integration Tests
1. Test with CodeEditor component
2. Test mode switching
3. Test syntax highlighting integration
4. Test responsive layout

### Visual Tests
1. Side-by-side layout on different screen sizes
2. Color contrast in light/dark modes
3. Syntax highlighting accuracy
4. Line number alignment

## Performance Considerations

### Optimization Strategies
1. **Virtual Scrolling**: For large diffs, implement virtual scrolling
2. **Lazy Syntax Highlighting**: Highlight only visible lines
3. **Memoization**: Cache diff computation results
4. **Web Workers**: Compute diffs in background for large files

### Performance Targets
- Diff computation: < 100ms for 1000 lines
- Initial render: < 200ms
- Mode switch: < 50ms
- Smooth scrolling at 60fps

## Accessibility

### ARIA Attributes
```html
<div role="region" aria-label="Code diff viewer">
    <div role="group" aria-label="Original code">
    <div role="group" aria-label="Modified code">
```

### Keyboard Navigation
- Tab: Navigate between panes
- Arrow keys: Navigate lines
- Space: Expand/collapse sections
- Ctrl+F: Search within diff

## Documentation

### Component Props Documentation
Each prop should be documented with:
- Type
- Default value
- Description
- Example usage

### Usage Examples
1. Basic diff view
2. Syntax highlighted diff
3. Interactive diff with callbacks
4. Custom styled diff
5. Diff with word-level highlights

## Implementation Timeline

### Phase 1: Core Implementation (Days 1-2)
- Add dependencies
- Create component structure
- Implement basic diff computation
- Create side-by-side view

### Phase 2: Inline View (Day 3)
- Implement inline view mode
- Add mode switching
- Test both modes

### Phase 3: Syntax Highlighting (Day 4)
- Integrate syntect
- Apply highlighting to diff lines
- Handle theme switching

### Phase 4: Gallery & Documentation (Day 5)
- Create gallery examples
- Write documentation
- Add to navigation

### Phase 5: Advanced Features (Days 6-7)
- Word-level diff
- Collapsible sections
- Interactive features

### Phase 6: Polish & Testing (Day 8)
- Performance optimization
- Cross-browser testing
- Accessibility testing
- Final documentation

## Success Criteria

### Functional Requirements
- [x] Computes accurate diffs
- [x] Displays in both modes
- [x] Syntax highlighting works
- [x] Responsive on all devices
- [x] Dark mode support

### Quality Requirements
- [x] Performance targets met
- [x] Accessible (WCAG 2.1 AA)
- [x] Well-documented
- [x] Comprehensive examples
- [x] Clean, maintainable code

## Notes for Junior Developers

### Getting Started
1. Read the existing CodeEditor component code first
2. Understand how Yew components work
3. Learn about the similar crate's API
4. Start with the simplest implementation

### Key Concepts to Understand
1. **Diff Algorithms**: Myers, LCS, Patience algorithms
2. **Yew Lifecycle**: Component creation, updates, rendering
3. **Virtual DOM**: How Yew efficiently updates the DOM
4. **Syntax Highlighting**: How syntect tokenizes code

### Common Pitfalls to Avoid
1. Don't re-compute diffs on every render (use memoization)
2. Don't forget to handle edge cases (empty strings, identical texts)
3. Don't neglect accessibility
4. Don't hardcode colors (use Tailwind classes)

### Resources
- [Similar Crate Documentation](https://docs.rs/similar/)
- [Yew Documentation](https://yew.rs/)
- [Syntect Documentation](https://docs.rs/syntect/)
- [Diff Algorithms Explained](https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/)

## Conclusion

This plan provides a comprehensive roadmap for implementing a professional-grade DiffView component. Following this plan, a junior developer should be able to create a fully functional, accessible, and performant diff viewer that integrates seamlessly with the WonopUI ecosystem.