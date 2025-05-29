use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(PageHeaderThemeEditor)]
pub fn page_header_theme_editor() -> Html {
    let fields = vec![
        (
            "page_header_container".to_string(),
            "Header Container".to_string(),
        ),
        ("page_header_title".to_string(), "Header Title".to_string()),
        (
            "page_header_actions".to_string(),
            "Header Actions".to_string(),
        ),
    ];

    let preview = html! {
        <PageHeaderDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(PageHeaderDemo)]
fn page_header_demo() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    html! {
        <>
            <PageHeader title="Simple Header" />
            <PageHeader title="Header with Actions">
                <Button>{"Action 1"}</Button>
                <Button>{"Action 2"}</Button>
            </PageHeader>
            <PageHeader title="Interactive Header">
                <Button {onclick}>{"Clicked "}{*counter}{" times"}</Button>
            </PageHeader>
        </>
    }
}

#[function_component(PageHeaderDocumentation)]
pub fn page_header_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "PageHeader Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The PageHeader component provides a flexible header element for pages, which includes a title and optional child elements. This can be useful for providing consistent headers across different sections of an application." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <PageHeaderDemo />
                }}
                customize={html! {
                    <PageHeaderThemeEditor />
                }}
                code={r#"
use wonopui::*;
use yew::prelude::*;

#[function_component(PageHeaderDemo)]
fn page_header_demo() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    html! {
        <>
            <PageHeader title="Simple Header" />
            <PageHeader title="Header with Actions">
                <Button>{"Action 1"}</Button>
                <Button>{"Action 2"}</Button>
            </PageHeader>
            <PageHeader title="Interactive Header">
                <Button {onclick}>{"Clicked "}{*counter}{" times"}</Button>
            </PageHeader>
        </>
    }
}"#.to_string()}
            />

            <Features features={vec!["PageHeader"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="PageHeaderProps"
                description="Props for the PageHeader component."
                props={vec![
                    ("title", "String", "The title to be displayed in the header."),
                    ("children", "Children", "Optional child elements to be rendered alongside the title."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The PageHeader component is designed to be used at the top of a page or section.".to_string(),
                    "Child elements are rendered to the right of the title in a flex container.".to_string(),
                    "The component is responsive and adjusts its layout on smaller screens.".to_string(),
                    "You can customize the appearance of the PageHeader using the theme editor.".to_string(),
                    "The PageHeader can contain interactive elements like buttons or form inputs.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"PageHeader".to_string()}
                class_descriptions={vec![
                    ("page_header_container".to_string(), "For the main container of the page header".to_string()),
                    ("page_header_title".to_string(), "For the title text in the header".to_string()),
                    ("page_header_actions".to_string(), "For the container of action elements".to_string()),
                ]}
            />
        </Container>
    }
}
