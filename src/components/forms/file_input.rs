use gloo::file::File as GlooFile;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::prelude::*;

/// File attachment input component with drag-and-drop support
#[derive(Properties, PartialEq, Clone)]
pub struct FileInputProps {
    /// Callback when files are selected or dropped
    pub on_files: Callback<Vec<web_sys::File>>,

    /// Custom CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Whether multiple files can be selected
    #[prop_or(false)]
    pub multiple: bool,

    /// Accepted file types (e.g., "image/*", ".pdf,.doc")
    #[prop_or_default]
    pub accept: Option<String>,

    /// Maximum file size in bytes (0 for unlimited)
    #[prop_or(0)]
    pub max_size: u64,

    /// Label text
    #[prop_or_else(|| "Choose files or drag and drop".to_string())]
    pub label: String,

    /// Theme name (light or dark)
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Show file preview for images
    #[prop_or(true)]
    pub show_preview: bool,
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let file_input_ref = use_node_ref();
    let is_drag_over = use_state(|| false);
    let selected_files = use_state(|| Vec::<String>::new());
    let error_message = use_state(|| Option::<String>::None);

    let theme_class = if props.theme == "dark" {
        "dark"
    } else {
        "light"
    };

    // Handle file selection from input
    let on_change = {
        let file_input_ref = file_input_ref.clone();
        let on_files = props.on_files.clone();
        let selected_files = selected_files.clone();
        let error_message = error_message.clone();
        let max_size = props.max_size;

        Callback::from(move |_e: Event| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                if let Some(files) = input.files() {
                    let mut file_list = Vec::new();
                    let mut names = Vec::new();
                    let mut error = None;

                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            // Check file size
                            if max_size > 0 && file.size() as u64 > max_size {
                                error = Some(format!(
                                    "File {} exceeds maximum size of {} bytes",
                                    file.name(),
                                    max_size
                                ));
                                break;
                            }
                            names.push(file.name());
                            file_list.push(file);
                        }
                    }

                    if let Some(err) = error {
                        error_message.set(Some(err));
                    } else {
                        error_message.set(None);
                        selected_files.set(names);
                        on_files.emit(file_list);
                    }
                }
            }
        })
    };

    // Handle drag over
    let on_drag_over = {
        let is_drag_over = is_drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_drag_over.set(true);
        })
    };

    // Handle drag leave
    let on_drag_leave = {
        let is_drag_over = is_drag_over.clone();
        Callback::from(move |_e: DragEvent| {
            is_drag_over.set(false);
        })
    };

    // Handle file drop
    let on_drop = {
        let is_drag_over = is_drag_over.clone();
        let on_files = props.on_files.clone();
        let selected_files = selected_files.clone();
        let error_message = error_message.clone();
        let max_size = props.max_size;

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_drag_over.set(false);

            if let Some(data_transfer) = e.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    let mut file_list = Vec::new();
                    let mut names = Vec::new();
                    let mut error = None;

                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            // Check file size
                            if max_size > 0 && file.size() as u64 > max_size {
                                error = Some(format!(
                                    "File {} exceeds maximum size of {} bytes",
                                    file.name(),
                                    max_size
                                ));
                                break;
                            }
                            names.push(file.name());
                            file_list.push(file);
                        }
                    }

                    if let Some(err) = error {
                        error_message.set(Some(err));
                    } else {
                        error_message.set(None);
                        selected_files.set(names);
                        on_files.emit(file_list);
                    }
                }
            }
        })
    };

    // Handle click on drop zone
    let on_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    html! {
        <div
            class={classes!(
                props.class.clone(),
                "file-input-container",
                theme_class
            )}
        >
            <div
                class={classes!(
                    "file-input-dropzone",
                    "border-2",
                    "border-dashed",
                    "rounded-lg",
                    "p-6",
                    "text-center",
                    "cursor-pointer",
                    "transition-colors",
                    if *is_drag_over {
                        "border-blue-500 bg-blue-50 dark:bg-blue-900"
                    } else {
                        "border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 hover:border-gray-400 dark:hover:border-gray-600"
                    },
                    if props.disabled {
                        "opacity-50 cursor-not-allowed"
                    } else {
                        ""
                    }
                )}
                ondragover={on_drag_over}
                ondragleave={on_drag_leave}
                ondrop={on_drop}
                onclick={if !props.disabled { Some(on_click) } else { None }}
            >
                <input
                    ref={file_input_ref.clone()}
                    type="file"
                    class="hidden"
                    multiple={props.multiple}
                    accept={props.accept.clone()}
                    disabled={props.disabled}
                    onchange={on_change}
                />

                <svg
                    class="w-12 h-12 mx-auto mb-4 text-gray-400 dark:text-gray-600"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                    />
                </svg>

                <p class="mb-2 text-sm font-semibold text-gray-700 dark:text-gray-300">
                    { &props.label }
                </p>

                <p class="text-xs text-gray-500 dark:text-gray-400">
                    if props.max_size > 0 {
                        { format!("Maximum file size: {} MB", props.max_size / 1024 / 1024) }
                    } else {
                        { "No size limit" }
                    }
                </p>

                if let Some(accept) = &props.accept {
                    <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                        { format!("Accepted types: {}", accept) }
                    </p>
                }
            </div>

            // Display error message
            if let Some(error) = (*error_message).clone() {
                <div class="mt-2 text-sm text-red-600 dark:text-red-400">
                    { error }
                </div>
            }

            // Display selected files
            if !selected_files.is_empty() {
                <div class="mt-4">
                    <p class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                        { "Selected files:" }
                    </p>
                    <ul class="space-y-1">
                        { for (*selected_files).iter().map(|name| {
                            html! {
                                <li class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
                                    <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                    </svg>
                                    <span>{ name }</span>
                                </li>
                            }
                        })}
                    </ul>
                </div>
            }
        </div>
    }
}
