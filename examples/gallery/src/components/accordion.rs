use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(AccordionThemeEditor)]
pub fn accordion_theme_editor() -> Html {
    let fields = vec![
        (
            "accordion_container".to_string(),
            "Accordion Container".to_string(),
        ),
        (
            "accordion_header".to_string(),
            "Accordion Header".to_string(),
        ),
        ("accordion_title".to_string(), "Accordion Title".to_string()),
        (
            "accordion_content".to_string(),
            "Accordion Content".to_string(),
        ),
    ];

    let preview = html! {
        <Accordion title="Example Accordion">
            <p>{"This is an example accordion content."}</p>
        </Accordion>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(AccordionDocumentation)]
pub fn accordion_documentation() -> Html {
    html! {
            <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
    //        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
                <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Accordion Component" }</h1>
                <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Accordion component is a collapsible section of content. It allows users to toggle the visibility of content sections, providing an efficient way to organize and present information." }</p>

                <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
                <ExampleCode
                    preview={html! {
                        <>
                            <Accordion title="What is the best about Switzerland?">
                                <p>{ "I don't know but the flag is a big plus." }</p>
                            </Accordion>
                            <Accordion title="Why is the sky blue?">
                                <p>{ "Because if it was green, we wouldn't know where to stop mowing." }</p>
                            </Accordion>
                            <Accordion title="Why don't scientists trust atoms?">
                                <p>{ "Because they make up everything!" }</p>
                            </Accordion>
                            <Accordion title="What do you call fake spaghetti?">
                                <p>{ "An impasta!" }</p>
                            </Accordion>
                            <Accordion title="Why did the scarecrow win an award?">
                                <p>{ "Because he was outstanding in his field!" }</p>
                            </Accordion>
                        </>
                    }}
                    customize={html! {
                        <AccordionThemeEditor />
                    }}
                    code={r#"
<Accordion title="Accordion example">
    <p>{ "This is the content of the accordion. It can be expanded or collapsed." }</p>
</Accordion>"#.to_string()}
                />

                <Features features={vec!["Accordion"]} />

                <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                    { "API" }
                </h2>

                <ApiSection
                    title="Accordion"
                    description="Props for the Accordion component."
                    props={vec![
                        ("title", "String", "The title of the accordion section."),
                        ("children", "Children", "The child elements to be rendered inside the accordion content."),
                        ("class", "Classes", "Additional CSS classes to apply to the accordion container."),
                    ]}
                />

                <NotesSection
                    title={"Usage Notes".to_string()}
                    notes={vec![
                        "The accordion uses internal state to manage its open/closed state.".to_string(),
                        "Clicking on the header toggles the visibility of the content.".to_string(),
                        "The content is only rendered when the accordion is open.".to_string(),
                        "Multiple accordions can be used together to create an accordion group.".to_string(),
                    ]}
                />

                <StylingSection
                    component_name={"Accordion".to_string()}
                    class_descriptions={vec![
                        ("accordion_container".to_string(), "For the main container of the accordion component".to_string()),
                        ("accordion_header".to_string(), "For the header section of the accordion".to_string()),
                        ("accordion_title".to_string(), "For the title text within the accordion header".to_string()),
                        ("accordion_content".to_string(), "For the content container of the accordion".to_string()),
                    ]}
                />
    //        </div>
            </Container>
        }
}
