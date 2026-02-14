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
}

/// Stub ComponentEditor - displays children and code in a basic layout
#[function_component(ComponentEditor)]
pub fn component_editor(props: &ComponentEditorProps) -> Html {
    html! {
        <div class={classes!("space-y-4", props.class.clone())}>
            if !props.title.is_empty() {
                <h3 class="text-lg font-semibold">{ &props.title }</h3>
            }
            if !props.description.is_empty() {
                <p class="text-gray-600 dark:text-gray-400">{ &props.description }</p>
            }
            <div class="border rounded-lg p-4 bg-white dark:bg-zinc-800">
                { for props.children.iter() }
            </div>
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
    }

    #[derive(Properties, PartialEq)]
    pub struct CodeEditorProps {
        #[prop_or_default]
        pub value: String,
        #[prop_or_default]
        pub language: String,
        #[prop_or_default]
        pub readonly: bool,
        #[prop_or_default]
        pub line_numbers: bool,
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
    }

    /// Stub CodeEditor - displays code in a pre block
    #[function_component(CodeEditor)]
    pub fn code_editor(props: &CodeEditorProps) -> Html {
        html! {
            <pre class={classes!("bg-gray-900", "text-gray-100", "p-4", "rounded-lg", "overflow-x-auto", "text-sm", "font-mono", props.class.clone())}>
                <code>{ &props.value }</code>
            </pre>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct DiffViewProps {
        #[prop_or_default]
        pub original: String,
        #[prop_or_default]
        pub modified: String,
        #[prop_or_default]
        pub language: String,
        #[prop_or_default]
        pub mode: DiffViewMode,
        #[prop_or_default]
        pub class: Classes,
    }

    /// Stub DiffView - displays both versions side by side
    #[function_component(DiffView)]
    pub fn diff_view(props: &DiffViewProps) -> Html {
        let base_class = "grid grid-cols-2 gap-4";
        let combined_class = if props.class.is_empty() {
            base_class.to_string()
        } else {
            format!("{} {}", base_class, props.class)
        };
        
        html! {
            <div class={combined_class}>
                <div>
                    <div class="text-sm text-gray-500 mb-1">{"Original"}</div>
                    <pre class="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono">
                        <code>{ &props.original }</code>
                    </pre>
                </div>
                <div>
                    <div class="text-sm text-gray-500 mb-1">{"Modified"}</div>
                    <pre class="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono">
                        <code>{ &props.modified }</code>
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
        html! {
            <div class={classes!(
                "flex items-center px-2 py-1.5 text-sm cursor-pointer rounded-sm",
                "text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700",
                if props.inset { "pl-8" } else { "" }
            )}>
                { for props.children.iter() }
                <span class="ml-auto">{"›"}</span>
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ContextMenuSubContentProps {
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(ContextMenuSubContent)]
    pub fn context_menu_sub_content(props: &ContextMenuSubContentProps) -> Html {
        html! {
            <div class="absolute left-full top-0 ml-1 bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-md shadow-lg p-1 min-w-[8rem]">
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
    }

    #[function_component(ContextMenuCheckboxItem)]
    pub fn context_menu_checkbox_item(props: &ContextMenuCheckboxItemProps) -> Html {
        let onclick = {
            let onchange = props.onchange.clone();
            let checked = props.checked;
            Callback::from(move |_: MouseEvent| {
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
    }

    #[function_component(ContextMenuRadioItem)]
    pub fn context_menu_radio_item(props: &ContextMenuRadioItemProps) -> Html {
        html! {
            <div class="flex items-center px-2 py-1.5 text-sm cursor-pointer rounded-sm text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700">
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
        html! {
            <h5 class={classes!("font-medium mb-1", props.class.clone())}>
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
        html! {
            <div class={classes!("text-sm", props.class.clone())}>
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

pub trait BlockTrait {
    fn block_type(&self) -> &'static str;
    fn render(&self) -> Html;
}
