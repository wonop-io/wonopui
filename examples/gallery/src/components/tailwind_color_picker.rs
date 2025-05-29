use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(TailwindColorPickerThemeEditor)]
pub fn tailwind_color_picker_theme_editor() -> Html {
    let fields = vec![
        (
            "tailwind_color_picker_container".to_string(),
            "Container".to_string(),
        ),
        (
            "tailwind_color_picker_button".to_string(),
            "Button".to_string(),
        ),
        (
            "tailwind_color_picker_palette".to_string(),
            "Palette".to_string(),
        ),
        (
            "tailwind_color_picker_color".to_string(),
            "Color".to_string(),
        ),
        (
            "tailwind_color_picker_shade".to_string(),
            "Shade".to_string(),
        ),
    ];

    let preview = html! {
        <TailwindColorPicker
            color={"blue".to_string()}
            shade={500}
            oncolorchange={Callback::from(|_| {})}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(TailwindColorPickerDemo)]
pub fn tailwind_color_picker_demo() -> Html {
    let color = use_state(|| "blue".to_string());
    let shade = use_state(|| 500);

    let oncolorchange = {
        let color = color.clone();
        let shade = shade.clone();
        Callback::from(move |(new_color, new_shade)| {
            color.set(new_color);
            shade.set(new_shade);
        })
    };

    html! {
        <div>
            <TailwindColorPicker
                color={(*color).clone()}
                shade={*shade}
                {oncolorchange}
            />
            <p class="mt-4">{"Selected color: "}{&*color}{", Shade: "}{*shade}</p>
        </div>
    }
}

#[function_component(TailwindColorPickerDocumentation)]
pub fn tailwind_color_picker_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "TailwindColorPicker Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The TailwindColorPicker component is a customizable color picker that allows
                users to choose a color and its corresponding shade. It is built using Tailwind CSS classes
                and provides a simple way to integrate a color selection functionality into your Yew application." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! { <TailwindColorPickerDemo /> }}
                customize={html! { <TailwindColorPickerThemeEditor /> }}
                code={r#"
#[function_component(TailwindColorPickerDemo)]
pub fn tailwind_color_picker_demo() -> Html {
    let color = use_state(|| "blue".to_string());
    let shade = use_state(|| 500);

    let oncolorchange = {
        let color = color.clone();
        let shade = shade.clone();
        Callback::from(move |(new_color, new_shade)| {
            color.set(new_color);
            shade.set(new_shade);
        })
    };

    html! {
        <div>
            <TailwindColorPicker
                color={(*color).clone()}
                shade={*shade}
                {oncolorchange}
            />
            <p class="mt-4">{"Selected color: "}{&*color}{", Shade: "}{*shade}</p>
        </div>
    }
}"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "Variations" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex flex-col space-y-4">
                        <TailwindColorPicker
                            color={"red".to_string()}
                            shade={600}
                            oncolorchange={Callback::from(|_| {})}
                        />
                        <TailwindColorPicker
                            color={"green".to_string()}
                            shade={300}
                            oncolorchange={Callback::from(|_| {})}
                        />
                        <TailwindColorPicker
                            color={"purple".to_string()}
                            shade={800}
                            oncolorchange={Callback::from(|_| {})}
                        />
                    </div>
                }}
                code={r#"
<TailwindColorPicker
    color={"red".to_string()}
    shade={600}
    oncolorchange={Callback::from(|_| {})}
/>
<TailwindColorPicker
    color={"green".to_string()}
    shade={300}
    oncolorchange={Callback::from(|_| {})}
/>
<TailwindColorPicker
    color={"purple".to_string()}
    shade={800}
    oncolorchange={Callback::from(|_| {})}
/>"#.to_string()}
            />

            <Features features={vec!["TailwindColorPicker"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="TailwindColorPicker"
                description="Props for the TailwindColorPicker component."
                props={vec![
                    ("color", "String", "Default selected color. Defaults to 'blue'."),
                    ("shade", "u16", "Default shade of the color. Defaults to 500."),
                    ("oncolorchange", "Callback<(String, u16)>", "Callback function called when a color is selected."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The TailwindColorPicker component uses internal state to manage the visibility of the color palette.".to_string(),
                    "Tailwind CSS classes are used for styling the colors and shades.".to_string(),
                    "The component keeps focus when the palette is open to manage interactions.".to_string(),
                    "The color picker supports all Tailwind CSS color names and shades.".to_string(),
                    "For accessibility, the component supports keyboard navigation and screen reader announcements.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"TailwindColorPicker".to_string()}
                class_descriptions={vec![
                    ("tailwind_color_picker_container".to_string(), "For the main container of the color picker".to_string()),
                    ("tailwind_color_picker_button".to_string(), "For the button that toggles the palette".to_string()),
                    ("tailwind_color_picker_palette".to_string(), "For the dropdown container of the color palette".to_string()),
                    ("tailwind_color_picker_color".to_string(), "For individual color options in the palette".to_string()),
                    ("tailwind_color_picker_shade".to_string(), "For individual shade options in the palette".to_string()),
                ]}
            />
        </Container>
    }
}
