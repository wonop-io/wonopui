use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(DatePickerDocumentation)]
pub fn date_picker_documentation() -> Html {
    /*
        html! {
            <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
                <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Date Picker Component" }</h1>
                <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The DatePicker component allows users to select a date from a calendar or input a date manually. It is designed to be user-friendly and provide a smooth date selection experience." }</p>

                <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
                <ExampleCode
                    preview={html! {
                        <DatePicker
                            selected_date={None}
                            onchange={Callback::from(|_| {})}
                        />
                    }}
                    code={r#"
    <DatePicker
        selected_date={None}
        onchange={Callback::from(|selected_date| {
            // Handle the date selection
        })}
    />"#.to_string()}
                />
                <Features features={vec!["DatePicker"]} />

                <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                    { "API" }
                </h2>

                <ApiSection
                    title="DatePicker"
                    description="Props for the DatePicker component."
                    props={vec![
                        ("selected_date", "Option<Date>", "The currently selected date, if any."),
                        ("onchange", "Callback<Option<Date>>", "A callback function that is called when a date is selected."),
                    ]}
                    template_params={None}
                />

                <NotesSection
                    title={"Notes".to_string()}
                    notes={vec![
                        "The DatePicker can be used to select single dates.".to_string(),
                        "To manage the selected date, use the `selected_date` prop and handle changes with the `onchange` callback.".to_string(),
                        "The component provides localized date formats and custom styling options.".to_string(),
                    ]}
                />

                <StylingSection
                    component_name={"DatePicker".to_string()}
                    class_descriptions={vec![
                        ("date_picker_container".to_string(), "For the main container of the date picker".to_string()),
                        ("date_picker_input".to_string(), "For the date input field".to_string()),
                        ("date_picker_calendar".to_string(), "For the calendar dropdown".to_string()),
                        ("date_picker_day".to_string(), "For individual day cells in the calendar".to_string()),
                    ]}
                />

            </Container>
        }
        */
    html! {
        { "TODO: implement date picker" }
    }
}
