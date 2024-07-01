use crate::config::BRANDGUIDE;
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
}

impl Default for CalendarProps {
    fn default() -> Self {
        let today = Local::today();
        CalendarProps {
            year: today.year(),
            month: today.month(),
            on_date_click: Callback::noop(),
        }
    }
}

#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let days_in_month = |year: i32, month: u32| -> u32 {
        match month {
            1 => 31,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => 0,
        }
    };

    let year = if props.year == 0 {
        Local::today().year()
    } else {
        props.year
    };
    let month = if props.month == 0 {
        Local::today().month()
    } else {
        props.month
    };

    let days = days_in_month(year, month);
    let onclick = props.on_date_click.clone();

    let first_day_of_month = NaiveDate::from_ymd(year, month, 1)
        .weekday()
        .num_days_from_sunday();
    let mut days_vec = vec![0; first_day_of_month as usize];
    days_vec.extend(1..=days);
    let weeks = days_vec.chunks(7);

    html! {
        <div class={classes!(BRANDGUIDE.calendar_container, "p-3", "rounded-md", "border")}>
            <div class="flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0">
                <div class="space-y-4 rdp-caption_start rdp-caption_end">
                    <div class="flex justify-center pt-1 relative items-center">
                        <div class="text-sm font-medium" aria-live="polite" role="presentation" id="react-day-picker-1">
                            { format!("{} - {}", year, month) }
                        </div>
                        <div class="space-x-1 flex items-center">
                            // Previous and next month buttons omitted for brevity
                        </div>
                    </div>
                    <table class={classes!(BRANDGUIDE.calendar_grid, "w-full", "border-collapse", "space-y-1")} role="grid" aria-labelledby="react-day-picker-1">
                        <thead class="rdp-head">
                            <tr class="flex">
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Sunday">{"Su"}</th>
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Monday">{"Mo"}</th>
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Tuesday">{"Tu"}</th>
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Wednesday">{"We"}</th>
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Thursday">{"Th"}</th>
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Friday">{"Fr"}</th>
                                <th scope="col" class="text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]" aria-label="Saturday">{"Sa"}</th>
                            </tr>
                        </thead>
                        <tbody class="rdp-tbody" role="rowgroup">
                            {

                                for weeks.map(move |week| html! {
                                <tr class="flex w-full mt-2">
                                    {
                                        for week.iter().map(|day| {
                                        let onclick = onclick.clone();
                                        let date = (props.year, props.month, *day);
                                        html! {
                                            <td class={classes!(BRANDGUIDE.calendar_day, "h-9", "w-9", "text-center", "text-sm", "p-0", "relative")} role="presentation">
                                                { if *day != 0 {
                                                    html! {
                                                        <button name="day" class="rdp-button_reset rdp-button inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9 p-0 font-normal aria-selected:opacity-100" role="gridcell" tabindex="-1" type="button" onclick={Callback::from(move |_| onclick.emit(date))}>
                                                            { day }
                                                        </button>
                                                    }
                                                } else {
                                                    html! { <div>{" "}</div> }
                                                }}
                                            </td>
                                        }
                                    })}
                                </tr>
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
