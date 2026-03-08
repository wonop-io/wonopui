//! Data Display blocks - Stats, Description Lists, Calendars
//!
//! Reimplementation of blocks_old/stats, blocks_old/description_lists, calendars
//! using wonopui components with shadcn styling.

use yew::prelude::*;
use chrono::prelude::*;
use wonopui::*;
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent, CardDescription};
use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::Calendar;
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::progress::{Progress, ProgressVariant};
use crate::blocks::BlockPreview;

/// Data Display category page
#[function_component(DataDisplayBlocks)]
pub fn data_display_blocks() -> Html {
    html! {
        <Container class="py-12">
            // Header
            <div class="mb-8">
                <nav class="mb-4">
                    <a href="/blocks" class="text-sm text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">
                        {"← Back to Blocks"}
                    </a>
                </nav>
                <h1 class="text-3xl font-bold tracking-tight text-zinc-900 dark:text-white">
                    {"Data Display"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Stats, description lists, and data visualization patterns."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                // Stats section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Stats"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Stats Grid"
                            description="4-column stats grid with values and labels."
                            code={STATS_SIMPLE_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <StatsSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Stats with Trend Indicators"
                            description="Stats showing change direction with colored indicators."
                            code={STATS_TREND_CODE}
                            min_height={220}
                            isolate={true}
                        >
                            <StatsTrend />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Stats in Cards"
                            description="Individual stat cards with icons and descriptions."
                            code={STATS_CARDS_CODE}
                            min_height={250}
                            isolate={true}
                        >
                            <StatsCards />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Stats with Progress"
                            description="Stats showing progress towards goals."
                            code={STATS_PROGRESS_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <StatsProgress />
                        </BlockPreview>
                    </div>
                </div>
                
                // Description Lists section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Description Lists"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Stacked Description List"
                            description="Vertical list with terms and descriptions stacked."
                            code={DL_STACKED_CODE}
                            min_height={350}
                            isolate={true}
                        >
                            <DescriptionListStacked />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Two-Column Description List"
                            description="Side-by-side layout with terms and descriptions."
                            code={DL_TWO_COLUMN_CODE}
                            min_height={300}
                            isolate={true}
                        >
                            <DescriptionListTwoColumn />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Description List in Card"
                            description="User profile details displayed in a card."
                            code={DL_CARD_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <DescriptionListCard />
                        </BlockPreview>
                    </div>
                </div>
                
                // Calendars section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Calendars"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Calendar"
                            description="Basic calendar with month navigation."
                            code={CALENDAR_SIMPLE_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <CalendarSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Calendar with Selected Date"
                            description="Calendar with date selection callback."
                            code={CALENDAR_SELECTED_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <CalendarWithSelection />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Calendar in Card"
                            description="Calendar embedded in a card with header."
                            code={CALENDAR_CARD_CODE}
                            min_height={500}
                            isolate={true}
                        >
                            <CalendarInCard />
                        </BlockPreview>
                    </div>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Stats Examples
// =============================================================================

#[function_component(StatsSimple)]
fn stats_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <dl class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Total Subscribers"}</dt>
                    <dd class="mt-1 text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"71,897"}</dd>
                </div>
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Avg. Open Rate"}</dt>
                    <dd class="mt-1 text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"58.16%"}</dd>
                </div>
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Avg. Click Rate"}</dt>
                    <dd class="mt-1 text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"24.57%"}</dd>
                </div>
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Revenue"}</dt>
                    <dd class="mt-1 text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"$405,091"}</dd>
                </div>
            </dl>
        </div>
    }
}

#[function_component(StatsTrend)]
fn stats_trend() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <dl class="grid grid-cols-1 gap-5 sm:grid-cols-3">
                // Positive trend
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Total Revenue"}</dt>
                    <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
                        <span class="text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"$45,231"}</span>
                        <span class="inline-flex items-baseline rounded-full px-2.5 py-0.5 text-sm font-medium bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400 md:mt-2 lg:mt-0">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-4 mr-0.5 shrink-0 self-center text-green-500">
                                <path fill-rule="evenodd" d="M10 17a.75.75 0 0 1-.75-.75V5.612L5.29 9.77a.75.75 0 0 1-1.08-1.04l5.25-5.5a.75.75 0 0 1 1.08 0l5.25 5.5a.75.75 0 1 1-1.08 1.04l-3.96-4.158V16.25A.75.75 0 0 1 10 17Z" clip-rule="evenodd" />
                            </svg>
                            {"12%"}
                        </span>
                    </dd>
                </div>
                // Negative trend
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Bounce Rate"}</dt>
                    <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
                        <span class="text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"32.4%"}</span>
                        <span class="inline-flex items-baseline rounded-full px-2.5 py-0.5 text-sm font-medium bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400 md:mt-2 lg:mt-0">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-4 mr-0.5 shrink-0 self-center text-red-500">
                                <path fill-rule="evenodd" d="M10 3a.75.75 0 0 1 .75.75v10.638l3.96-4.158a.75.75 0 1 1 1.08 1.04l-5.25 5.5a.75.75 0 0 1-1.08 0l-5.25-5.5a.75.75 0 1 1 1.08-1.04l3.96 4.158V3.75A.75.75 0 0 1 10 3Z" clip-rule="evenodd" />
                            </svg>
                            {"4.1%"}
                        </span>
                    </dd>
                </div>
                // Neutral
                <div class="relative overflow-hidden rounded-lg bg-white dark:bg-zinc-900 px-4 py-5 shadow-sm border border-zinc-200 dark:border-zinc-800 sm:px-6">
                    <dt class="truncate text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Avg. Session"}</dt>
                    <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
                        <span class="text-3xl font-semibold tracking-tight text-zinc-900 dark:text-white">{"4m 23s"}</span>
                        <span class="inline-flex items-baseline rounded-full px-2.5 py-0.5 text-sm font-medium bg-zinc-100 text-zinc-800 dark:bg-zinc-800 dark:text-zinc-300 md:mt-2 lg:mt-0">
                            {"0%"}
                        </span>
                    </dd>
                </div>
            </dl>
        </div>
    }
}

#[function_component(StatsCards)]
fn stats_cards() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8">
            <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                <Card>
                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle class="text-sm font-medium">{"Total Revenue"}</CardTitle>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-4 text-zinc-400">
                            <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
                        </svg>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold text-zinc-900 dark:text-white">{"$45,231.89"}</div>
                        <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
                            {"+20.1% from last month"}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle class="text-sm font-medium">{"Subscriptions"}</CardTitle>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-4 text-zinc-400">
                            <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
                            <circle cx="9" cy="7" r="4" />
                            <path d="M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75" />
                        </svg>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold text-zinc-900 dark:text-white">{"+2,350"}</div>
                        <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
                            {"+180.1% from last month"}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle class="text-sm font-medium">{"Active Now"}</CardTitle>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-4 text-zinc-400">
                            <path d="M22 12h-4l-3 9L9 3l-3 9H2" />
                        </svg>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold text-zinc-900 dark:text-white">{"+573"}</div>
                        <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
                            {"+201 since last hour"}
                        </p>
                    </CardContent>
                </Card>
            </div>
        </div>
    }
}

#[function_component(StatsProgress)]
fn stats_progress() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
                <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-6">
                    <Progress 
                        value={76.0}
                        label="Storage Used"
                        show_value={true}
                    />
                    <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">{"76GB of 100GB used"}</p>
                </div>
                
                <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-6">
                    <Progress 
                        value={89.0}
                        label="Monthly Goal"
                        show_value={true}
                        variant={ProgressVariant::Success}
                    />
                    <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">{"$8,900 of $10,000 target"}</p>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// Calendar Examples
// =============================================================================

#[function_component(CalendarSimple)]
fn calendar_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8 flex items-center justify-center">
            <Calendar />
        </div>
    }
}

#[function_component(CalendarWithSelection)]
fn calendar_with_selection() -> Html {
    let selected_date = use_state(|| None::<NaiveDate>);
    
    let on_date_click = {
        let selected_date = selected_date.clone();
        Callback::from(move |date: NaiveDate| {
            selected_date.set(Some(date));
        })
    };
    
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="flex flex-col items-center gap-4">
                <Calendar 
                    selected_date={*selected_date}
                    on_date_click={on_date_click}
                />
                <p class="text-sm text-zinc-600 dark:text-zinc-400">
                    if let Some(date) = *selected_date {
                        {format!("Selected: {}", date.format("%B %d, %Y"))}
                    } else {
                        {"Click a date to select it"}
                    }
                </p>
            </div>
        </div>
    }
}

#[function_component(CalendarInCard)]
fn calendar_in_card() -> Html {
    let selected_date = use_state(|| None::<NaiveDate>);
    
    let on_date_click = {
        let selected_date = selected_date.clone();
        Callback::from(move |date: NaiveDate| {
            selected_date.set(Some(date));
        })
    };
    
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8 flex items-center justify-center">
            <Card class="w-fit">
                <CardHeader>
                    <CardTitle>{"Schedule Meeting"}</CardTitle>
                    <CardDescription>{"Select a date for your meeting"}</CardDescription>
                </CardHeader>
                <CardContent class="flex flex-col items-center">
                    <Calendar 
                        selected_date={*selected_date}
                        on_date_click={on_date_click}
                        class="border-0 shadow-none p-0"
                    />
                    if let Some(date) = *selected_date {
                        <div class="mt-4 pt-4 border-t border-zinc-200 dark:border-zinc-800 w-full">
                            <p class="text-sm text-zinc-600 dark:text-zinc-300 text-center">
                                {format!("Meeting scheduled for {}", date.format("%B %d, %Y"))}
                            </p>
                            <div class="mt-3 flex gap-2 justify-center">
                                <Button variant={ButtonVariant::Default}>{"Confirm"}</Button>
                                <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                            </div>
                        </div>
                    }
                </CardContent>
            </Card>
        </div>
    }
}

// =============================================================================
// Description List Examples
// =============================================================================

#[function_component(DescriptionListStacked)]
fn description_list_stacked() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="px-4 sm:px-0">
                <h3 class="text-base font-semibold leading-7 text-zinc-900 dark:text-white">{"Applicant Information"}</h3>
                <p class="mt-1 max-w-2xl text-sm leading-6 text-zinc-500 dark:text-zinc-400">{"Personal details and application."}</p>
            </div>
            <div class="mt-6 border-t border-zinc-200 dark:border-zinc-800">
                <dl class="divide-y divide-zinc-200 dark:divide-zinc-800">
                    <div class="px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                        <dt class="text-sm font-medium leading-6 text-zinc-900 dark:text-white">{"Full name"}</dt>
                        <dd class="mt-1 text-sm leading-6 text-zinc-500 dark:text-zinc-400 sm:col-span-2 sm:mt-0">{"Margot Foster"}</dd>
                    </div>
                    <div class="px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                        <dt class="text-sm font-medium leading-6 text-zinc-900 dark:text-white">{"Application for"}</dt>
                        <dd class="mt-1 text-sm leading-6 text-zinc-500 dark:text-zinc-400 sm:col-span-2 sm:mt-0">{"Backend Developer"}</dd>
                    </div>
                    <div class="px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                        <dt class="text-sm font-medium leading-6 text-zinc-900 dark:text-white">{"Email address"}</dt>
                        <dd class="mt-1 text-sm leading-6 text-zinc-500 dark:text-zinc-400 sm:col-span-2 sm:mt-0">{"margotfoster@example.com"}</dd>
                    </div>
                    <div class="px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                        <dt class="text-sm font-medium leading-6 text-zinc-900 dark:text-white">{"Salary expectation"}</dt>
                        <dd class="mt-1 text-sm leading-6 text-zinc-500 dark:text-zinc-400 sm:col-span-2 sm:mt-0">{"$120,000"}</dd>
                    </div>
                    <div class="px-4 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                        <dt class="text-sm font-medium leading-6 text-zinc-900 dark:text-white">{"About"}</dt>
                        <dd class="mt-1 text-sm leading-6 text-zinc-500 dark:text-zinc-400 sm:col-span-2 sm:mt-0">
                            {"Fugiat ipsum ipsum deserunt culpa aute sint do nostrud anim incididunt cillum culpa consequat."}
                        </dd>
                    </div>
                </dl>
            </div>
        </div>
    }
}

#[function_component(DescriptionListTwoColumn)]
fn description_list_two_column() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                <div class="border-l-2 border-zinc-200 dark:border-zinc-700 pl-4">
                    <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Company"}</dt>
                    <dd class="mt-1 text-sm text-zinc-900 dark:text-white">{"Acme Corporation"}</dd>
                </div>
                <div class="border-l-2 border-zinc-200 dark:border-zinc-700 pl-4">
                    <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Industry"}</dt>
                    <dd class="mt-1 text-sm text-zinc-900 dark:text-white">{"Technology"}</dd>
                </div>
                <div class="border-l-2 border-zinc-200 dark:border-zinc-700 pl-4">
                    <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Location"}</dt>
                    <dd class="mt-1 text-sm text-zinc-900 dark:text-white">{"San Francisco, CA"}</dd>
                </div>
                <div class="border-l-2 border-zinc-200 dark:border-zinc-700 pl-4">
                    <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Employees"}</dt>
                    <dd class="mt-1 text-sm text-zinc-900 dark:text-white">{"500-1000"}</dd>
                </div>
                <div class="border-l-2 border-zinc-200 dark:border-zinc-700 pl-4">
                    <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Founded"}</dt>
                    <dd class="mt-1 text-sm text-zinc-900 dark:text-white">{"2015"}</dd>
                </div>
                <div class="border-l-2 border-zinc-200 dark:border-zinc-700 pl-4">
                    <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">{"Website"}</dt>
                    <dd class="mt-1 text-sm text-zinc-900 dark:text-white">
                        <span class="text-blue-600 dark:text-blue-400">{"www.acme.com"}</span>
                    </dd>
                </div>
            </dl>
        </div>
    }
}

#[function_component(DescriptionListCard)]
fn description_list_card() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8 flex items-center justify-center">
            <Card class="w-full max-w-lg">
                <CardHeader>
                    <div class="flex items-center space-x-4">
                        <div class="size-12 rounded-full bg-zinc-200 dark:bg-zinc-800 flex items-center justify-center">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 text-zinc-600 dark:text-zinc-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" />
                            </svg>
                        </div>
                        <div>
                            <CardTitle>{"John Smith"}</CardTitle>
                            <CardDescription>{"Senior Product Designer"}</CardDescription>
                        </div>
                    </div>
                </CardHeader>
                <CardContent>
                    <dl class="divide-y divide-zinc-200 dark:divide-zinc-800">
                        <div class="py-3 flex justify-between">
                            <dt class="text-sm text-zinc-500 dark:text-zinc-400">{"Email"}</dt>
                            <dd class="text-sm text-zinc-900 dark:text-white">{"john@example.com"}</dd>
                        </div>
                        <div class="py-3 flex justify-between">
                            <dt class="text-sm text-zinc-500 dark:text-zinc-400">{"Phone"}</dt>
                            <dd class="text-sm text-zinc-900 dark:text-white">{"+1 (555) 123-4567"}</dd>
                        </div>
                        <div class="py-3 flex justify-between">
                            <dt class="text-sm text-zinc-500 dark:text-zinc-400">{"Location"}</dt>
                            <dd class="text-sm text-zinc-900 dark:text-white">{"New York, NY"}</dd>
                        </div>
                        <div class="py-3 flex justify-between">
                            <dt class="text-sm text-zinc-500 dark:text-zinc-400">{"Status"}</dt>
                            <dd>
                                <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                            </dd>
                        </div>
                    </dl>
                </CardContent>
            </Card>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const STATS_SIMPLE_CODE: &str = r##"// Simple 4-column stats grid
html! {
    <dl class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        <div class="rounded-lg bg-white px-4 py-5 shadow-sm border">
            <dt class="text-sm font-medium text-zinc-500">{"Total Subscribers"}</dt>
            <dd class="mt-1 text-3xl font-semibold text-zinc-900">{"71,897"}</dd>
        </div>
        // ... more stat items
    </dl>
}
"##;

const STATS_TREND_CODE: &str = r##"// Stats with trend indicators
html! {
    <div class="rounded-lg bg-white px-4 py-5 shadow-sm border">
        <dt class="text-sm font-medium text-zinc-500">{"Total Revenue"}</dt>
        <dd class="mt-1 flex items-baseline justify-between">
            <span class="text-3xl font-semibold">{"$45,231"}</span>
            <span class="inline-flex items-baseline rounded-full px-2.5 py-0.5 text-sm font-medium bg-green-100 text-green-800">
                // Arrow up icon
                {"12%"}
            </span>
        </dd>
    </div>
}
"##;

const STATS_CARDS_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent};

html! {
    <Card>
        <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">{"Total Revenue"}</CardTitle>
            // Icon
        </CardHeader>
        <CardContent>
            <div class="text-2xl font-bold">{"$45,231.89"}</div>
            <p class="text-xs text-zinc-500">{"+20.1% from last month"}</p>
        </CardContent>
    </Card>
}
"##;

const STATS_PROGRESS_CODE: &str = r##"use wonopui::wonopui_progress::{Progress, ProgressVariant};

html! {
    <div class="rounded-lg border p-6">
        <Progress 
            value={76.0}
            label="Storage Used"
            show_value={true}
        />
        <p class="mt-2 text-sm text-zinc-500">{"76GB of 100GB used"}</p>
    </div>
    
    <div class="rounded-lg border p-6">
        <Progress 
            value={89.0}
            label="Monthly Goal"
            show_value={true}
            variant={ProgressVariant::Success}
        />
        <p class="mt-2 text-sm text-zinc-500">{"$8,900 of $10,000 target"}</p>
    </div>
}
"##;

const DL_STACKED_CODE: &str = r##"// Stacked description list
html! {
    <dl class="divide-y divide-zinc-200">
        <div class="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
            <dt class="text-sm font-medium text-zinc-900">{"Full name"}</dt>
            <dd class="mt-1 text-sm text-zinc-500 sm:col-span-2 sm:mt-0">
                {"Margot Foster"}
            </dd>
        </div>
        // ... more items
    </dl>
}
"##;

const DL_TWO_COLUMN_CODE: &str = r##"// Two-column description list
html! {
    <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
        <div class="border-l-2 border-zinc-200 pl-4">
            <dt class="text-sm font-medium text-zinc-500">{"Company"}</dt>
            <dd class="mt-1 text-sm text-zinc-900">{"Acme Corporation"}</dd>
        </div>
        // ... more items
    </dl>
}
"##;

const DL_CARD_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_badge::{Badge, BadgeVariant};

html! {
    <Card>
        <CardHeader>
            <CardTitle>{"John Smith"}</CardTitle>
            <CardDescription>{"Senior Product Designer"}</CardDescription>
        </CardHeader>
        <CardContent>
            <dl class="divide-y divide-zinc-200">
                <div class="py-3 flex justify-between">
                    <dt class="text-sm text-zinc-500">{"Email"}</dt>
                    <dd class="text-sm text-zinc-900">{"john@example.com"}</dd>
                </div>
                <div class="py-3 flex justify-between">
                    <dt class="text-sm text-zinc-500">{"Status"}</dt>
                    <dd>
                        <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                    </dd>
                </div>
            </dl>
        </CardContent>
    </Card>
}
"##;

const CALENDAR_SIMPLE_CODE: &str = r##"use wonopui::wonopui_calendar::Calendar;

html! {
    <Calendar />
}
"##;

const CALENDAR_SELECTED_CODE: &str = r##"use wonopui::wonopui_calendar::Calendar;
use chrono::prelude::*;

let selected_date = use_state(|| None::<NaiveDate>);

let on_date_click = {
    let selected_date = selected_date.clone();
    Callback::from(move |date: NaiveDate| {
        selected_date.set(Some(date));
    })
};

html! {
    <div class="flex flex-col items-center gap-4">
        <Calendar 
            selected_date={*selected_date}
            on_date_click={on_date_click}
        />
        <p class="text-sm text-zinc-600">
            if let Some(date) = *selected_date {
                {format!("Selected: {}", date.format("%B %d, %Y"))}
            } else {
                {"Click a date to select it"}
            }
        </p>
    </div>
}
"##;

const CALENDAR_CARD_CODE: &str = r##"use wonopui::wonopui_calendar::Calendar;
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_button::{Button, ButtonVariant};
use chrono::prelude::*;

let selected_date = use_state(|| None::<NaiveDate>);

let on_date_click = {
    let selected_date = selected_date.clone();
    Callback::from(move |date: NaiveDate| {
        selected_date.set(Some(date));
    })
};

html! {
    <Card class="w-fit">
        <CardHeader>
            <CardTitle>{"Schedule Meeting"}</CardTitle>
            <CardDescription>{"Select a date for your meeting"}</CardDescription>
        </CardHeader>
        <CardContent class="flex flex-col items-center">
            <Calendar 
                selected_date={*selected_date}
                on_date_click={on_date_click}
                class="border-0 shadow-none p-0"
            />
            if let Some(date) = *selected_date {
                <div class="mt-4 pt-4 border-t w-full">
                    <p class="text-sm text-zinc-600 text-center">
                        {format!("Meeting scheduled for {}", date.format("%B %d, %Y"))}
                    </p>
                    <div class="mt-3 flex gap-2 justify-center">
                        <Button variant={ButtonVariant::Default}>{"Confirm"}</Button>
                        <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                    </div>
                </div>
            }
        </CardContent>
    </Card>
}
"##;
