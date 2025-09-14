# Wonop UI Sidebar Layout Tutorial

This tutorial will guide you through creating a basic layout with a collapsible sidebar using Wonop UI. The sidebar will support nested sections with primary and secondary navigation items, and can be toggled open/closed.

## Prerequisites

- A Yew project with Wonop UI installed
- Basic knowledge of Rust and Yew framework

## Step 1: Basic Setup

First, create a new Rust file for your application (e.g., `src/main.rs`) and add the necessary imports:

```rust
use wonopui::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
```

## Step 2: Define Your Routes

Create an enum for your application routes:

```rust
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Feed,
    #[at("/my-stuff")]
    MyStuff,
    #[at("/playground")]
    Playground,
    #[at("/learn")]
    Learn,
    #[at("/learn/courses")]
    Courses,
    #[at("/learn/ebooks")]
    Ebooks,
    #[at("/learn/downloads")]
    Downloads,
    #[at("/community")]
    Community,
    #[at("/community/announcements")]
    Announcements,
    #[at("/community/questions")]
    AskQuestions,
    #[at("/community/wins")]
    ShareWins,
    #[at("/profile")]
    UserMenu,
    #[at("/settings")]
    Settings,
}
```

## Step 3: Create the Main App Component

Create your main application component that will use the `Layout` and `Sidebar` components:

```rust
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <LayoutProvider>
                <Switch<Route> render={switch} />
            </LayoutProvider>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    html! {
        <MainLayout>
            {match routes {
                Route::Feed => html! { <FeedPage /> },
                Route::MyStuff => html! { <MyStuffPage /> },
                Route::Playground => html! { <PlaygroundPage /> },
                Route::Learn => html! { <LearnPage /> },
                Route::Courses => html! { <CoursesPage /> },
                Route::Ebooks => html! { <EbooksPage /> },
                Route::Downloads => html! { <DownloadsPage /> },
                Route::Community => html! { <CommunityPage /> },
                Route::Announcements => html! { <AnnouncementsPage /> },
                Route::AskQuestions => html! { <QuestionsPage /> },
                Route::ShareWins => html! { <WinsPage /> },
                Route::UserMenu => html! { <UserMenuPage /> },
                Route::Settings => html! { <SettingsPage /> },
            }}
        </MainLayout>
    }
}
```

## Step 4: Create the Main Layout with Sidebar

Now, create the main layout component with the collapsible sidebar:

```rust
#[function_component(MainLayout)]
fn main_layout(props: &MainLayoutProps) -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let sidebar_folded = layout_context.sidebar_folded;
    let mobile_menu_open = layout_context.mobile_menu_open;
    
    // Toggle sidebar fold state
    let toggle_sidebar = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            layout_context.dispatch(LayoutAction::SetSidebarFolded(!sidebar_folded));
        })
    };
    
    // Toggle mobile menu
    let toggle_mobile_menu = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(!mobile_menu_open));
        })
    };
    
    html! {
        <Layout>
            <Sidebar
                header={html!{
                    <SidebarHeader>
                        <div class="flex items-center justify-between p-4">
                            <span class="text-lg font-semibold">{"My App"}</span>
                            <button 
                                onclick={toggle_sidebar}
                                class="p-1 rounded hover:bg-gray-200 dark:hover:bg-zinc-700"
                            >
                                if sidebar_folded {
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                    </svg>
                                } else {
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                    </svg>
                                }
                            </button>
                        </div>
                    </SidebarHeader>
                }}
                footer={html!{
                    <SidebarFooter>
                        <div class="p-4 border-t border-gray-200 dark:border-zinc-700">
                            <SidebarLink<Route> 
                                label="User Menu" 
                                to={Route::UserMenu}
                                icon={html!{
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                    </svg>
                                }}
                            />
                        </div>
                    </SidebarFooter>
                }}
            >
                // Main navigation items
                <SidebarMenu>
                    <SidebarLink<Route> 
                        label="Feed" 
                        to={Route::Feed}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9.5a2 2 0 00-2-2h-2"/>
                            </svg>
                        }}
                    />
                    <SidebarLink<Route> 
                        label="My Stuff" 
                        to={Route::MyStuff}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"/>
                            </svg>
                        }}
                    />
                    <SidebarLink<Route> 
                        label="Playground" 
                        to={Route::Playground}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        }}
                    />
                </SidebarMenu>
                
                // Learn section with nested items
                <SidebarHeading>{"Learn"}</SidebarHeading>
                <SidebarMenu>
                    <SidebarLink<Route> 
                        label="Courses" 
                        to={Route::Courses}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
                            </svg>
                        }}
                    />
                    <SidebarLink<Route> 
                        label="Ebooks" 
                        to={Route::Ebooks}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
                            </svg>
                        }}
                    />
                    <SidebarLink<Route> 
                        label="Downloads" 
                        to={Route::Downloads}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
                            </svg>
                        }}
                    />
                </SidebarMenu>
                
                // Community Space section
                <SidebarHeading>{"Community Space"}</SidebarHeading>
                <SidebarMenu>
                    <SidebarLink<Route> 
                        label="Announcements" 
                        to={Route::Announcements}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>
                            </svg>
                        }}
                    />
                    <SidebarLink<Route> 
                        label="Ask questions" 
                        to={Route::AskQuestions}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        }}
                    />
                    <SidebarLink<Route> 
                        label="Share your wins" 
                        to={Route::ShareWins}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        }}
                    />
                </SidebarMenu>
                
                // Settings section
                <SidebarHeading>{"Settings"}</SidebarHeading>
                <SidebarMenu>
                    <SidebarLink<Route> 
                        label="User Settings" 
                        to={Route::Settings}
                        icon={html!{
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        }}
                    />
                </SidebarMenu>
            </Sidebar>
            
            // Main content area
            <div class="flex-1 flex flex-col">
                // Mobile menu toggle button
                <div class="lg:hidden flex items-center justify-between p-4 border-b border-gray-200 dark:border-zinc-700">
                    <span class="text-lg font-semibold">{"My App"}</span>
                    <button 
                        onclick={toggle_mobile_menu}
                        class="p-2 rounded hover:bg-gray-200 dark:hover:bg-zinc-700"
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    </button>
                </div>
                
                // Page content
                <main class="flex-1 p-6">
                    {props.children.clone()}
                </main>
            </div>
        </Layout>
    }
}

#[derive(Properties, PartialEq)]
struct MainLayoutProps {
    pub children: Children,
}
```

## Step 5: Create Page Components

Create simple page components for each route:

```rust
#[function_component(FeedPage)]
fn feed_page() -> Html {
    html! {
        <Container>
            <H1>{"Feed"}</H1>
            <Paragraph>{"Welcome to your feed. Here you'll see updates and news."}</Paragraph>
        </Container>
    }
}

#[function_component(MyStuffPage)]
fn my_stuff_page() -> Html {
    html! {
        <Container>
            <H1>{"My Stuff"}</H1>
            <Paragraph>{"Your personal content and items."}</Paragraph>
        </Container>
    }
}

#[function_component(PlaygroundPage)]
fn playground_page() -> Html {
    html! {
        <Container>
            <H1>{"Playground"}</H1>
            <Paragraph>{"Experiment and play with different features here."}</Paragraph>
        </Container>
    }
}

#[function_component(CoursesPage)]
fn courses_page() -> Html {
    html! {
        <Container>
            <H1>{"Courses"}</H1>
            <Paragraph>{"Browse available courses and learning materials."}</Paragraph>
        </Container>
    }
}

// Add similar components for other pages...

#[function_component(SettingsPage)]
fn settings_page() -> Html {
    html! {
        <Container>
            <H1>{"User Settings"}</H1>
            <Paragraph>{"Manage your account settings, billing information, and subscription preferences."}</Paragraph>
            
            <div class="mt-6">
                <Tabs default_value="account">
                    <TabsList>
                        <TabsTrigger value="account">
                            <div class="flex items-center space-x-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                </svg>
                                <span>{"Account Settings"}</span>
                            </div>
                        </TabsTrigger>
                        <TabsTrigger value="billing">
                            <div class="flex items-center space-x-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"/>
                                </svg>
                                <span>{"Billing"}</span>
                            </div>
                        </TabsTrigger>
                        <TabsTrigger value="subscriptions">
                            <div class="flex items-center space-x-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                <span>{"Subscriptions"}</span>
                            </div>
                        </TabsTrigger>
                    </TabsList>
                    
                    <TabsContent value="account">
                        <Card>
                            <CardHeader>
                                <H2>{"Account Settings"}</H2>
                                <Paragraph>{"Update your account information and preferences"}</Paragraph>
                            </CardHeader>
                            <CardContent>
                                <div class="space-y-4">
                                    <div>
                                        <Label>{"Username"}</Label>
                                        <Input 
                                            placeholder="Enter your username" 
                                            value="john_doe"
                                        />
                                    </div>
                                    <div>
                                        <Label>{"Email"}</Label>
                                        <Input 
                                            input_type={InputType::Email}
                                            placeholder="Enter your email" 
                                            value="john@example.com"
                                        />
                                    </div>
                                    <div>
                                        <Label>{"Display Name"}</Label>
                                        <Input 
                                            placeholder="Enter your display name" 
                                            value="John Doe"
                                        />
                                    </div>
                                    <div>
                                        <Label>{"Bio"}</Label>
                                        <Textarea 
                                            placeholder="Tell us about yourself"
                                            rows={4}
                                        />
                                    </div>
                                    <div class="flex items-center space-x-2">
                                        <Switch id="notifications" />
                                        <Label html_for="notifications">{"Email notifications"}</Label>
                                    </div>
                                    <div class="flex items-center space-x-2">
                                        <Switch id="newsletter" />
                                        <Label html_for="newsletter">{"Subscribe to newsletter"}</Label>
                                    </div>
                                </div>
                            </CardContent>
                            <CardFooter>
                                <Button variant={ButtonVariant::Primary}>{"Save Changes"}</Button>
                                <Button variant={ButtonVariant::Ghost}>{"Cancel"}</Button>
                            </CardFooter>
                        </Card>
                    </TabsContent>
                    
                    <TabsContent value="billing">
                        <Card>
                            <CardHeader>
                                <H2>{"Billing Information"}</H2>
                                <Paragraph>{"Manage your payment methods and billing details"}</Paragraph>
                            </CardHeader>
                            <CardContent>
                                <div class="space-y-6">
                                    // Current Plan
                                    <div class="p-4 bg-gray-50 dark:bg-zinc-800 rounded-lg">
                                        <H3>{"Current Plan"}</H3>
                                        <div class="mt-2 flex items-center justify-between">
                                            <div>
                                                <p class="text-lg font-semibold">{"Pro Plan"}</p>
                                                <p class="text-sm text-gray-600 dark:text-zinc-400">{"$29/month"}</p>
                                            </div>
                                            <Badge variant={BadgeVariant::Success}>{"Active"}</Badge>
                                        </div>
                                    </div>
                                    
                                    // Payment Method
                                    <div>
                                        <H3>{"Payment Method"}</H3>
                                        <div class="mt-3 space-y-3">
                                            <div class="flex items-center justify-between p-3 border rounded-lg">
                                                <div class="flex items-center space-x-3">
                                                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"/>
                                                    </svg>
                                                    <div>
                                                        <p class="font-medium">{"•••• •••• •••• 4242"}</p>
                                                        <p class="text-sm text-gray-600 dark:text-zinc-400">{"Expires 12/24"}</p>
                                                    </div>
                                                </div>
                                                <Button variant={ButtonVariant::Ghost} size={ButtonSize::Small}>{"Edit"}</Button>
                                            </div>
                                        </div>
                                        <Button variant={ButtonVariant::Secondary} class="mt-3">
                                            {"Add Payment Method"}
                                        </Button>
                                    </div>
                                    
                                    // Billing Address
                                    <div>
                                        <H3>{"Billing Address"}</H3>
                                        <div class="mt-3 space-y-3">
                                            <div class="grid grid-cols-2 gap-3">
                                                <div>
                                                    <Label>{"First Name"}</Label>
                                                    <Input value="John" />
                                                </div>
                                                <div>
                                                    <Label>{"Last Name"}</Label>
                                                    <Input value="Doe" />
                                                </div>
                                            </div>
                                            <div>
                                                <Label>{"Address"}</Label>
                                                <Input placeholder="123 Main St" />
                                            </div>
                                            <div class="grid grid-cols-2 gap-3">
                                                <div>
                                                    <Label>{"City"}</Label>
                                                    <Input placeholder="New York" />
                                                </div>
                                                <div>
                                                    <Label>{"ZIP Code"}</Label>
                                                    <Input placeholder="10001" />
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </CardContent>
                            <CardFooter>
                                <Button variant={ButtonVariant::Primary}>{"Update Billing"}</Button>
                            </CardFooter>
                        </Card>
                    </TabsContent>
                    
                    <TabsContent value="subscriptions">
                        <Card>
                            <CardHeader>
                                <H2>{"Subscriptions"}</H2>
                                <Paragraph>{"Manage your subscription plans and features"}</Paragraph>
                            </CardHeader>
                            <CardContent>
                                <div class="space-y-6">
                                    // Available Plans
                                    <div class="grid gap-4">
                                        // Basic Plan
                                        <div class="border rounded-lg p-4">
                                            <div class="flex items-center justify-between mb-3">
                                                <div>
                                                    <H3>{"Basic"}</H3>
                                                    <p class="text-2xl font-bold mt-1">{"$9"}<span class="text-sm font-normal text-gray-600 dark:text-zinc-400">{"/month"}</span></p>
                                                </div>
                                            </div>
                                            <ul class="space-y-2 mb-4">
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"5 Projects"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Basic Support"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"1GB Storage"}
                                                </li>
                                            </ul>
                                            <Button variant={ButtonVariant::Secondary} class="w-full">{"Downgrade"}</Button>
                                        </div>
                                        
                                        // Pro Plan (Current)
                                        <div class="border-2 border-blue-500 rounded-lg p-4 relative">
                                            <Badge variant={BadgeVariant::Primary} class="absolute top-2 right-2">{"Current Plan"}</Badge>
                                            <div class="flex items-center justify-between mb-3">
                                                <div>
                                                    <H3>{"Pro"}</H3>
                                                    <p class="text-2xl font-bold mt-1">{"$29"}<span class="text-sm font-normal text-gray-600 dark:text-zinc-400">{"/month"}</span></p>
                                                </div>
                                            </div>
                                            <ul class="space-y-2 mb-4">
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Unlimited Projects"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Priority Support"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"50GB Storage"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Advanced Analytics"}
                                                </li>
                                            </ul>
                                            <Button variant={ButtonVariant::Primary} class="w-full" disabled={true}>{"Current Plan"}</Button>
                                        </div>
                                        
                                        // Enterprise Plan
                                        <div class="border rounded-lg p-4">
                                            <div class="flex items-center justify-between mb-3">
                                                <div>
                                                    <H3>{"Enterprise"}</H3>
                                                    <p class="text-2xl font-bold mt-1">{"$99"}<span class="text-sm font-normal text-gray-600 dark:text-zinc-400">{"/month"}</span></p>
                                                </div>
                                            </div>
                                            <ul class="space-y-2 mb-4">
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Everything in Pro"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Dedicated Support"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Unlimited Storage"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"Custom Integrations"}
                                                </li>
                                                <li class="flex items-center text-sm">
                                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                    {"SLA Guarantee"}
                                                </li>
                                            </ul>
                                            <Button variant={ButtonVariant::Primary} class="w-full">{"Upgrade"}</Button>
                                        </div>
                                    </div>
                                    
                                    // Subscription Settings
                                    <div class="border-t pt-6">
                                        <H3>{"Subscription Settings"}</H3>
                                        <div class="mt-4 space-y-3">
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <p class="font-medium">{"Auto-renewal"}</p>
                                                    <p class="text-sm text-gray-600 dark:text-zinc-400">{"Automatically renew subscription"}</p>
                                                </div>
                                                <Switch checked={true} />
                                            </div>
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <p class="font-medium">{"Usage alerts"}</p>
                                                    <p class="text-sm text-gray-600 dark:text-zinc-400">{"Get notified when reaching limits"}</p>
                                                </div>
                                                <Switch checked={true} />
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="flex items-center justify-between pt-4 border-t">
                                        <Button variant={ButtonVariant::Destructive}>{"Cancel Subscription"}</Button>
                                        <Button variant={ButtonVariant::Ghost}>{"Download Invoice"}</Button>
                                    </div>
                                </div>
                            </CardContent>
                        </Card>
                    </TabsContent>
                </Tabs>
            </div>
        </Container>
    }
}
```

## Step 6: Advanced Features

### Collapsible Sections

You can make sections collapsible by using state management:

```rust
#[function_component(CollapsibleSection)]
fn collapsible_section(props: &CollapsibleSectionProps) -> Html {
    let expanded = use_state(|| true);
    let toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };
    
    html! {
        <>
            <button 
                onclick={toggle}
                class="flex items-center justify-between w-full p-2 hover:bg-gray-100 dark:hover:bg-zinc-800 rounded"
            >
                <SidebarHeading>{props.title.clone()}</SidebarHeading>
                <svg 
                    class={classes!("w-4", "h-4", "transition-transform", if *expanded { "rotate-180" } else { "" })}
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                </svg>
            </button>
            if *expanded {
                <div class="ml-2">
                    {props.children.clone()}
                </div>
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct CollapsibleSectionProps {
    pub title: String,
    pub children: Children,
}
```

### Custom Sidebar State

You can control the sidebar state programmatically:

```rust
// In any component
let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");

// Toggle sidebar
layout_context.dispatch(LayoutAction::SetSidebarFolded(true));

// Toggle mobile menu
layout_context.dispatch(LayoutAction::SetMobileMenuOpen(false));
```

## Step 7: Styling and Customization

Wonop UI components support custom styling through the `class` prop:

```rust
<Sidebar class="bg-gray-50 dark:bg-zinc-900 border-r border-gray-200 dark:border-zinc-700">
    // sidebar content
</Sidebar>
```

You can also customize individual sidebar components:

```rust
<SidebarItem 
    class="hover:bg-blue-50 dark:hover:bg-blue-900/20"
    active={true}
>
    <div class="flex items-center space-x-2">
        <span class="w-2 h-2 bg-green-500 rounded-full"></span>
        <span>{"Active Item"}</span>
    </div>
</SidebarItem>
```

## Complete Example

Here's a minimal complete example to get you started:

```rust
use wonopui::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <LayoutProvider>
                <Layout>
                    <Sidebar>
                        <SidebarHeader>
                            <div class="p-4">
                                <h1 class="text-xl font-bold">{"My App"}</h1>
                            </div>
                        </SidebarHeader>
                        <SidebarMenu>
                            <SidebarLink<Route> label="Home" to={Route::Home} />
                            <SidebarLink<Route> label="About" to={Route::About} />
                        </SidebarMenu>
                    </Sidebar>
                    <main class="flex-1 p-6">
                        <Switch<Route> render={switch} />
                    </main>
                </Layout>
            </LayoutProvider>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Welcome Home!"}</h1> },
        Route::About => html! { <h1>{"About Us"}</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## Tips and Best Practices

1. **Use LayoutProvider**: Always wrap your app with `LayoutProvider` to enable sidebar state management.

2. **Responsive Design**: The sidebar automatically adapts to mobile screens. Use the mobile menu toggle for better mobile UX.

3. **Icons**: Use consistent icon sizes (w-5 h-5) for better visual alignment.

4. **Accessibility**: Add proper ARIA labels and keyboard navigation support.

5. **State Management**: Use the `LayoutContext` for global sidebar state and local state for component-specific behavior.

## Conclusion

This tutorial covered creating a functional sidebar layout with Wonop UI, including:
- Basic sidebar setup with navigation items
- Nested sections with headings
- Collapsible/foldable sidebar functionality
- Mobile responsive behavior
- Custom styling options

You can extend this foundation by adding more complex features like search, notifications, or user profiles in the sidebar.