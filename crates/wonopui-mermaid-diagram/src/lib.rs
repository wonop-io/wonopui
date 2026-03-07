//! Mermaid diagram renderer component for WonopUI.
//!
//! Renders Mermaid.js diagrams from code.

use gloo_utils::document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wonopui_core::*;

/// Default CSS classes for mermaid diagram styling.
pub mod classes {
    /// Container styles.
    pub const CONTAINER: &str = "mermaid-container w-full overflow-auto";

    /// Loading state styles.
    pub const LOADING: &str = "flex items-center justify-center p-4 text-zinc-500 dark:text-zinc-400";

    /// Error state styles.
    pub const ERROR: &str = "p-4 text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-lg";
}

/// Check if Mermaid.js is loaded in the page.
fn is_mermaid_loaded() -> bool {
    let window = web_sys::window().expect("no window");
    window.get("mermaid").is_some()
}

/// Properties for the MermaidDiagram component.
#[derive(Properties, PartialEq)]
pub struct MermaidDiagramProps {
    /// The Mermaid diagram code.
    pub code: String,

    /// Unique ID for this diagram instance.
    #[prop_or_default]
    pub id: Option<String>,

    /// Theme for the diagram (default, dark, forest, neutral).
    #[prop_or("default".to_string())]
    pub theme: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,
}

/// Render state for the diagram.
#[derive(Clone, PartialEq)]
enum RenderState {
    Loading,
    Rendered(String),
    Error(String),
}

/// Renders a Mermaid.js diagram.
///
/// Note: This component requires Mermaid.js to be loaded in the page.
/// Add the following to your HTML:
/// ```html
/// <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
/// ```
///
/// # Example
///
/// ```rust
/// use wonopui_mermaid_diagram::MermaidDiagram;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let code = r#"
///         graph TD
///             A[Start] --> B{Decision}
///             B -->|Yes| C[OK]
///             B -->|No| D[Cancel]
///     "#;
///
///     html! {
///         <MermaidDiagram code={code.to_string()} />
///     }
/// }
/// ```
#[function_component(MermaidDiagram)]
pub fn mermaid_diagram(props: &MermaidDiagramProps) -> Html {
    let state = use_state(|| RenderState::Loading);
    let container_ref = use_node_ref();

    // Generate a unique ID if not provided
    let diagram_id = props.id.clone().unwrap_or_else(|| {
        format!("mermaid-{}", js_sys::Math::random().to_bits())
    });

    // Render the diagram when the code changes
    {
        let state = state.clone();
        let code = props.code.clone();
        let theme = props.theme.clone();
        let container_ref = container_ref.clone();
        let diagram_id = diagram_id.clone();

        use_effect_with((code.clone(), theme.clone()), move |_| {
            if !is_mermaid_loaded() {
                state.set(RenderState::Error(
                    "Mermaid.js is not loaded. Please include it in your HTML.".to_string()
                ));
                return;
            }

            // Render using a pre element that mermaid will process
            if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
                // Create a div with the mermaid class
                let div = document()
                    .create_element("div")
                    .expect("failed to create div");
                div.set_class_name("mermaid");
                div.set_text_content(Some(&code));
                div.set_id(&diagram_id);

                // Clear and append
                container.set_inner_html("");
                container.append_child(&div).expect("failed to append");

                // Trigger mermaid to render
                let window = web_sys::window().expect("no window");
                if let Some(mermaid) = window.get("mermaid") {
                    let init_fn = js_sys::Reflect::get(&mermaid, &JsValue::from_str("init"))
                        .expect("no init");
                    if let Ok(init) = init_fn.dyn_into::<js_sys::Function>() {
                        let _ = init.call1(&mermaid, &div);
                    }
                }

                state.set(RenderState::Rendered(code));
            }
        });
    }

    match &*state {
        RenderState::Loading => html! {
            <div class={classes!(classes::LOADING, props.class.clone())}>
                {"Loading diagram..."}
            </div>
        },
        RenderState::Error(err) => html! {
            <div class={classes!(classes::ERROR, props.class.clone())}>
                { err }
            </div>
        },
        RenderState::Rendered(_) => html! {
            <div
                ref={container_ref}
                class={classes!(classes::CONTAINER, props.class.clone())}
            />
        },
    }
}
