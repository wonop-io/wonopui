//! Elements blocks - Buttons, Alerts, Badges, Avatars, Button Groups, Dividers
//!
//! Reimplementation of blocks_old/buttons, blocks_old/alerts, etc.
//! using wonopui components with shadcn styling.

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_alert::{Alert, AlertVariant};
use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_group_button::{GroupButton, GroupButtonTrigger, GroupButtonDirection};
use wonopui::wonopui_divider::Divider;
use crate::blocks::BlockPreview;

/// Elements category page
#[function_component(ElementsBlocks)]
pub fn elements_blocks() -> Html {
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
                    {"Elements"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Basic UI elements: Buttons, Alerts, Badges, and Avatars."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                // Buttons section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Buttons"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Primary Button"
                            description="Primary action button with dark background."
                            code={BUTTON_PRIMARY_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <ButtonExample1 />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Secondary Button"
                            description="Secondary action button with muted background."
                            code={BUTTON_SECONDARY_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <ButtonExample2 />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Success Button"
                            description="Success button for positive actions."
                            code={BUTTON_SUCCESS_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <ButtonExample3 />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Danger Button"
                            description="Danger button for destructive actions."
                            code={BUTTON_DANGER_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <ButtonExample4 />
                        </BlockPreview>
                    </div>
                </div>
                
                // Alerts section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Alerts"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Success Alert"
                            description="Success feedback message."
                            code={ALERT_SUCCESS_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <AlertExample1 />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Error Alert"
                            description="Error feedback message."
                            code={ALERT_ERROR_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <AlertExample2 />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Warning Alert"
                            description="Warning feedback message."
                            code={ALERT_WARNING_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <AlertExample3 />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Info Alert"
                            description="Informational message."
                            code={ALERT_INFO_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <AlertExample4 />
                        </BlockPreview>
                    </div>
                </div>
                
                // Badges section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Badges"}</h2>
                    <BlockPreview 
                        title="Badge Variants"
                        description="Different badge styles for status and labels."
                        code={BADGES_CODE}
                        min_height={150}
                        isolate={true}
                    >
                        <BadgesExample />
                    </BlockPreview>
                </div>
                
                // Avatars section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Avatars"}</h2>
                    <BlockPreview 
                        title="Avatar Sizes"
                        description="Avatar component in different sizes."
                        code={AVATARS_CODE}
                        min_height={150}
                        isolate={true}
                    >
                        <AvatarsExample />
                    </BlockPreview>
                </div>
                
                // Button Groups section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Button Groups"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Button Group"
                            description="Toggle between mutually exclusive options."
                            code={BUTTON_GROUP_SIMPLE_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <ButtonGroupSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Button Group with Icons"
                            description="Button group with icon labels."
                            code={BUTTON_GROUP_ICONS_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <ButtonGroupIcons />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Vertical Button Group"
                            description="Vertically stacked button group."
                            code={BUTTON_GROUP_VERTICAL_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <ButtonGroupVertical />
                        </BlockPreview>
                    </div>
                </div>
                
                // Dividers section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Dividers"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Divider"
                            description="Basic horizontal divider line."
                            code={DIVIDER_SIMPLE_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <DividerSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Divider with Text"
                            description="Divider with centered text label."
                            code={DIVIDER_TEXT_CODE}
                            min_height={150}
                            isolate={true}
                        >
                            <DividerWithText />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Content Dividers"
                            description="Dividers between content sections."
                            code={DIVIDER_CONTENT_CODE}
                            min_height={300}
                            isolate={true}
                        >
                            <DividerContent />
                        </BlockPreview>
                    </div>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Button Examples - Reimplemented from blocks_old/buttons
// =============================================================================

#[function_component(ButtonExample1)]
fn button_example1() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <Button variant={ButtonVariant::Primary}>
                {"Primary Button"}
            </Button>
        </div>
    }
}

#[function_component(ButtonExample2)]
fn button_example2() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <Button variant={ButtonVariant::Secondary}>
                {"Secondary Button"}
            </Button>
        </div>
    }
}

#[function_component(ButtonExample3)]
fn button_example3() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <Button variant={ButtonVariant::Success}>
                {"Success Button"}
            </Button>
        </div>
    }
}

#[function_component(ButtonExample4)]
fn button_example4() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <Button variant={ButtonVariant::Danger}>
                {"Danger Button"}
            </Button>
        </div>
    }
}

// =============================================================================
// Alert Examples - Reimplemented from blocks_old/alerts
// =============================================================================

#[function_component(AlertExample1)]
fn alert_example1() -> Html {
    html! {
        <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
            <Alert variant={AlertVariant::Success}>
                {"This is a success alert!"}
            </Alert>
        </div>
    }
}

#[function_component(AlertExample2)]
fn alert_example2() -> Html {
    html! {
        <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
            <Alert variant={AlertVariant::Error}>
                {"This is an error alert!"}
            </Alert>
        </div>
    }
}

#[function_component(AlertExample3)]
fn alert_example3() -> Html {
    html! {
        <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
            <Alert variant={AlertVariant::Warning}>
                {"This is a warning alert!"}
            </Alert>
        </div>
    }
}

#[function_component(AlertExample4)]
fn alert_example4() -> Html {
    html! {
        <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
            <Alert variant={AlertVariant::Info}>
                {"This is an info alert!"}
            </Alert>
        </div>
    }
}

// =============================================================================
// Badge Examples
// =============================================================================

#[function_component(BadgesExample)]
fn badges_example() -> Html {
    html! {
        <div class="flex flex-wrap items-center justify-center gap-4 p-8">
            <Badge>{"Default"}</Badge>
            <Badge variant={BadgeVariant::Success}>{"Success"}</Badge>
            <Badge variant={BadgeVariant::Warning}>{"Warning"}</Badge>
            <Badge variant={BadgeVariant::Error}>{"Error"}</Badge>
            <Badge variant={BadgeVariant::Info}>{"Info"}</Badge>
        </div>
    }
}

// =============================================================================
// Avatar Examples
// =============================================================================

#[function_component(AvatarsExample)]
fn avatars_example() -> Html {
    html! {
        <div class="flex flex-wrap items-center justify-center gap-4 p-8">
            <Avatar src="https://i.pravatar.cc/150?img=1" size={AvatarSize::Small} />
            <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Medium} />
            <Avatar src="https://i.pravatar.cc/150?img=3" size={AvatarSize::Large} />
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const BUTTON_PRIMARY_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <div class="flex items-center justify-center p-8">
        <Button variant={ButtonVariant::Primary}>
            {"Primary Button"}
        </Button>
    </div>
}
"##;

const BUTTON_SECONDARY_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <div class="flex items-center justify-center p-8">
        <Button variant={ButtonVariant::Secondary}>
            {"Secondary Button"}
        </Button>
    </div>
}
"##;

const BUTTON_SUCCESS_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <div class="flex items-center justify-center p-8">
        <Button variant={ButtonVariant::Success}>
            {"Success Button"}
        </Button>
    </div>
}
"##;

const BUTTON_DANGER_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <div class="flex items-center justify-center p-8">
        <Button variant={ButtonVariant::Danger}>
            {"Danger Button"}
        </Button>
    </div>
}
"##;

const ALERT_SUCCESS_CODE: &str = r##"use wonopui::wonopui_alert::{Alert, AlertVariant};

html! {
    <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
        <Alert variant={AlertVariant::Success}>
            {"This is a success alert!"}
        </Alert>
    </div>
}
"##;

const ALERT_ERROR_CODE: &str = r##"use wonopui::wonopui_alert::{Alert, AlertVariant};

html! {
    <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
        <Alert variant={AlertVariant::Error}>
            {"This is an error alert!"}
        </Alert>
    </div>
}
"##;

const ALERT_WARNING_CODE: &str = r##"use wonopui::wonopui_alert::{Alert, AlertVariant};

html! {
    <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
        <Alert variant={AlertVariant::Warning}>
            {"This is a warning alert!"}
        </Alert>
    </div>
}
"##;

const ALERT_INFO_CODE: &str = r##"use wonopui::wonopui_alert::{Alert, AlertVariant};

html! {
    <div class="w-full p-8 flex items-center justify-center bg-white dark:bg-zinc-950">
        <Alert variant={AlertVariant::Info}>
            {"This is an info alert!"}
        </Alert>
    </div>
}
"##;

const BADGES_CODE: &str = r##"use wonopui::wonopui_badge::{Badge, BadgeVariant};

html! {
    <div class="flex flex-wrap items-center justify-center gap-4 p-8">
        <Badge>{"Default"}</Badge>
        <Badge variant={BadgeVariant::Success}>{"Success"}</Badge>
        <Badge variant={BadgeVariant::Warning}>{"Warning"}</Badge>
        <Badge variant={BadgeVariant::Error}>{"Error"}</Badge>
        <Badge variant={BadgeVariant::Info}>{"Info"}</Badge>
    </div>
}
"##;

const AVATARS_CODE: &str = r##"use wonopui::wonopui_avatar::{Avatar, AvatarSize};

html! {
    <div class="flex flex-wrap items-center justify-center gap-4 p-8">
        <Avatar src="https://i.pravatar.cc/150?img=1" size={AvatarSize::Small} />
        <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Medium} />
        <Avatar src="https://i.pravatar.cc/150?img=3" size={AvatarSize::Large} />
    </div>
}
"##;

// =============================================================================
// Button Group Examples
// =============================================================================

#[function_component(ButtonGroupSimple)]
fn button_group_simple() -> Html {
    html! {
        <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
            <GroupButton default_value="monthly">
                <GroupButtonTrigger value="daily">{"Daily"}</GroupButtonTrigger>
                <GroupButtonTrigger value="weekly">{"Weekly"}</GroupButtonTrigger>
                <GroupButtonTrigger value="monthly">{"Monthly"}</GroupButtonTrigger>
                <GroupButtonTrigger value="yearly">{"Yearly"}</GroupButtonTrigger>
            </GroupButton>
        </div>
    }
}

#[function_component(ButtonGroupIcons)]
fn button_group_icons() -> Html {
    html! {
        <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
            <GroupButton default_value="grid">
                <GroupButtonTrigger value="list">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
                    </svg>
                    {"List"}
                </GroupButtonTrigger>
                <GroupButtonTrigger value="grid">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z" />
                    </svg>
                    {"Grid"}
                </GroupButtonTrigger>
                <GroupButtonTrigger value="board">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z" />
                    </svg>
                    {"Board"}
                </GroupButtonTrigger>
            </GroupButton>
        </div>
    }
}

#[function_component(ButtonGroupVertical)]
fn button_group_vertical() -> Html {
    html! {
        <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
            <GroupButton default_value="startup" direction={GroupButtonDirection::Column}>
                <GroupButtonTrigger value="startup">{"Startup"}</GroupButtonTrigger>
                <GroupButtonTrigger value="business">{"Business"}</GroupButtonTrigger>
                <GroupButtonTrigger value="enterprise">{"Enterprise"}</GroupButtonTrigger>
            </GroupButton>
        </div>
    }
}

// =============================================================================
// Divider Examples
// =============================================================================

#[function_component(DividerSimple)]
fn divider_simple() -> Html {
    html! {
        <div class="w-full p-8 bg-white dark:bg-zinc-950">
            <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">{"Content above the divider"}</p>
            <Divider />
            <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-4">{"Content below the divider"}</p>
        </div>
    }
}

#[function_component(DividerWithText)]
fn divider_with_text() -> Html {
    html! {
        <div class="w-full p-8 bg-white dark:bg-zinc-950">
            <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">{"Content above"}</p>
            <Divider text="OR" />
            <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-4">{"Content below"}</p>
        </div>
    }
}

#[function_component(DividerContent)]
fn divider_content() -> Html {
    html! {
        <div class="w-full p-8 bg-white dark:bg-zinc-950">
            <div class="space-y-4">
                <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                    <h3 class="text-sm font-semibold text-zinc-900 dark:text-white">{"Section One"}</h3>
                    <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{"First section content goes here."}</p>
                </div>
                
                <Divider text="CONTINUE" />
                
                <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                    <h3 class="text-sm font-semibold text-zinc-900 dark:text-white">{"Section Two"}</h3>
                    <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{"Second section content goes here."}</p>
                </div>
                
                <Divider />
                
                <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                    <h3 class="text-sm font-semibold text-zinc-900 dark:text-white">{"Section Three"}</h3>
                    <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{"Third section content goes here."}</p>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// Button Group Code Constants
// =============================================================================

const BUTTON_GROUP_SIMPLE_CODE: &str = r##"use wonopui::wonopui_group_button::{GroupButton, GroupButtonTrigger};

html! {
    <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
        <GroupButton default_value="monthly">
            <GroupButtonTrigger value="daily">{"Daily"}</GroupButtonTrigger>
            <GroupButtonTrigger value="weekly">{"Weekly"}</GroupButtonTrigger>
            <GroupButtonTrigger value="monthly">{"Monthly"}</GroupButtonTrigger>
            <GroupButtonTrigger value="yearly">{"Yearly"}</GroupButtonTrigger>
        </GroupButton>
    </div>
}
"##;

const BUTTON_GROUP_ICONS_CODE: &str = r##"use wonopui::wonopui_group_button::{GroupButton, GroupButtonTrigger};

html! {
    <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
        <GroupButton default_value="grid">
            <GroupButtonTrigger value="list">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
                </svg>
                {"List"}
            </GroupButtonTrigger>
            <GroupButtonTrigger value="grid">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z" />
                </svg>
                {"Grid"}
            </GroupButtonTrigger>
            <GroupButtonTrigger value="board">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z" />
                </svg>
                {"Board"}
            </GroupButtonTrigger>
        </GroupButton>
    </div>
}
"##;

const BUTTON_GROUP_VERTICAL_CODE: &str = r##"use wonopui::wonopui_group_button::{GroupButton, GroupButtonTrigger, GroupButtonDirection};

html! {
    <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
        <GroupButton default_value="startup" direction={GroupButtonDirection::Column}>
            <GroupButtonTrigger value="startup">{"Startup"}</GroupButtonTrigger>
            <GroupButtonTrigger value="business">{"Business"}</GroupButtonTrigger>
            <GroupButtonTrigger value="enterprise">{"Enterprise"}</GroupButtonTrigger>
        </GroupButton>
    </div>
}
"##;

// =============================================================================
// Divider Code Constants
// =============================================================================

const DIVIDER_SIMPLE_CODE: &str = r##"use wonopui::wonopui_divider::Divider;

html! {
    <div class="w-full p-8 bg-white dark:bg-zinc-950">
        <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">{"Content above the divider"}</p>
        <Divider />
        <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-4">{"Content below the divider"}</p>
    </div>
}
"##;

const DIVIDER_TEXT_CODE: &str = r##"use wonopui::wonopui_divider::Divider;

html! {
    <div class="w-full p-8 bg-white dark:bg-zinc-950">
        <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">{"Content above"}</p>
        <Divider text="OR" />
        <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-4">{"Content below"}</p>
    </div>
}
"##;

const DIVIDER_CONTENT_CODE: &str = r##"use wonopui::wonopui_divider::Divider;

html! {
    <div class="w-full p-8 bg-white dark:bg-zinc-950">
        <div class="space-y-4">
            <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                <h3 class="text-sm font-semibold text-zinc-900 dark:text-white">{"Section One"}</h3>
                <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{"First section content goes here."}</p>
            </div>
            
            <Divider text="CONTINUE" />
            
            <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                <h3 class="text-sm font-semibold text-zinc-900 dark:text-white">{"Section Two"}</h3>
                <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{"Second section content goes here."}</p>
            </div>
            
            <Divider />
            
            <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                <h3 class="text-sm font-semibold text-zinc-900 dark:text-white">{"Section Three"}</h3>
                <p class="text-sm text-zinc-500 dark:text-zinc-400 mt-1">{"Third section content goes here."}</p>
            </div>
        </div>
    </div>
}
"##;
