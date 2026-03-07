//! Navigation blocks - Navbars, Tabs, Breadcrumbs, Pagination
//! 
//! Navigation patterns using WonopUI components

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_topbar::{Topbar, TopbarStart, TopbarCenter, TopbarEnd, TopbarPosition};
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_input::Input;
use wonopui::wonopui_badge::Badge;
use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
use wonopui::wonopui_breadcrumb::{Breadcrumb, BreadcrumbItem};
use wonopui::wonopui_pagination::Pagination;
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent};
use crate::blocks::BlockPreview;

/// Navigation category page
#[function_component(NavigationBlocks)]
pub fn navigation_blocks() -> Html {
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
                    {"Navigation"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Navbars, tabs, breadcrumbs, and pagination patterns using WonopUI components."}
                </p>
            </div>
            
            // Navbars Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Navbars"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Simple Topbar"
                        description="Basic navigation bar with logo and links."
                        code={NAVBAR_SIMPLE_CODE}
                        min_height={80}
                    >
                        <NavbarSimple />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Topbar with Search"
                        description="Navigation bar with centered search input."
                        code={NAVBAR_SEARCH_CODE}
                        min_height={80}
                    >
                        <NavbarWithSearch />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Topbar with User Menu"
                        description="Navigation bar with user avatar and actions."
                        code={NAVBAR_USER_CODE}
                        min_height={80}
                    >
                        <NavbarWithUser />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Topbar with Badge"
                        description="Navigation bar with notification badge."
                        code={NAVBAR_BADGE_CODE}
                        min_height={80}
                    >
                        <NavbarWithBadge />
                    </BlockPreview>
                </div>
            </div>
            
            // Tabs Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Tabs"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Simple Tabs"
                        description="Basic tab navigation with content panels."
                        code={TABS_SIMPLE_CODE}
                        min_height={300}
                        isolate={true}
                    >
                        <TabsSimple />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Tabs with Icons"
                        description="Tab triggers with icons and labels."
                        code={TABS_ICONS_CODE}
                        min_height={300}
                        isolate={true}
                    >
                        <TabsWithIcons />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Tabs in Card"
                        description="Tabs integrated within a card component."
                        code={TABS_CARD_CODE}
                        min_height={350}
                        isolate={true}
                    >
                        <TabsInCard />
                    </BlockPreview>
                </div>
            </div>
            
            // Breadcrumbs Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Breadcrumbs"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Simple Breadcrumb"
                        description="Basic breadcrumb navigation."
                        code={BREADCRUMB_SIMPLE_CODE}
                        min_height={100}
                        isolate={true}
                    >
                        <BreadcrumbSimple />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Breadcrumb with Icons"
                        description="Breadcrumb with home icon."
                        code={BREADCRUMB_ICONS_CODE}
                        min_height={100}
                        isolate={true}
                    >
                        <BreadcrumbWithIcon />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Breadcrumb in Page Header"
                        description="Breadcrumb integrated with page title."
                        code={BREADCRUMB_HEADER_CODE}
                        min_height={180}
                        isolate={true}
                    >
                        <BreadcrumbInHeader />
                    </BlockPreview>
                </div>
            </div>
            
            // Pagination Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Pagination"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Simple Pagination"
                        description="Basic page navigation."
                        code={PAGINATION_SIMPLE_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <PaginationSimple />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Pagination with Info"
                        description="Pagination with results count."
                        code={PAGINATION_INFO_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <PaginationWithInfo />
                    </BlockPreview>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Navbar Examples
// =============================================================================

#[function_component(NavbarSimple)]
fn navbar_simple() -> Html {
    html! {
        <Topbar position={TopbarPosition::Relative}>
            <TopbarStart>
                <span class="text-lg font-bold text-zinc-900 dark:text-white">{"Acme"}</span>
                <nav class="hidden md:flex items-center gap-x-6 ml-6">
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"Home"}
                    </a>
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"Products"}
                    </a>
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"About"}
                    </a>
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"Contact"}
                    </a>
                </nav>
            </TopbarStart>
            <TopbarEnd>
                <Button variant={ButtonVariant::Ghost}>{"Sign in"}</Button>
                <Button variant={ButtonVariant::Default}>{"Get started"}</Button>
            </TopbarEnd>
        </Topbar>
    }
}

#[function_component(NavbarWithSearch)]
fn navbar_with_search() -> Html {
    html! {
        <Topbar position={TopbarPosition::Relative}>
            <TopbarStart>
                <span class="text-lg font-bold text-zinc-900 dark:text-white">{"Acme"}</span>
            </TopbarStart>
            <TopbarCenter>
                <div class="w-full max-w-md">
                    <Input 
                        placeholder="Search..." 
                        class="w-full"
                    />
                </div>
            </TopbarCenter>
            <TopbarEnd>
                <Button variant={ButtonVariant::Ghost}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                    </svg>
                </Button>
                <Avatar src="https://i.pravatar.cc/150?img=10" size={AvatarSize::Small} />
            </TopbarEnd>
        </Topbar>
    }
}

#[function_component(NavbarWithUser)]
fn navbar_with_user() -> Html {
    html! {
        <Topbar position={TopbarPosition::Relative}>
            <TopbarStart>
                <span class="text-lg font-bold text-zinc-900 dark:text-white">{"Dashboard"}</span>
                <nav class="hidden md:flex items-center gap-x-6 ml-6">
                    <a href="#" class="text-sm font-medium text-zinc-900 dark:text-white">
                        {"Overview"}
                    </a>
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"Analytics"}
                    </a>
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"Reports"}
                    </a>
                    <a href="#" class="text-sm font-medium text-zinc-600 hover:text-zinc-900 dark:text-zinc-300 dark:hover:text-white transition-colors">
                        {"Settings"}
                    </a>
                </nav>
            </TopbarStart>
            <TopbarEnd>
                <Button variant={ButtonVariant::Outline} class="hidden sm:flex">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4 mr-2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                    {"New Project"}
                </Button>
                <div class="flex items-center gap-x-3">
                    <Avatar src="https://i.pravatar.cc/150?img=20" size={AvatarSize::Small} />
                    <div class="hidden md:block">
                        <p class="text-sm font-medium text-zinc-900 dark:text-white">{"John Doe"}</p>
                        <p class="text-xs text-zinc-500 dark:text-zinc-400">{"john@example.com"}</p>
                    </div>
                </div>
            </TopbarEnd>
        </Topbar>
    }
}

#[function_component(NavbarWithBadge)]
fn navbar_with_badge() -> Html {
    html! {
        <Topbar position={TopbarPosition::Relative}>
            <TopbarStart>
                <span class="text-lg font-bold text-zinc-900 dark:text-white">{"Inbox"}</span>
            </TopbarStart>
            <TopbarCenter>
                <nav class="flex items-center gap-x-1">
                    <Button variant={ButtonVariant::Ghost} class="relative">
                        {"Primary"}
                        <Badge class="ml-2">{"12"}</Badge>
                    </Button>
                    <Button variant={ButtonVariant::Ghost}>
                        {"Social"}
                    </Button>
                    <Button variant={ButtonVariant::Ghost}>
                        {"Updates"}
                    </Button>
                    <Button variant={ButtonVariant::Ghost}>
                        {"Promotions"}
                    </Button>
                </nav>
            </TopbarCenter>
            <TopbarEnd>
                <Button variant={ButtonVariant::Ghost}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                    </svg>
                </Button>
                <div class="relative">
                    <Button variant={ButtonVariant::Ghost}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
                        </svg>
                    </Button>
                    <span class="absolute top-1 right-1 size-2 rounded-full bg-red-500"></span>
                </div>
                <Avatar src="https://i.pravatar.cc/150?img=25" size={AvatarSize::Small} />
            </TopbarEnd>
        </Topbar>
    }
}

// =============================================================================
// Tabs Examples
// =============================================================================

#[function_component(TabsSimple)]
fn tabs_simple() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-950">
            <Tabs default_value="account">
                <TabsList>
                    <TabsTrigger value="account">{"Account"}</TabsTrigger>
                    <TabsTrigger value="password">{"Password"}</TabsTrigger>
                    <TabsTrigger value="notifications">{"Notifications"}</TabsTrigger>
                </TabsList>
                <TabsContent value="account" class="mt-4">
                    <div class="p-4 border border-zinc-200 dark:border-zinc-800 rounded-lg">
                        <h3 class="text-lg font-medium text-zinc-900 dark:text-white">{"Account Settings"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Make changes to your account here. Click save when you're done."}
                        </p>
                    </div>
                </TabsContent>
                <TabsContent value="password" class="mt-4">
                    <div class="p-4 border border-zinc-200 dark:border-zinc-800 rounded-lg">
                        <h3 class="text-lg font-medium text-zinc-900 dark:text-white">{"Password Settings"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Change your password here. After saving, you'll be logged out."}
                        </p>
                    </div>
                </TabsContent>
                <TabsContent value="notifications" class="mt-4">
                    <div class="p-4 border border-zinc-200 dark:border-zinc-800 rounded-lg">
                        <h3 class="text-lg font-medium text-zinc-900 dark:text-white">{"Notification Preferences"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Configure how you receive notifications."}
                        </p>
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    }
}

fn user_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" />
        </svg>
    }
}

fn key_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 0 1 3 3m3 0a6 6 0 0 1-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1 1 21.75 8.25Z" />
        </svg>
    }
}

fn bell_icon_small() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
        </svg>
    }
}

#[function_component(TabsWithIcons)]
fn tabs_with_icons() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-950">
            <Tabs default_value="profile">
                <TabsList>
                    <TabsTrigger value="profile" class="flex items-center gap-2">
                        {user_icon()}
                        {"Profile"}
                    </TabsTrigger>
                    <TabsTrigger value="security" class="flex items-center gap-2">
                        {key_icon()}
                        {"Security"}
                    </TabsTrigger>
                    <TabsTrigger value="alerts" class="flex items-center gap-2">
                        {bell_icon_small()}
                        {"Alerts"}
                    </TabsTrigger>
                </TabsList>
                <TabsContent value="profile" class="mt-4">
                    <div class="p-4 border border-zinc-200 dark:border-zinc-800 rounded-lg">
                        <h3 class="text-lg font-medium text-zinc-900 dark:text-white">{"Profile Information"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Update your profile photo and personal details."}
                        </p>
                    </div>
                </TabsContent>
                <TabsContent value="security" class="mt-4">
                    <div class="p-4 border border-zinc-200 dark:border-zinc-800 rounded-lg">
                        <h3 class="text-lg font-medium text-zinc-900 dark:text-white">{"Security Settings"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Manage your security preferences and two-factor authentication."}
                        </p>
                    </div>
                </TabsContent>
                <TabsContent value="alerts" class="mt-4">
                    <div class="p-4 border border-zinc-200 dark:border-zinc-800 rounded-lg">
                        <h3 class="text-lg font-medium text-zinc-900 dark:text-white">{"Alert Settings"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Choose which alerts you want to receive."}
                        </p>
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    }
}

#[function_component(TabsInCard)]
fn tabs_in_card() -> Html {
    html! {
        <div class="p-6 bg-zinc-100 dark:bg-zinc-900">
            <Card class="max-w-xl mx-auto">
                <CardHeader>
                    <CardTitle>{"Settings"}</CardTitle>
                </CardHeader>
                <CardContent>
                    <Tabs default_value="general">
                        <TabsList class="w-full">
                            <TabsTrigger value="general" class="flex-1">{"General"}</TabsTrigger>
                            <TabsTrigger value="privacy" class="flex-1">{"Privacy"}</TabsTrigger>
                            <TabsTrigger value="billing" class="flex-1">{"Billing"}</TabsTrigger>
                        </TabsList>
                        <TabsContent value="general" class="mt-4">
                            <p class="text-sm text-zinc-500 dark:text-zinc-400">
                                {"General settings for your account including display name, timezone, and language preferences."}
                            </p>
                        </TabsContent>
                        <TabsContent value="privacy" class="mt-4">
                            <p class="text-sm text-zinc-500 dark:text-zinc-400">
                                {"Privacy settings to control who can see your profile and activity."}
                            </p>
                        </TabsContent>
                        <TabsContent value="billing" class="mt-4">
                            <p class="text-sm text-zinc-500 dark:text-zinc-400">
                                {"Manage your subscription, payment methods, and billing history."}
                            </p>
                        </TabsContent>
                    </Tabs>
                </CardContent>
            </Card>
        </div>
    }
}

// =============================================================================
// Breadcrumb Examples
// =============================================================================

#[function_component(BreadcrumbSimple)]
fn breadcrumb_simple() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-950">
            <Breadcrumb>
                <BreadcrumbItem label="Home" href="#" />
                <BreadcrumbItem label="Products" href="#" />
                <BreadcrumbItem label="Electronics" href="#" />
                <BreadcrumbItem label="Laptops" />
            </Breadcrumb>
        </div>
    }
}

fn home_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
            <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
        </svg>
    }
}

#[function_component(BreadcrumbWithIcon)]
fn breadcrumb_with_icon() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-950">
            // Note: For icons in breadcrumbs, use custom rendering
            <nav aria-label="Breadcrumb">
                <ol class="flex flex-wrap items-center gap-1.5 text-sm text-zinc-500 dark:text-zinc-400">
                    <li class="inline-flex items-center gap-1.5">
                        <a href="#" class="flex items-center gap-1.5 transition-colors hover:text-zinc-950 dark:hover:text-zinc-50">
                            {home_icon()}
                            {"Home"}
                        </a>
                    </li>
                    <li class="text-zinc-400 dark:text-zinc-500">{"/"}</li>
                    <li class="inline-flex items-center gap-1.5">
                        <a href="#" class="transition-colors hover:text-zinc-950 dark:hover:text-zinc-50">{"Settings"}</a>
                    </li>
                    <li class="text-zinc-400 dark:text-zinc-500">{"/"}</li>
                    <li class="inline-flex items-center gap-1.5">
                        <span class="font-normal text-zinc-950 dark:text-zinc-50">{"Account"}</span>
                    </li>
                </ol>
            </nav>
        </div>
    }
}

#[function_component(BreadcrumbInHeader)]
fn breadcrumb_in_header() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-950 border-b border-zinc-200 dark:border-zinc-800">
            <Breadcrumb class="mb-4">
                <BreadcrumbItem label="Dashboard" href="#" />
                <BreadcrumbItem label="Projects" href="#" />
                <BreadcrumbItem label="Project Alpha" />
            </Breadcrumb>
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-bold text-zinc-900 dark:text-white">{"Project Alpha"}</h1>
                    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">{"Manage your project settings and team members."}</p>
                </div>
                <Button variant={ButtonVariant::Default}>{"Edit Project"}</Button>
            </div>
        </div>
    }
}

// =============================================================================
// Pagination Examples
// =============================================================================

#[function_component(PaginationSimple)]
fn pagination_simple() -> Html {
    let current_page = use_state(|| 1usize);
    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: usize| {
            current_page.set(page);
        })
    };

    html! {
        <div class="p-6 bg-white dark:bg-zinc-950 flex justify-center">
            <Pagination 
                current_page={*current_page}
                total_pages={10}
                on_page_change={on_page_change}
            />
        </div>
    }
}

#[function_component(PaginationWithInfo)]
fn pagination_with_info() -> Html {
    let current_page = use_state(|| 1usize);
    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: usize| {
            current_page.set(page);
        })
    };
    
    let start = (*current_page - 1) * 10 + 1;
    let end = (*current_page * 10).min(97);

    html! {
        <div class="p-6 bg-white dark:bg-zinc-950">
            <div class="flex items-center justify-between">
                <p class="text-sm text-zinc-500 dark:text-zinc-400">
                    {"Showing "}<span class="font-medium text-zinc-900 dark:text-white">{start}</span>
                    {" to "}<span class="font-medium text-zinc-900 dark:text-white">{end}</span>
                    {" of "}<span class="font-medium text-zinc-900 dark:text-white">{"97"}</span>{" results"}
                </p>
                <Pagination 
                    current_page={*current_page}
                    total_pages={10}
                    on_page_change={on_page_change}
                />
            </div>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const NAVBAR_SIMPLE_CODE: &str = r##"use wonopui::wonopui_topbar::{Topbar, TopbarStart, TopbarEnd};
use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <Topbar>
        <TopbarStart>
            <span class="text-lg font-bold">{"Acme"}</span>
            <nav class="flex items-center gap-x-6 ml-6">
                <a href="#">{"Home"}</a>
                <a href="#">{"Products"}</a>
                <a href="#">{"About"}</a>
            </nav>
        </TopbarStart>
        <TopbarEnd>
            <Button variant={ButtonVariant::Ghost}>{"Sign in"}</Button>
            <Button variant={ButtonVariant::Default}>{"Get started"}</Button>
        </TopbarEnd>
    </Topbar>
}
"##;

const NAVBAR_SEARCH_CODE: &str = r##"use wonopui::wonopui_topbar::{Topbar, TopbarStart, TopbarCenter, TopbarEnd};
use wonopui::wonopui_input::Input;
use wonopui::wonopui_avatar::{Avatar, AvatarSize};

html! {
    <Topbar>
        <TopbarStart>
            <span class="text-lg font-bold">{"Acme"}</span>
        </TopbarStart>
        <TopbarCenter>
            <Input placeholder="Search..." class="w-full max-w-md" />
        </TopbarCenter>
        <TopbarEnd>
            <Avatar src="avatar.jpg" size={AvatarSize::Small} />
        </TopbarEnd>
    </Topbar>
}
"##;

const NAVBAR_USER_CODE: &str = r##"use wonopui::wonopui_topbar::{Topbar, TopbarStart, TopbarEnd};
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};

html! {
    <Topbar>
        <TopbarStart>
            <span class="text-lg font-bold">{"Dashboard"}</span>
            <nav class="flex items-center gap-x-6 ml-6">
                <a href="#">{"Overview"}</a>
                <a href="#">{"Analytics"}</a>
            </nav>
        </TopbarStart>
        <TopbarEnd>
            <Button variant={ButtonVariant::Outline}>{"New Project"}</Button>
            <Avatar src="avatar.jpg" size={AvatarSize::Small} />
        </TopbarEnd>
    </Topbar>
}
"##;

const NAVBAR_BADGE_CODE: &str = r##"use wonopui::wonopui_topbar::{Topbar, TopbarStart, TopbarCenter, TopbarEnd};
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_badge::Badge;

html! {
    <Topbar>
        <TopbarStart>
            <span class="text-lg font-bold">{"Inbox"}</span>
        </TopbarStart>
        <TopbarCenter>
            <nav class="flex items-center gap-x-1">
                <Button variant={ButtonVariant::Ghost}>
                    {"Primary"}
                    <Badge class="ml-2">{"12"}</Badge>
                </Button>
                <Button variant={ButtonVariant::Ghost}>{"Social"}</Button>
            </nav>
        </TopbarCenter>
    </Topbar>
}
"##;

const TABS_SIMPLE_CODE: &str = r##"use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};

html! {
    <Tabs default_value="account">
        <TabsList>
            <TabsTrigger value="account">{"Account"}</TabsTrigger>
            <TabsTrigger value="password">{"Password"}</TabsTrigger>
            <TabsTrigger value="notifications">{"Notifications"}</TabsTrigger>
        </TabsList>
        <TabsContent value="account" class="mt-4">
            <div class="p-4 border rounded-lg">
                <h3 class="text-lg font-medium">{"Account Settings"}</h3>
                <p class="mt-2 text-sm text-zinc-500">
                    {"Make changes to your account here."}
                </p>
            </div>
        </TabsContent>
        <TabsContent value="password" class="mt-4">
            // Password content...
        </TabsContent>
    </Tabs>
}
"##;

const TABS_ICONS_CODE: &str = r##"use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};

html! {
    <Tabs default_value="profile">
        <TabsList>
            <TabsTrigger value="profile" class="flex items-center gap-2">
                <UserIcon />
                {"Profile"}
            </TabsTrigger>
            <TabsTrigger value="security" class="flex items-center gap-2">
                <KeyIcon />
                {"Security"}
            </TabsTrigger>
            <TabsTrigger value="alerts" class="flex items-center gap-2">
                <BellIcon />
                {"Alerts"}
            </TabsTrigger>
        </TabsList>
        <TabsContent value="profile" class="mt-4">
            // Profile content...
        </TabsContent>
    </Tabs>
}
"##;

const TABS_CARD_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent};
use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};

html! {
    <Card>
        <CardHeader>
            <CardTitle>{"Settings"}</CardTitle>
        </CardHeader>
        <CardContent>
            <Tabs default_value="general">
                <TabsList class="w-full">
                    <TabsTrigger value="general" class="flex-1">{"General"}</TabsTrigger>
                    <TabsTrigger value="privacy" class="flex-1">{"Privacy"}</TabsTrigger>
                    <TabsTrigger value="billing" class="flex-1">{"Billing"}</TabsTrigger>
                </TabsList>
                <TabsContent value="general" class="mt-4">
                    // General settings content...
                </TabsContent>
            </Tabs>
        </CardContent>
    </Card>
}
"##;

const BREADCRUMB_SIMPLE_CODE: &str = r##"use wonopui::wonopui_breadcrumb::{Breadcrumb, BreadcrumbItem};

html! {
    <Breadcrumb>
        <BreadcrumbItem href="#">{"Home"}</BreadcrumbItem>
        <BreadcrumbItem href="#">{"Products"}</BreadcrumbItem>
        <BreadcrumbItem href="#">{"Electronics"}</BreadcrumbItem>
        <BreadcrumbItem current={true}>{"Laptops"}</BreadcrumbItem>
    </Breadcrumb>
}
"##;

const BREADCRUMB_ICONS_CODE: &str = r##"use wonopui::wonopui_breadcrumb::{Breadcrumb, BreadcrumbItem};

html! {
    <Breadcrumb>
        <BreadcrumbItem href="#">
            <span class="flex items-center gap-1.5">
                <HomeIcon />
                {"Home"}
            </span>
        </BreadcrumbItem>
        <BreadcrumbItem href="#">{"Settings"}</BreadcrumbItem>
        <BreadcrumbItem current={true}>{"Account"}</BreadcrumbItem>
    </Breadcrumb>
}
"##;

const BREADCRUMB_HEADER_CODE: &str = r##"use wonopui::wonopui_breadcrumb::{Breadcrumb, BreadcrumbItem};
use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <div class="p-6 border-b">
        <Breadcrumb class="mb-4">
            <BreadcrumbItem href="#">{"Dashboard"}</BreadcrumbItem>
            <BreadcrumbItem href="#">{"Projects"}</BreadcrumbItem>
            <BreadcrumbItem current={true}>{"Project Alpha"}</BreadcrumbItem>
        </Breadcrumb>
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-2xl font-bold">{"Project Alpha"}</h1>
                <p class="mt-1 text-sm text-zinc-500">{"Manage your project settings."}</p>
            </div>
            <Button variant={ButtonVariant::Default}>{"Edit Project"}</Button>
        </div>
    </div>
}
"##;

const PAGINATION_SIMPLE_CODE: &str = r##"use wonopui::wonopui_pagination::Pagination;

let current_page = use_state(|| 1u32);
let on_page_change = {
    let current_page = current_page.clone();
    Callback::from(move |page: u32| {
        current_page.set(page);
    })
};

html! {
    <Pagination 
        current_page={*current_page}
        total_pages={10}
        on_page_change={on_page_change}
    />
}
"##;

const PAGINATION_INFO_CODE: &str = r##"use wonopui::wonopui_pagination::Pagination;

html! {
    <div class="flex items-center justify-between">
        <p class="text-sm text-zinc-500">
            {"Showing "}<span class="font-medium">{"1"}</span>
            {" to "}<span class="font-medium">{"10"}</span>
            {" of "}<span class="font-medium">{"97"}</span>{" results"}
        </p>
        <Pagination 
            current_page={current_page}
            total_pages={10}
            on_page_change={on_page_change}
        />
    </div>
}
"##;
