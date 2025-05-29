use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ResizableThemeEditor)]
pub fn resizable_theme_editor() -> Html {
    let fields = vec![
        ("resizable_container".to_string(), "Container".to_string()),
        ("resizable_box".to_string(), "Box".to_string()),
        (
            "resizable_handle_visible".to_string(),
            "Visible Handle".to_string(),
        ),
        (
            "resizable_handle_hidden".to_string(),
            "Hidden Handle".to_string(),
        ),
        (
            "resizable_handle_nw".to_string(),
            "Northwest Handle".to_string(),
        ),
        (
            "resizable_handle_ne".to_string(),
            "Northeast Handle".to_string(),
        ),
        (
            "resizable_handle_sw".to_string(),
            "Southwest Handle".to_string(),
        ),
        (
            "resizable_handle_se".to_string(),
            "Southeast Handle".to_string(),
        ),
        ("resizable_handle_n".to_string(), "North Handle".to_string()),
        ("resizable_handle_s".to_string(), "South Handle".to_string()),
        ("resizable_handle_w".to_string(), "West Handle".to_string()),
        ("resizable_handle_e".to_string(), "East Handle".to_string()),
    ];

    let preview = html! {
        <ResizableDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ResizableDemo)]
fn resizable_demo() -> Html {
    let coordinates = use_state(|| (100., 100., 400., 400.));
    let on_coordinates_change = {
        let coordinates = coordinates.clone();
        Callback::from(move |new_coords: (f64, f64, f64, f64)| {
            coordinates.set(new_coords);
        })
    };

    html! {
        <div class="flex flex-col space-y-4">
            <div class="container relative" style="height: 300px;">
                <Resizable
                    coordinates={*coordinates}
                    north=true
                    east=true
                    south=true
                    west=true
                    north_east=true
                    south_east=true
                    south_west=true
                    north_west=true
                    on_coordinates_change={on_coordinates_change.clone()}
                >
                    <div class="border-2 border-blue-500 border-dashed absolute bg-gray-500 p-4">
                        { "Resizable Content" }
                    </div>
                </Resizable>
            </div>

            <div class="container relative" style="height: 200px;">
                <Resizable
                    coordinates={(50., 50., 200., 200.)}
                    north=false
                    east=true
                    south=true
                    west=false
                    on_coordinates_change={Callback::from(|_| {})}
                >
                    <div class="border-2 border-blue-500 border-dashed absolute bg-gray-500 p-2">
                        { "Limited Resize" }
                    </div>
                </Resizable>
            </div>
        </div>
    }
}

#[function_component(ResizableDocumentation)]
pub fn resizable_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Resizable Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Resizable component allows you to create resizable areas within your application. Users can resize these areas using various handles positioned around the element's edges and corners." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <ResizableDemo />
                }}
                customize={html! {
                    <ResizableThemeEditor />
                }}
                code={r#"
// State to manage coordinates
let coordinates = use_state(|| (100., 100., 400., 400.));
let on_coordinates_change = {
    let coordinates = coordinates.clone();
    Callback::from(move |new_coords: (f64, f64, f64, f64)| {
        coordinates.set(new_coords);
    })
};

html! {
    <div class="flex flex-col space-y-4">
        <div class="container relative" style="height: 300px;">
            <Resizable
                coordinates={*coordinates}
                north=true
                east=true
                south=true
                west=true
                north_east=true
                south_east=true
                south_west=true
                north_west=true
                on_coordinates_change={on_coordinates_change.clone()}
            >
                <div class="border-2 border-blue-500 border-dashed absolute bg-gray-500 p-4">
                    { "Resizable Content" }
                </div>
            </Resizable>
        </div>

        <div class="container relative" style="height: 200px;">
            <Resizable
                coordinates={(50., 50., 200., 200.)}
                north=false
                east=true
                south=true
                west=false
                on_coordinates_change={Callback::from(|_| {})}
            >
                <div class="border-2 border-blue-500 border-dashed absolute bg-gray-500 p-2">
                    { "Limited Resize" }
                </div>
            </Resizable>
        </div>
    </div>
}"#.to_string()}
            />
            <Features features={vec!["Resizable"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Resizable"
                description="Props for the Resizable component."
                props={vec![
                    ("coordinates", "(f64, f64, f64, f64)", "Initial coordinates of the resizable area: (start_x, start_y, end_x, end_y)."),
                    ("on_coordinates_change", "Callback<(f64, f64, f64, f64)>", "Callback function that is called when the coordinates change."),
                    ("children", "Children", "The child elements to be rendered inside the resizable area."),

                    // Directional props
                    ("north", "bool", "Enable the north handle for resizing."),
                    ("north_west", "bool", "Enable the north-west corner handle for resizing."),
                    ("north_east", "bool", "Enable the north-east corner handle for resizing."),
                    ("east", "bool", "Enable the east handle for resizing."),
                    ("south_east", "bool", "Enable the south-east corner handle for resizing."),
                    ("south", "bool", "Enable the south handle for resizing."),
                    ("south_west", "bool", "Enable the south-west corner handle for resizing."),
                    ("west", "bool", "Enable the west handle for resizing."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Resizable component utilizes DragPoint components for the resize handles.".to_string(),
                    "Ensure that your elements inside the resizable area can adapt to the changing size.".to_string(),
                    "Handles can be individually enabled or disabled via the directional boolean props.".to_string(),
                    "The on_coordinates_change callback is triggered with the new coordinates whenever a resize operation completes.".to_string(),
                    "The component supports both mouse and touch interactions for resizing.".to_string(),
                    "For accessibility, consider providing alternative ways to adjust sizes for users who rely on keyboard navigation.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Resizable".to_string()}
                class_descriptions={vec![
                    ("resizable_container".to_string(), "For the main container of the resizable component".to_string()),
                    ("resizable_box".to_string(), "For the resizable box itself".to_string()),
                    ("resizable_handle_visible".to_string(), "For visible resize handles".to_string()),
                    ("resizable_handle_hidden".to_string(), "For hidden resize handles".to_string()),
                    ("resizable_handle_nw".to_string(), "For the northwest corner handle".to_string()),
                    ("resizable_handle_ne".to_string(), "For the northeast corner handle".to_string()),
                    ("resizable_handle_sw".to_string(), "For the southwest corner handle".to_string()),
                    ("resizable_handle_se".to_string(), "For the southeast corner handle".to_string()),
                    ("resizable_handle_n".to_string(), "For the north edge handle".to_string()),
                    ("resizable_handle_s".to_string(), "For the south edge handle".to_string()),
                    ("resizable_handle_w".to_string(), "For the west edge handle".to_string()),
                    ("resizable_handle_e".to_string(), "For the east edge handle".to_string()),
                ]}
            />

        </Container>
    }
}
