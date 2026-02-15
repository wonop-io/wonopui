//! Calendar component for WonopUI.
//!
//! A date picker calendar component.

use chrono::prelude::*;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Calendar component
pub mod classes {
    pub const CONTAINER: &str = "p-3 rounded-md border bg-background";
    pub const WRAPPER: &str = "flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0";
    pub const HEADER: &str = "space-y-4";
    pub const TITLE: &str = "flex justify-center pt-1 relative items-center";
    pub const MONTH_YEAR: &str = "text-sm font-medium";
    pub const NAV: &str = "space-x-1 flex items-center";
    pub const NAV_BUTTON: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input hover:bg-accent hover:text-accent-foreground h-7 w-7 bg-transparent p-0 opacity-50 hover:opacity-100";
    pub const GRID: &str = "w-full border-collapse space-y-1";
    pub const THEAD: &str = "";
    pub const WEEKDAYS: &str = "flex";
    pub const WEEKDAY: &str =
        "text-muted-foreground rounded-md w-9 font-normal text-[0.8rem] text-center";
    pub const TBODY: &str = "";
    pub const WEEK: &str = "flex w-full mt-2";
    pub const DAY: &str =
        "h-9 w-9 text-center text-sm p-0 relative focus-within:relative focus-within:z-20";
    pub const DAY_BUTTON: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9 p-0 font-normal";
    pub const DAY_TODAY: &str = "bg-accent text-accent-foreground";
    pub const DAY_SELECTED: &str = "bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground focus:bg-primary focus:text-primary-foreground";
    pub const DAY_OUTSIDE: &str = "text-muted-foreground opacity-50";
}

#[derive(Properties, PartialEq)]
pub struct CalendarProps {
    #[prop_or_default]
    pub year: Option<i32>,
    #[prop_or_default]
    pub month: Option<u32>,
    #[prop_or_default]
    pub on_date_click: Callback<NaiveDate>,
    #[prop_or_default]
    pub on_month_change: Callback<(i32, u32)>,
    #[prop_or_default]
    pub selected_date: Option<NaiveDate>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let today = Local::now().date_naive();

    let viewing_year = use_state(|| props.year.unwrap_or(today.year()));
    let viewing_month = use_state(|| props.month.unwrap_or(today.month()));

    // Update viewing state when props change
    {
        let viewing_year = viewing_year.clone();
        let viewing_month = viewing_month.clone();
        let prop_year = props.year;
        let prop_month = props.month;
        use_effect_with((prop_year, prop_month), move |(year, month)| {
            if let Some(y) = year {
                viewing_year.set(*y);
            }
            if let Some(m) = month {
                viewing_month.set(*m);
            }
            || ()
        });
    }

    let year = *viewing_year;
    let month = *viewing_month;

    let current_month_first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

    // Calculate days in month
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    let days_in_month = next_month
        .map(|nm| nm.signed_duration_since(current_month_first).num_days() as u32)
        .unwrap_or(31);

    let first_day_of_month = current_month_first.weekday().num_days_from_sunday();

    let on_prev_month = {
        let viewing_year = viewing_year.clone();
        let viewing_month = viewing_month.clone();
        let on_month_change = props.on_month_change.clone();
        Callback::from(move |_: MouseEvent| {
            let (new_year, new_month) = if *viewing_month == 1 {
                (*viewing_year - 1, 12)
            } else {
                (*viewing_year, *viewing_month - 1)
            };
            viewing_year.set(new_year);
            viewing_month.set(new_month);
            on_month_change.emit((new_year, new_month));
        })
    };

    let on_next_month = {
        let viewing_year = viewing_year.clone();
        let viewing_month = viewing_month.clone();
        let on_month_change = props.on_month_change.clone();
        Callback::from(move |_: MouseEvent| {
            let (new_year, new_month) = if *viewing_month == 12 {
                (*viewing_year + 1, 1)
            } else {
                (*viewing_year, *viewing_month + 1)
            };
            viewing_year.set(new_year);
            viewing_month.set(new_month);
            on_month_change.emit((new_year, new_month));
        })
    };

    // Build weeks
    let mut days_vec: Vec<Option<u32>> = vec![None; first_day_of_month as usize];
    days_vec.extend((1..=days_in_month).map(Some));

    // Pad to complete last week
    while days_vec.len() % 7 != 0 {
        days_vec.push(None);
    }

    let weeks: Vec<Vec<Option<u32>>> = days_vec.chunks(7).map(|c| c.to_vec()).collect();

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            <div class={classes::WRAPPER}>
                <div class={classes::HEADER}>
                    <div class={classes::TITLE}>
                        <div class={classes::MONTH_YEAR}>
                            { format!("{} {}", current_month_first.format("%B"), year) }
                        </div>
                        <div class={classes::NAV}>
                            <button
                                onclick={on_prev_month}
                                aria-label="Go to previous month"
                                class={classes::NAV_BUTTON}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                                    <path d="m15 18-6-6 6-6"/>
                                </svg>
                            </button>
                            <button
                                onclick={on_next_month}
                                aria-label="Go to next month"
                                class={classes::NAV_BUTTON}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
                                    <path d="m9 18 6-6-6-6"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                    <table class={classes::GRID} role="grid">
                        <thead class={classes::THEAD}>
                            <tr class={classes::WEEKDAYS}>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Sunday">{"Su"}</th>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Monday">{"Mo"}</th>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Tuesday">{"Tu"}</th>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Wednesday">{"We"}</th>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Thursday">{"Th"}</th>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Friday">{"Fr"}</th>
                                <th scope="col" class={classes::WEEKDAY} aria-label="Saturday">{"Sa"}</th>
                            </tr>
                        </thead>
                        <tbody class={classes::TBODY} role="rowgroup">
                            { for weeks.iter().enumerate().map(|(week_index, week)| {
                                html! {
                                    <tr key={week_index} class={classes::WEEK}>
                                        { for week.iter().enumerate().map(|(day_index, &day)| {
                                            let date = day.and_then(|d| NaiveDate::from_ymd_opt(year, month, d));
                                            let is_today = date == Some(today);
                                            let is_selected = props.selected_date.is_some_and(|selected| date == Some(selected));
                                            let is_outside_month = day.is_none();

                                            let day_classes = merge_classes(&[
                                                classes::DAY,
                                                if is_today { classes::DAY_TODAY } else { "" },
                                                if is_selected { classes::DAY_SELECTED } else { "" },
                                                if is_outside_month { classes::DAY_OUTSIDE } else { "" },
                                            ]);

                                            html! {
                                                <td key={day_index} class={day_classes} role="presentation">
                                                    {
                                                        if let Some(d) = day {
                                                            let onclick = props.on_date_click.clone();
                                                            let click_date = NaiveDate::from_ymd_opt(year, month, d);
                                                            html! {
                                                                <button
                                                                    class={classes::DAY_BUTTON}
                                                                    role="gridcell"
                                                                    tabindex="-1"
                                                                    type="button"
                                                                    onclick={Callback::from(move |_| {
                                                                        if let Some(date) = click_date {
                                                                            onclick.emit(date);
                                                                        }
                                                                    })}
                                                                >
                                                                    { d }
                                                                </button>
                                                            }
                                                        } else {
                                                            html! { <div class="h-9 w-9">{""}</div> }
                                                        }
                                                    }
                                                </td>
                                            }
                                        }) }
                                    </tr>
                                }
                            }) }
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
