//! Headings blocks - Page headings, Section headings, Card headings
//!
//! Reimplementation of blocks_old/page_headings, section_headings, card_headings
//! using wonopui components with shadcn styling.

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_dropdown::{Dropdown, DropdownItem};
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent, CardDescription};
use wonopui::wonopui_typography::{H1, H2, H3};
use crate::blocks::BlockPreview;

/// Headings category page
#[function_component(HeadingsBlocks)]
pub fn headings_blocks() -> Html {
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
                    {"Headings"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Page headings, section headings, and card headings for structuring content."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                // Page Headings section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Page Headings"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Page Heading"
                            description="Page heading with title and action buttons."
                            code={PAGE_HEADING_SIMPLE_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <PageHeadingSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Page Heading with Metadata"
                            description="Page heading with badges, icons, and metadata."
                            code={PAGE_HEADING_METADATA_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <PageHeadingMetadata />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Profile Page Heading"
                            description="Page heading with cover image and avatar."
                            code={PAGE_HEADING_PROFILE_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <PageHeadingProfile />
                        </BlockPreview>
                    </div>
                </div>
                
                // Section Headings section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Section Headings"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Section Heading"
                            description="Basic section heading with optional description."
                            code={SECTION_HEADING_SIMPLE_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <SectionHeadingSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Centered Section Heading"
                            description="Centered section heading for marketing pages."
                            code={SECTION_HEADING_CENTERED_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <SectionHeadingCentered />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Section Heading with Action"
                            description="Section heading with action button."
                            code={SECTION_HEADING_ACTION_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <SectionHeadingAction />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Section Heading with Tabs"
                            description="Section heading with navigation tabs."
                            code={SECTION_HEADING_TABS_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <SectionHeadingTabs />
                        </BlockPreview>
                    </div>
                </div>
                
                // Card Headings section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Card Headings"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Card Heading"
                            description="Card with basic title and content."
                            code={CARD_HEADING_SIMPLE_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <CardHeadingSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Card with Description"
                            description="Card with title, description, and content."
                            code={CARD_HEADING_DESCRIPTION_CODE}
                            min_height={250}
                            isolate={true}
                        >
                            <CardHeadingDescription />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Card with Actions"
                            description="Card heading with action buttons."
                            code={CARD_HEADING_ACTIONS_CODE}
                            min_height={250}
                            isolate={true}
                        >
                            <CardHeadingActions />
                        </BlockPreview>
                    </div>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Page Heading Examples
// =============================================================================

#[function_component(PageHeadingSimple)]
fn page_heading_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="flex w-full md:space-x-4 flex-col-reverse md:flex-row md:justify-between md:items-center">
                <H2 class="text-zinc-900 dark:text-white">
                    {"Company Wiki Page"}
                </H2>
                <div class="flex mb-4 space-x-2 md:mb-0">
                    <Button variant={ButtonVariant::Secondary}>
                        {"Edit"}
                    </Button>
                    <Button variant={ButtonVariant::Primary}>
                        {"Publish"}
                    </Button>
                </div>
            </div>
        </div>
    }
}

#[function_component(PageHeadingMetadata)]
fn page_heading_metadata() -> Html {
    let more_items = vec![
        DropdownItem::Action {
            label: "API Documentation".to_string(),
            icon: None,
            onclick: Callback::noop(),
            disabled: false,
        },
        DropdownItem::Action {
            label: "Usage Examples".to_string(),
            icon: None,
            onclick: Callback::noop(),
            disabled: false,
        },
        DropdownItem::Action {
            label: "Support".to_string(),
            icon: None,
            onclick: Callback::noop(),
            disabled: false,
        },
    ];
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="lg:flex lg:items-start lg:justify-between w-full">
                <div class="min-w-0 flex-1">
                    <H2 class="text-2xl font-bold leading-7 text-zinc-900 dark:text-white sm:truncate sm:text-3xl sm:tracking-tight">
                        {"Deploy LLM API Endpoint"}
                    </H2>
                    <div class="mt-2 flex flex-col sm:flex-row sm:space-x-3 items-start sm:items-center">
                        <div class="flex space-x-2 mb-2 sm:mb-0">
                            <Badge variant={BadgeVariant::Warning}>{"llama-3"}</Badge>
                            <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                        </div>
                        <div class="flex items-center text-sm text-zinc-500 dark:text-zinc-400 space-x-2">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5" />
                            </svg>
                            <span>{"Last used: October 1, 2023"}</span>
                        </div>
                    </div>
                </div>
                <div class="mt-5 flex lg:ml-4 lg:mt-0 space-x-2">
                    <Button variant={ButtonVariant::Ghost}>{"Edit"}</Button>
                    <Button variant={ButtonVariant::Ghost}>{"Test"}</Button>
                    <Dropdown items={more_items}>
                        <Button variant={ButtonVariant::Ghost}>{"More"}</Button>
                    </Dropdown>
                    <Button variant={ButtonVariant::Primary}>{"Deploy"}</Button>
                </div>
            </div>
        </div>
    }
}

#[function_component(PageHeadingProfile)]
fn page_heading_profile() -> Html {
    let menu_items = vec![
        DropdownItem::Action {
            label: "Edit Profile".to_string(),
            icon: None,
            onclick: Callback::noop(),
            disabled: false,
        },
        DropdownItem::Action {
            label: "Settings".to_string(),
            icon: None,
            onclick: Callback::noop(),
            disabled: false,
        },
        DropdownItem::Action {
            label: "Sign Out".to_string(),
            icon: None,
            onclick: Callback::noop(),
            disabled: false,
        },
    ];
    
    html! {
        <div class="w-full bg-white dark:bg-zinc-950">
            // Cover image
            <div>
                <img 
                    class="h-48 w-full object-cover lg:h-56" 
                    src="https://images.unsplash.com/photo-1444628838545-ac4016a5418a?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1950&q=80" 
                    alt="Cover"
                />
            </div>
            // Profile section
            <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
                <div class="-mt-12 sm:-mt-16 sm:flex sm:items-end sm:space-x-5">
                    <div class="flex">
                        <Avatar 
                            size={AvatarSize::Large} 
                            src="https://images.unsplash.com/photo-1463453091185-61582044d556?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80" 
                            alt="Profile"
                            class="ring-4 ring-white dark:ring-zinc-950"
                        />
                    </div>
                    <div class="mt-6 sm:flex sm:min-w-0 sm:flex-1 sm:items-center sm:justify-end sm:space-x-6 sm:pb-1">
                        <div class="mt-6 min-w-0 flex-1 sm:hidden md:block">
                            <H1 class="truncate text-2xl font-bold text-zinc-900 dark:text-white">
                                {"John Doe"}
                            </H1>
                            <p class="text-sm text-zinc-500 dark:text-zinc-400">{"@johndoe"}</p>
                        </div>
                        <div class="mt-6 flex flex-col justify-stretch space-y-3 sm:flex-row sm:space-x-4 sm:space-y-0">
                            <Button variant={ButtonVariant::Secondary}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4 mr-2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75" />
                                </svg>
                                {"Message"}
                            </Button>
                            <Dropdown items={menu_items} position={PopoverPosition::SouthEnd}>
                                <Button variant={ButtonVariant::Ghost}>
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                                    </svg>
                                </Button>
                            </Dropdown>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// Section Heading Examples
// =============================================================================

#[function_component(SectionHeadingSimple)]
fn section_heading_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="border-b border-zinc-200 dark:border-zinc-800 pb-5">
                <H3 class="text-lg font-semibold leading-6 text-zinc-900 dark:text-white">
                    {"Recent Activity"}
                </H3>
                <p class="mt-2 max-w-4xl text-sm text-zinc-500 dark:text-zinc-400">
                    {"Your recent activity across all projects and workspaces."}
                </p>
            </div>
        </div>
    }
}

#[function_component(SectionHeadingCentered)]
fn section_heading_centered() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="text-center">
                <Badge variant={BadgeVariant::Info} class="mb-4">{"New Feature"}</Badge>
                <H2 class="text-3xl font-bold tracking-tight text-zinc-900 dark:text-white sm:text-4xl">
                    {"Everything you need to deploy your app"}
                </H2>
                <p class="mx-auto mt-4 max-w-2xl text-lg text-zinc-500 dark:text-zinc-400">
                    {"Quis tellus eget adipiscing convallis sit sit eget aliquet quis. Suspendisse eget egestas a elementum pulvinar et feugiat blandit."}
                </p>
            </div>
        </div>
    }
}

#[function_component(SectionHeadingAction)]
fn section_heading_action() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="flex items-center justify-between border-b border-zinc-200 dark:border-zinc-800 pb-5">
                <div>
                    <H3 class="text-lg font-semibold leading-6 text-zinc-900 dark:text-white">
                        {"Job Postings"}
                    </H3>
                    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
                        {"Manage your open positions"}
                    </p>
                </div>
                <Button variant={ButtonVariant::Primary}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4 mr-2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                    {"Create Job"}
                </Button>
            </div>
        </div>
    }
}

#[function_component(SectionHeadingTabs)]
fn section_heading_tabs() -> Html {
    let active_tab = use_state(|| "overview".to_string());
    
    let tabs = vec!["Overview", "Analytics", "Reports", "Notifications"];
    
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="border-b border-zinc-200 dark:border-zinc-800">
                <nav class="-mb-px flex space-x-8" aria-label="Tabs">
                    {tabs.iter().map(|tab| {
                        let tab_lower = tab.to_lowercase();
                        let is_active = *active_tab == tab_lower;
                        let on_click = {
                            let active_tab = active_tab.clone();
                            let tab_lower = tab_lower.clone();
                            Callback::from(move |_: MouseEvent| active_tab.set(tab_lower.clone()))
                        };
                        
                        let class = if is_active {
                            "border-zinc-950 text-zinc-950 dark:border-white dark:text-white whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium cursor-pointer"
                        } else {
                            "border-transparent text-zinc-500 hover:border-zinc-300 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium cursor-pointer"
                        };
                        
                        html! {
                            <span {class} onclick={on_click}>
                                {tab}
                            </span>
                        }
                    }).collect::<Html>()}
                </nav>
            </div>
        </div>
    }
}

// =============================================================================
// Card Heading Examples
// =============================================================================

#[function_component(CardHeadingSimple)]
fn card_heading_simple() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8 flex items-center justify-center">
            <Card class="w-full max-w-md">
                <CardHeader>
                    <CardTitle>{"Account Settings"}</CardTitle>
                </CardHeader>
                <CardContent>
                    <p class="text-sm text-zinc-500 dark:text-zinc-400">
                        {"Manage your account preferences and security settings."}
                    </p>
                </CardContent>
            </Card>
        </div>
    }
}

#[function_component(CardHeadingDescription)]
fn card_heading_description() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8 flex items-center justify-center">
            <Card class="w-full max-w-md">
                <CardHeader>
                    <CardTitle>{"Team Members"}</CardTitle>
                    <CardDescription>{"Invite your team members to collaborate."}</CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="flex -space-x-2">
                        <Avatar src="https://i.pravatar.cc/150?img=1" size={AvatarSize::Small} class="ring-2 ring-white dark:ring-zinc-950" />
                        <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Small} class="ring-2 ring-white dark:ring-zinc-950" />
                        <Avatar src="https://i.pravatar.cc/150?img=3" size={AvatarSize::Small} class="ring-2 ring-white dark:ring-zinc-950" />
                        <div class="flex items-center justify-center size-8 rounded-full bg-zinc-100 dark:bg-zinc-800 text-xs font-medium text-zinc-600 dark:text-zinc-300 ring-2 ring-white dark:ring-zinc-950">
                            {"+5"}
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

#[function_component(CardHeadingActions)]
fn card_heading_actions() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8 flex items-center justify-center">
            <Card class="w-full max-w-md">
                <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                    <div>
                        <CardTitle>{"Notifications"}</CardTitle>
                        <CardDescription>{"Configure how you receive notifications."}</CardDescription>
                    </div>
                    <Button variant={ButtonVariant::Ghost} class="h-8 w-8 p-0">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                        </svg>
                    </Button>
                </CardHeader>
                <CardContent>
                    <div class="flex items-center space-x-4 rounded-md border border-zinc-200 dark:border-zinc-800 p-4">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-500">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                        </svg>
                        <div class="flex-1 space-y-1">
                            <p class="text-sm font-medium leading-none text-zinc-900 dark:text-white">{"Push Notifications"}</p>
                            <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Send notifications to device."}</p>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const PAGE_HEADING_SIMPLE_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_typography::H2;

html! {
    <div class="flex justify-between items-center">
        <H2>{"Company Wiki Page"}</H2>
        <div class="flex space-x-2">
            <Button variant={ButtonVariant::Secondary}>{"Edit"}</Button>
            <Button variant={ButtonVariant::Primary}>{"Publish"}</Button>
        </div>
    </div>
}
"##;

const PAGE_HEADING_METADATA_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::wonopui_dropdown::{Dropdown, DropdownItem};
use wonopui::wonopui_typography::H2;

html! {
    <div class="lg:flex lg:items-start lg:justify-between">
        <div class="min-w-0 flex-1">
            <H2>{"Deploy LLM API Endpoint"}</H2>
            <div class="mt-2 flex items-center space-x-3">
                <Badge variant={BadgeVariant::Warning}>{"llama-3"}</Badge>
                <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                <span class="text-sm text-zinc-500">{"Last used: Oct 1, 2023"}</span>
            </div>
        </div>
        <div class="mt-5 flex lg:mt-0 space-x-2">
            <Button variant={ButtonVariant::Ghost}>{"Edit"}</Button>
            <Button variant={ButtonVariant::Ghost}>{"Test"}</Button>
            <Dropdown items={menu_items}>
                <Button variant={ButtonVariant::Ghost}>{"More"}</Button>
            </Dropdown>
            <Button variant={ButtonVariant::Primary}>{"Deploy"}</Button>
        </div>
    </div>
}
"##;

const PAGE_HEADING_PROFILE_CODE: &str = r##"use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_typography::H1;

html! {
    <div>
        // Cover image
        <img class="h-48 w-full object-cover" src="cover.jpg" alt="Cover" />
        
        // Profile section
        <div class="px-4 -mt-12 sm:flex sm:items-end">
            <Avatar 
                size={AvatarSize::Large} 
                src="profile.jpg"
                class="ring-4 ring-white"
            />
            <div class="mt-6 flex-1 flex justify-between">
                <div>
                    <H1>{"John Doe"}</H1>
                    <p class="text-sm text-zinc-500">{"@johndoe"}</p>
                </div>
                <Button variant={ButtonVariant::Secondary}>{"Message"}</Button>
            </div>
        </div>
    </div>
}
"##;

const SECTION_HEADING_SIMPLE_CODE: &str = r##"use wonopui::wonopui_typography::H3;

html! {
    <div class="border-b border-zinc-200 pb-5">
        <H3>{"Recent Activity"}</H3>
        <p class="mt-2 text-sm text-zinc-500">
            {"Your recent activity across all projects."}
        </p>
    </div>
}
"##;

const SECTION_HEADING_CENTERED_CODE: &str = r##"use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::wonopui_typography::H2;

html! {
    <div class="text-center">
        <Badge variant={BadgeVariant::Info}>{"New Feature"}</Badge>
        <H2 class="mt-4 text-3xl font-bold">
            {"Everything you need to deploy your app"}
        </H2>
        <p class="mx-auto mt-4 max-w-2xl text-lg text-zinc-500">
            {"Description text here..."}
        </p>
    </div>
}
"##;

const SECTION_HEADING_ACTION_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_typography::H3;

html! {
    <div class="flex items-center justify-between border-b pb-5">
        <div>
            <H3>{"Job Postings"}</H3>
            <p class="mt-1 text-sm text-zinc-500">{"Manage open positions"}</p>
        </div>
        <Button variant={ButtonVariant::Primary}>{"Create Job"}</Button>
    </div>
}
"##;

const SECTION_HEADING_TABS_CODE: &str = r##"// Tab navigation with active state
html! {
    <div class="border-b border-zinc-200">
        <nav class="-mb-px flex space-x-8">
            <a href="#" class="border-zinc-950 text-zinc-950 border-b-2 py-4 px-1 text-sm font-medium">
                {"Overview"}
            </a>
            <a href="#" class="border-transparent text-zinc-500 hover:text-zinc-700 border-b-2 py-4 px-1 text-sm font-medium">
                {"Analytics"}
            </a>
        </nav>
    </div>
}
"##;

const CARD_HEADING_SIMPLE_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent};

html! {
    <Card>
        <CardHeader>
            <CardTitle>{"Account Settings"}</CardTitle>
        </CardHeader>
        <CardContent>
            <p class="text-sm text-zinc-500">
                {"Manage your account preferences."}
            </p>
        </CardContent>
    </Card>
}
"##;

const CARD_HEADING_DESCRIPTION_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};

html! {
    <Card>
        <CardHeader>
            <CardTitle>{"Team Members"}</CardTitle>
            <CardDescription>{"Invite team members to collaborate."}</CardDescription>
        </CardHeader>
        <CardContent>
            <div class="flex -space-x-2">
                <Avatar src="avatar1.jpg" size={AvatarSize::Small} />
                <Avatar src="avatar2.jpg" size={AvatarSize::Small} />
                <Avatar src="avatar3.jpg" size={AvatarSize::Small} />
            </div>
        </CardContent>
    </Card>
}
"##;

const CARD_HEADING_ACTIONS_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <Card>
        <CardHeader class="flex flex-row items-center justify-between">
            <div>
                <CardTitle>{"Notifications"}</CardTitle>
                <CardDescription>{"Configure notifications."}</CardDescription>
            </div>
            <Button variant={ButtonVariant::Ghost}>
                // Menu icon
            </Button>
        </CardHeader>
        <CardContent>
            // Content here
        </CardContent>
    </Card>
}
"##;
