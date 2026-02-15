//! Date Picker component for WonopUI.
//!
//! A date selection component that combines an input with a calendar popup.

use chrono::prelude::*;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the DatePicker component
pub mod classes {
    pub const CONTAINER: &str = "relative inline-block";
    pub const INPUT: &str = "w-full px-3 py-2 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-ring";
    pub const CALENDAR_POPUP: &str =
        "absolute z-50 mt-1 bg-background border rounded-md shadow-lg p-3";
    pub const CALENDAR_HEADER: &str = "flex justify-between items-center mb-2";
    pub const NAV_BUTTON: &str = "p-1 hover:bg-accent rounded";
    pub const MONTH_YEAR: &str = "text-sm font-medium";
    pub const WEEKDAYS: &str = "grid grid-cols-7 gap-1 mb-1";
    pub const WEEKDAY: &str = "text-center text-xs text-muted-foreground";
    pub const DAYS: &str = "grid grid-cols-7 gap-1";
    pub const DAY: &str =
        "w-8 h-8 flex items-center justify-center text-sm rounded hover:bg-accent cursor-pointer";
    pub const DAY_SELECTED: &str = "w-8 h-8 flex items-center justify-center text-sm rounded bg-primary text-primary-foreground cursor-pointer";
    pub const DAY_TODAY: &str =
        "w-8 h-8 flex items-center justify-center text-sm rounded bg-accent cursor-pointer";
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
