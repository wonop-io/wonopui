use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(TypographyThemeEditor)]
pub fn typography_theme_editor() -> Html {
    let fields = vec![
        ("typography_h1".to_string(), "H1".to_string()),
        ("typography_h2".to_string(), "H2".to_string()),
        ("typography_h3".to_string(), "H3".to_string()),
        ("typography_h4".to_string(), "H4".to_string()),
        ("typography_h5".to_string(), "H5".to_string()),
        ("typography_h6".to_string(), "H6".to_string()),
        ("typography_p".to_string(), "Paragraph".to_string()),
    ];

    let preview = html! {
        <div>
            <H1>{ "This is an H1 Heading" }</H1>
            <H2>{ "This is an H2 Heading" }</H2>
            <H3>{ "This is an H3 Heading" }</H3>
            <H4>{ "This is an H4 Heading" }</H4>
            <H5>{ "This is an H5 Heading" }</H5>
            <H6>{ "This is an H6 Heading" }</H6>
            <Paragraph>{ "This is a paragraph with some example text." }</Paragraph>
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(TypographyDemo)]
pub fn typography_demo() -> Html {
    html! {
        <div>
            <H1 class={classes!("mb-4")}>{ "This is an H1 Heading" }</H1>
            <H2 class={classes!("mb-4")}>{ "This is an H2 Heading" }</H2>
            <H3 class={classes!("mb-4")}>{ "This is an H3 Heading" }</H3>
            <H4 class={classes!("mb-4")}>{ "This is an H4 Heading" }</H4>
            <H5 class={classes!("mb-4")}>{ "This is an H5 Heading" }</H5>
            <H6 class={classes!("mb-4")}>{ "This is an H6 Heading" }</H6>
            <Paragraph class={classes!("mb-4")}>{ "This is a paragraph with some example text." }</Paragraph>
        </div>
    }
}

#[function_component(TypographyDocumentation)]
pub fn typography_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Typography Components" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Typography components provide styled HTML heading and paragraph elements. Each component applies consistent styling based on your application's brand guidelines." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <TypographyDemo />
                }}
                customize={html! {
                    <TypographyThemeEditor />
                }}
                code={r#"
<div>
    <H1 class={classes!("mb-4")}>{ "This is an H1 Heading" }</H1>
    <H2 class={classes!("mb-4")}>{ "This is an H2 Heading" }</H2>
    <H3 class={classes!("mb-4")}>{ "This is an H3 Heading" }</H3>
    <H4 class={classes!("mb-4")}>{ "This is an H4 Heading" }</H4>
    <H5 class={classes!("mb-4")}>{ "This is an H5 Heading" }</H5>
    <H6 class={classes!("mb-4")}>{ "This is an H6 Heading" }</H6>
    <Paragraph class={classes!("mb-4")}>{ "This is a paragraph with some example text." }</Paragraph>
</div>"#.to_string()}
            />
            <Features features={vec!["H1", "H2", "H3", "H4", "H5", "H6", "Paragraph"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="TypographyProps"
                description="Props for all typography components."
                props={vec![
                    ("class", "Classes", "Additional CSS classes to apply."),
                    ("children", "Children", "Child elements to be rendered inside the component."),
                ]}
            />

            <NotesSection
                title={"Notes".to_string()}
                notes={vec![
                    "The components use the BRANDGUIDE configuration for consistent styling.".to_string(),
                    "Each component accepts additional CSS classes through the 'class' prop.".to_string(),
                    "Children passed to the components are rendered inside the corresponding HTML element.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Typography".to_string()}
                class_descriptions={vec![
                    ("typography_h1".to_string(), "For H1 elements".to_string()),
                    ("typography_h2".to_string(), "For H2 elements".to_string()),
                    ("typography_h3".to_string(), "For H3 elements".to_string()),
                    ("typography_h4".to_string(), "For H4 elements".to_string()),
                    ("typography_h5".to_string(), "For H5 elements".to_string()),
                    ("typography_h6".to_string(), "For H6 elements".to_string()),
                    ("typography_p".to_string(), "For paragraph elements".to_string()),
                ]}
            />
        </Container>
    }
}
