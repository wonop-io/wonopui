//! Application Shell blocks - Complete application layouts using WonopUI components
//! 
//! Demonstrates proper usage of:
//! - Sidebar (with folded mode, mobile responsive)
//! - LayoutProvider/LayoutContext for state management
//! - Topbar integration
//! - Interactive toggle controls

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_input::Input;
use wonopui::wonopui_sidebar::{Sidebar, SidebarHeader, SidebarFooter, SidebarMenu, SidebarItem, SidebarHeading};
use wonopui::wonopui_topbar::{Topbar, TopbarStart, TopbarCenter, TopbarEnd, TopbarPosition};
use wonopui::wonopui_card::{Card, CardHeader, CardContent, CardTitle};
use wonopui::wonopui_layout::{LayoutProvider, LayoutState, LayoutAction, use_layout, Layout};
use crate::blocks::BlockPreview;

/// Application Shells category page
#[function_component(ApplicationShellsBlocks)]
pub fn application_shells_blocks() -> Html {
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
                    {"Application Shells"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Complete application layouts using WonopUI Sidebar, Topbar, and Layout components with interactive state management."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                <BlockPreview 
                    title="Sidebar with Folding Toggle"
                    description="Dashboard layout with collapsible sidebar. Click the toggle button to switch between normal and narrow modes."
                    code={SIDEBAR_FOLDING_CODE}
                    min_height={550}
                >
                    <SidebarFoldingExample />
                </BlockPreview>
                
                <BlockPreview 
                    title="Mobile Responsive Sidebar"
                    description="Sidebar with mobile menu overlay. Resize the preview or click the menu button to see mobile behavior."
                    code={MOBILE_SIDEBAR_CODE}
                    min_height={550}
                >
                    <MobileSidebarExample />
                </BlockPreview>
                
                <BlockPreview 
                    title="Stacked Layout"
                    description="Mobile-first layout with WonopUI Topbar for navigation."
                    code={STACKED_LAYOUT_CODE}
                    min_height={400}
                >
                    <StackedLayoutExample />
                </BlockPreview>
                
                <BlockPreview 
                    title="Multi-Column Layout"
                    description="Three-column chat layout with narrow sidebar, list panel, and detail view."
                    code={MULTI_COLUMN_LAYOUT_CODE}
                    min_height={500}
                >
                    <MultiColumnLayoutExample />
                </BlockPreview>
            </div>
        </Container>
    }
}

// =============================================================================
// Icons
// =============================================================================

fn menu_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
        </svg>
    }
}

fn close_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
        </svg>
    }
}

fn chevron_left_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5 8.25 12l7.5-7.5" />
        </svg>
    }
}

fn chevron_right_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="m8.25 4.5 7.5 7.5-7.5 7.5" />
        </svg>
    }
}

fn dashboard_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
        </svg>
    }
}

fn team_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z" />
        </svg>
    }
}

fn projects_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
        </svg>
    }
}

fn calendar_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5" />
        </svg>
    }
}

fn settings_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
        </svg>
    }
}

fn bell_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
        </svg>
    }
}

// =============================================================================
// Example 1: Sidebar with Folding Toggle
// =============================================================================

#[function_component(SidebarFoldingExample)]
fn sidebar_folding_example() -> Html {
    // Wrap in LayoutProvider for state management
    html! {
        <LayoutProvider>
            <SidebarFoldingInner />
        </LayoutProvider>
    }
}

#[function_component(SidebarFoldingInner)]
fn sidebar_folding_inner() -> Html {
    let layout = use_layout();
    let folded = layout.sidebar_folded;
    
    let toggle_folded = {
        let layout = layout.clone();
        Callback::from(move |_: MouseEvent| {
            layout.dispatch(LayoutAction::ToggleSidebarFolded);
        })
    };
    
    let sidebar_width = if folded { "w-18" } else { "w-64" };
    let content_padding = if folded { "pl-18" } else { "pl-64" };
    
    let sidebar_header = html! {
        <SidebarHeader>
            <div class="flex items-center gap-2">
                <div class="h-8 w-8 rounded-lg bg-zinc-900 dark:bg-zinc-100 shrink-0"></div>
                if !folded {
                    <span class="font-semibold text-zinc-900 dark:text-white">{"Acme Inc"}</span>
                }
            </div>
        </SidebarHeader>
    };
    
    let sidebar_footer = html! {
        <SidebarFooter>
            <div class={classes!("flex", "items-center", if folded { "justify-center" } else { "gap-3" })}>
                <Avatar src="https://i.pravatar.cc/150?img=1" size={AvatarSize::Small} />
                if !folded {
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium text-zinc-900 truncate dark:text-white">{"Tom Cook"}</p>
                        <p class="text-xs text-zinc-500 truncate dark:text-zinc-400">{"tom@example.com"}</p>
                    </div>
                }
            </div>
        </SidebarFooter>
    };

    html! {
        <div class="relative flex h-[550px] w-full bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
            // Sidebar - uses folded prop from context
            <Sidebar 
                header={sidebar_header}
                footer={sidebar_footer}
                folded={folded}
                class={classes!("!relative", sidebar_width)}
            >
                <SidebarMenu>
                    <SidebarItem icon={dashboard_icon()} active={true}>
                        if !folded { {"Dashboard"} }
                    </SidebarItem>
                    <SidebarItem icon={team_icon()}>
                        if !folded { {"Team"} }
                    </SidebarItem>
                    <SidebarItem icon={projects_icon()}>
                        if !folded { {"Projects"} }
                    </SidebarItem>
                    <SidebarItem icon={calendar_icon()}>
                        if !folded { {"Calendar"} }
                    </SidebarItem>
                    <SidebarItem icon={settings_icon()}>
                        if !folded { {"Settings"} }
                    </SidebarItem>
                </SidebarMenu>
            </Sidebar>
            
            // Main content area
            <div class="flex flex-1 flex-col min-w-0">
                // Topbar with fold toggle button
                <Topbar position={TopbarPosition::Relative}>
                    <TopbarStart>
                        <Button 
                            variant={ButtonVariant::Ghost} 
                            size={ButtonSize::Icon}
                            onclick={toggle_folded}
                        >
                            if folded {
                                {chevron_right_icon()}
                            } else {
                                {chevron_left_icon()}
                            }
                        </Button>
                        <h1 class="text-lg font-semibold text-zinc-900 dark:text-white ml-2">{"Dashboard"}</h1>
                    </TopbarStart>
                    <TopbarEnd>
                        <Input placeholder="Search..." class="w-48" />
                        <Button variant={ButtonVariant::Primary} size={ButtonSize::Small}>
                            {"New Project"}
                        </Button>
                    </TopbarEnd>
                </Topbar>
                
                // Status indicator
                <div class="px-6 py-2 bg-blue-50 dark:bg-blue-900/20 border-b border-blue-100 dark:border-blue-800">
                    <p class="text-sm text-blue-700 dark:text-blue-300">
                        {"Sidebar state: "}<strong>{if folded { "Folded (narrow)" } else { "Normal (wide)" }}</strong>
                        {" — Click the arrow button to toggle"}
                    </p>
                </div>
                
                // Content with Cards
                <main class="flex-1 overflow-auto p-6">
                    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
                        {for ["Revenue", "Users", "Orders", "Growth"].iter().map(|title| html! {
                            <Card>
                                <CardHeader>
                                    <CardTitle class="text-sm font-medium text-zinc-500 dark:text-zinc-400">
                                        {*title}
                                    </CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div class="text-2xl font-bold text-zinc-900 dark:text-white">{"$12,345"}</div>
                                </CardContent>
                            </Card>
                        })}
                    </div>
                </main>
            </div>
        </div>
    }
}

// =============================================================================
// Example 2: Mobile Responsive Sidebar
// =============================================================================

#[function_component(MobileSidebarExample)]
fn mobile_sidebar_example() -> Html {
    html! {
        <LayoutProvider>
            <MobileSidebarInner />
        </LayoutProvider>
    }
}

#[function_component(MobileSidebarInner)]
fn mobile_sidebar_inner() -> Html {
    let layout = use_layout();
    let mobile_open = layout.mobile_menu_open;
    
    let toggle_mobile = {
        let layout = layout.clone();
        Callback::from(move |_: MouseEvent| {
            layout.dispatch(LayoutAction::ToggleMobileMenu);
        })
    };
    
    let close_mobile = {
        let layout = layout.clone();
        Callback::from(move |_: MouseEvent| {
            layout.dispatch(LayoutAction::SetMobileMenuOpen(false));
        })
    };
    
    let sidebar_header = html! {
        <SidebarHeader>
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center gap-2">
                    <div class="h-8 w-8 rounded-lg bg-zinc-900 dark:bg-zinc-100"></div>
                    <span class="font-semibold text-zinc-900 dark:text-white">{"Acme Inc"}</span>
                </div>
                // Close button visible in mobile overlay
                <Button 
                    variant={ButtonVariant::Ghost} 
                    size={ButtonSize::Icon}
                    onclick={close_mobile.clone()}
                    class="lg:hidden"
                >
                    {close_icon()}
                </Button>
            </div>
        </SidebarHeader>
    };
    
    let sidebar_footer = html! {
        <SidebarFooter>
            <div class="flex items-center gap-3">
                <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Small} />
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-zinc-900 truncate dark:text-white">{"Jane Smith"}</p>
                    <p class="text-xs text-zinc-500 truncate dark:text-zinc-400">{"jane@example.com"}</p>
                </div>
            </div>
        </SidebarFooter>
    };

    html! {
        <div class="relative flex h-[550px] w-full bg-zinc-100 dark:bg-zinc-900 overflow-hidden">
            // Mobile overlay backdrop
            if mobile_open {
                <div 
                    class="absolute inset-0 bg-black/50 z-40 lg:hidden"
                    onclick={close_mobile.clone()}
                />
            }
            
            // Sidebar - hidden on mobile by default, shown via overlay
            <div class={classes!(
                "absolute", "inset-y-0", "left-0", "z-50", "lg:relative", "lg:translate-x-0",
                "transition-transform", "duration-300", "ease-in-out",
                if mobile_open { "translate-x-0" } else { "-translate-x-full lg:translate-x-0" }
            )}>
                <Sidebar 
                    header={sidebar_header}
                    footer={sidebar_footer}
                    class="!relative h-full"
                >
                    <SidebarMenu>
                        <SidebarItem icon={dashboard_icon()} active={true} onclick={close_mobile.clone()}>
                            {"Dashboard"}
                        </SidebarItem>
                        <SidebarItem icon={team_icon()} onclick={close_mobile.clone()}>
                            {"Team"}
                        </SidebarItem>
                        <SidebarItem icon={projects_icon()} onclick={close_mobile.clone()}>
                            {"Projects"}
                        </SidebarItem>
                        <SidebarItem icon={calendar_icon()} onclick={close_mobile.clone()}>
                            {"Calendar"}
                        </SidebarItem>
                        <SidebarItem icon={settings_icon()} onclick={close_mobile.clone()}>
                            {"Settings"}
                        </SidebarItem>
                    </SidebarMenu>
                </Sidebar>
            </div>
            
            // Main content area
            <div class="flex flex-1 flex-col min-w-0">
                // Topbar with hamburger menu
                <Topbar position={TopbarPosition::Relative}>
                    <TopbarStart>
                        // Hamburger menu button - visible on mobile
                        <Button 
                            variant={ButtonVariant::Ghost} 
                            size={ButtonSize::Icon}
                            onclick={toggle_mobile}
                            class="lg:hidden"
                        >
                            {menu_icon()}
                        </Button>
                        <h1 class="text-lg font-semibold text-zinc-900 dark:text-white">{"Dashboard"}</h1>
                    </TopbarStart>
                    <TopbarEnd>
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                            {bell_icon()}
                        </Button>
                        <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Small} />
                    </TopbarEnd>
                </Topbar>
                
                // Status indicator
                <div class="px-6 py-2 bg-amber-50 dark:bg-amber-900/20 border-b border-amber-100 dark:border-amber-800">
                    <p class="text-sm text-amber-700 dark:text-amber-300">
                        {"Mobile menu: "}<strong>{if mobile_open { "Open" } else { "Closed" }}</strong>
                        {" — Click the hamburger menu (☰) or resize to test responsive behavior"}
                    </p>
                </div>
                
                // Content
                <main class="flex-1 overflow-auto p-6">
                    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                        {for (0..6).map(|i| html! {
                            <Card>
                                <CardHeader>
                                    <CardTitle>{format!("Card {}", i + 1)}</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <p class="text-sm text-zinc-500 dark:text-zinc-400">
                                        {"This layout is responsive. On large screens, the sidebar is always visible. On smaller screens, it slides in from the left."}
                                    </p>
                                </CardContent>
                            </Card>
                        })}
                    </div>
                </main>
            </div>
        </div>
    }
}

// =============================================================================
// Example 3: Stacked Layout
// =============================================================================

#[function_component(StackedLayoutExample)]
fn stacked_layout_example() -> Html {
    html! {
        <div class="flex h-[400px] w-full flex-col bg-zinc-100 dark:bg-zinc-900">
            // WonopUI Topbar as main navigation
            <Topbar position={TopbarPosition::Relative}>
                <TopbarStart>
                    <div class="h-8 w-8 rounded-lg bg-zinc-900 dark:bg-zinc-100"></div>
                    <nav class="hidden md:flex md:gap-6 ml-8">
                        <a href="#" class="text-sm font-medium text-zinc-900 dark:text-white">{"Dashboard"}</a>
                        <a href="#" class="text-sm font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Team"}</a>
                        <a href="#" class="text-sm font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Projects"}</a>
                        <a href="#" class="text-sm font-medium text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">{"Calendar"}</a>
                    </nav>
                </TopbarStart>
                <TopbarEnd>
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                        {bell_icon()}
                    </Button>
                    <Avatar src="https://i.pravatar.cc/150?img=2" size={AvatarSize::Small} />
                </TopbarEnd>
            </Topbar>
            
            // Page header
            <div class="border-b border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-950">
                <div class="mx-auto max-w-7xl px-4 py-6">
                    <h1 class="text-2xl font-bold text-zinc-900 dark:text-white">{"Dashboard"}</h1>
                </div>
            </div>
            
            // Content with WonopUI Cards
            <main class="flex-1 overflow-auto">
                <div class="mx-auto max-w-7xl px-4 py-6">
                    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                        {for (0..6).map(|i| html! {
                            <Card>
                                <CardHeader>
                                    <CardTitle>{format!("Card {}", i + 1)}</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <p class="text-sm text-zinc-500 dark:text-zinc-400">
                                        {"This is a stacked layout with top navigation."}
                                    </p>
                                </CardContent>
                            </Card>
                        })}
                    </div>
                </div>
            </main>
        </div>
    }
}

// =============================================================================
// Example 4: Multi-Column Layout
// =============================================================================

#[function_component(MultiColumnLayoutExample)]
fn multi_column_layout_example() -> Html {
    html! {
        <div class="flex h-[500px] w-full bg-zinc-100 dark:bg-zinc-900">
            // Narrow sidebar using WonopUI Sidebar
            <Sidebar class="!relative !w-20 items-center" folded={true}>
                <div class="h-10 w-10 rounded-xl bg-zinc-900 dark:bg-zinc-100 mx-auto mt-4"></div>
                <SidebarMenu class="mt-8 items-center">
                    <SidebarItem icon={dashboard_icon()} active={true} class="!justify-center !p-3" />
                    <SidebarItem icon={team_icon()} class="!justify-center !p-3" />
                    <SidebarItem icon={projects_icon()} class="!justify-center !p-3" />
                    <SidebarItem icon={settings_icon()} class="!justify-center !p-3" />
                </SidebarMenu>
            </Sidebar>
            
            // List panel
            <aside class="w-80 overflow-auto border-r border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-950">
                <div class="sticky top-0 border-b border-zinc-200 bg-white p-4 dark:border-zinc-800 dark:bg-zinc-950">
                    <Input placeholder="Search messages..." />
                </div>
                <div class="divide-y divide-zinc-200 dark:divide-zinc-800">
                    {for (0..8).map(|i| html! {
                        <div class={classes!(
                            "flex", "cursor-pointer", "items-center", "gap-3", "p-4",
                            if i == 0 { "bg-zinc-50 dark:bg-zinc-800/50" } else { "hover:bg-zinc-50 dark:hover:bg-zinc-800/50" }
                        )}>
                            <Avatar src={format!("https://i.pravatar.cc/150?img={}", i + 10)} size={AvatarSize::Small} />
                            <div class="flex-1 min-w-0">
                                <p class="text-sm font-medium text-zinc-900 truncate dark:text-white">{format!("User {}", i + 1)}</p>
                                <p class="text-xs text-zinc-500 truncate dark:text-zinc-400">{"Last message preview..."}</p>
                            </div>
                            <Badge variant={BadgeVariant::Default}>{"2m"}</Badge>
                        </div>
                    })}
                </div>
            </aside>
            
            // Detail panel
            <main class="flex flex-1 flex-col bg-white dark:bg-zinc-950">
                // WonopUI Topbar for chat header
                <Topbar position={TopbarPosition::Relative}>
                    <TopbarStart>
                        <Avatar src="https://i.pravatar.cc/150?img=10" />
                        <div class="ml-3">
                            <h2 class="font-semibold text-zinc-900 dark:text-white">{"User 1"}</h2>
                            <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Online"}</p>
                        </div>
                    </TopbarStart>
                    <TopbarEnd>
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 0 0 2.25-2.25v-1.372c0-.516-.351-.966-.852-1.091l-4.423-1.106c-.44-.11-.902.055-1.173.417l-.97 1.293c-.282.376-.769.542-1.21.38a12.035 12.035 0 0 1-7.143-7.143c-.162-.441.004-.928.38-1.21l1.293-.97c.363-.271.527-.734.417-1.173L6.963 3.102a1.125 1.125 0 0 0-1.091-.852H4.5A2.25 2.25 0 0 0 2.25 4.5v2.25Z" />
                            </svg>
                        </Button>
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m15.75 10.5 4.72-4.72a.75.75 0 0 1 1.28.53v11.38a.75.75 0 0 1-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25h-9A2.25 2.25 0 0 0 2.25 7.5v9a2.25 2.25 0 0 0 2.25 2.25Z" />
                            </svg>
                        </Button>
                    </TopbarEnd>
                </Topbar>
                
                // Chat messages
                <div class="flex-1 overflow-auto p-6">
                    <div class="space-y-4">
                        <div class="flex justify-start">
                            <Card class="max-w-xs !bg-zinc-100 dark:!bg-zinc-800">
                                <CardContent class="!p-3">
                                    <p class="text-sm text-zinc-900 dark:text-white">{"Hey, how are you?"}</p>
                                </CardContent>
                            </Card>
                        </div>
                        <div class="flex justify-end">
                            <Card class="max-w-xs !bg-zinc-900 dark:!bg-zinc-100">
                                <CardContent class="!p-3">
                                    <p class="text-sm text-white dark:text-zinc-900">{"I'm doing great, thanks!"}</p>
                                </CardContent>
                            </Card>
                        </div>
                    </div>
                </div>
                
                // Message input
                <div class="border-t border-zinc-200 p-4 dark:border-zinc-800">
                    <div class="flex gap-2">
                        <Input placeholder="Type a message..." class="flex-1" />
                        <Button variant={ButtonVariant::Primary}>{"Send"}</Button>
                    </div>
                </div>
            </main>
        </div>
    }
}

// =============================================================================
// Code Constants - Complete, working examples
// =============================================================================

const SIDEBAR_FOLDING_CODE: &str = r##"use yew::prelude::*;
use wonopui::*;

/// Wrap your app or page in LayoutProvider for sidebar state management
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <LayoutProvider>
            <DashboardLayout />
        </LayoutProvider>
    }
}

#[function_component(DashboardLayout)]
pub fn dashboard_layout() -> Html {
    // Access layout context for sidebar state
    let layout = use_layout();
    let folded = layout.sidebar_folded;
    
    // Toggle callback
    let toggle_folded = {
        let layout = layout.clone();
        Callback::from(move |_: MouseEvent| {
            layout.dispatch(LayoutAction::ToggleSidebarFolded);
        })
    };
    
    let sidebar_header = html! {
        <SidebarHeader>
            <div class="flex items-center gap-2">
                <Logo />
                if !folded { <span class="font-semibold">{"Acme Inc"}</span> }
            </div>
        </SidebarHeader>
    };
    
    let sidebar_footer = html! {
        <SidebarFooter>
            <Avatar src="..." size={AvatarSize::Small} />
            if !folded {
                <div>
                    <p class="text-sm font-medium">{"Tom Cook"}</p>
                    <p class="text-xs text-zinc-500">{"tom@example.com"}</p>
                </div>
            }
        </SidebarFooter>
    };

    html! {
        <div class="flex h-screen">
            // Sidebar with folded prop from context
            <Sidebar 
                header={sidebar_header} 
                footer={sidebar_footer}
                folded={folded}
            >
                <SidebarMenu>
                    <SidebarItem icon={dashboard_icon()} active={true}>
                        if !folded { {"Dashboard"} }
                    </SidebarItem>
                    <SidebarItem icon={team_icon()}>
                        if !folded { {"Team"} }
                    </SidebarItem>
                    <SidebarItem icon={projects_icon()}>
                        if !folded { {"Projects"} }
                    </SidebarItem>
                </SidebarMenu>
            </Sidebar>
            
            <div class="flex flex-1 flex-col">
                <Topbar>
                    <TopbarStart>
                        // Toggle button to fold/unfold sidebar
                        <Button 
                            variant={ButtonVariant::Ghost} 
                            size={ButtonSize::Icon}
                            onclick={toggle_folded}
                        >
                            if folded { <ChevronRightIcon /> } else { <ChevronLeftIcon /> }
                        </Button>
                        <h1 class="text-lg font-semibold">{"Dashboard"}</h1>
                    </TopbarStart>
                    <TopbarEnd>
                        <Input placeholder="Search..." />
                        <Button variant={ButtonVariant::Primary}>{"New Project"}</Button>
                    </TopbarEnd>
                </Topbar>
                
                <main class="flex-1 p-6">
                    // Your content here
                </main>
            </div>
        </div>
    }
}
"##;

const MOBILE_SIDEBAR_CODE: &str = r##"use yew::prelude::*;
use wonopui::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <LayoutProvider>
            <MobileResponsiveLayout />
        </LayoutProvider>
    }
}

#[function_component(MobileResponsiveLayout)]
pub fn mobile_responsive_layout() -> Html {
    let layout = use_layout();
    let mobile_open = layout.mobile_menu_open;
    
    let toggle_mobile = {
        let layout = layout.clone();
        Callback::from(move |_| {
            layout.dispatch(LayoutAction::ToggleMobileMenu);
        })
    };
    
    let close_mobile = {
        let layout = layout.clone();
        Callback::from(move |_| {
            layout.dispatch(LayoutAction::SetMobileMenuOpen(false));
        })
    };

    html! {
        <div class="relative flex h-screen">
            // Backdrop overlay for mobile
            if mobile_open {
                <div 
                    class="fixed inset-0 bg-black/50 z-40 lg:hidden"
                    onclick={close_mobile.clone()}
                />
            }
            
            // Sidebar - slides in on mobile, always visible on desktop
            <div class={classes!(
                "fixed inset-y-0 left-0 z-50 lg:relative",
                "transition-transform duration-300",
                if mobile_open { "translate-x-0" } else { "-translate-x-full lg:translate-x-0" }
            )}>
                <Sidebar header={sidebar_header(close_mobile)} footer={sidebar_footer()}>
                    <SidebarMenu>
                        <SidebarItem icon={dashboard_icon()} onclick={close_mobile.clone()}>
                            {"Dashboard"}
                        </SidebarItem>
                        // ... more items
                    </SidebarMenu>
                </Sidebar>
            </div>
            
            <div class="flex flex-1 flex-col">
                <Topbar>
                    <TopbarStart>
                        // Hamburger button - only visible on mobile
                        <Button 
                            variant={ButtonVariant::Ghost} 
                            size={ButtonSize::Icon}
                            onclick={toggle_mobile}
                            class="lg:hidden"
                        >
                            <MenuIcon />
                        </Button>
                        <h1>{"Dashboard"}</h1>
                    </TopbarStart>
                </Topbar>
                <main class="flex-1 p-6">
                    // Your content
                </main>
            </div>
        </div>
    }
}
"##;

const STACKED_LAYOUT_CODE: &str = r##"use yew::prelude::*;
use wonopui::*;

#[function_component(StackedLayout)]
pub fn stacked_layout() -> Html {
    html! {
        <div class="flex min-h-screen flex-col">
            // Top navigation using Topbar
            <Topbar>
                <TopbarStart>
                    <Logo />
                    <nav class="hidden md:flex md:gap-6 ml-8">
                        <a href="#" class="text-sm font-medium">{"Dashboard"}</a>
                        <a href="#" class="text-sm text-zinc-500">{"Team"}</a>
                        <a href="#" class="text-sm text-zinc-500">{"Projects"}</a>
                    </nav>
                </TopbarStart>
                <TopbarEnd>
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                        <BellIcon />
                    </Button>
                    <Avatar src="..." size={AvatarSize::Small} />
                </TopbarEnd>
            </Topbar>
            
            // Page header
            <div class="border-b bg-white dark:bg-zinc-950">
                <div class="mx-auto max-w-7xl px-4 py-6">
                    <h1 class="text-2xl font-bold">{"Dashboard"}</h1>
                </div>
            </div>
            
            // Main content
            <main class="flex-1">
                <div class="mx-auto max-w-7xl px-4 py-6">
                    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                        {for items.iter().map(|item| html! {
                            <Card>
                                <CardHeader>
                                    <CardTitle>{item.title}</CardTitle>
                                </CardHeader>
                                <CardContent>{item.content}</CardContent>
                            </Card>
                        })}
                    </div>
                </div>
            </main>
        </div>
    }
}
"##;

const MULTI_COLUMN_LAYOUT_CODE: &str = r##"use yew::prelude::*;
use wonopui::*;

#[function_component(MultiColumnLayout)]
pub fn multi_column_layout() -> Html {
    html! {
        <div class="flex h-screen">
            // Narrow icon-only sidebar (always folded)
            <Sidebar class="!w-20 items-center" folded={true}>
                <Logo class="mt-4" />
                <SidebarMenu class="mt-8 items-center">
                    <SidebarItem icon={home_icon()} active={true} class="!justify-center" />
                    <SidebarItem icon={users_icon()} class="!justify-center" />
                    <SidebarItem icon={mail_icon()} class="!justify-center" />
                </SidebarMenu>
            </Sidebar>
            
            // List/index panel
            <aside class="w-80 border-r overflow-auto">
                <div class="sticky top-0 border-b bg-white p-4">
                    <Input placeholder="Search..." />
                </div>
                {for users.iter().map(|user| html! {
                    <div class="flex items-center gap-3 p-4 hover:bg-zinc-50">
                        <Avatar src={user.avatar} size={AvatarSize::Small} />
                        <div class="flex-1 min-w-0">
                            <p class="font-medium truncate">{user.name}</p>
                            <p class="text-sm text-zinc-500 truncate">{user.message}</p>
                        </div>
                        <Badge>{user.time}</Badge>
                    </div>
                })}
            </aside>
            
            // Detail/content panel
            <main class="flex flex-1 flex-col">
                <Topbar>
                    <TopbarStart>
                        <Avatar src={selected.avatar} />
                        <div class="ml-3">
                            <h2 class="font-semibold">{selected.name}</h2>
                            <p class="text-sm text-zinc-500">{"Online"}</p>
                        </div>
                    </TopbarStart>
                    <TopbarEnd>
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                            <PhoneIcon />
                        </Button>
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Icon}>
                            <VideoIcon />
                        </Button>
                    </TopbarEnd>
                </Topbar>
                
                // Chat/detail content
                <div class="flex-1 overflow-auto p-6">
                    // Messages...
                </div>
                
                // Input area
                <div class="border-t p-4">
                    <div class="flex gap-2">
                        <Input placeholder="Type a message..." class="flex-1" />
                        <Button variant={ButtonVariant::Primary}>{"Send"}</Button>
                    </div>
                </div>
            </main>
        </div>
    }
}
"##;
