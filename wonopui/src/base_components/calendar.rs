#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::ClassesStr;
use chrono::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CalendarProps {
    #[prop_or_default]
    pub year: i32,
    #[prop_or_default]
    pub month: u32,
    #[prop_or_default]
    pub on_date_click: Callback<(i32, u32, u32)>,
    #[prop_or_default]
    pub selected_date: Option<NaiveDate>,
}

impl Default for CalendarProps {
    fn default() -> Self {
        let today = Local::today();
        CalendarProps {
            year: today.year(),
            month: today.month(),
            on_date_click: Callback::noop(),
            selected_date: None,
        }
    }
}

#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let year = props.year;
    let month = props.month;
    let today = Local::today().naive_local();
    let current_month = NaiveDate::from_ymd(year, month, 1);
    let days_in_month = current_month
        .with_day(1)
        .unwrap()
        .with_month(month + 1)
        .unwrap_or_else(|| {
            current_month
                .with_year(year + 1)
                .unwrap()
                .with_month(1)
                .unwrap()
        })
        - chrono::Duration::days(1);
    let first_day_of_month = current_month.weekday().num_days_from_sunday();

    let onclick = props.on_date_click.clone();

    let mut days_vec = vec![None; first_day_of_month as usize];
    days_vec.extend((1..=days_in_month.day()).map(Some));
    let weeks = days_vec.chunks(7);

    html! {
        <div class={classes!(&brandguide.calendar_container)}>
            <div class={classes!(&brandguide.calendar_wrapper)}>
                <div class={classes!(&brandguide.calendar_header)}>
                    <div class={classes!(&brandguide.calendar_title)}>
                        <div class={classes!(&brandguide.calendar_month_year)} aria-live="polite" role="presentation" id="react-day-picker-1">
                            { format!("{} {}", current_month.format("%B"), year) }
                        </div>
                        <div class={classes!(&brandguide.calendar_nav)}>
                            <button name="previous-month" aria-label="Go to previous month" class={classes!(&brandguide.calendar_nav_button)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-left h-4 w-4">
                                    <path d="m15 18-6-6 6-6"></path>
                                </svg>
                            </button>
                            <button name="next-month" aria-label="Go to next month" class={classes!(&brandguide.calendar_nav_button)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevron-right h-4 w-4">
                                    <path d="m9 18 6-6-6-6"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                    <table class={classes!(&brandguide.calendar_grid)} role="grid" aria-labelledby="react-day-picker-1">
                        <thead class={classes!(&brandguide.calendar_thead)}>
                            <tr class={classes!(&brandguide.calendar_weekdays)}>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Sunday">{"Su"}</th>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Monday">{"Mo"}</th>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Tuesday">{"Tu"}</th>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Wednesday">{"We"}</th>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Thursday">{"Th"}</th>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Friday">{"Fr"}</th>
                                <th scope="col" class={classes!(&brandguide.calendar_weekday)} aria-label="Saturday">{"Sa"}</th>
                            </tr>
                        </thead>
                        <tbody class={classes!(&brandguide.calendar_tbody)} role="rowgroup">
                            {
                                for weeks.enumerate().map(|(week_index, week)| {
                                    html! {
                                        <tr key={week_index} class={classes!(&brandguide.calendar_week)}>
                                            {
                                                for week.iter().enumerate().map(|(day_index, &day)| {
                                                    let date = day.map(|d| NaiveDate::from_ymd(year, month, d));
                                                    let is_today = date.map_or(false, |d| d == today);
                                                    let is_selected = props.selected_date.map_or(false, |selected| date.map_or(false, |d| d == selected));
                                                    let is_outside_month = day.is_none();

                                                    let day_classes = classes!(
                                                        &brandguide.calendar_day,
                                                        if is_today { brandguide.calendar_day_today.clone() } else { ClassesStr::empty() },
                                                        if is_selected { brandguide.calendar_day_selected.clone() } else { ClassesStr::empty() },
                                                        if is_outside_month { brandguide.calendar_day_outside.clone() } else { ClassesStr::empty() },
                                                    );

                                                    html! {
                                                        <td key={day_index} class={day_classes} role="presentation">
                                                            {
                                                                if let Some(d) = day {
                                                                    let onclick = onclick.clone();
                                                                    html! {
                                                                        <button
                                                                            name="day"
                                                                            class={classes!(&brandguide.calendar_day_button)}
                                                                            role="gridcell"
                                                                            tabindex="-1"
                                                                            type="button"
                                                                            onclick={Callback::from(move |_| onclick.emit((year, month, d)))}
                                                                        >
                                                                            { d }
                                                                        </button>
                                                                    }
                                                                } else {
                                                                    html! { <div>{" "}</div> }
                                                                }
                                                            }
                                                        </td>
                                                    }
                                                })
                                            }
                                        </tr>
                                    }
                                })
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}

// Snippets to update brandguide:
// ("calendar_container".to_string(), "rdp p-3 rounded-md border".to_string()),
// ("calendar_wrapper".to_string(), "flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0".to_string()),
// ("calendar_header".to_string(), "space-y-4 rdp-caption_start rdp-caption_end".to_string()),
// ("calendar_title".to_string(), "flex justify-center pt-1 relative items-center".to_string()),
// ("calendar_month_year".to_string(), "text-sm font-medium".to_string()),
// ("calendar_nav".to_string(), "space-x-1 flex items-center".to_string()),
// ("calendar_nav_button".to_string(), "rdp-button_reset rdp-button inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input hover:bg-accent hover:text-accent-foreground h-7 w-7 bg-transparent p-0 opacity-50 hover:opacity-100".to_string()),
// ("calendar_grid".to_string(), "w-full border-collapse space-y-1".to_string()),
// ("calendar_thead".to_string(), "rdp-head".to_string()),
// ("calendar_weekdays".to_string(), "flex".to_string()),
// ("calendar_weekday".to_string(), "text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]".to_string()),
// ("calendar_tbody".to_string(), "rdp-tbody".to_string()),
// ("calendar_week".to_string(), "flex w-full mt-2".to_string()),
// ("calendar_day".to_string(), "h-9 w-9 text-center text-sm p-0 relative [&:has([aria-selected].day-range-end)]:rounded-r-md [&:has([aria-selected].day-outside)]:bg-accent/50 [&:has([aria-selected])]:bg-accent first:[&:has([aria-selected])]:rounded-l-md last:[&:has([aria-selected])]:rounded-r-md focus-within:relative focus-within:z-20".to_string()),
// ("calendar_day_button".to_string(), "rdp-button_reset rdp-button inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9 p-0 font-normal aria-selected:opacity-100".to_string()),
// ("calendar_day_today".to_string(), "bg-accent text-accent-foreground".to_string()),
// ("calendar_day_selected".to_string(), "bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground focus:bg-primary focus:text-primary-foreground".to_string()),
// ("calendar_day_outside".to_string(), "text-muted-foreground opacity-50 aria-selected:bg-accent/50 aria-selected:text-muted-foreground aria-selected:opacity-30".to_string()),
//
// pub calendar_container: ClassesContainer<T>,
// pub calendar_wrapper: ClassesContainer<T>,
// pub calendar_header: ClassesContainer<T>,
// pub calendar_title: ClassesContainer<T>,
// pub calendar_month_year: ClassesContainer<T>,
// pub calendar_nav: ClassesContainer<T>,
// pub calendar_nav_button: ClassesContainer<T>,
// pub calendar_grid: ClassesContainer<T>,
// pub calendar_thead: ClassesContainer<T>,
// pub calendar_weekdays: ClassesContainer<T>,
// pub calendar_weekday: ClassesContainer<T>,
// pub calendar_tbody: ClassesContainer<T>,
// pub calendar_week: ClassesContainer<T>,
// pub calendar_day: ClassesContainer<T>,
// pub calendar_day_button: ClassesContainer<T>,
// pub calendar_day_today: ClassesContainer<T>,
// pub calendar_day_selected: ClassesContainer<T>,
// pub calendar_day_outside: ClassesContainer<T>,
//
// calendar_container: self.calendar_container.to_owned(),
// calendar_wrapper: self.calendar_wrapper.to_owned(),
// calendar_header: self.calendar_header.to_owned(),
// calendar_title: self.calendar_title.to_owned(),
// calendar_month_year: self.calendar_month_year.to_owned(),
// calendar_nav: self.calendar_nav.to_owned(),
// calendar_nav_button: self.calendar_nav_button.to_owned(),
// calendar_grid: self.calendar_grid.to_owned(),
// calendar_thead: self.calendar_thead.to_owned(),
// calendar_weekdays: self.calendar_weekdays.to_owned(),
// calendar_weekday: self.calendar_weekday.to_owned(),
// calendar_tbody: self.calendar_tbody.to_owned(),
// calendar_week: self.calendar_week.to_owned(),
// calendar_day: self.calendar_day.to_owned(),
// calendar_day_button: self.calendar_day_button.to_owned(),
// calendar_day_today: self.calendar_day_today.to_owned(),
// calendar_day_selected: self.calendar_day_selected.to_owned(),
// calendar_day_outside: self.calendar_day_outside.to_owned(),
//
// calendar_container: default_config_hm
// .get("calendar_container")
// .expect("Template parameter missing")
// .clone(),
// calendar_wrapper: default_config_hm
// .get("calendar_wrapper")
// .expect("Template parameter missing")
// .clone(),
// calendar_header: default_config_hm
// .get("calendar_header")
// .expect("Template parameter missing")
// .clone(),
// calendar_title: default_config_hm
// .get("calendar_title")
// .expect("Template parameter missing")
// .clone(),
// calendar_month_year: default_config_hm
// .get("calendar_month_year")
// .expect("Template parameter missing")
// .clone(),
// calendar_nav: default_config_hm
// .get("calendar_nav")
// .expect("Template parameter missing")
// .clone(),
// calendar_nav_button: default_config_hm
// .get("calendar_nav_button")
// .expect("Template parameter missing")
// .clone(),
// calendar_grid: default_config_hm
// .get("calendar_grid")
// .expect("Template parameter missing")
// .clone(),
// calendar_thead: default_config_hm
// .get("calendar_thead")
// .expect("Template parameter missing")
// .clone(),
// calendar_weekdays: default_config_hm
// .get("calendar_weekdays")
// .expect("Template parameter missing")
// .clone(),
// calendar_weekday: default_config_hm
// .get("calendar_weekday")
// .expect("Template parameter missing")
// .clone(),
// calendar_tbody: default_config_hm
// .get("calendar_tbody")
// .expect("Template parameter missing")
// .clone(),
// calendar_week: default_config_hm
// .get("calendar_week")
// .expect("Template parameter missing")
// .clone(),
// calendar_day: default_config_hm
// .get("calendar_day")
// .expect("Template parameter missing")
// .clone(),
// calendar_day_button: default_config_hm
// .get("calendar_day_button")
// .expect("Template parameter missing")
// .clone(),
// calendar_day_today: default_config_hm
// .get("calendar_day_today")
// .expect("Template parameter missing")
// .clone(),
// calendar_day_selected: default_config_hm
// .get("calendar_day_selected")
// .expect("Template parameter missing")
// .clone(),
// calendar_day_outside: default_config_hm
// .get("calendar_day_outside")
// .expect("Template parameter missing")
// .clone(),
