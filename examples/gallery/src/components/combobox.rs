use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ComboboxThemeEditor)]
pub fn combobox_theme_editor() -> Html {
    let fields = vec![
        ("combobox_button".to_string(), "Button Style".to_string()),
        (
            "combobox_button_open".to_string(),
            "Open Button Style".to_string(),
        ),
        (
            "combobox_button_disabled".to_string(),
            "Disabled Button Style".to_string(),
        ),
        (
            "combobox_list".to_string(),
            "List Container Style".to_string(),
        ),
        ("combobox_item".to_string(), "Item Style".to_string()),
        (
            "combobox_item_selected".to_string(),
            "Selected Item Style".to_string(),
        ),
        (
            "combobox_heading".to_string(),
            "Selected Item Style".to_string(),
        ),
    ];

    let preview = html! {
        <Combobox
            options={vec![
                ("1".to_string(), "Option 1".to_string()),
                ("2".to_string(), "Option 2".to_string()),
                ("3".to_string(), "Option 3".to_string())
            ]}
            on_select={Callback::from(|_| {})}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ComboboxDocumentation)]
pub fn combobox_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Combobox Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Combobox component is a versatile dropdown component that combines the functionality of a text input and a select dropdown. It allows users to choose from a predefined list of options or enter custom values, providing a flexible and accessible interface for option selection." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Combobox
                        options={vec![
                            ("1".to_string(), "Option 1".to_string()),
                            ("2".to_string(), "Option 2".to_string()),
                            ("3".to_string(), "Option 3".to_string())
                        ]}
                        on_select={Callback::from(|selected: String| {
                            // Handle selection
                        })}
                    />
                }}
                customize={html! {
                    <ComboboxThemeEditor />
                }}
                code={r#"
<Combobox
    options={vec![
        ("1".to_string(), "Option 1".to_string()),
        ("2".to_string(), "Option 2".to_string()),
        ("3".to_string(), "Option 3".to_string())
    ]}
    on_select={Callback::from(|selected: String| {
        // Handle selection
    })}
/>"#.to_string()}
            />

            <Features features={vec!["Combobox"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Combobox"
                description="Props for the Combobox component."
                props={vec![
                    ("id", "String", "The unique identifier for the combobox."),
                    ("options", "Vec<(String, String)>", "A list of options where each option is a tuple of (value, label)."),
                    ("on_select", "Callback<String>", "The callback to be called when an option is selected."),
                    ("disabled", "bool", "Whether the combobox is disabled. Default is false."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Combobox uses a button to toggle the options list.".to_string(),
                    "Options are displayed in a dropdown list when the combobox is open.".to_string(),
                    "The component is keyboard accessible and can be navigated using arrow keys.".to_string(),
                    "The selected option is visually distinguished in the list.".to_string(),
                    "The component manages its own open/closed state internally.".to_string(),
                    "Users can type to filter options, providing a quick way to find specific items in large lists.".to_string(),
                    "The Combobox supports both mouse and keyboard interactions for improved accessibility.".to_string(),
                    "Consider using Combobox when you have a large number of options or when users might need to search through the options.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Combobox".to_string()}
                class_descriptions={vec![
                    ("combobox_button".to_string(), "Styles for the main button of the combobox. This includes the default state styling.".to_string()),
                    ("combobox_button_open".to_string(), "Additional styles applied when the combobox dropdown is open. Use this to indicate the active state.".to_string()),
                    ("combobox_button_disabled".to_string(), "Styles applied when the combobox is disabled. This should visually communicate that the component is not interactive.".to_string()),
                    ("combobox_list".to_string(), "Styles for the dropdown list container. This affects the appearance of the option list when it's visible.".to_string()),
                    ("combobox_item".to_string(), "Styles for individual items in the dropdown list. This determines how each option looks.".to_string()),
                    ("combobox_item_selected".to_string(), "Styles applied to the currently selected item. This should make the selected option stand out from the others.".to_string()),
                    ("combobox_heading".to_string(), "Styles applied to the currently selected item. This should make the selected option stand out from the others.".to_string()),

                ]}
            />
        </Container>
    }
}
