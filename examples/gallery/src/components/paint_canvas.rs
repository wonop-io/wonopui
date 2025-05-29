use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(PaintCanvasThemeEditor)]
pub fn paint_canvas_theme_editor() -> Html {
    let fields = vec![
        (
            "paint_canvas_container".to_string(),
            "Canvas Container".to_string(),
        ),
        ("paint_canvas".to_string(), "Canvas".to_string()),
        ("paint_canvas_image".to_string(), "Loaded Image".to_string()),
    ];

    let preview = html! {
        <PaintCanvasDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(PaintCanvasDemo)]
fn paint_canvas_demo() -> Html {
    let image_src = use_state(|| None::<String>);

    let on_load_image = {
        let image_src = image_src.clone();
        Callback::from(move |_| {
            image_src.set(Some("https://example.com/image.png".to_string()));
        })
    };

    html! {
        <div>
            <PaintCanvas image_src={(*image_src).clone()} />
            <button onclick={on_load_image} class="mt-4 px-4 py-2 bg-blue-500 text-white rounded">
                {"Load Image"}
            </button>
        </div>
    }
}

#[function_component(PaintCanvasDocumentation)]
pub fn paint_canvas_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "PaintCanvas Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The PaintCanvas component is a versatile drawing canvas that supports image loading and drawing with mouse events. It is useful for creating interactive web drawing applications." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <PaintCanvasDemo />
                }}
                customize={html! {
                    <PaintCanvasThemeEditor />
                }}
                code={r#"
let image_src = use_state(|| None::<String>);

let on_load_image = {
    let image_src = image_src.clone();
    Callback::from(move |_| {
        image_src.set(Some("https://example.com/image.png".to_string()));
    })
};

html! {
    <div>
        <PaintCanvas image_src={(*image_src).clone()} />
        <button onclick={on_load_image} class="mt-4 px-4 py-2 bg-blue-500 text-white rounded">
            {"Load Image"}
        </button>
    </div>
}
                "#.to_string()}
            />
            <Features features={vec!["PaintCanvas"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>

            <ApiSection
                title="PaintCanvas"
                description="Props for the PaintCanvas component."
                props={vec![
                    ("image_src", "Option<String>", "Optional image source URL to load into the canvas."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The PaintCanvas component supports drawing with mouse events.".to_string(),
                    "If an image source is provided, it will be loaded into the canvas.".to_string(),
                    "The canvas resizes to fit its container when an image is loaded.".to_string(),
                    "Drawing is possible both with and without a loaded image.".to_string(),
                    "The component handles window resizing, adjusting the canvas size accordingly.".to_string(),
                    "For best results, ensure the parent container has a defined width and height.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"PaintCanvas".to_string()}
                class_descriptions={vec![
                    ("paint_canvas_container".to_string(), "For the main container of the paint canvas".to_string()),
                    ("paint_canvas".to_string(), "For the canvas element itself".to_string()),
                    ("paint_canvas_image".to_string(), "For the loaded image within the canvas".to_string()),
                ]}
            />
        </Container>
    }
}
