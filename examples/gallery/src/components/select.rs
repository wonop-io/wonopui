use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(SelectThemeEditor)]
pub fn select_theme_editor() -> Html {
    let fields = vec![
        (
            "select_container".to_string(),
            "Select Container".to_string(),
        ),
        ("select_trigger".to_string(), "Select Trigger".to_string()),
        (
            "select_trigger_placeholder".to_string(),
            "Trigger Placeholder".to_string(),
        ),
        (
            "select_trigger_icon".to_string(),
            "Trigger Icon".to_string(),
        ),
        (
            "select_content_container".to_string(),
            "Content Container".to_string(),
        ),
        (
            "select_content_list".to_string(),
            "Content List".to_string(),
        ),
        ("select_item".to_string(), "Select Item".to_string()),
    ];

    let preview = html! {
        <Select<String>
            options={vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]}
            selected={None::<String>}
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

#[function_component(SelectDocumentation)]
pub fn select_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Select Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Select component is a customizable dropdown that allows users to choose an option from a list. It provides a clean and intuitive interface for selection tasks, with support for various data types." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Select<String>
                        options={vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]}
                        selected={None::<String>}
                        onchange={Callback::from(|_| {})}
                    />
                }}
                customize={html! {
                    <SelectThemeEditor />
                }}
                code={r#"
<Select<String>
    options={vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]}
    selected={None::<String>}
    onchange={Callback::from(|selected| {
        // Handle the selection
    })}
/>"#.to_string()}
            />
            <Features features={vec!["Select"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Select<T>"
                description="Props for the Select component."
                props={vec![
                    ("options", "Vec<T>", "A vector of options to display in the dropdown."),
                    ("selected", "Option<T>", "The currently selected option, if any."),
                    ("onchange", "Callback<T>", "A callback function that is called when an option is selected."),
                ]}
                template_params={Some(vec![
                    ("T", "The type of the options. Must implement Clone, PartialEq, ToString, and 'static."),
                ])}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The component uses internal state to manage the open/closed state of the dropdown.".to_string(),
                    "The selected option is displayed in the trigger button.".to_string(),
                    "The dropdown list is only rendered when the select is open.".to_string(),
                    "The Select component is generic and can work with various data types, as long as they implement the required traits.".to_string(),
                    "For accessibility, the component supports keyboard navigation and screen reader announcements.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Select".to_string()}
                class_descriptions={vec![
                    ("select_container".to_string(), "For the main container of the select component".to_string()),
                    ("select_trigger".to_string(), "For the trigger button that opens the dropdown".to_string()),
                    ("select_trigger_placeholder".to_string(), "For the placeholder text in the trigger button".to_string()),
                    ("select_trigger_icon".to_string(), "For the dropdown icon in the trigger button".to_string()),
                    ("select_content_container".to_string(), "For the container of the dropdown content".to_string()),
                    ("select_content_list".to_string(), "For the list of options in the dropdown".to_string()),
                    ("select_item".to_string(), "For individual items in the dropdown list".to_string()),
                ]}
            />

        </Container>
    }
}
