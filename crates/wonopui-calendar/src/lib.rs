//! Calendar component for WonopUI.
//!
//! A date picker calendar component.

use chrono::prelude::*;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Calendar component (shadcn v4)
pub mod classes {
    /// Root container - premium rounded border with subtle shadow
    pub const CONTAINER: &str = "bg-background p-3 rounded-xl border border-zinc-200 dark:border-zinc-800 shadow-sm";
    /// Wrapper for multiple months layout
    pub const WRAPPER: &str = "flex flex-col sm:flex-row gap-4";
    /// Header containing nav and month display
    pub const HEADER: &str = "flex flex-col gap-4";
    /// Caption/title area with month navigation
    pub const CAPTION: &str = "relative flex items-center justify-center pt-1";
    /// Month and year text display
    pub const CAPTION_LABEL: &str = "text-sm font-semibold text-zinc-900 dark:text-zinc-50 tracking-tight";
    /// Navigation container (prev/next buttons)
    pub const NAV: &str = "absolute right-0 flex items-center gap-1";
    /// Navigation button base styling
    pub const NAV_BUTTON: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 border border-zinc-200 bg-white shadow-xs hover:bg-zinc-50 hover:text-zinc-900 dark:border-zinc-800 dark:bg-zinc-950 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 size-7 rounded-md focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none opacity-50 hover:opacity-100";
    /// Previous month button
    pub const NAV_BUTTON_PREVIOUS: &str = "absolute left-0";
    /// Next month button
    pub const NAV_BUTTON_NEXT: &str = "";
    /// Calendar grid/table
    pub const GRID: &str = "w-full border-collapse";
    /// Table head
    pub const THEAD: &str = "";
    /// Weekdays row
    pub const WEEKDAYS: &str = "flex";
    /// Individual weekday header cell
    pub const WEEKDAY: &str = "text-zinc-500 dark:text-zinc-400 rounded-md w-9 font-normal text-xs text-center select-none";
    /// Table body
    pub const TBODY: &str = "";
    /// Week row
    pub const WEEK: &str = "flex w-full mt-2";
    /// Day cell container
    pub const DAY_CELL: &str = "relative h-9 w-9 text-center text-sm p-0 focus-within:relative focus-within:z-20";
    /// Day button base styling - premium feel with smooth transitions
    pub const DAY_BUTTON: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 h-9 w-9 p-0 rounded-md hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-900 dark:text-zinc-50 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none cursor-pointer";
    /// Today indicator - subtle accent
    pub const DAY_TODAY: &str = "bg-zinc-100 dark:bg-zinc-800 font-semibold";
    /// Selected day - primary emphasis with smooth hover
    pub const DAY_SELECTED: &str = "bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-200 shadow-sm";
    /// Days outside current month
    pub const DAY_OUTSIDE: &str = "text-zinc-400 dark:text-zinc-500 opacity-50 hover:opacity-70";
    /// Disabled day
    pub const DAY_DISABLED: &str = "text-zinc-400 dark:text-zinc-500 opacity-50 cursor-not-allowed hover:bg-transparent";
    /// Range start
    pub const DAY_RANGE_START: &str = "rounded-l-md";
    /// Range end
    pub const DAY_RANGE_END: &str = "rounded-r-md";
    /// Days in range (middle)
    pub const DAY_RANGE_MIDDLE: &str = "bg-zinc-100 dark:bg-zinc-800 rounded-none";
    /// Hidden day
    pub const DAY_HIDDEN: &str = "invisible";
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
        <div data-slot="calendar" class={container_class}>
            <div data-slot="calendar-months" class={classes::WRAPPER}>
                <div data-slot="calendar-month" class={classes::HEADER}>
                    <div data-slot="calendar-caption" class={classes::CAPTION}>
                        <span data-slot="calendar-caption-label" class={classes::CAPTION_LABEL}>
                            { format!("{} {}", current_month_first.format("%B"), year) }
                        </span>
                        <div data-slot="calendar-nav" class={classes::NAV}>
                            <button
                                data-slot="calendar-button-previous"
                                onclick={on_prev_month}
                                aria-label="Go to previous month"
                                class={merge_classes(&[classes::NAV_BUTTON, classes::NAV_BUTTON_PREVIOUS])}
                                type="button"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                    <path d="m15 18-6-6 6-6"/>
                                </svg>
                            </button>
                            <button
                                data-slot="calendar-button-next"
                                onclick={on_next_month}
                                aria-label="Go to next month"
                                class={merge_classes(&[classes::NAV_BUTTON, classes::NAV_BUTTON_NEXT])}
                                type="button"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                    <path d="m9 18 6-6-6-6"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                    <table data-slot="calendar-grid" class={classes::GRID} role="grid">
                        <thead data-slot="calendar-grid-head" class={classes::THEAD}>
                            <tr data-slot="calendar-weekdays" class={classes::WEEKDAYS}>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Sunday">{"Su"}</th>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Monday">{"Mo"}</th>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Tuesday">{"Tu"}</th>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Wednesday">{"We"}</th>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Thursday">{"Th"}</th>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Friday">{"Fr"}</th>
                                <th scope="col" data-slot="calendar-weekday" class={classes::WEEKDAY} aria-label="Saturday">{"Sa"}</th>
                            </tr>
                        </thead>
                        <tbody data-slot="calendar-grid-body" class={classes::TBODY} role="rowgroup">
                            { for weeks.iter().enumerate().map(|(week_index, week)| {
                                html! {
                                    <tr key={week_index} data-slot="calendar-week" class={classes::WEEK}>
                                        { for week.iter().enumerate().map(|(day_index, &day)| {
                                            let date = day.and_then(|d| NaiveDate::from_ymd_opt(year, month, d));
                                            let is_today = date == Some(today);
                                            let is_selected = props.selected_date.is_some_and(|selected| date == Some(selected));
                                            let is_outside_month = day.is_none();

                                            let cell_classes = classes::DAY_CELL;

                                            let button_classes = merge_classes(&[
                                                classes::DAY_BUTTON,
                                                if is_today { classes::DAY_TODAY } else { "" },
                                                if is_selected { classes::DAY_SELECTED } else { "" },
                                            ]);

                                            html! {
                                                <td key={day_index} data-slot="calendar-day-cell" class={cell_classes} role="presentation">
                                                    {
                                                        if let Some(d) = day {
                                                            let onclick = props.on_date_click.clone();
                                                            let click_date = NaiveDate::from_ymd_opt(year, month, d);
                                                            html! {
                                                                <button
                                                                    data-slot="calendar-day"
                                                                    data-today={is_today.then_some("true")}
                                                                    data-selected={is_selected.then_some("true")}
                                                                    class={button_classes}
                                                                    role="gridcell"
                                                                    tabindex={if is_selected { "0" } else { "-1" }}
                                                                    type="button"
                                                                    aria-selected={is_selected.to_string()}
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
                                                            html! { <div class={classes::DAY_OUTSIDE}></div> }
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
