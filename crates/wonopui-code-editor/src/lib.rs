//! Code Editor component for WonopUI
//!
//! A feature-rich code editor component with:
//! - Syntax highlighting (with optional `syntax-highlighting` feature for full syntect support)
//! - Line numbers
//! - Diff indicators
//! - Code annotations (errors, warnings, info, success)
//! - Type hints
//! - Tab handling
//! - Read-only mode
//!
//! # Example
//!
//! ```rust,ignore
//! use wonopui_code_editor::{CodeEditor, CodeEditorProps, Diff, Annotation};
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let code = use_state(|| "fn main() {\n    println!(\"Hello, world!\");\n}".to_string());
//!
//!     let on_change = {
//!         let code = code.clone();
//!         Callback::from(move |new_code: String| {
//!             code.set(new_code);
//!         })
//!     };
//!
//!     html! {
//!         <CodeEditor
//!             code={(*code).clone()}
//!             language="rust"
//!             on_change={on_change}
//!         />
//!     }
//! }
//! ```
//!
//! # Features
//!
//! - `syntax-highlighting`: Enable full syntax highlighting via syntect library

pub mod annotation;
pub mod diff;
pub mod diff_types;
pub mod editor;
pub mod styles;
pub mod syntax_highlighter;
pub mod type_hint;

// Re-export main types
pub use annotation::{Annotation, AnnotationType};
pub use diff::{Diff, DiffType};
pub use diff_types::{
    ChangeType, DiffHunk, DiffLine, DiffLineInfo, DiffSide, DiffViewMode, WordChange,
};
pub use editor::{CodeEditor, CodeEditorMsg, CodeEditorProps};
pub use styles::CodeEditorStyles;
pub use syntax_highlighter::*;
pub use type_hint::TypeHint;
