use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::*;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use yew::prelude::*;

#[function_component(ColThemeEditor)]
pub fn col_theme_editor() -> Html {
    let fields = vec![("col_container".to_string(), "Col Base".to_string())];

    let preview = html! {
        <Col>
            <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 1" }</div>
            <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 2" }</div>
            <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 3" }</div>
        </Col>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ColDocumentation)]
pub fn col_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Col Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Col component is a flexible container that arranges its children in a vertical layout. It can be customized with additional classes and an HTML tag of choice." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Col>
                        <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 1" }</div>
                        <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 2" }</div>
                        <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 3" }</div>
                    </Col>
                }}
                customize={html! {
                    <ColThemeEditor />
                }}
                code={r#"
<Col>
    <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 1" }</div>
    <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 2" }</div>
    <div class="bg-zinc-200 dark:bg-zinc-800 p-4">{ "Child 3" }</div>
</Col>
"#.to_string()}
            />

            <Features features={vec!["Col"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Col"
                description="Props for the Col component."
                props={vec![
                    ("children", "Children", "Elements to be displayed inside the Col."),
                    ("class", "Classes", "Additional CSS classes to apply to the Col."),
                    ("tag", "String", "The HTML tag to use for the Col component (default: div)."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Col component uses Flexbox for layout with 'flex' and 'flex-col' classes.".to_string(),
                    "You can customize the HTML tag used for the Col container using the 'tag' prop.".to_string(),
                    "Ensure that your children are valid HTML elements for the context in which they are used.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Col".to_string()}
                class_descriptions={vec![
                    ("col_base".to_string(), "For the main container of the col component".to_string()),
                ]}
            />
        </Container>
    }
}
