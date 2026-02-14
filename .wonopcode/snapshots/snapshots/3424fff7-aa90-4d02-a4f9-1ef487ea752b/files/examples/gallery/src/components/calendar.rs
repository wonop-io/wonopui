use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use chrono::Datelike;
use chrono::Local;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(CalendarThemeEditor)]
pub fn calendar_theme_editor() -> Html {
    let fields = vec![
        (
            "calendar_container".to_string(),
            "Calendar Container".to_string(),
        ),
        (
            "calendar_wrapper".to_string(),
            "Calendar Wrapper".to_string(),
        ),
        ("calendar_header".to_string(), "Calendar Header".to_string()),
        ("calendar_title".to_string(), "Calendar Title".to_string()),
        (
            "calendar_month_year".to_string(),
            "Month/Year Display".to_string(),
        ),
        ("calendar_nav".to_string(), "Navigation Buttons".to_string()),
        ("calendar_grid".to_string(), "Calendar Grid".to_string()),
        ("calendar_thead".to_string(), "Table Header".to_string()),
        ("calendar_weekdays".to_string(), "Weekdays Row".to_string()),
        ("calendar_weekday".to_string(), "Weekday Cell".to_string()),
        ("calendar_tbody".to_string(), "Table Body".to_string()),
        ("calendar_week".to_string(), "Week Row".to_string()),
        ("calendar_day".to_string(), "Day Cell".to_string()),
        ("calendar_day_button".to_string(), "Day Button".to_string()),
    ];

    let preview = html! {
        <CalendarDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CalendarDemo)]
pub fn calendar_demo() -> Html {
    let today = Local::now().date_naive();
    let selected_date = use_state(|| None);
    let on_date_click = {
        let selected_date = selected_date.clone();
        Callback::from(move |date: (i32, u32, u32)| {
            selected_date.set(Some(date));
        })
    };

    html! {
        <div>
            <Calendar
                year={today.year()}
                month={today.month()}
                on_date_click={on_date_click.clone()}
                selected_date={(*selected_date).map(|(y, m, d)| chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap_or_default())}
            />
            {
                if let Some((year, month, day)) = *selected_date {
                    html! { <p>{ format!("Selected date: {}-{:02}-{:02}", year, month, day) }</p> }
                } else {
                    html! { <p>{ "No date selected" }</p> }
                }
            }
        </div>
    }
}

#[function_component(CalendarDocumentation)]
pub fn calendar_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Calendar Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Calendar component displays a monthly calendar view, allowing users to select a date. It provides a flexible and responsive interface for date selection tasks." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! { <CalendarDemo /> }}
                customize={html! { <CalendarThemeEditor /> }}
                code={r#"
use chrono::Local;
use yew::prelude::*;

#[function_component(CalendarDemo)]
pub fn calendar_demo() -> Html {
    let today = Local::today();
    let selected_date = use_state(|| None);
    let on_date_click = {
        let selected_date = selected_date.clone();
        Callback::from(move |date: (i32, u32, u32)| {
            selected_date.set(Some(date));
        })
    };

    html! {
        <div>
            <Calendar
                year={today.year()}
                month={today.month()}
                on_date_click={on_date_click.clone()}
                selected_date={(*selected_date).map(|(y, m, d)| chrono::NaiveDate::from_ymd(y, m, d))}
            />
            {
                if let Some((year, month, day)) = *selected_date {
                    html! { <p>{ format!("Selected date: {}-{:02}-{:02}", year, month, day) }</p> }
                } else {
                    html! { <p>{ "No date selected" }</p> }
                }
            }
        </div>
    }
}
                "#.to_string()}
            />

            <Features features={vec![
                "Customizable appearance through theming",
                "Automatic calculation of days in month, including leap years",
                "Callback for date selection",
                "Accessible design with proper ARIA attributes",
                "Responsive layout",
                "Displays current month by default",
                "Highlights selected date"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Calendar"
                description="Props for the Calendar component."
                props={vec![
                    ("year", "i32", "The year to display. Defaults to the current year if 0."),
                    ("month", "u32", "The month to display (1-12). Defaults to the current month if 0."),
                    ("on_date_click", "Callback<(i32, u32, u32)>", "A callback function that is called when a date is clicked, with the year, month, and day of the clicked date."),
                    ("selected_date", "Option<NaiveDate>", "The currently selected date, if any."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Calendar component calculates the number of days in the displayed month, including leap years.".to_string(),
                    "The days from the previous and next months are not displayed; empty slots are filled with spaces.".to_string(),
                    "The component uses the local date and time for calculations.".to_string(),
                    "For accessibility, the component uses appropriate ARIA attributes and roles.".to_string(),
                    "The calendar starts the week on Sunday and displays abbreviated weekday names.".to_string(),
                    "Navigation buttons for previous and next months are not implemented in this version but can be added as needed.".to_string(),
                    "The selected date is highlighted when set.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Calendar".to_string()}
                class_descriptions={vec![
                    ("calendar_container".to_string(), "Main container of the calendar component".to_string()),
                    ("calendar_wrapper".to_string(), "Wrapper for the calendar content".to_string()),
                    ("calendar_header".to_string(), "Header section of the calendar".to_string()),
                    ("calendar_title".to_string(), "Title area containing month/year display".to_string()),
                    ("calendar_month_year".to_string(), "Display for current month and year".to_string()),
                    ("calendar_nav".to_string(), "Container for navigation buttons".to_string()),
                    ("calendar_grid".to_string(), "Grid layout for the calendar days".to_string()),
                    ("calendar_thead".to_string(), "Header of the calendar table".to_string()),
                    ("calendar_weekdays".to_string(), "Row containing weekday abbreviations".to_string()),
                    ("calendar_weekday".to_string(), "Individual weekday cell".to_string()),
                    ("calendar_tbody".to_string(), "Body of the calendar table".to_string()),
                    ("calendar_week".to_string(), "Row representing a week".to_string()),
                    ("calendar_day".to_string(), "Cell for each day in the calendar".to_string()),
                    ("calendar_day_button".to_string(), "Button for selecting a specific day".to_string()),
                    ("calendar_day_selected".to_string(), "Styling for the selected day".to_string()),
                ]}
            />
        </Container>
    }
}
