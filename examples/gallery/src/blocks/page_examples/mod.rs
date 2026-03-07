//! Page Examples - Full page layouts including dashboards, detail views, settings

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_badge::Badge;
use wonopui::wonopui_input::Input;
use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent, CardDescription};
use wonopui::wonopui_switch::SwitchButton;
use crate::blocks::BlockPreview;

/// Page Examples category page
#[function_component(PageExamplesBlocks)]
pub fn page_examples_blocks() -> Html {
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
                    {"Page Examples"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Full page layouts including dashboards, detail views, and settings pages."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                <BlockPreview 
                    title="Dashboard"
                    description="Analytics dashboard with stats, charts, and recent activity."
                    code={DASHBOARD_CODE}
                    min_height={600}
                >
                    <DashboardExample />
                </BlockPreview>
                
                <BlockPreview 
                    title="User Profile"
                    description="User profile page with tabs for different sections."
                    code={USER_PROFILE_CODE}
                    min_height={500}
                >
                    <UserProfileExample />
                </BlockPreview>
                
                <BlockPreview 
                    title="Settings"
                    description="Settings page with form sections and save actions."
                    code={SETTINGS_CODE}
                    min_height={600}
                >
                    <SettingsExample />
                </BlockPreview>
            </div>
        </Container>
    }
}

// =============================================================================
// Example Components
// =============================================================================

#[function_component(DashboardExample)]
fn dashboard_example() -> Html {
    html! {
        <div class="min-h-[600px] bg-zinc-50 p-6 dark:bg-zinc-900">
            // Page header
            <div class="mb-8 flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-bold text-zinc-900 dark:text-white">{"Dashboard"}</h1>
                    <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Welcome back, here's what's happening."}</p>
                </div>
                <div class="flex items-center gap-4">
                    <Input placeholder="Search..." class="w-64" />
                    <Button variant={ButtonVariant::Default}>{"Download Report"}</Button>
                </div>
            </div>
            
            // Stats grid using Card component
            <div class="mb-8 grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
                <StatCard title="Total Revenue" value="$45,231.89" change="+20.1%" positive={true} />
                <StatCard title="Subscriptions" value="+2,350" change="+180.1%" positive={true} />
                <StatCard title="Sales" value="+12,234" change="+19%" positive={true} />
                <StatCard title="Active Now" value="+573" change="-4%" positive={false} />
            </div>
            
            // Charts and activity
            <div class="grid gap-6 lg:grid-cols-7">
                // Chart placeholder
                <Card class="lg:col-span-4">
                    <CardHeader>
                        <CardTitle>{"Overview"}</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="flex h-64 items-end justify-around gap-2">
                            {for [40, 65, 45, 80, 55, 70, 90, 60, 75, 50, 85, 95].iter().map(|h| html! {
                                <div 
                                    class="w-full max-w-8 rounded-t bg-zinc-900 dark:bg-zinc-100" 
                                    style={format!("height: {}%;", h)}
                                ></div>
                            })}
                        </div>
                    </CardContent>
                </Card>
                
                // Recent sales
                <Card class="lg:col-span-3">
                    <CardHeader>
                        <CardTitle>{"Recent Sales"}</CardTitle>
                        <CardDescription>{"You made 265 sales this month."}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div class="space-y-6">
                            {for [
                                ("Olivia Martin", "olivia@email.com", "+$1,999.00", 30),
                                ("Jackson Lee", "jackson@email.com", "+$39.00", 31),
                                ("Isabella Nguyen", "isabella@email.com", "+$299.00", 32),
                                ("William Kim", "will@email.com", "+$99.00", 33),
                            ].iter().map(|(name, email, amount, img)| html! {
                                <div class="flex items-center gap-4">
                                    <Avatar src={format!("https://i.pravatar.cc/150?img={}", img)} size={AvatarSize::Small} />
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm font-medium text-zinc-900 truncate dark:text-white">{name}</p>
                                        <p class="text-xs text-zinc-500 truncate dark:text-zinc-400">{email}</p>
                                    </div>
                                    <span class="text-sm font-medium text-zinc-900 dark:text-white">{amount}</span>
                                </div>
                            })}
                        </div>
                    </CardContent>
                </Card>
            </div>
        </div>
    }
}

/// Stat card component using Card
#[derive(Properties, PartialEq)]
struct StatCardProps {
    title: &'static str,
    value: &'static str,
    change: &'static str,
    positive: bool,
}

#[function_component(StatCard)]
fn stat_card(props: &StatCardProps) -> Html {
    html! {
        <Card>
            <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle class="text-sm font-medium">{props.title}</CardTitle>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4 text-zinc-400">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                </svg>
            </CardHeader>
            <CardContent>
                <div class="flex items-baseline gap-2">
                    <span class="text-2xl font-bold text-zinc-900 dark:text-white">{props.value}</span>
                    <span class={classes!(
                        "text-xs", "font-medium",
                        if props.positive { "text-green-600" } else { "text-red-600" }
                    )}>
                        {props.change}
                    </span>
                </div>
            </CardContent>
        </Card>
    }
}

#[function_component(UserProfileExample)]
fn user_profile_example() -> Html {
    html! {
        <div class="min-h-[500px] p-6">
            // Profile header
            <div class="mb-8 flex items-start gap-6">
                <Avatar src="https://i.pravatar.cc/150?img=35" size={AvatarSize::Large} />
                <div class="flex-1">
                    <div class="flex items-center gap-4">
                        <h1 class="text-2xl font-bold text-zinc-900 dark:text-white">{"Sarah Johnson"}</h1>
                        <Badge>{"Pro"}</Badge>
                    </div>
                    <p class="mt-1 text-zinc-500 dark:text-zinc-400">{"sarah.johnson@example.com"}</p>
                    <p class="mt-2 max-w-lg text-sm text-zinc-600 dark:text-zinc-400">
                        {"Product designer with 8+ years of experience. Passionate about creating beautiful, functional interfaces that delight users."}
                    </p>
                    <div class="mt-4 flex items-center gap-6">
                        <div class="flex items-center gap-2 text-sm text-zinc-500 dark:text-zinc-400">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 10.5a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1 1 15 0Z" />
                            </svg>
                            {"San Francisco, CA"}
                        </div>
                        <div class="flex items-center gap-2 text-sm text-zinc-500 dark:text-zinc-400">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244" />
                            </svg>
                            {"sarahjohnson.design"}
                        </div>
                    </div>
                </div>
                <Button variant={ButtonVariant::Outline}>{"Edit Profile"}</Button>
            </div>
            
            // Tabs
            <Tabs default_value="projects">
                <TabsList class="border-b border-zinc-200 dark:border-zinc-800">
                    <TabsTrigger value="projects" class="px-4 py-2">{"Projects"}</TabsTrigger>
                    <TabsTrigger value="activity" class="px-4 py-2">{"Activity"}</TabsTrigger>
                    <TabsTrigger value="settings" class="px-4 py-2">{"Settings"}</TabsTrigger>
                </TabsList>
                
                <TabsContent value="projects" class="pt-6">
                    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                        <ProjectCard 
                            title="Mobile App Redesign" 
                            description="A complete redesign of our mobile application" 
                            progress={85} 
                        />
                        <ProjectCard 
                            title="Brand Guidelines" 
                            description="Creating comprehensive brand documentation" 
                            progress={100} 
                        />
                        <ProjectCard 
                            title="Dashboard UI" 
                            description="Analytics dashboard for enterprise clients" 
                            progress={60} 
                        />
                    </div>
                </TabsContent>
                
                <TabsContent value="activity" class="pt-6">
                    <div class="text-zinc-500 dark:text-zinc-400">{"Activity feed coming soon..."}</div>
                </TabsContent>
                
                <TabsContent value="settings" class="pt-6">
                    <div class="text-zinc-500 dark:text-zinc-400">{"Settings panel coming soon..."}</div>
                </TabsContent>
            </Tabs>
        </div>
    }
}

/// Project card component using Card
#[derive(Properties, PartialEq)]
struct ProjectCardProps {
    title: &'static str,
    description: &'static str,
    progress: u32,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle class="text-base">{props.title}</CardTitle>
                <CardDescription>{props.description}</CardDescription>
            </CardHeader>
            <CardContent>
                <div class="flex justify-between text-sm">
                    <span class="text-zinc-500 dark:text-zinc-400">{"Progress"}</span>
                    <span class="font-medium text-zinc-900 dark:text-white">{format!("{}%", props.progress)}</span>
                </div>
                <div class="mt-2 h-2 w-full overflow-hidden rounded-full bg-zinc-100 dark:bg-zinc-800">
                    <div 
                        class="h-full rounded-full bg-zinc-900 dark:bg-zinc-100" 
                        style={format!("width: {}%;", props.progress)}
                    ></div>
                </div>
            </CardContent>
        </Card>
    }
}

#[function_component(SettingsExample)]
fn settings_example() -> Html {
    // State for switches
    let email_notifications = use_state(|| true);
    let push_notifications = use_state(|| true);
    let marketing_emails = use_state(|| false);
    
    let on_email_toggle = {
        let email_notifications = email_notifications.clone();
        Callback::from(move |_: MouseEvent| {
            email_notifications.set(!*email_notifications);
        })
    };
    
    let on_push_toggle = {
        let push_notifications = push_notifications.clone();
        Callback::from(move |_: MouseEvent| {
            push_notifications.set(!*push_notifications);
        })
    };
    
    let on_marketing_toggle = {
        let marketing_emails = marketing_emails.clone();
        Callback::from(move |_: MouseEvent| {
            marketing_emails.set(!*marketing_emails);
        })
    };
    
    html! {
        <div class="min-h-[600px] p-6">
            <div class="mx-auto max-w-2xl">
                // Header
                <div class="mb-8">
                    <h1 class="text-2xl font-bold text-zinc-900 dark:text-white">{"Settings"}</h1>
                    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
                        {"Manage your account settings and preferences."}
                    </p>
                </div>
                
                // Profile section using Card
                <Card class="mb-8">
                    <CardHeader>
                        <CardTitle>{"Profile"}</CardTitle>
                        <CardDescription>{"This information will be displayed publicly."}</CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-6">
                        <div class="flex items-center gap-6">
                            <Avatar src="https://i.pravatar.cc/150?img=35" size={AvatarSize::Large} />
                            <div class="flex gap-4">
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>
                                    {"Change avatar"}
                                </Button>
                                <Button variant={ButtonVariant::Ghost} size={ButtonSize::Small}>
                                    {"Remove"}
                                </Button>
                            </div>
                        </div>
                        
                        <div class="grid gap-6 sm:grid-cols-2">
                            <div>
                                <label class="mb-2 block text-sm font-medium text-zinc-900 dark:text-white">
                                    {"First name"}
                                </label>
                                <Input value="Sarah" />
                            </div>
                            <div>
                                <label class="mb-2 block text-sm font-medium text-zinc-900 dark:text-white">
                                    {"Last name"}
                                </label>
                                <Input value="Johnson" />
                            </div>
                        </div>
                        
                        <div>
                            <label class="mb-2 block text-sm font-medium text-zinc-900 dark:text-white">
                                {"Email"}
                            </label>
                            <Input value="sarah.johnson@example.com" kind="email" />
                        </div>
                        
                        <div>
                            <label class="mb-2 block text-sm font-medium text-zinc-900 dark:text-white">
                                {"Bio"}
                            </label>
                            <textarea 
                                rows="3"
                                class="w-full rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm placeholder:text-zinc-400 focus:outline-none focus:ring-2 focus:ring-zinc-900 focus:ring-offset-2 dark:border-zinc-800 dark:bg-zinc-950 dark:placeholder:text-zinc-500 dark:focus:ring-zinc-100"
                            >
                                {"Product designer with 8+ years of experience."}
                            </textarea>
                        </div>
                    </CardContent>
                </Card>
                
                // Notifications section using Card and Switch
                <Card class="mb-8">
                    <CardHeader>
                        <CardTitle>{"Notifications"}</CardTitle>
                        <CardDescription>{"Choose what notifications you want to receive."}</CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-zinc-900 dark:text-white">{"Email notifications"}</p>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Receive email updates about your account."}</p>
                            </div>
                            <SwitchButton 
                                checked={Some(*email_notifications)} 
                                on_toggle={on_email_toggle}
                            />
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-zinc-900 dark:text-white">{"Push notifications"}</p>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Receive push notifications on your device."}</p>
                            </div>
                            <SwitchButton 
                                checked={Some(*push_notifications)} 
                                on_toggle={on_push_toggle}
                            />
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-zinc-900 dark:text-white">{"Marketing emails"}</p>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Receive emails about new features and offers."}</p>
                            </div>
                            <SwitchButton 
                                checked={Some(*marketing_emails)} 
                                on_toggle={on_marketing_toggle}
                            />
                        </div>
                    </CardContent>
                </Card>
                
                // Save button
                <div class="flex justify-end gap-4">
                    <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                    <Button variant={ButtonVariant::Default}>{"Save changes"}</Button>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const DASHBOARD_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent, CardDescription};
use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_input::Input;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="min-h-screen bg-zinc-50 p-6">
            // Page header
            <div class="mb-8 flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-bold">{"Dashboard"}</h1>
                    <p class="text-sm text-zinc-500">{"Welcome back"}</p>
                </div>
                <Button>{"Download Report"}</Button>
            </div>
            
            // Stats grid using Card
            <div class="mb-8 grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
                <Card>
                    <CardHeader class="pb-2">
                        <CardTitle class="text-sm font-medium">{"Revenue"}</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold">{"$45,231"}</div>
                        <span class="text-xs text-green-600">{"+20.1%"}</span>
                    </CardContent>
                </Card>
                // ... more cards
            </div>
            
            // Charts and activity
            <div class="grid gap-6 lg:grid-cols-7">
                <Card class="lg:col-span-4">
                    <CardHeader>
                        <CardTitle>{"Overview"}</CardTitle>
                    </CardHeader>
                    <CardContent>
                        // Chart content
                    </CardContent>
                </Card>
                <Card class="lg:col-span-3">
                    <CardHeader>
                        <CardTitle>{"Recent Sales"}</CardTitle>
                        <CardDescription>{"265 sales this month"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        // Sales list
                    </CardContent>
                </Card>
            </div>
        </div>
    }
}
"##;

const USER_PROFILE_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_badge::Badge;
use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};

#[function_component(UserProfile)]
pub fn user_profile() -> Html {
    html! {
        <div class="min-h-screen p-6">
            // Profile header
            <div class="mb-8 flex items-start gap-6">
                <Avatar src="avatar.jpg" size={AvatarSize::Large} />
                <div class="flex-1">
                    <h1 class="text-2xl font-bold">{"Sarah Johnson"}</h1>
                    <Badge>{"Pro"}</Badge>
                    <p class="mt-1 text-zinc-500">{"sarah@example.com"}</p>
                </div>
                <Button variant={ButtonVariant::Outline}>{"Edit Profile"}</Button>
            </div>
            
            // Tabs
            <Tabs default_value="projects">
                <TabsList>
                    <TabsTrigger value="projects">{"Projects"}</TabsTrigger>
                    <TabsTrigger value="activity">{"Activity"}</TabsTrigger>
                </TabsList>
                
                <TabsContent value="projects">
                    // Project cards using Card component
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Mobile App Redesign"}</CardTitle>
                            <CardDescription>{"Complete redesign"}</CardDescription>
                        </CardHeader>
                        <CardContent>
                            // Progress bar
                        </CardContent>
                    </Card>
                </TabsContent>
            </Tabs>
        </div>
    }
}
"##;

const SETTINGS_CODE: &str = r##"use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_switch::SwitchButton;
use wonopui::wonopui_input::Input;
use wonopui::wonopui_button::{Button, ButtonVariant};

#[function_component(Settings)]
pub fn settings() -> Html {
    let email_notifications = use_state(|| true);
    let on_toggle = {
        let state = email_notifications.clone();
        Callback::from(move |_| state.set(!*state))
    };
    
    html! {
        <div class="min-h-screen p-6">
            <div class="mx-auto max-w-2xl">
                <h1 class="text-2xl font-bold">{"Settings"}</h1>
                
                // Profile section using Card
                <Card class="mt-6">
                    <CardHeader>
                        <CardTitle>{"Profile"}</CardTitle>
                        <CardDescription>{"Public information"}</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div class="grid gap-6 sm:grid-cols-2">
                            <Input label="First name" />
                            <Input label="Last name" />
                        </div>
                        <Input label="Email" type="email" />
                    </CardContent>
                </Card>
                
                // Notifications using Card and Switch
                <Card class="mt-6">
                    <CardHeader>
                        <CardTitle>{"Notifications"}</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="font-medium">{"Email notifications"}</p>
                                <p class="text-sm text-zinc-500">{"Receive updates"}</p>
                            </div>
                            <SwitchButton 
                                checked={Some(*email_notifications)} 
                                on_toggle={on_toggle}
                            />
                        </div>
                    </CardContent>
                </Card>
                
                <div class="mt-6 flex justify-end gap-4">
                    <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                    <Button>{"Save changes"}</Button>
                </div>
            </div>
        </div>
    }
}
"##;
