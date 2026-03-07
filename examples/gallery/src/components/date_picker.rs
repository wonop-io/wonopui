use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(DatePickerDocumentation)]
pub fn date_picker_documentation() -> Html {
    let selected_date = use_state(|| None);
    
    let onchange = {
        let selected_date = selected_date.clone();
        Callback::from(move |date| {
            selected_date.set(Some(date));
            log::info!("Selected date: {:?}", date);
        })
    };
    
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Date Picker Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The DatePicker component allows users to select a date from a calendar. It provides an input field that opens a calendar popup when clicked." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="max-w-sm">
                        <DatePicker
                            value={*selected_date}
                            onchange={onchange}
                            placeholder={"Select a date"}
                        />
                    </div>
                }}
                code={r#"
let selected_date = use_state(|| None);

let onchange = {
    let selected_date = selected_date.clone();
    Callback::from(move |date| {
        selected_date.set(Some(date));
    })
};

html! {
    <DatePicker
        value={*selected_date}
        onchange={onchange}
        placeholder={"Select a date"}
    />
}"#.to_string()}
            />
            <Features features={vec!["Calendar popup", "Date selection", "Today indicator", "Month navigation", "Keyboard accessible"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="DatePicker"
                description="Props for the DatePicker component."
                props={vec![
                    ("value", "Option<NaiveDate>", "The currently selected date."),
                    ("onchange", "Callback<NaiveDate>", "Called when a date is selected."),
                    ("placeholder", "Option<String>", "Placeholder text for the input."),
                    ("disabled", "bool", "Whether the date picker is disabled."),
                    ("class", "Classes", "Additional CSS classes."),
                ]}
            />

            <NotesSection
                title={"Notes".to_string()}
                notes={vec![
                    "The DatePicker uses chrono's NaiveDate for date handling.".to_string(),
                    "Click on the input to open the calendar popup.".to_string(),
                    "Navigate between months using the arrow buttons.".to_string(),
                    "The current day is highlighted with a different background.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"DatePicker".to_string()}
                class_descriptions={vec![
                    ("container".to_string(), "Root container element".to_string()),
                    ("input".to_string(), "The text input that shows the selected date".to_string()),
                    ("calendar_popup".to_string(), "The calendar dropdown container".to_string()),
                    ("day".to_string(), "Individual day cells".to_string()),
                    ("day_selected".to_string(), "The selected day cell".to_string()),
                    ("day_today".to_string(), "Today's date cell".to_string()),
                ]}
            />

        </Container>
    }
}
