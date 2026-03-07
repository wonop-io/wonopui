//! Lists blocks - Stacked Lists, Grid Lists, Feeds, Tables
//!
//! Reimplementation of blocks_old/stacked_lists, grid_lists, feeds, tables
//! using wonopui components with shadcn styling.

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent, CardDescription};
use wonopui::wonopui_table::{Table, TableHead, TableBody, TableRow, TableHeadCell, TableCell, TableFooter};
use wonopui::wonopui_checkbox::Checkbox;
use crate::blocks::BlockPreview;

/// Lists category page
#[function_component(ListsBlocks)]
pub fn lists_blocks() -> Html {
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
                    {"Lists"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Stacked lists, grid lists, and activity feeds."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                // Stacked Lists section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Stacked Lists"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Stacked List"
                            description="Basic stacked list with dividers."
                            code={STACKED_SIMPLE_CODE}
                            min_height={300}
                            isolate={true}
                        >
                            <StackedListSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Stacked List with Avatars"
                            description="User list with avatars and metadata."
                            code={STACKED_AVATARS_CODE}
                            min_height={350}
                            isolate={true}
                        >
                            <StackedListAvatars />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Stacked List with Actions"
                            description="List items with action buttons."
                            code={STACKED_ACTIONS_CODE}
                            min_height={350}
                            isolate={true}
                        >
                            <StackedListActions />
                        </BlockPreview>
                    </div>
                </div>
                
                // Grid Lists section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Grid Lists"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Card Grid"
                            description="Responsive card grid layout."
                            code={GRID_CARDS_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <GridCards />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Image Grid"
                            description="Gallery-style image grid."
                            code={GRID_IMAGES_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <GridImages />
                        </BlockPreview>
                    </div>
                </div>
                
                // Feeds section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Feeds"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Activity Feed"
                            description="Timeline-style activity feed."
                            code={FEED_ACTIVITY_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <ActivityFeed />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Comment Thread"
                            description="Nested comment thread with replies."
                            code={FEED_COMMENTS_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <CommentThread />
                        </BlockPreview>
                    </div>
                </div>
                
                // Tables section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Tables"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Table"
                            description="Basic table with header and data rows."
                            code={TABLE_SIMPLE_CODE}
                            min_height={350}
                            isolate={true}
                        >
                            <TableSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Table with Actions"
                            description="Table with action buttons for each row."
                            code={TABLE_ACTIONS_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <TableWithActions />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Striped Table"
                            description="Table with alternating row colors."
                            code={TABLE_STRIPED_CODE}
                            min_height={350}
                            isolate={true}
                        >
                            <TableStriped />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Table with Checkboxes"
                            description="Selectable rows with checkboxes."
                            code={TABLE_CHECKBOX_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <TableWithCheckboxes />
                        </BlockPreview>
                    </div>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Stacked List Examples
// =============================================================================

#[function_component(StackedListSimple)]
fn stacked_list_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <ul role="list" class="divide-y divide-zinc-200 dark:divide-zinc-800">
                <li class="py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="min-w-0 flex-auto">
                            <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Project Alpha"}</p>
                            <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Updated 2 hours ago"}</p>
                        </div>
                        <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                    </div>
                </li>
                <li class="py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="min-w-0 flex-auto">
                            <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Project Beta"}</p>
                            <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Updated 5 hours ago"}</p>
                        </div>
                        <Badge variant={BadgeVariant::Warning}>{"In Progress"}</Badge>
                    </div>
                </li>
                <li class="py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="min-w-0 flex-auto">
                            <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Project Gamma"}</p>
                            <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Updated yesterday"}</p>
                        </div>
                        <Badge>{"Draft"}</Badge>
                    </div>
                </li>
                <li class="py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="min-w-0 flex-auto">
                            <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Project Delta"}</p>
                            <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Updated 3 days ago"}</p>
                        </div>
                        <Badge variant={BadgeVariant::Error}>{"Archived"}</Badge>
                    </div>
                </li>
            </ul>
        </div>
    }
}

#[function_component(StackedListAvatars)]
fn stacked_list_avatars() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <ul role="list" class="divide-y divide-zinc-200 dark:divide-zinc-800">
                <li class="flex items-center gap-x-4 py-4">
                    <Avatar src="https://i.pravatar.cc/150?img=1" size={AvatarSize::Medium} />
                    <div class="min-w-0 flex-auto">
                        <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Leslie Alexander"}</p>
                        <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"leslie.alexander@example.com"}</p>
                    </div>
                    <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                        <p class="text-sm leading-6 text-zinc-900 dark:text-white">{"Co-Founder / CEO"}</p>
                        <p class="mt-1 text-xs leading-5 text-zinc-500 dark:text-zinc-400">
                            {"Last seen "}
                            <span class="text-green-500">{"online"}</span>
                        </p>
                    </div>
                </li>
                <li class="flex items-center gap-x-4 py-4">
                    <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Medium} />
                    <div class="min-w-0 flex-auto">
                        <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Michael Foster"}</p>
                        <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"michael.foster@example.com"}</p>
                    </div>
                    <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                        <p class="text-sm leading-6 text-zinc-900 dark:text-white">{"Co-Founder / CTO"}</p>
                        <p class="mt-1 text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Last seen 3h ago"}</p>
                    </div>
                </li>
                <li class="flex items-center gap-x-4 py-4">
                    <Avatar src="https://i.pravatar.cc/150?img=3" size={AvatarSize::Medium} />
                    <div class="min-w-0 flex-auto">
                        <p class="text-sm font-semibold leading-6 text-zinc-900 dark:text-white">{"Dries Vincent"}</p>
                        <p class="mt-1 truncate text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"dries.vincent@example.com"}</p>
                    </div>
                    <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                        <p class="text-sm leading-6 text-zinc-900 dark:text-white">{"Business Relations"}</p>
                        <p class="mt-1 text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Last seen 1d ago"}</p>
                    </div>
                </li>
            </ul>
        </div>
    }
}

#[function_component(StackedListActions)]
fn stacked_list_actions() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <ul role="list" class="divide-y divide-zinc-200 dark:divide-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-800">
                <li class="flex items-center justify-between gap-x-4 px-4 py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="size-10 rounded-lg bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-600 dark:text-zinc-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
                            </svg>
                        </div>
                        <div class="min-w-0">
                            <p class="text-sm font-semibold text-zinc-900 dark:text-white">{"Documents"}</p>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400">{"12 files"}</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-x-2">
                        <Button variant={ButtonVariant::Ghost} class="h-8 px-3 text-xs">{"View"}</Button>
                        <Button variant={ButtonVariant::Ghost} class="h-8 px-3 text-xs">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                            </svg>
                        </Button>
                    </div>
                </li>
                <li class="flex items-center justify-between gap-x-4 px-4 py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="size-10 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-blue-600 dark:text-blue-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
                            </svg>
                        </div>
                        <div class="min-w-0">
                            <p class="text-sm font-semibold text-zinc-900 dark:text-white">{"Images"}</p>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400">{"47 files"}</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-x-2">
                        <Button variant={ButtonVariant::Ghost} class="h-8 px-3 text-xs">{"View"}</Button>
                        <Button variant={ButtonVariant::Ghost} class="h-8 px-3 text-xs">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                            </svg>
                        </Button>
                    </div>
                </li>
                <li class="flex items-center justify-between gap-x-4 px-4 py-4">
                    <div class="flex items-center gap-x-3">
                        <div class="size-10 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-green-600 dark:text-green-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                            </svg>
                        </div>
                        <div class="min-w-0">
                            <p class="text-sm font-semibold text-zinc-900 dark:text-white">{"Spreadsheets"}</p>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400">{"8 files"}</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-x-2">
                        <Button variant={ButtonVariant::Ghost} class="h-8 px-3 text-xs">{"View"}</Button>
                        <Button variant={ButtonVariant::Ghost} class="h-8 px-3 text-xs">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                            </svg>
                        </Button>
                    </div>
                </li>
            </ul>
        </div>
    }
}

// =============================================================================
// Grid List Examples
// =============================================================================

#[function_component(GridCards)]
fn grid_cards() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8">
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <Card>
                    <CardHeader>
                        <CardTitle class="text-base">{"Analytics Dashboard"}</CardTitle>
                        <CardDescription>{"Track your business metrics"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Real-time insights into your business performance with customizable charts."}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle class="text-base">{"User Management"}</CardTitle>
                        <CardDescription>{"Manage team access"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Add, remove, and manage user permissions across your organization."}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle class="text-base">{"API Integration"}</CardTitle>
                        <CardDescription>{"Connect your services"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Seamlessly integrate with third-party services using our REST API."}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle class="text-base">{"Billing"}</CardTitle>
                        <CardDescription>{"Manage subscriptions"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"View invoices, manage payment methods, and update your plan."}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle class="text-base">{"Security"}</CardTitle>
                        <CardDescription>{"Protect your data"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Configure two-factor authentication and manage security settings."}
                        </p>
                    </CardContent>
                </Card>
                
                <Card>
                    <CardHeader>
                        <CardTitle class="text-base">{"Support"}</CardTitle>
                        <CardDescription>{"Get help when needed"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Access documentation, submit tickets, and chat with our support team."}
                        </p>
                    </CardContent>
                </Card>
            </div>
        </div>
    }
}

#[function_component(GridImages)]
fn grid_images() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
                {(1..=8).map(|i| {
                    html! {
                        <div class="group relative aspect-square overflow-hidden rounded-lg bg-zinc-200 dark:bg-zinc-800">
                            <img 
                                src={format!("https://picsum.photos/seed/{}/300/300", i)}
                                alt={format!("Image {}", i)}
                                class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                            />
                            <div class="absolute inset-0 bg-black/0 group-hover:bg-black/40 transition-colors duration-300">
                                <div class="absolute bottom-0 left-0 right-0 p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                                    <p class="text-sm font-medium text-white">{format!("Image {}", i)}</p>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}

// =============================================================================
// Feed Examples
// =============================================================================

#[function_component(ActivityFeed)]
fn activity_feed() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <ul role="list" class="space-y-6">
                <li class="relative flex gap-x-4">
                    <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                        <div class="w-px bg-zinc-200 dark:bg-zinc-800"></div>
                    </div>
                    <div class="relative flex h-6 w-6 flex-none items-center justify-center bg-white dark:bg-zinc-950">
                        <div class="h-1.5 w-1.5 rounded-full bg-zinc-200 ring-1 ring-zinc-300 dark:bg-zinc-700 dark:ring-zinc-600"></div>
                    </div>
                    <p class="flex-auto py-0.5 text-sm leading-5 text-zinc-500 dark:text-zinc-400">
                        <span class="font-medium text-zinc-900 dark:text-white">{"Chelsea Hagon"}</span>
                        {" created the invoice."}
                    </p>
                    <span class="flex-none py-0.5 text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"7d ago"}</span>
                </li>
                
                <li class="relative flex gap-x-4">
                    <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                        <div class="w-px bg-zinc-200 dark:bg-zinc-800"></div>
                    </div>
                    <div class="relative flex h-6 w-6 flex-none items-center justify-center bg-white dark:bg-zinc-950">
                        <Avatar src="https://i.pravatar.cc/150?img=4" size={AvatarSize::Small} />
                    </div>
                    <div class="flex-auto rounded-md py-0.5">
                        <div class="text-sm leading-5 text-zinc-500 dark:text-zinc-400">
                            <span class="font-medium text-zinc-900 dark:text-white">{"Chelsea Hagon"}</span>
                            {" commented"}
                        </div>
                        <p class="mt-0.5 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Called client, they reassured me the check is in the mail."}
                        </p>
                    </div>
                    <span class="flex-none py-0.5 text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"3d ago"}</span>
                </li>
                
                <li class="relative flex gap-x-4">
                    <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                        <div class="w-px bg-zinc-200 dark:bg-zinc-800"></div>
                    </div>
                    <div class="relative flex h-6 w-6 flex-none items-center justify-center bg-white dark:bg-zinc-950">
                        <div class="h-1.5 w-1.5 rounded-full bg-zinc-200 ring-1 ring-zinc-300 dark:bg-zinc-700 dark:ring-zinc-600"></div>
                    </div>
                    <p class="flex-auto py-0.5 text-sm leading-5 text-zinc-500 dark:text-zinc-400">
                        <span class="font-medium text-zinc-900 dark:text-white">{"Alex Curren"}</span>
                        {" paid the invoice."}
                    </p>
                    <span class="flex-none py-0.5 text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"1d ago"}</span>
                </li>
                
                <li class="relative flex gap-x-4">
                    <div class="relative flex h-6 w-6 flex-none items-center justify-center bg-white dark:bg-zinc-950">
                        <div class="h-1.5 w-1.5 rounded-full bg-green-500 ring-1 ring-green-500"></div>
                    </div>
                    <p class="flex-auto py-0.5 text-sm leading-5 text-zinc-500 dark:text-zinc-400">
                        <span class="font-medium text-zinc-900 dark:text-white">{"Invoice"}</span>
                        {" marked as complete."}
                    </p>
                    <span class="flex-none py-0.5 text-xs leading-5 text-zinc-500 dark:text-zinc-400">{"Just now"}</span>
                </li>
            </ul>
        </div>
    }
}

#[function_component(CommentThread)]
fn comment_thread() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="space-y-6">
                // Parent comment
                <div class="flex gap-x-3">
                    <Avatar src="https://i.pravatar.cc/150?img=5" size={AvatarSize::Medium} />
                    <div class="flex-1">
                        <div class="flex items-center gap-x-2">
                            <span class="text-sm font-semibold text-zinc-900 dark:text-white">{"Whitney Francis"}</span>
                            <span class="text-xs text-zinc-500 dark:text-zinc-400">{"2 hours ago"}</span>
                        </div>
                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-300">
                            {"Ducimus quas delectus ad maxime totam doloribus reiciendis ex. Tempore dolorem maiores. Similique voluptatibus tempore non ut."}
                        </p>
                        <div class="mt-2 flex items-center gap-x-4">
                            <button class="text-xs font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Reply"}</button>
                            <button class="text-xs font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Like"}</button>
                        </div>
                        
                        // Nested reply
                        <div class="mt-4 ml-4 border-l-2 border-zinc-200 dark:border-zinc-800 pl-4 space-y-4">
                            <div class="flex gap-x-3">
                                <Avatar src="https://i.pravatar.cc/150?img=6" size={AvatarSize::Small} />
                                <div class="flex-1">
                                    <div class="flex items-center gap-x-2">
                                        <span class="text-sm font-semibold text-zinc-900 dark:text-white">{"Lindsay Walton"}</span>
                                        <span class="text-xs text-zinc-500 dark:text-zinc-400">{"1 hour ago"}</span>
                                    </div>
                                    <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-300">
                                        {"Et ut autem. Voluptatem eum dolores sint necessitatibus quos."}
                                    </p>
                                    <div class="mt-2 flex items-center gap-x-4">
                                        <button class="text-xs font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Reply"}</button>
                                        <button class="text-xs font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Like"}</button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                
                // Second parent comment
                <div class="flex gap-x-3">
                    <Avatar src="https://i.pravatar.cc/150?img=7" size={AvatarSize::Medium} />
                    <div class="flex-1">
                        <div class="flex items-center gap-x-2">
                            <span class="text-sm font-semibold text-zinc-900 dark:text-white">{"Leonard Krasner"}</span>
                            <span class="text-xs text-zinc-500 dark:text-zinc-400">{"30 minutes ago"}</span>
                        </div>
                        <p class="mt-1 text-sm text-zinc-600 dark:text-zinc-300">
                            {"Nihil cupiditate. Suscipit accusamus sint distinctio placeat."}
                        </p>
                        <div class="mt-2 flex items-center gap-x-4">
                            <button class="text-xs font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Reply"}</button>
                            <button class="text-xs font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Like"}</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// Table Examples
// =============================================================================

#[function_component(TableSimple)]
fn table_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <Table>
                <TableHead>
                    <TableRow head={true}>
                        <TableHeadCell>{"Name"}</TableHeadCell>
                        <TableHeadCell>{"Title"}</TableHeadCell>
                        <TableHeadCell>{"Email"}</TableHeadCell>
                        <TableHeadCell>{"Role"}</TableHeadCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell class="font-medium">{"Lindsay Walton"}</TableCell>
                        <TableCell>{"Front-end Developer"}</TableCell>
                        <TableCell>{"lindsay.walton@example.com"}</TableCell>
                        <TableCell>{"Member"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">{"Courtney Henry"}</TableCell>
                        <TableCell>{"Designer"}</TableCell>
                        <TableCell>{"courtney.henry@example.com"}</TableCell>
                        <TableCell>{"Admin"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">{"Tom Cook"}</TableCell>
                        <TableCell>{"Director of Product"}</TableCell>
                        <TableCell>{"tom.cook@example.com"}</TableCell>
                        <TableCell>{"Member"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">{"Whitney Francis"}</TableCell>
                        <TableCell>{"Copywriter"}</TableCell>
                        <TableCell>{"whitney.francis@example.com"}</TableCell>
                        <TableCell>{"Admin"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    }
}

#[function_component(TableWithActions)]
fn table_with_actions() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <Table>
                <TableHead>
                    <TableRow head={true}>
                        <TableHeadCell>{"Name"}</TableHeadCell>
                        <TableHeadCell>{"Status"}</TableHeadCell>
                        <TableHeadCell>{"Role"}</TableHeadCell>
                        <TableHeadCell class="text-right">{"Actions"}</TableHeadCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell>
                            <div class="flex items-center gap-3">
                                <Avatar src="https://i.pravatar.cc/150?img=1" size={AvatarSize::Small} />
                                <div>
                                    <div class="font-medium text-zinc-900 dark:text-white">{"Lindsay Walton"}</div>
                                    <div class="text-sm text-zinc-500">{"lindsay@example.com"}</div>
                                </div>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                        </TableCell>
                        <TableCell>{"Member"}</TableCell>
                        <TableCell class="text-right">
                            <Button variant={ButtonVariant::Ghost} class="h-8 px-2">{"Edit"}</Button>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <div class="flex items-center gap-3">
                                <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Small} />
                                <div>
                                    <div class="font-medium text-zinc-900 dark:text-white">{"Courtney Henry"}</div>
                                    <div class="text-sm text-zinc-500">{"courtney@example.com"}</div>
                                </div>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant={BadgeVariant::Warning}>{"Pending"}</Badge>
                        </TableCell>
                        <TableCell>{"Admin"}</TableCell>
                        <TableCell class="text-right">
                            <Button variant={ButtonVariant::Ghost} class="h-8 px-2">{"Edit"}</Button>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <div class="flex items-center gap-3">
                                <Avatar src="https://i.pravatar.cc/150?img=3" size={AvatarSize::Small} />
                                <div>
                                    <div class="font-medium text-zinc-900 dark:text-white">{"Tom Cook"}</div>
                                    <div class="text-sm text-zinc-500">{"tom@example.com"}</div>
                                </div>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant={BadgeVariant::Error}>{"Inactive"}</Badge>
                        </TableCell>
                        <TableCell>{"Member"}</TableCell>
                        <TableCell class="text-right">
                            <Button variant={ButtonVariant::Ghost} class="h-8 px-2">{"Edit"}</Button>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    }
}

#[function_component(TableStriped)]
fn table_striped() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <Table>
                <TableHead>
                    <TableRow head={true}>
                        <TableHeadCell>{"Invoice"}</TableHeadCell>
                        <TableHeadCell>{"Status"}</TableHeadCell>
                        <TableHeadCell>{"Method"}</TableHeadCell>
                        <TableHeadCell class="text-right">{"Amount"}</TableHeadCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow class="bg-zinc-50 dark:bg-zinc-900/50">
                        <TableCell class="font-medium">{"INV001"}</TableCell>
                        <TableCell>{"Paid"}</TableCell>
                        <TableCell>{"Credit Card"}</TableCell>
                        <TableCell class="text-right">{"$250.00"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">{"INV002"}</TableCell>
                        <TableCell>{"Pending"}</TableCell>
                        <TableCell>{"PayPal"}</TableCell>
                        <TableCell class="text-right">{"$150.00"}</TableCell>
                    </TableRow>
                    <TableRow class="bg-zinc-50 dark:bg-zinc-900/50">
                        <TableCell class="font-medium">{"INV003"}</TableCell>
                        <TableCell>{"Unpaid"}</TableCell>
                        <TableCell>{"Bank Transfer"}</TableCell>
                        <TableCell class="text-right">{"$350.00"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">{"INV004"}</TableCell>
                        <TableCell>{"Paid"}</TableCell>
                        <TableCell>{"Credit Card"}</TableCell>
                        <TableCell class="text-right">{"$450.00"}</TableCell>
                    </TableRow>
                    <TableRow class="bg-zinc-50 dark:bg-zinc-900/50">
                        <TableCell class="font-medium">{"INV005"}</TableCell>
                        <TableCell>{"Paid"}</TableCell>
                        <TableCell>{"PayPal"}</TableCell>
                        <TableCell class="text-right">{"$550.00"}</TableCell>
                    </TableRow>
                </TableBody>
                <TableFooter>
                    <TableRow>
                        <TableCell colspan={3}>{"Total"}</TableCell>
                        <TableCell class="text-right font-bold">{"$1,750.00"}</TableCell>
                    </TableRow>
                </TableFooter>
            </Table>
        </div>
    }
}

#[function_component(TableWithCheckboxes)]
fn table_with_checkboxes() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <Table>
                <TableHead>
                    <TableRow head={true}>
                        <TableHeadCell class="w-12">
                            <Checkbox />
                        </TableHeadCell>
                        <TableHeadCell>{"Name"}</TableHeadCell>
                        <TableHeadCell>{"Email"}</TableHeadCell>
                        <TableHeadCell>{"Department"}</TableHeadCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell>
                            <Checkbox />
                        </TableCell>
                        <TableCell class="font-medium">{"Lindsay Walton"}</TableCell>
                        <TableCell>{"lindsay.walton@example.com"}</TableCell>
                        <TableCell>{"Engineering"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Checkbox checked={true} />
                        </TableCell>
                        <TableCell class="font-medium">{"Courtney Henry"}</TableCell>
                        <TableCell>{"courtney.henry@example.com"}</TableCell>
                        <TableCell>{"Design"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Checkbox />
                        </TableCell>
                        <TableCell class="font-medium">{"Tom Cook"}</TableCell>
                        <TableCell>{"tom.cook@example.com"}</TableCell>
                        <TableCell>{"Product"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Checkbox checked={true} />
                        </TableCell>
                        <TableCell class="font-medium">{"Whitney Francis"}</TableCell>
                        <TableCell>{"whitney.francis@example.com"}</TableCell>
                        <TableCell>{"Marketing"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const STACKED_SIMPLE_CODE: &str = r##"use wonopui::wonopui_badge::{Badge, BadgeVariant};

html! {
    <ul class="divide-y divide-zinc-200">
        <li class="py-4">
            <div class="flex items-center gap-x-3">
                <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold text-zinc-900">{"Project Alpha"}</p>
                    <p class="text-xs text-zinc-500">{"Updated 2 hours ago"}</p>
                </div>
                <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
            </div>
        </li>
        // ... more items
    </ul>
}
"##;

const STACKED_AVATARS_CODE: &str = r##"use wonopui::wonopui_avatar::{Avatar, AvatarSize};

html! {
    <ul class="divide-y divide-zinc-200">
        <li class="flex items-center gap-x-4 py-4">
            <Avatar src="avatar.jpg" size={AvatarSize::Medium} />
            <div class="min-w-0 flex-auto">
                <p class="text-sm font-semibold">{"Leslie Alexander"}</p>
                <p class="text-xs text-zinc-500">{"leslie@example.com"}</p>
            </div>
            <div class="text-right">
                <p class="text-sm">{"Co-Founder / CEO"}</p>
                <p class="text-xs text-zinc-500">{"Last seen online"}</p>
            </div>
        </li>
        // ... more items
    </ul>
}
"##;

const STACKED_ACTIONS_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <ul class="divide-y rounded-lg border">
        <li class="flex items-center justify-between px-4 py-4">
            <div class="flex items-center gap-x-3">
                <div class="size-10 rounded-lg bg-zinc-100 flex items-center justify-center">
                    // Icon
                </div>
                <div>
                    <p class="text-sm font-semibold">{"Documents"}</p>
                    <p class="text-xs text-zinc-500">{"12 files"}</p>
                </div>
            </div>
            <div class="flex gap-x-2">
                <Button variant={ButtonVariant::Ghost}>{"View"}</Button>
                <Button variant={ButtonVariant::Ghost}>{"..."}</Button>
            </div>
        </li>
    </ul>
}
"##;

const GRID_CARDS_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};

html! {
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <Card>
            <CardHeader>
                <CardTitle>{"Analytics Dashboard"}</CardTitle>
                <CardDescription>{"Track your metrics"}</CardDescription>
            </CardHeader>
            <CardContent>
                <p class="text-sm text-zinc-500">{"Description..."}</p>
            </CardContent>
        </Card>
        // ... more cards
    </div>
}
"##;

const GRID_IMAGES_CODE: &str = r##"// Responsive image grid with hover effects
html! {
    <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
        <div class="group relative aspect-square overflow-hidden rounded-lg">
            <img 
                src="image.jpg"
                class="h-full w-full object-cover transition-transform group-hover:scale-105"
            />
            <div class="absolute inset-0 bg-black/0 group-hover:bg-black/40 transition-colors">
                <div class="absolute bottom-0 p-3 opacity-0 group-hover:opacity-100">
                    <p class="text-sm text-white">{"Image title"}</p>
                </div>
            </div>
        </div>
    </div>
}
"##;

const FEED_ACTIVITY_CODE: &str = r##"// Timeline-style activity feed
html! {
    <ul class="space-y-6">
        <li class="relative flex gap-x-4">
            // Vertical line
            <div class="absolute left-0 top-0 flex w-6 justify-center -bottom-6">
                <div class="w-px bg-zinc-200"></div>
            </div>
            // Dot or avatar
            <div class="relative flex h-6 w-6 items-center justify-center bg-white">
                <div class="h-1.5 w-1.5 rounded-full bg-zinc-200 ring-1 ring-zinc-300"></div>
            </div>
            // Content
            <p class="flex-auto text-sm text-zinc-500">
                <span class="font-medium text-zinc-900">{"User"}</span>
                {" performed action."}
            </p>
            <span class="text-xs text-zinc-500">{"7d ago"}</span>
        </li>
    </ul>
}
"##;

const FEED_COMMENTS_CODE: &str = r##"use wonopui::wonopui_avatar::{Avatar, AvatarSize};

// Nested comment thread
html! {
    <div class="space-y-6">
        <div class="flex gap-x-3">
            <Avatar src="avatar.jpg" size={AvatarSize::Medium} />
            <div class="flex-1">
                <div class="flex items-center gap-x-2">
                    <span class="text-sm font-semibold">{"User Name"}</span>
                    <span class="text-xs text-zinc-500">{"2 hours ago"}</span>
                </div>
                <p class="mt-1 text-sm text-zinc-600">{"Comment text..."}</p>
                <div class="mt-2 flex gap-x-4">
                    <button class="text-xs font-medium text-zinc-500">{"Reply"}</button>
                    <button class="text-xs font-medium text-zinc-500">{"Like"}</button>
                </div>
                
                // Nested replies
                <div class="mt-4 ml-4 border-l-2 border-zinc-200 pl-4">
                    // Reply items...
                </div>
            </div>
        </div>
    </div>
}
"##;

const TABLE_SIMPLE_CODE: &str = r##"use wonopui::wonopui_table::*;

html! {
    <Table>
        <TableHead>
            <TableRow head={true}>
                <TableHeadCell>{"Name"}</TableHeadCell>
                <TableHeadCell>{"Title"}</TableHeadCell>
                <TableHeadCell>{"Email"}</TableHeadCell>
                <TableHeadCell>{"Role"}</TableHeadCell>
            </TableRow>
        </TableHead>
        <TableBody>
            <TableRow>
                <TableCell class="font-medium">{"Lindsay Walton"}</TableCell>
                <TableCell>{"Front-end Developer"}</TableCell>
                <TableCell>{"lindsay.walton@example.com"}</TableCell>
                <TableCell>{"Member"}</TableCell>
            </TableRow>
            // ... more rows
        </TableBody>
    </Table>
}
"##;

const TABLE_ACTIONS_CODE: &str = r##"use wonopui::wonopui_table::*;
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_badge::{Badge, BadgeVariant};
use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <Table>
        <TableHead>
            <TableRow head={true}>
                <TableHeadCell>{"Name"}</TableHeadCell>
                <TableHeadCell>{"Status"}</TableHeadCell>
                <TableHeadCell>{"Role"}</TableHeadCell>
                <TableHeadCell class="text-right">{"Actions"}</TableHeadCell>
            </TableRow>
        </TableHead>
        <TableBody>
            <TableRow>
                <TableCell>
                    <div class="flex items-center gap-3">
                        <Avatar src="avatar.jpg" size={AvatarSize::Small} />
                        <div>
                            <div class="font-medium">{"Lindsay Walton"}</div>
                            <div class="text-sm text-zinc-500">{"lindsay@example.com"}</div>
                        </div>
                    </div>
                </TableCell>
                <TableCell>
                    <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                </TableCell>
                <TableCell>{"Member"}</TableCell>
                <TableCell class="text-right">
                    <Button variant={ButtonVariant::Ghost}>{"Edit"}</Button>
                </TableCell>
            </TableRow>
        </TableBody>
    </Table>
}
"##;

const TABLE_STRIPED_CODE: &str = r##"use wonopui::wonopui_table::*;

html! {
    <Table>
        <TableHead>
            <TableRow head={true}>
                <TableHeadCell>{"Invoice"}</TableHeadCell>
                <TableHeadCell>{"Status"}</TableHeadCell>
                <TableHeadCell>{"Method"}</TableHeadCell>
                <TableHeadCell class="text-right">{"Amount"}</TableHeadCell>
            </TableRow>
        </TableHead>
        <TableBody>
            // Alternate row backgrounds for striped effect
            <TableRow class="bg-zinc-50 dark:bg-zinc-900/50">
                <TableCell class="font-medium">{"INV001"}</TableCell>
                <TableCell>{"Paid"}</TableCell>
                <TableCell>{"Credit Card"}</TableCell>
                <TableCell class="text-right">{"$250.00"}</TableCell>
            </TableRow>
            <TableRow>
                <TableCell class="font-medium">{"INV002"}</TableCell>
                <TableCell>{"Pending"}</TableCell>
                <TableCell>{"PayPal"}</TableCell>
                <TableCell class="text-right">{"$150.00"}</TableCell>
            </TableRow>
        </TableBody>
        <TableFooter>
            <TableRow>
                <TableCell colspan={3}>{"Total"}</TableCell>
                <TableCell class="text-right font-bold">{"$1,750.00"}</TableCell>
            </TableRow>
        </TableFooter>
    </Table>
}
"##;

const TABLE_CHECKBOX_CODE: &str = r##"use wonopui::wonopui_table::*;
use wonopui::wonopui_checkbox::Checkbox;

html! {
    <Table>
        <TableHead>
            <TableRow head={true}>
                <TableHeadCell class="w-12">
                    <Checkbox />
                </TableHeadCell>
                <TableHeadCell>{"Name"}</TableHeadCell>
                <TableHeadCell>{"Email"}</TableHeadCell>
                <TableHeadCell>{"Department"}</TableHeadCell>
            </TableRow>
        </TableHead>
        <TableBody>
            <TableRow>
                <TableCell>
                    <Checkbox />
                </TableCell>
                <TableCell class="font-medium">{"Lindsay Walton"}</TableCell>
                <TableCell>{"lindsay.walton@example.com"}</TableCell>
                <TableCell>{"Engineering"}</TableCell>
            </TableRow>
            <TableRow>
                <TableCell>
                    <Checkbox checked={true} />
                </TableCell>
                <TableCell class="font-medium">{"Courtney Henry"}</TableCell>
                <TableCell>{"courtney.henry@example.com"}</TableCell>
                <TableCell>{"Design"}</TableCell>
            </TableRow>
        </TableBody>
    </Table>
}
"##;
