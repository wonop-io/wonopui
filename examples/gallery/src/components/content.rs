use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ContentThemeEditor)]
pub fn content_theme_editor() -> Html {
    let fields = vec![
        (
            "content_with_aside".to_string(),
            "Content With Aside".to_string(),
        ),
        ("content_aside".to_string(), "Aside Container".to_string()),
        (
            "content_aside_container".to_string(),
            "Aside Inner Container".to_string(),
        ),
    ];

    let preview = html! {
        <MainContent
            expanding={true}
            padding_x={true}
            padding_y={true}
            aside={Some(html! { <p>{"Aside content here"}</p> })}
        >
            <p>{ "Main content goes here" }</p>
        </MainContent>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ContentDocumentation)]
pub fn content_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "MainContent Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The MainContent component is designed to wrap the main content of the application, with optional padding and an adjacent aside element for additional content. It provides a flexible layout structure for creating responsive designs." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <MainContent expanding={true} padding_x={true} padding_y={true} aside={Some(html! { <p>{"Aside content here"}</p> })}>
                        <p>{ "Main content goes here" }</p>
                    </MainContent>
                }}
                customize={html! {
                    <ContentThemeEditor />
                }}
                code={r#"
<MainContent expanding={true} padding_x={true} padding_y={true} aside={Some(html! { <p>{"Aside content here"}</p> })}>
    <p>{ "Main content goes here" }</p>
</MainContent>"#.to_string()}
            />

            <Features features={vec!["MainContent"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="MainContent"
                description="Props for the MainContent component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the main content."),
                    ("class", "Classes", "Additional CSS classes to be applied to the main container."),
                    ("expanding", "bool", "Determines if the main content should expand to fill the available space. Default is true."),
                    ("padding_x", "bool", "If true, applies padding on the x-axis. Default is true."),
                    ("padding_y", "bool", "If true, applies padding on the y-axis. Default is true."),
                    ("aside", "Option<Html>", "Optional content to be rendered in an adjacent aside element."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The component automatically adjusts margins if an aside is present.".to_string(),
                    "The aside content is rendered inside a fixed container that appears on larger screens.".to_string(),
                    "Ensure to pass the appropriate padding and expansion properties based on the layout requirement.".to_string(),
                    "The aside element is hidden on smaller screens and becomes visible on medium-sized screens and larger.".to_string(),
                    "The main content area adjusts its width when an aside is present to prevent overlap.".to_string(),
                    "Consider the content hierarchy when using the aside feature to ensure important information is not hidden on smaller screens.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"MainContent".to_string()}
                class_descriptions={vec![
                    ("content_with_aside".to_string(), "Applied to the main content when an aside is present, adjusting the layout".to_string()),
                    ("content_aside".to_string(), "Styles the aside container, controlling its position and appearance".to_string()),
                    ("content_aside_container".to_string(), "Styles the inner container of the aside content".to_string()),
                ]}
            />
        </Container>
    }
}
