use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(PlaceholderThemeEditor)]
pub fn placeholder_theme_editor() -> Html {
    let fields = vec![
        ("placeholder_container".to_string(), "Container".to_string()),
        ("placeholder_svg".to_string(), "SVG Background".to_string()),
        ("placeholder_text".to_string(), "Text".to_string()),
    ];

    let preview = html! {
        <PlaceholderDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(PlaceholderDemo)]
fn placeholder_demo() -> Html {
    html! {
        <>
            <Placeholder text="Default Placeholder" />
            <Placeholder text="Custom Class" class={classes!("mt-4", "h-32")} />
        </>
    }
}

#[function_component(PlaceholderDocumentation)]
pub fn placeholder_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Placeholder Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Placeholder component serves as a content placeholder with a dashed background pattern. It's useful for indicating where actual content will eventually go or for displaying loading states." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <PlaceholderDemo />
                }}
                customize={html! {
                    <PlaceholderThemeEditor />
                }}
                code={r#"
<Placeholder text="Default Placeholder" />
<Placeholder text="Custom Class" class={classes!("mt-4", "h-32")} />
<Placeholder text="Long Text Placeholder" class={classes!("mt-4", "h-48")}>
    {"This is a longer placeholder text that demonstrates how the component handles more content."}
</Placeholder>
"#.to_string()}
            />

            <Features features={vec!["Placeholder"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="PlaceholderProps"
                description="Props for the Placeholder component."
                props={vec![
                    ("class", "Classes", "Additional CSS classes to apply to the Placeholder component."),
                    ("text", "String", r#"The text to display inside the Placeholder. Defaults to "Content placeholder"."#),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Placeholder component is fully responsive and adjusts to the size of its container.".to_string(),
                    "You can customize the appearance further by providing additional CSS classes via the `class` prop.".to_string(),
                    "The component uses a SVG pattern for the dashed background, ensuring crisp rendering at any size.".to_string(),
                    "For accessibility, consider adding appropriate aria attributes when using this component in a loading context.".to_string(),
                    "The text content can be customized, allowing for informative messages or instructions to be displayed.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Placeholder".to_string()}
                class_descriptions={vec![
                    ("placeholder_container".to_string(), "Styles for the main container of the placeholder".to_string()),
                    ("placeholder_svg".to_string(), "Styles for the SVG element that creates the dashed background".to_string()),
                    ("placeholder_text".to_string(), "Styles for the text content within the placeholder".to_string()),
                ]}
            />
        </Container>
    }
}
