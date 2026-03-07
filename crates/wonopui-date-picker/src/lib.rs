//! Date Picker component for WonopUI.
//!
//! A date selection component that combines an input with a calendar popup.

use chrono::prelude::*;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the DatePicker component (shadcn v4)
pub mod classes {
    /// Root container
    pub const CONTAINER: &str = "relative inline-block";
    /// Trigger button - premium input styling
    pub const INPUT: &str = "flex h-9 w-full items-center gap-2 rounded-md border border-zinc-200 bg-transparent px-3 py-2 text-sm shadow-xs transition-all duration-200 placeholder:text-zinc-500 focus-visible:border-zinc-950 focus-visible:ring-zinc-950/50 focus-visible:ring-[3px] focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:border-zinc-800 dark:placeholder:text-zinc-400 dark:focus-visible:border-zinc-300 dark:focus-visible:ring-zinc-300/50 text-zinc-900 dark:text-zinc-50 cursor-pointer [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
    /// Calendar popup - premium popover style
    pub const CALENDAR_POPUP: &str = "absolute z-50 mt-1.5 overflow-hidden rounded-xl border border-zinc-200 bg-white p-3 text-zinc-950 shadow-lg dark:border-zinc-800 dark:bg-zinc-950 dark:text-zinc-50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";
    /// Calendar header
    pub const CALENDAR_HEADER: &str = "relative flex items-center justify-center pt-1 mb-4";
    /// Navigation button
    pub const NAV_BUTTON: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 border border-zinc-200 bg-white shadow-xs hover:bg-zinc-50 hover:text-zinc-900 dark:border-zinc-800 dark:bg-zinc-950 dark:hover:bg-zinc-800 dark:hover:text-zinc-50 size-7 rounded-md focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none";
    /// Month/year display
    pub const MONTH_YEAR: &str = "text-sm font-semibold text-zinc-900 dark:text-zinc-50 tracking-tight";
    /// Weekdays header row
    pub const WEEKDAYS: &str = "grid grid-cols-7 gap-1 mb-1";
    /// Individual weekday
    pub const WEEKDAY: &str = "text-center text-xs font-normal text-zinc-500 dark:text-zinc-400 select-none";
    /// Days grid
    pub const DAYS: &str = "grid grid-cols-7 gap-1";
    /// Regular day button
    pub const DAY: &str = "size-8 flex items-center justify-center text-sm rounded-md hover:bg-zinc-100 dark:hover:bg-zinc-800 cursor-pointer text-zinc-900 dark:text-zinc-50 transition-all duration-200 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none";
    /// Selected day
    pub const DAY_SELECTED: &str = "size-8 flex items-center justify-center text-sm rounded-md bg-zinc-900 dark:bg-zinc-50 text-zinc-50 dark:text-zinc-900 cursor-pointer shadow-sm hover:bg-zinc-800 dark:hover:bg-zinc-200 transition-all duration-200";
    /// Today indicator
    pub const DAY_TODAY: &str = "size-8 flex items-center justify-center text-sm rounded-md bg-zinc-100 dark:bg-zinc-800 cursor-pointer text-zinc-900 dark:text-zinc-50 font-semibold";
    /// Calendar icon
    pub const ICON: &str = "size-4 text-zinc-500 dark:text-zinc-400";
}

#[derive(Properties, PartialEq)]
pub struct DatePickerProps {
    #[prop_or_default]
    pub value: Option<NaiveDate>,
    #[prop_or_default]
    pub onchange: Callback<NaiveDate>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProps) -> Html {
    let is_open = use_state(|| false);
    let selected_date = use_state(|| props.value);
    let today = Local::now().date_naive();
    let viewing_year = use_state(|| props.value.map(|d| d.year()).unwrap_or(today.year()));
    let viewing_month = use_state(|| props.value.map(|d| d.month()).unwrap_or(today.month()));

    // Sync with prop changes
    {
        let selected_date = selected_date.clone();
        let viewing_year = viewing_year.clone();
        let viewing_month = viewing_month.clone();
        let prop_value = props.value;
        use_effect_with(prop_value, move |value| {
            selected_date.set(*value);
            if let Some(date) = value {
                viewing_year.set(date.year());
                viewing_month.set(date.month());
            }
            || ()
        });
    }

    let toggle_calendar = {
        let is_open = is_open.clone();
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                is_open.set(!*is_open);
            }
        })
    };

    let on_prev_month = {
        let viewing_year = viewing_year.clone();
        let viewing_month = viewing_month.clone();
        Callback::from(move |_: MouseEvent| {
            if *viewing_month == 1 {
                viewing_year.set(*viewing_year - 1);
                viewing_month.set(12);
            } else {
                viewing_month.set(*viewing_month - 1);
            }
        })
    };

    let on_next_month = {
        let viewing_year = viewing_year.clone();
        let viewing_month = viewing_month.clone();
        Callback::from(move |_: MouseEvent| {
            if *viewing_month == 12 {
                viewing_year.set(*viewing_year + 1);
                viewing_month.set(1);
            } else {
                viewing_month.set(*viewing_month + 1);
            }
        })
    };

    let on_date_click = {
        let selected_date = selected_date.clone();
        let is_open = is_open.clone();
        let onchange = props.onchange.clone();
        let year = *viewing_year;
        let month = *viewing_month;
        Callback::from(move |day: u32| {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                selected_date.set(Some(date));
                is_open.set(false);
                onchange.emit(date);
            }
        })
    };

    // Calculate days in current viewing month
    let year = *viewing_year;
    let month = *viewing_month;
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    let days_in_month = next_month
        .map(|nm| nm.signed_duration_since(first_day).num_days() as u32)
        .unwrap_or(31);
    let first_weekday = first_day.weekday().num_days_from_sunday();

    let display_value = selected_date
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Select date".to_string());

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            <input
                type="text"
                class={classes::INPUT}
                value={display_value}
                placeholder={placeholder}
                readonly=true
                onclick={toggle_calendar}
                disabled={props.disabled}
            />

            if *is_open {
                <div class={classes::CALENDAR_POPUP}>
                    <div class={classes::CALENDAR_HEADER}>
                        <button class={classes::NAV_BUTTON} onclick={on_prev_month}>
                            {"←"}
                        </button>
                        <span class={classes::MONTH_YEAR}>
                            { format!("{} {}", first_day.format("%B"), year) }
                        </span>
                        <button class={classes::NAV_BUTTON} onclick={on_next_month}>
                            {"→"}
                        </button>
                    </div>
                    <div class={classes::WEEKDAYS}>
                        <div class={classes::WEEKDAY}>{"Su"}</div>
                        <div class={classes::WEEKDAY}>{"Mo"}</div>
                        <div class={classes::WEEKDAY}>{"Tu"}</div>
                        <div class={classes::WEEKDAY}>{"We"}</div>
                        <div class={classes::WEEKDAY}>{"Th"}</div>
                        <div class={classes::WEEKDAY}>{"Fr"}</div>
                        <div class={classes::WEEKDAY}>{"Sa"}</div>
                    </div>
                    <div class={classes::DAYS}>
                        // Empty cells for days before the first of the month
                        { for (0..first_weekday).map(|_| html! { <div /> }) }
                        // Days of the month
                        { for (1..=days_in_month).map(|day| {
                            let date = NaiveDate::from_ymd_opt(year, month, day);
                            let is_selected = *selected_date == date;
                            let is_today = date == Some(today);
                            let on_click = on_date_click.clone();

                            let day_class = if is_selected {
                                classes::DAY_SELECTED
                            } else if is_today {
                                classes::DAY_TODAY
                            } else {
                                classes::DAY
                            };

                            html! {
                                <div class={day_class} onclick={Callback::from(move |_| on_click.emit(day))}>
                                    { day }
                                </div>
                            }
                        }) }
                    </div>
                </div>
            }
        </div>
    }
}
