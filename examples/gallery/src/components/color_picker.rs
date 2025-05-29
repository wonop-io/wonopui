use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ColorPickerThemeEditor)]
pub fn color_picker_theme_editor() -> Html {
    let fields = vec![
        (
            "color_picker_container".to_string(),
            "Color Picker Container".to_string(),
        ),
        (
            "color_picker_canvas".to_string(),
            "Color Picker Canvas".to_string(),
        ),
        (
            "color_picker_indicator".to_string(),
            "Color Picker Indicator".to_string(),
        ),
        (
            "color_picker_input_container".to_string(),
            "Input Container".to_string(),
        ),
        (
            "color_picker_swatch".to_string(),
            "Color Swatch".to_string(),
        ),
        (
            "color_picker_input".to_string(),
            "Color Picker Input".to_string(),
        ),
    ];

    let preview = html! {
        <ColorPicker
            value={"#FF0000".to_string()}
            width={200}
            height={200}
            onchange={Callback::from(|_| {})}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ColorPickerDocumentation)]
pub fn color_picker_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Color Picker Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Color Picker component provides an interactive interface for users to select a color from a canvas. It allows for precise color selection and displays the currently selected color in multiple formats." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <ColorPicker
                        value={"#FF0000".to_string()}
                        width={200}
                        height={200}
                        onchange={Callback::from(|color: String| {
                            // Handle color change
                        })}
                    />
                }}
                customize={html! {
                    <ColorPickerThemeEditor />
                }}
                code={r#"
<ColorPicker
    value={"\#FF0000".to_string()}
    width={200}
    height={200}
    onchange={Callback::from(|color: String| {
        // Handle color change
    })}
/>"#.to_string()}
            />

            <Features features={vec!["ColorPicker"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="ColorPicker"
                description="Props for the ColorPicker component."
                props={vec![
                    ("value", "String", "The initial color value of the picker (e.g., \"#FF0000\")."),
                    ("onchange", "Callback<String>", "A callback function that is called when the color is changed."),
                    ("width", "u32", "The width of the color picker canvas. Defaults to 200."),
                    ("height", "u32", "The height of the color picker canvas. Defaults to 200."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The component uses a canvas to display a 2D color gradient.".to_string(),
                    "The color picker allows for selection by clicking and dragging on the canvas.".to_string(),
                    "The current color is displayed as a swatch alongside the canvas.".to_string(),
                    "The hex value of the selected color can be manually input and edited.".to_string(),
                    "The component updates in real-time as the user interacts with the canvas or input field.".to_string(),
                    "The color picker supports a wide range of colors, including all standard web colors.".to_string(),
                    "For accessibility, the component supports keyboard navigation and provides ARIA attributes for screen readers.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"ColorPicker".to_string()}
                class_descriptions={vec![
                    ("color_picker_container".to_string(), "Styles the main container of the color picker component".to_string()),
                    ("color_picker_canvas".to_string(), "Styles the canvas element where colors are displayed".to_string()),
                    ("color_picker_indicator".to_string(), "Styles the indicator that shows the currently selected color on the canvas".to_string()),
                    ("color_picker_input_container".to_string(), "Styles the container for the color swatch and input field".to_string()),
                    ("color_picker_swatch".to_string(), "Styles the swatch displaying the currently selected color".to_string()),
                    ("color_picker_input".to_string(), "Styles the input field where the color value is displayed and can be edited".to_string()),
                ]}
            />
        </Container>
    }
}
