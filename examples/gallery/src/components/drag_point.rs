use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::events::PointerEvent;
use yew::prelude::*;

#[function_component(DragPointThemeEditor)]
pub fn drag_point_theme_editor() -> Html {
    let fields = vec![("drag_point".to_string(), "Drag Point".to_string())];

    let preview = html! {
        <DragPoint
            class={classes!("w-8", "h-8", "bg-blue-500", "rounded-full", "cursor-move")}
            onstart={Callback::from(|_| log::info!("Drag started"))}
            onstop={Callback::from(|_| log::info!("Drag stopped"))}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(DragPointDocumentation)]
pub fn drag_point_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "DragPoint Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The DragPoint component allows users to create a draggable element. It provides callbacks for when dragging starts and stops, enabling custom behavior during these events." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <DragPoint
                        class={classes!("w-8", "h-8", "bg-blue-500", "rounded-full", "cursor-move")}
                        onstart={Callback::from(|e: PointerEvent| {
                            log::info!("Drag started at: ({}, {})", e.client_x(), e.client_y());
                        })}
                        onstop={Callback::from(|_| {
                            log::info!("Drag stopped");
                        })}
                    />
                }}
                customize={html! {
                    <DragPointThemeEditor />
                }}
                code={r#"
<DragPoint
    class={classes!("w-8", "h-8", "bg-blue-500", "rounded-full", "cursor-move")}
    onstart={Callback::from(|e: PointerEvent| {
        log::info!("Drag started at: ({}, {})", e.client_x(), e.client_y());
    })}
    onstop={Callback::from(|_| {
        log::info!("Drag stopped");
    })}
/>"#.to_string()}
            />
            <Features features={vec!["Draggable", "Customizable", "Event Callbacks"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="DragPoint"
                description="Properties for the DragPoint component."
                props={vec![
                    ("onstart", "Callback<PointerEvent>", "Callback executed when dragging starts."),
                    ("onstop", "Callback<()>", "Callback executed when dragging stops."),
                    ("class", "Classes", "CSS classes for styling the draggable element."),
                    ("tag", "String", "HTML tag to use for the component's root element. Default is 'div'."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The DragPoint component uses the Pointer Events API, which provides better performance and cross-device compatibility compared to mouse events.".to_string(),
                    "The component handles pointer capture internally, ensuring that the drag operation continues even if the pointer leaves the element.".to_string(),
                    "The `onstart` callback provides a PointerEvent, allowing access to detailed information about the drag start, such as coordinates.".to_string(),
                    "The `onstop` callback is triggered when the drag operation ends, either by releasing the pointer or by the pointer leaving the window.".to_string(),
                    "For accessibility, consider adding appropriate ARIA attributes and ensuring keyboard navigation alternatives where applicable.".to_string(),
                    "The component uses Rust's `wasm_bindgen` to interact with the browser's JavaScript API, enabling efficient pointer event handling.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"DragPoint".to_string()}
                class_descriptions={vec![
                    ("drag_point".to_string(), "Applied to the root element of the DragPoint component. Use this to style the draggable element.".to_string()),
                ]}
            />
        </Container>
    }
}
