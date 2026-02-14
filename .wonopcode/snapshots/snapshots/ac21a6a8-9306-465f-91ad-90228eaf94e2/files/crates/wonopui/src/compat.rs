//! Compatibility module for components not yet fully migrated
//!
//! This module provides stub implementations that allow code to compile
//! while the full migration is in progress.

use yew::prelude::*;

// ============================================================================
// ThemeProvider - stub implementation
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

/// Stub ThemeProvider - currently just passes through children
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    html! {
        { for props.children.iter() }
    }
}

// ============================================================================
// BrandGuide types - stubs for backward compatibility
// ============================================================================

/// Stub type for BrandGuideType
pub struct BrandGuideType;

/// Alias for backward compatibility
pub type BrandGuide = BrandGuideType;

/// Stub for ClassesStr
pub type ClassesStr = &'static str;

/// Stub BRANDGUIDE constant
pub static BRANDGUIDE: BrandGuideType = BrandGuideType;

// ============================================================================
// ComponentEditor - stub implementation
// ============================================================================

/// Field definition for ComponentEditor
#[derive(Clone, PartialEq)]
pub struct ComponentField {
    pub name: String,
    pub value: Html,
}

#[derive(Properties, PartialEq)]
pub struct ComponentEditorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub description: String,
    #[prop_or_default]
    pub code: String,
    /// Preview content to show
    #[prop_or_default]
    pub preview: Html,
    /// Fields to display (label/value pairs) - accepts Html values
    #[prop_or_default]
    pub fields: Vec<(String, String)>,
}

/// Stub ComponentEditor - displays preview and fields
#[function_component(ComponentEditor)]
pub fn component_editor(props: &ComponentEditorProps) -> Html {
    let class = format!("space-y-4 {}", props.class.to_string());
    
    html! {
        <div class={class}>
            if !props.title.is_empty() {
                <h3 class="text-lg font-semibold">{ &props.title }</h3>
            }
            if !props.description.is_empty() {
                <p class="text-gray-600 dark:text-gray-400">{ &props.description }</p>
            }
            // Preview section
            <div class="border rounded-lg p-4 bg-white dark:bg-zinc-800">
                { props.preview.clone() }
                { for props.children.iter() }
            </div>
            // Fields section
            if !props.fields.is_empty() {
                <div class="grid gap-4">
                    { for props.fields.iter().map(|(label, value)| html! {
                        <div class="flex items-center gap-4">
                            <label class="font-medium text-sm min-w-32">{ label }</label>
                            <div>{ value }</div>
                        </div>
                    })}
                </div>
            }
            if !props.code.is_empty() {
                <pre class="bg-gray-100 dark:bg-zinc-900 p-4 rounded-lg overflow-x-auto text-sm">
                    <code>{ &props.code }</code>
                </pre>
            }
        </div>
    }
}

// ============================================================================
// CodeEditor types - stubs
// ============================================================================

pub mod code_editor {
    use yew::prelude::*;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Diff {
        pub line: usize,
        pub diff_type: DiffType,
        pub message: Option<String>,
    }

    impl Diff {
        pub fn added(line: usize) -> Self {
            Self { line, diff_type: DiffType::Added, message: None }
        }

        pub fn removed(line: usize) -> Self {
            Self { line, diff_type: DiffType::Removed, message: None }
        }

        pub fn modified(line: usize) -> Self {
            Self { line, diff_type: DiffType::Modified, message: None }
        }

        pub fn with_message(mut self, message: &str) -> Self {
            self.message = Some(message.to_string());
            self
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum DiffType {
        Added,
        Removed,
        Modified,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Annotation {
        pub line: usize,
        pub message: String,
        pub annotation_type: AnnotationType,
        pub column_range: Option<(usize, usize)>,
        pub is_inline: bool,
    }

    impl Annotation {
        pub fn new(line: usize, message: &str, annotation_type: AnnotationType) -> Self {
            Self {
                line,
                message: message.to_string(),
                annotation_type,
                column_range: None,
                is_inline: false,
            }
        }

        pub fn error(line: usize, message: &str) -> Self {
            Self::new(line, message, AnnotationType::Error)
        }

        pub fn warning(line: usize, message: &str) -> Self {
            Self::new(line, message, AnnotationType::Warning)
        }

        pub fn info(line: usize, message: &str) -> Self {
            Self::new(line, message, AnnotationType::Info)
        }

        pub fn success(line: usize, message: &str) -> Self {
            Self::new(line, message, AnnotationType::Success)
        }

        pub fn inline(mut self) -> Self {
            self.is_inline = true;
            self
        }

        pub fn with_column_range(mut self, start: usize, end: usize) -> Self {
            self.column_range = Some((start, end));
            self
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum AnnotationType {
        Error,
        Warning,
        Info,
        Success,
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct TypeHint {
        pub line: usize,
        pub hint: String,
        pub column: Option<usize>,
    }

    impl TypeHint {
        pub fn new(line: usize, hint: &str) -> Self {
            Self {
                line,
                hint: hint.to_string(),
                column: None,
            }
        }

        pub fn at_column(mut self, column: usize) -> Self {
            self.column = Some(column);
            self
        }
    }

    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    pub enum DiffViewMode {
        #[default]
        Split,
        Unified,
        /// Alias for Split (side by side view)
        SideBySide,
        /// Alias for Unified (inline view)
        Inline,
    }

    #[derive(Properties, PartialEq)]
    pub struct CodeEditorProps {
        #[prop_or_default]
        pub value: String,
        /// Alias for value
        #[prop_or_default]
        pub code: String,
        #[prop_or_default]
        pub language: String,
        #[prop_or_default]
        pub readonly: bool,
        #[prop_or_default]
        pub line_numbers: bool,
        #[prop_or_default]
        pub show_line_numbers: bool,
        #[prop_or_default]
        pub diffs: Vec<Diff>,
        #[prop_or_default]
        pub annotations: Vec<Annotation>,
        #[prop_or_default]
        pub type_hints: Vec<TypeHint>,
        #[prop_or_default]
        pub on_change: Callback<String>,
        #[prop_or_default]
        pub class: Classes,
        #[prop_or_default]
        pub font_size: Option<String>,
        #[prop_or_default]
        pub line_height: Option<String>,
        #[prop_or_default]
        pub theme: Option<String>,
    }

    /// Stub CodeEditor - displays code in a pre block
    #[function_component(CodeEditor)]
    pub fn code_editor(props: &CodeEditorProps) -> Html {
        let base_class = "bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono";
        let combined_class = if props.class.is_empty() {
            base_class.to_string()
        } else {
            format!("{} {}", base_class, props.class.to_string())
        };
        
        // Use code as alias for value
        let content = if !props.code.is_empty() { &props.code } else { &props.value };
        
        html! {
            <pre class={combined_class}>
                <code>{ content }</code>
            </pre>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct DiffViewProps {
        #[prop_or_default]
        pub original: String,
        #[prop_or_default]
        pub modified: String,
        /// Old text - alias for original
        #[prop_or_default]
        pub old_text: String,
        /// New text - alias for modified
        #[prop_or_default]
        pub new_text: String,
        #[prop_or_default]
        pub language: String,
        #[prop_or_default]
        pub mode: DiffViewMode,
        #[prop_or_default]
        pub class: Classes,
        #[prop_or_default]
        pub show_line_numbers: bool,
        #[prop_or_default]
        pub context_lines: usize,
        #[prop_or_default]
        pub font_size: Option<String>,
        #[prop_or_default]
        pub line_height: Option<String>,
        #[prop_or_default]
        pub theme: Option<String>,
        #[prop_or_default]
        pub unified_diff: bool,
    }

    /// Stub DiffView - displays both versions side by side
    #[function_component(DiffView)]
    pub fn diff_view(props: &DiffViewProps) -> Html {
        let base_class = "grid grid-cols-2 gap-4";
        let combined_class = if props.class.is_empty() {
            base_class.to_string()
        } else {
            format!("{} {}", base_class, props.class.to_string())
        };
        
        // Use old_text/new_text if provided, otherwise fall back to original/modified
        let old = if !props.old_text.is_empty() { &props.old_text } else { &props.original };
        let new = if !props.new_text.is_empty() { &props.new_text } else { &props.modified };
        
        html! {
            <div class={combined_class}>
                <div>
                    <div class="text-sm text-gray-500 mb-1">{"Original"}</div>
                    <pre class="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono">
                        <code>{ old }</code>
                    </pre>
                </div>
                <div>
                    <div class="text-sm text-gray-500 mb-1">{"Modified"}</div>
                    <pre class="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono">
                        <code>{ new }</code>
                    </pre>
                </div>
            </div>
        }
    }
}

// ============================================================================
// Context Menu additional components - stubs
// ============================================================================

pub mod context_menu_extras {
    use yew::prelude::*;

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuSubProps {
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(ContextMenuSub)]
    pub fn context_menu_sub(props: &ContextMenuSubProps) -> Html {
        html! {
            <div class="relative">
                { for props.children.iter() }
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuSubTriggerProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or(false)]
        pub inset: bool,
    }

    #[function_component(ContextMenuSubTrigger)]
    pub fn context_menu_sub_trigger(props: &ContextMenuSubTriggerProps) -> Html {
        let class = format!(
            "flex items-center px-2 py-1.5 text-sm cursor-pointer rounded-sm text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700{}",
            if props.inset { " pl-8" } else { "" }
        );
        
        html! {
            <div class={class}>
                { for props.children.iter() }
                <span class="ml-auto">{"›"}</span>
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuSubContentProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub class: Classes,
    }

    #[function_component(ContextMenuSubContent)]
    pub fn context_menu_sub_content(props: &ContextMenuSubContentProps) -> Html {
        let class = format!("absolute left-full top-0 ml-1 bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-md shadow-lg p-1 min-w-[8rem] {}", props.class.to_string());
        html! {
            <div class={class}>
                { for props.children.iter() }
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuCheckboxItemProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or(false)]
        pub checked: bool,
        #[prop_or_default]
        pub onchange: Callback<bool>,
        #[prop_or_default]
        pub onclick: Callback<MouseEvent>,
    }

    #[function_component(ContextMenuCheckboxItem)]
    pub fn context_menu_checkbox_item(props: &ContextMenuCheckboxItemProps) -> Html {
        let onclick = {
            let onchange = props.onchange.clone();
            let onclick = props.onclick.clone();
            let checked = props.checked;
            Callback::from(move |e: MouseEvent| {
                onclick.emit(e);
                onchange.emit(!checked);
            })
        };

        html! {
            <div 
                class="flex items-center px-2 py-1.5 text-sm cursor-pointer rounded-sm text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700"
                {onclick}
            >
                <span class="w-4 h-4 mr-2 flex items-center justify-center">
                    if props.checked {
                        {"✓"}
                    }
                </span>
                { for props.children.iter() }
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuRadioGroupProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub value: String,
        #[prop_or_default]
        pub onchange: Callback<String>,
    }

    #[function_component(ContextMenuRadioGroup)]
    pub fn context_menu_radio_group(props: &ContextMenuRadioGroupProps) -> Html {
        html! {
            <div role="radiogroup">
                { for props.children.iter() }
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuRadioItemProps {
        #[prop_or_default]
        pub children: Children,
        pub value: String,
        #[prop_or(false)]
        pub checked: bool,
        #[prop_or_default]
        pub onclick: Callback<MouseEvent>,
    }

    #[function_component(ContextMenuRadioItem)]
    pub fn context_menu_radio_item(props: &ContextMenuRadioItemProps) -> Html {
        let onclick = props.onclick.clone();
        html! {
            <div 
                class="flex items-center px-2 py-1.5 text-sm cursor-pointer rounded-sm text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700"
                onclick={onclick}
            >
                <span class="w-4 h-4 mr-2 flex items-center justify-center">
                    if props.checked {
                        {"●"}
                    } else {
                        {"○"}
                    }
                </span>
                { for props.children.iter() }
            </div>
        }
    }
}

// ============================================================================
// Alert extras - stubs
// ============================================================================

pub mod alert_extras {
    use yew::prelude::*;

    #[derive(Properties, PartialEq)]
    pub struct AlertTitleProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub class: Classes,
    }

    #[function_component(AlertTitle)]
    pub fn alert_title(props: &AlertTitleProps) -> Html {
        let class = format!("font-medium mb-1 {}", props.class.to_string());
        html! {
            <h5 class={class}>
                { for props.children.iter() }
            </h5>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct AlertDescriptionProps {
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub class: Classes,
    }

    #[function_component(AlertDescription)]
    pub fn alert_description(props: &AlertDescriptionProps) -> Html {
        let class = format!("text-sm {}", props.class.to_string());
        html! {
            <div class={class}>
                { for props.children.iter() }
            </div>
        }
    }
}

// ============================================================================
// ContentEditableWithCommands - stub
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct ContentEditableWithCommandsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ContentEditableWithCommands)]
pub fn content_editable_with_commands(props: &ContentEditableWithCommandsProps) -> Html {
    html! {
        <div class={props.class.clone()} contenteditable="true">
            { for props.children.iter() }
        </div>
    }
}

// ============================================================================
// BlockTrait - stub trait
// ============================================================================

use yew::Html;

/// BlockTrait for the markdown editor blocks
pub trait BlockTrait: Clone + PartialEq + Sized {
    /// Create a new empty block
    fn new_block() -> Self;
    
    /// Convert the block to markdown
    fn to_markdown(&self) -> String;
    
    /// Get the icon for this block type
    fn icon(&self) -> Html;
    
    /// Get the human-readable name of this block type
    fn name(&self) -> String;
    
    /// Get command triggers that activate the block menu
    fn command_triggers() -> Vec<String>;
    
    /// Search for block types matching a query
    fn search(query: Option<String>) -> Vec<Self>;
    
    /// Check if this block can be deleted (e.g., empty)
    fn can_delete(&self) -> bool;
    
    /// Get the block type identifier
    fn block_type(&self) -> &'static str {
        "unknown"
    }
    
    /// Render the block
    fn render(&self) -> Html {
        yew::html! {}
    }
}
