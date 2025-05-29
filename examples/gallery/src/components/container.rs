use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ContainerThemeEditor)]
pub fn container_theme_editor() -> Html {
    let fields = vec![
        (
            "container_padding_x".to_string(),
            "Horizontal Padding".to_string(),
        ),
        (
            "container_padding_y".to_string(),
            "Vertical Padding".to_string(),
        ),
        (
            "container_expanding".to_string(),
            "Expanding Container".to_string(),
        ),
        ("container_small".to_string(), "Small Variant".to_string()),
        ("container_narrow".to_string(), "Narrow Variant".to_string()),
        ("container_large".to_string(), "Large Variant".to_string()),
        (
            "container_responsive".to_string(),
            "Responsive Variant".to_string(),
        ),
    ];

    let preview = html! {
        <Container
            class={classes!("custom-class")}
            tag="section"
            expanding={true}
            padding_x={true}
            padding_y={true}
            variant={ContainerVariant::Large}
            style={Some("background-color: lightgrey;".to_string())}
        >
            <p>{ "This is a container with the large variant." }</p>
        </Container>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ContainerDocumentation)]
pub fn container_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Container Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Container component is a versatile layout element that wraps content with optional padding, expanding, and size variations. It helps in creating responsive designs that adjust according to the screen size." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Container
                        class={classes!("custom-class")}
                        tag="section"
                        expanding={true}
                        padding_x={true}
                        padding_y={true}
                        variant={ContainerVariant::Large}
                        style={Some("background-color: lightgrey;".to_string())}
                    >
                        <p>{ "This is a container with the large variant." }</p>
                    </Container>
                }}
                customize={html! {
                    <ContainerThemeEditor />
                }}
                code={r#"
<Container
    class={classes!("custom-class")}
    tag="section"
    expanding={true}
    padding_x={true}
    padding_y={true}
    variant={ContainerVariant::Large}
    style={Some("background-color: lightgrey;".to_string())}
>
    <p>{ "This is a container with the large variant." }</p>
</Container>"#.to_string()}
            />
            <Features features={vec!["Container"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="ContainerProps"
                description="Props for the Container component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the container."),
                    ("class", "Classes", "Additional classes to apply to the container."),
                    ("tag", "String", "The HTML tag to use for the container. Default is 'div'."),
                    ("expanding", "bool", "Whether the container should expand. Default is true."),
                    ("padding_x", "bool", "Whether to apply horizontal padding. Default is true."),
                    ("padding_y", "bool", "Whether to apply vertical padding. Default is true."),
                    ("variant", "ContainerVariant", "The size variant of the container. Default is 'Responsive'."),
                    ("style", "Option<String>", "Optional inline styles for the container."),
                ]}
            />

            <ApiSection
                title="ContainerVariant"
                description="Variants for the Container component."
                props={vec![
                    ("Small", "Small", "A small container."),
                    ("Narrow", "Narrow", "A narrow container."),
                    ("Large", "Large", "A large container."),
                    ("Responsive", "Responsive", "A responsive container (default)."),
                    ("None", "None", "No specific sizing applied."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Container component uses classes based on the Tailwind CSS framework.".to_string(),
                    "Different size variants help in creating responsive layouts.".to_string(),
                    "The expanding prop lets the container grow to fill available space.".to_string(),
                    "Horizontal and vertical padding can be controlled independently.".to_string(),
                    "The component supports both ThemeProvider and non-ThemeProvider configurations.".to_string(),
                    "Custom styles can be applied using the style prop for more specific layout needs.".to_string(),
                    "The tag prop allows for semantic HTML structure by changing the container's HTML element.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Container".to_string()}
                class_descriptions={vec![
                    ("container_padding_x".to_string(), "Controls the horizontal padding of the container".to_string()),
                    ("container_padding_y".to_string(), "Controls the vertical padding of the container".to_string()),
                    ("container_expanding".to_string(), "Makes the container expand to fill available space".to_string()),
                    ("container_small".to_string(), "Applies styles for the small variant".to_string()),
                    ("container_narrow".to_string(), "Applies styles for the narrow variant".to_string()),
                    ("container_large".to_string(), "Applies styles for the large variant".to_string()),
                    ("container_responsive".to_string(), "Applies styles for the responsive variant".to_string()),
                ]}
            />
        </Container>
    }
}
