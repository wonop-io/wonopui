use wonopui::code_editor::{DiffView, DiffViewMode};
use wonopui::prelude::*;
use yew::prelude::*;
use web_sys::{HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
use wasm_bindgen::JsCast;

#[function_component(DiffViewPlayground)]
pub fn diffview_playground() -> Html {
    // State for the text inputs
    let old_text = use_state(|| r#"function greet(name) {
    console.log("Hello, " + name);
    return "Hello, " + name;
}"#.to_string());
    
    let new_text = use_state(|| r#"function greet(name, title = '') {
    const greeting = title ? `Hello, ${title} ${name}` : `Hello, ${name}`;
    console.log(greeting);
    return greeting;
}"#.to_string());
    
    // Control states
    let mode = use_state(|| DiffViewMode::SideBySide);
    let language = use_state(|| "javascript".to_string());
    let theme = use_state(|| "auto".to_string());
    let show_line_numbers = use_state(|| true);
    let unified_diff = use_state(|| false);
    let context_lines = use_state(|| 3usize);
    let font_size = use_state(|| 14u8);
    let line_height = use_state(|| 1.5f32);
    
    // Preset examples
    let load_preset = {
        let old_text = old_text.clone();
        let new_text = new_text.clone();
        let language = language.clone();
        
        Callback::from(move |preset: &str| {
            match preset {
                "refactor" => {
                    old_text.set(r#"def calculate_price(items):
    total = 0
    for item in items:
        total = total + item.price * item.quantity
    return total"#.to_string());
                    new_text.set(r#"def calculate_price(items):
    return sum(item.price * item.quantity for item in items)"#.to_string());
                    language.set("python".to_string());
                }
                "bugfix" => {
                    old_text.set(r#"public String processName(String name) {
    return name.toUpperCase();
}"#.to_string());
                    new_text.set(r#"public String processName(String name) {
    if (name == null) {
        return "";
    }
    return name.trim().toUpperCase();
}"#.to_string());
                    language.set("java".to_string());
                }
                "feature" => {
                    old_text.set(r#"interface User {
    id: number;
    name: string;
}"#.to_string());
                    new_text.set(r#"interface User {
    id: number;
    name: string;
    email: string;
    createdAt: Date;
    lastLogin?: Date;
    roles: string[];
}"#.to_string());
                    language.set("typescript".to_string());
                }
                _ => {}
            }
        })
    };
    
    // Input handlers
    let on_old_text_change = {
        let old_text = old_text.clone();
        Callback::from(move |e: Event| {
            if let Some(textarea) = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()) {
                old_text.set(textarea.value());
            }
        })
    };
    
    let on_new_text_change = {
        let new_text = new_text.clone();
        Callback::from(move |e: Event| {
            if let Some(textarea) = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()) {
                new_text.set(textarea.value());
            }
        })
    };
    
    html! {
        <div class="space-y-6">
            <div class="bg-blue-50 dark:bg-blue-950 p-4 rounded-lg border border-blue-200 dark:border-blue-800">
                <h3 class="text-lg font-bold text-blue-900 dark:text-blue-100 mb-2">
                    {"🎮 Interactive Playground"}
                </h3>
                <p class="text-blue-800 dark:text-blue-200">
                    {"Edit the text below or load a preset example to see the diff visualization in real-time."}
                </p>
            </div>
            
            // Preset buttons
            <div class="flex flex-wrap gap-2">
                <span class="text-sm font-medium text-gray-600 dark:text-gray-400 self-center">
                    {"Load Preset:"}
                </span>
                <button
                    class="px-3 py-1 bg-purple-500 text-white rounded hover:bg-purple-600 transition-colors text-sm"
                    onclick={
                        let load_preset = load_preset.clone();
                        Callback::from(move |_| load_preset.emit("refactor"))
                    }
                >
                    {"Python Refactor"}
                </button>
                <button
                    class="px-3 py-1 bg-orange-500 text-white rounded hover:bg-orange-600 transition-colors text-sm"
                    onclick={
                        let load_preset = load_preset.clone();
                        Callback::from(move |_| load_preset.emit("bugfix"))
                    }
                >
                    {"Java Bug Fix"}
                </button>
                <button
                    class="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600 transition-colors text-sm"
                    onclick={
                        let load_preset = load_preset.clone();
                        Callback::from(move |_| load_preset.emit("feature"))
                    }
                >
                    {"TypeScript Feature"}
                </button>
            </div>
            
            // Text input areas
            <div class="grid md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        {"Original Text"}
                    </label>
                    <textarea
                        class="w-full h-40 p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 font-mono text-sm"
                        value={(*old_text).clone()}
                        onchange={on_old_text_change}
                        placeholder="Enter original text..."
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        {"Modified Text"}
                    </label>
                    <textarea
                        class="w-full h-40 p-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 font-mono text-sm"
                        value={(*new_text).clone()}
                        onchange={on_new_text_change}
                        placeholder="Enter modified text..."
                    />
                </div>
            </div>
            
            // Controls panel
            <div class="bg-gray-100 dark:bg-gray-800 p-4 rounded-lg space-y-4">
                <h4 class="font-semibold text-gray-700 dark:text-gray-300">{"Display Options"}</h4>
                
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                    // View mode
                    <div>
                        <label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                            {"View Mode"}
                        </label>
                        <select
                            class="w-full px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700"
                            onchange={
                                let mode = mode.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(select) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                                        mode.set(match select.value().as_str() {
                                            "inline" => DiffViewMode::Inline,
                                            _ => DiffViewMode::SideBySide,
                                        });
                                    }
                                })
                            }
                        >
                            <option value="side-by-side">{"Side by Side"}</option>
                            <option value="inline">{"Inline"}</option>
                        </select>
                    </div>
                    
                    // Language
                    <div>
                        <label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                            {"Language"}
                        </label>
                        <select
                            class="w-full px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700"
                            value={(*language).clone()}
                            onchange={
                                let language = language.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(select) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                                        language.set(select.value());
                                    }
                                })
                            }
                        >
                            <option value="rust">{"Rust"}</option>
                            <option value="javascript">{"JavaScript"}</option>
                            <option value="typescript">{"TypeScript"}</option>
                            <option value="python">{"Python"}</option>
                            <option value="java">{"Java"}</option>
                            <option value="go">{"Go"}</option>
                            <option value="css">{"CSS"}</option>
                            <option value="html">{"HTML"}</option>
                            <option value="json">{"JSON"}</option>
                            <option value="yaml">{"YAML"}</option>
                            <option value="sql">{"SQL"}</option>
                            <option value="plaintext">{"Plain Text"}</option>
                        </select>
                    </div>
                    
                    // Theme
                    <div>
                        <label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                            {"Theme"}
                        </label>
                        <select
                            class="w-full px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700"
                            value={(*theme).clone()}
                            onchange={
                                let theme = theme.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(select) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                                        theme.set(select.value());
                                    }
                                })
                            }
                        >
                            <option value="auto">{"Auto"}</option>
                            <option value="light">{"Light"}</option>
                            <option value="dark">{"Dark"}</option>
                        </select>
                    </div>
                    
                    // Context lines
                    <div>
                        <label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                            {"Context Lines"}
                        </label>
                        <input
                            type="number"
                            min="0"
                            max="10"
                            class="w-full px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700"
                            value={context_lines.to_string()}
                            onchange={
                                let context_lines = context_lines.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                        if let Ok(val) = input.value().parse::<usize>() {
                                            context_lines.set(val.min(10));
                                        }
                                    }
                                })
                            }
                        />
                    </div>
                    
                    // Font size
                    <div>
                        <label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                            {format!("Font Size: {}px", *font_size)}
                        </label>
                        <input
                            type="range"
                            min="10"
                            max="20"
                            class="w-full"
                            value={font_size.to_string()}
                            onchange={
                                let font_size = font_size.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                        if let Ok(val) = input.value().parse::<u8>() {
                                            font_size.set(val);
                                        }
                                    }
                                })
                            }
                        />
                    </div>
                    
                    // Line height
                    <div>
                        <label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
                            {format!("Line Height: {:.1}", *line_height)}
                        </label>
                        <input
                            type="range"
                            min="10"
                            max="20"
                            step="1"
                            class="w-full"
                            value={((*line_height * 10.0) as i32).to_string()}
                            onchange={
                                let line_height = line_height.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                        if let Ok(val) = input.value().parse::<i32>() {
                                            line_height.set(val as f32 / 10.0);
                                        }
                                    }
                                })
                            }
                        />
                    </div>
                </div>
                
                // Toggle options
                <div class="flex flex-wrap gap-4">
                    <label class="flex items-center gap-2">
                        <input
                            type="checkbox"
                            checked={*show_line_numbers}
                            onchange={
                                let show_line_numbers = show_line_numbers.clone();
                                Callback::from(move |_| {
                                    show_line_numbers.set(!*show_line_numbers);
                                })
                            }
                        />
                        <span class="text-sm text-gray-700 dark:text-gray-300">{"Show Line Numbers"}</span>
                    </label>
                    
                    <label class="flex items-center gap-2">
                        <input
                            type="checkbox"
                            checked={*unified_diff}
                            onchange={
                                let unified_diff = unified_diff.clone();
                                Callback::from(move |_| {
                                    unified_diff.set(!*unified_diff);
                                })
                            }
                        />
                        <span class="text-sm text-gray-700 dark:text-gray-300">{"Show Unified Diff Headers"}</span>
                    </label>
                </div>
            </div>
            
            // Live preview
            <div>
                <h4 class="font-semibold text-gray-700 dark:text-gray-300 mb-2">{"Preview"}</h4>
                <div class="border-2 border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
                    <DiffView
                        old_text={(*old_text).clone()}
                        new_text={(*new_text).clone()}
                        mode={(*mode).clone()}
                        language={(*language).clone()}
                        theme={(*theme).clone()}
                        show_line_numbers={*show_line_numbers}
                        unified_diff={*unified_diff}
                        context_lines={*context_lines}
                        font_size={*font_size}
                        line_height={*line_height}
                        class="shadow-inner"
                    />
                </div>
            </div>
        </div>
    }
}