//! UI Blocks showcase - pre-built application layouts and designs
//! Similar to Tailwind Plus UI Blocks

use wonopui::*;
use wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui_avatar::{Avatar, AvatarSize};
use wonopui_badge::Badge;
use wonopui_input::Input;
use wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
use yew::prelude::*;

#[function_component(UIBlocksDocumentation)]
pub fn ui_blocks_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <div class="space-y-12">
                // Header
                <div class="border-b border-zinc-200 dark:border-zinc-800 pb-8">
                    <h1 class="text-4xl font-bold text-zinc-900 dark:text-white mb-3">
                        { "UI Blocks" }
                    </h1>
                    <p class="text-lg text-zinc-600 dark:text-zinc-400 max-w-3xl">
                        { "Pre-built application layouts and design patterns. Use these as starting points for your own projects." }
                    </p>
                </div>

                // Application Shells Section
                <section class="space-y-6">
                    <div>
                        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-white mb-2">
                            { "Application Shells" }
                        </h2>
                        <p class="text-zinc-600 dark:text-zinc-400">
                            { "Complete application layouts with navigation, headers, and content areas." }
                        </p>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <BlockPreview
                            title="Dashboard Layout"
                            description="A classic dashboard with sidebar navigation"
                            code={DASHBOARD_SHELL_CODE}
                        >
                            <DashboardShell />
                        </BlockPreview>

                        <BlockPreview
                            title="Stacked Layout"
                            description="Mobile-first layout with stacked navigation"
                            code={STACKED_SHELL_CODE}
                        >
                            <StackedShell />
                        </BlockPreview>
                    </div>
                </section>

                // Hero Sections
                <section class="space-y-6">
                    <div>
                        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-white mb-2">
                            { "Hero Sections" }
                        </h2>
                        <p class="text-zinc-600 dark:text-zinc-400">
                            { "Eye-catching hero sections for landing pages." }
                        </p>
                    </div>

                    <div class="space-y-6">
                        <BlockPreview
                            title="Centered Hero"
                            description="Simple centered hero with CTA"
                            code={CENTERED_HERO_CODE}
                        >
                            <CenteredHero />
                        </BlockPreview>

                        <BlockPreview
                            title="Split Hero"
                            description="Hero with content and image side by side"
                            code={SPLIT_HERO_CODE}
                        >
                            <SplitHero />
                        </BlockPreview>
                    </div>
                </section>

                // Feature Sections
                <section class="space-y-6">
                    <div>
                        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-white mb-2">
                            { "Feature Sections" }
                        </h2>
                        <p class="text-zinc-600 dark:text-zinc-400">
                            { "Showcase your product features with these layouts." }
                        </p>
                    </div>

                    <BlockPreview
                        title="Feature Grid"
                        description="Grid of features with icons"
                        code={FEATURE_GRID_CODE}
                    >
                        <FeatureGrid />
                    </BlockPreview>
                </section>

                // Pricing Sections
                <section class="space-y-6">
                    <div>
                        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-white mb-2">
                            { "Pricing" }
                        </h2>
                        <p class="text-zinc-600 dark:text-zinc-400">
                            { "Pricing tables and cards for SaaS products." }
                        </p>
                    </div>

                    <BlockPreview
                        title="Pricing Cards"
                        description="Three-tier pricing layout"
                        code={PRICING_SECTION_CODE}
                    >
                        <PricingSection />
                    </BlockPreview>
                </section>

                // Team Sections
                <section class="space-y-6">
                    <div>
                        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-white mb-2">
                            { "Team" }
                        </h2>
                        <p class="text-zinc-600 dark:text-zinc-400">
                            { "Show off your team members." }
                        </p>
                    </div>

                    <BlockPreview
                        title="Team Grid"
                        description="Team members in a grid layout"
                        code={TEAM_GRID_CODE}
                    >
                        <TeamGrid />
                    </BlockPreview>
                </section>

                // Forms
                <section class="space-y-6">
                    <div>
                        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-white mb-2">
                            { "Forms" }
                        </h2>
                        <p class="text-zinc-600 dark:text-zinc-400">
                            { "Common form layouts and patterns." }
                        </p>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <BlockPreview
                            title="Sign In"
                            description="Simple sign in form"
                            code={SIGN_IN_FORM_CODE}
                        >
                            <SignInForm />
                        </BlockPreview>

                        <BlockPreview
                            title="Contact Form"
                            description="Contact form with validation"
                            code={CONTACT_FORM_CODE}
                        >
                            <ContactForm />
                        </BlockPreview>
                    </div>
                </section>
            </div>
        </Container>
    }
}

// ============================================================================
// Code snippets for each block
// ============================================================================

const DASHBOARD_SHELL_CODE: &str = r##"use wonopui::*;
use wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui_avatar::Avatar;

html! {
    <div class="flex h-screen">
        // Sidebar
        <div class="w-56 border-r border-zinc-200 p-4 flex flex-col">
            <div class="flex items-center gap-2 mb-6">
                <div class="size-8 rounded-lg bg-zinc-900"></div>
                <span class="font-semibold">{ "Acme Inc" }</span>
            </div>
            <nav class="space-y-1 flex-1">
                <a href="#" class="flex items-center gap-2 px-3 py-2 
                    rounded-lg bg-zinc-200 font-medium text-sm">
                    <span class="size-4">{ /* icon */ }</span>
                    { "Dashboard" }
                </a>
                <a href="#" class="flex items-center gap-2 px-3 py-2 
                    rounded-lg text-zinc-600 hover:bg-zinc-100 text-sm">
                    { "Team" }
                </a>
                // ... more nav items
            </nav>
            <div class="pt-4 border-t border-zinc-200">
                <div class="flex items-center gap-3">
                    <Avatar alt="John Doe" />
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium truncate">
                            { "John Doe" }
                        </p>
                        <p class="text-xs text-zinc-500 truncate">
                            { "john@example.com" }
                        </p>
                    </div>
                </div>
            </div>
        </div>
        // Main content
        <div class="flex-1 flex flex-col">
            <header class="h-14 border-b px-6 flex items-center 
                justify-between">
                <h1 class="text-lg font-semibold">{ "Dashboard" }</h1>
                <Button variant={ButtonVariant::Primary}>
                    { "New Project" }
                </Button>
            </header>
            <main class="flex-1 p-6 overflow-auto">
                <div class="grid grid-cols-3 gap-4 mb-6">
                    // Stat cards
                    <div class="rounded-lg border p-4">
                        <p class="text-sm text-zinc-500">
                            { "Total Revenue" }
                        </p>
                        <p class="text-2xl font-bold">{ "$45,231" }</p>
                        <p class="text-sm text-green-600">{ "+20.1%" }</p>
                    </div>
                    // ... more stat cards
                </div>
            </main>
        </div>
    </div>
}"##;

const STACKED_SHELL_CODE: &str = r##"use wonopui_button::{Button, ButtonVariant, ButtonSize};

html! {
    <div class="h-screen flex flex-col">
        <header class="border-b bg-zinc-50">
            <div class="px-4 py-3 flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div class="size-8 rounded-lg bg-zinc-900"></div>
                    <nav class="hidden sm:flex items-center gap-1">
                        <a href="#" class="px-3 py-2 rounded-lg text-sm 
                            font-medium bg-zinc-200">
                            { "Home" }
                        </a>
                        <a href="#" class="px-3 py-2 rounded-lg text-sm 
                            text-zinc-600 hover:bg-zinc-100">
                            { "Features" }
                        </a>
                        // ... more nav items
                    </nav>
                </div>
                <div class="flex items-center gap-2">
                    <Button variant={ButtonVariant::Ghost}>
                        { "Sign In" }
                    </Button>
                    <Button variant={ButtonVariant::Primary}>
                        { "Get Started" }
                    </Button>
                </div>
            </div>
        </header>
        <main class="flex-1 p-6 overflow-auto">
            // Your content here
        </main>
        <footer class="border-t px-4 py-3 text-center text-sm 
            text-zinc-500">
            { "© 2024 Acme Inc. All rights reserved." }
        </footer>
    </div>
}"##;

const CENTERED_HERO_CODE: &str = r##"use wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui_badge::Badge;

html! {
    <div class="py-16 px-6 text-center bg-gradient-to-b 
        from-zinc-50 to-white">
        <Badge class="mb-4">{ "Just shipped v1.0" }</Badge>
        <h1 class="text-4xl font-bold mb-4 max-w-2xl mx-auto">
            { "Build beautiful apps with WonopUI" }
        </h1>
        <p class="text-lg text-zinc-600 mb-8 max-w-xl mx-auto">
            { "A premium component library for Yew." }
        </p>
        <div class="flex justify-center gap-3">
            <Button 
                variant={ButtonVariant::Primary} 
                size={ButtonSize::Large}
            >
                { "Get Started" }
            </Button>
            <Button 
                variant={ButtonVariant::Outline} 
                size={ButtonSize::Large}
            >
                { "View on GitHub" }
            </Button>
        </div>
    </div>
}"##;

const SPLIT_HERO_CODE: &str = r##"use wonopui_button::{Button, ButtonVariant};

html! {
    <div class="flex flex-col md:flex-row">
        <div class="flex-1 p-8 flex flex-col justify-center">
            <h1 class="text-3xl font-bold mb-4">
                { "The future of web development" }
            </h1>
            <p class="text-zinc-600 mb-6">
                { "Build type-safe, performant web apps with Rust." }
            </p>
            <div class="flex gap-3">
                <Button variant={ButtonVariant::Primary}>
                    { "Start Building" }
                </Button>
                <Button variant={ButtonVariant::Ghost}>
                    { "Watch Demo" }
                </Button>
            </div>
        </div>
        <div class="flex-1 bg-zinc-100 min-h-[300px] 
            flex items-center justify-center">
            <img 
                src="/hero-image.png" 
                alt="Hero" 
                class="object-cover w-full h-full" 
            />
        </div>
    </div>
}"##;

const FEATURE_GRID_CODE: &str = r##"html! {
    <div class="py-12 px-6">
        <div class="text-center mb-10">
            <h2 class="text-2xl font-bold mb-3">
                { "Everything you need" }
            </h2>
            <p class="text-zinc-600">
                { "A complete toolkit for modern web apps." }
            </p>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <FeatureCard
                icon={html! { <IconTypeSafe /> }}
                title="Type Safe"
                description="Full Rust type safety for UI."
            />
            <FeatureCard
                icon={html! { <IconFast /> }}
                title="Fast"
                description="Compile to WebAssembly."
            />
            // ... more feature cards
        </div>
    </div>
}

// Feature card component
#[function_component(FeatureCard)]
fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div class="p-6 rounded-xl border 
            hover:border-zinc-300 transition-colors">
            <div class="size-10 rounded-lg bg-zinc-100 mb-4 
                flex items-center justify-center">
                { props.icon.clone() }
            </div>
            <h3 class="font-semibold mb-2">{ props.title }</h3>
            <p class="text-sm text-zinc-600">{ props.description }</p>
        </div>
    }
}"##;

const PRICING_SECTION_CODE: &str = r##"use wonopui_button::{Button, ButtonVariant};
use wonopui_badge::Badge;

html! {
    <div class="py-12 px-6">
        <div class="text-center mb-10">
            <h2 class="text-2xl font-bold mb-3">
                { "Simple, transparent pricing" }
            </h2>
            <p class="text-zinc-600">
                { "Choose the plan that's right for you." }
            </p>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 
            max-w-4xl mx-auto">
            // Standard tier
            <div class="p-6 rounded-xl border">
                <div class="text-center mb-6">
                    <h3 class="font-semibold">{ "Starter" }</h3>
                    <p class="text-sm text-zinc-500 mb-3">
                        { "Perfect for side projects" }
                    </p>
                    <div class="flex items-baseline justify-center gap-1">
                        <span class="text-3xl font-bold">{ "$0" }</span>
                        <span class="text-zinc-500">{ "/month" }</span>
                    </div>
                </div>
                <ul class="space-y-2 mb-6">
                    <li class="flex items-center gap-2 text-sm">
                        <span class="text-green-500">{ "✓" }</span>
                        { "5 projects" }
                    </li>
                    // ... more features
                </ul>
                <Button variant={ButtonVariant::Outline} class="w-full">
                    { "Get Started" }
                </Button>
            </div>

            // Highlighted tier (add border-2 border-zinc-900)
            <div class="p-6 rounded-xl border-2 border-zinc-900 relative">
                <div class="absolute -top-3 left-1/2 -translate-x-1/2">
                    <Badge>{ "Popular" }</Badge>
                </div>
                // ... pricing content
                <Button variant={ButtonVariant::Primary} class="w-full">
                    { "Get Started" }
                </Button>
            </div>
        </div>
    </div>
}"##;

const TEAM_GRID_CODE: &str = r##"use wonopui_avatar::{Avatar, AvatarSize};

html! {
    <div class="py-12 px-6">
        <div class="text-center mb-10">
            <h2 class="text-2xl font-bold mb-3">
                { "Meet our team" }
            </h2>
            <p class="text-zinc-600">
                { "The people behind the product." }
            </p>
        </div>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 
            max-w-3xl mx-auto">
            <TeamMember 
                avatar="https://example.com/alice.jpg"
                name="Alice Johnson" 
                role="CEO" 
            />
            <TeamMember 
                avatar="https://example.com/bob.jpg"
                name="Bob Smith" 
                role="CTO" 
            />
            // ... more team members
        </div>
    </div>
}

#[function_component(TeamMember)]
fn team_member(props: &TeamMemberProps) -> Html {
    html! {
        <div class="text-center">
            <Avatar 
                src={props.avatar} 
                alt={props.name} 
                size={AvatarSize::Large} 
                class="mx-auto mb-3" 
            />
            <h3 class="font-medium">{ props.name }</h3>
            <p class="text-sm text-zinc-500">{ props.role }</p>
        </div>
    }
}"##;

const SIGN_IN_FORM_CODE: &str = r##"use wonopui_button::{Button, ButtonVariant};
use wonopui_input::Input;

html! {
    <div class="p-8 max-w-sm mx-auto">
        <div class="text-center mb-6">
            <h2 class="text-xl font-bold">{ "Sign in" }</h2>
            <p class="text-sm text-zinc-500">
                { "Enter your credentials to continue" }
            </p>
        </div>
        <form class="space-y-4">
            <div>
                <label class="block text-sm font-medium mb-1.5">
                    { "Email" }
                </label>
                <Input placeholder="you@example.com" kind="email" />
            </div>
            <div>
                <label class="block text-sm font-medium mb-1.5">
                    { "Password" }
                </label>
                <Input placeholder="••••••••" kind="password" />
            </div>
            <div class="flex items-center justify-between text-sm">
                <label class="flex items-center gap-2 text-zinc-600">
                    <input type="checkbox" class="rounded" />
                    { "Remember me" }
                </label>
                <a href="#" class="hover:underline">
                    { "Forgot password?" }
                </a>
            </div>
            <Button variant={ButtonVariant::Primary} class="w-full">
                { "Sign in" }
            </Button>
        </form>
        <p class="text-center text-sm text-zinc-500 mt-6">
            { "Don't have an account? " }
            <a href="#" class="hover:underline">{ "Sign up" }</a>
        </p>
    </div>
}"##;

const CONTACT_FORM_CODE: &str = r##"use wonopui_button::{Button, ButtonVariant};
use wonopui_input::Input;

html! {
    <div class="p-8 max-w-sm mx-auto">
        <div class="text-center mb-6">
            <h2 class="text-xl font-bold">{ "Contact us" }</h2>
            <p class="text-sm text-zinc-500">
                { "We'd love to hear from you" }
            </p>
        </div>
        <form class="space-y-4">
            <div class="grid grid-cols-2 gap-4">
                <div>
                    <label class="block text-sm font-medium mb-1.5">
                        { "First name" }
                    </label>
                    <Input placeholder="John" />
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1.5">
                        { "Last name" }
                    </label>
                    <Input placeholder="Doe" />
                </div>
            </div>
            <div>
                <label class="block text-sm font-medium mb-1.5">
                    { "Email" }
                </label>
                <Input placeholder="you@example.com" kind="email" />
            </div>
            <div>
                <label class="block text-sm font-medium mb-1.5">
                    { "Message" }
                </label>
                <textarea 
                    class="flex min-h-[120px] w-full rounded-md border 
                        bg-transparent px-3.5 py-2.5 text-sm outline-none"
                    placeholder="Your message..."
                ></textarea>
            </div>
            <Button variant={ButtonVariant::Primary} class="w-full">
                { "Send message" }
            </Button>
        </form>
    </div>
}"##;

// Block Preview wrapper component with code viewer
#[derive(Properties, PartialEq)]
struct BlockPreviewProps {
    title: &'static str,
    description: &'static str,
    children: Children,
    #[prop_or_default]
    code: &'static str,
}

#[function_component(BlockPreview)]
fn block_preview(props: &BlockPreviewProps) -> Html {
    html! {
        <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 overflow-hidden bg-white dark:bg-zinc-950">
            <div class="border-b border-zinc-200 dark:border-zinc-800 px-4 py-3 bg-zinc-50 dark:bg-zinc-900">
                <h3 class="font-semibold text-zinc-900 dark:text-white">{ props.title }</h3>
                <p class="text-sm text-zinc-500 dark:text-zinc-400">{ props.description }</p>
            </div>
            <Tabs default_value="preview">
                <div class="border-b border-zinc-200 dark:border-zinc-800 px-4 bg-zinc-50 dark:bg-zinc-900">
                    <TabsList class="bg-transparent h-10 p-0 gap-4">
                        <TabsTrigger value="preview" class="data-[state=active]:bg-transparent data-[state=active]:shadow-none data-[state=active]:border-b-2 data-[state=active]:border-zinc-900 dark:data-[state=active]:border-white rounded-none px-0 pb-3 pt-2 text-sm">
                            { "Preview" }
                        </TabsTrigger>
                        <TabsTrigger value="code" class="data-[state=active]:bg-transparent data-[state=active]:shadow-none data-[state=active]:border-b-2 data-[state=active]:border-zinc-900 dark:data-[state=active]:border-white rounded-none px-0 pb-3 pt-2 text-sm">
                            { "Code" }
                        </TabsTrigger>
                    </TabsList>
                </div>
                <TabsContent value="preview" class="mt-0">
                    <div class="p-4 bg-zinc-100/50 dark:bg-zinc-900/50">
                        <div class="rounded-lg overflow-hidden border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950">
                            { props.children.clone() }
                        </div>
                    </div>
                </TabsContent>
                <TabsContent value="code" class="mt-0">
                    <div class="p-4 bg-zinc-950 max-h-[500px] overflow-auto">
                        <pre class="text-sm text-zinc-300 font-mono whitespace-pre-wrap">
                            <code>{ props.code }</code>
                        </pre>
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    }
}

// Dashboard Shell
#[function_component(DashboardShell)]
fn dashboard_shell() -> Html {
    html! {
        <div class="flex h-[400px]">
            // Sidebar
            <div class="w-56 border-r border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-900 p-4 flex flex-col">
                <div class="flex items-center gap-2 mb-6">
                    <div class="size-8 rounded-lg bg-zinc-900 dark:bg-white"></div>
                    <span class="font-semibold text-zinc-900 dark:text-white">{ "Acme Inc" }</span>
                </div>
                <nav class="space-y-1 flex-1">
                    <SidebarItem active=true icon="home">{ "Dashboard" }</SidebarItem>
                    <SidebarItem icon="users">{ "Team" }</SidebarItem>
                    <SidebarItem icon="folder">{ "Projects" }</SidebarItem>
                    <SidebarItem icon="chart">{ "Analytics" }</SidebarItem>
                    <SidebarItem icon="settings">{ "Settings" }</SidebarItem>
                </nav>
                <div class="pt-4 border-t border-zinc-200 dark:border-zinc-800">
                    <div class="flex items-center gap-3">
                        <Avatar alt="John Doe" />
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-zinc-900 dark:text-white truncate">{ "John Doe" }</p>
                            <p class="text-xs text-zinc-500 dark:text-zinc-400 truncate">{ "john@example.com" }</p>
                        </div>
                    </div>
                </div>
            </div>
            // Main content
            <div class="flex-1 flex flex-col">
                <header class="h-14 border-b border-zinc-200 dark:border-zinc-800 px-6 flex items-center justify-between">
                    <h1 class="text-lg font-semibold text-zinc-900 dark:text-white">{ "Dashboard" }</h1>
                    <Button variant={ButtonVariant::Primary} size={ButtonSize::Small}>{ "New Project" }</Button>
                </header>
                <main class="flex-1 p-6 overflow-auto">
                    <div class="grid grid-cols-3 gap-4 mb-6">
                        <StatCard label="Total Revenue" value="$45,231" change="+20.1%" />
                        <StatCard label="Subscriptions" value="+2,350" change="+18.2%" />
                        <StatCard label="Active Users" value="+12,234" change="+4.3%" />
                    </div>
                    <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
                        <h2 class="font-medium text-zinc-900 dark:text-white mb-3">{ "Recent Activity" }</h2>
                        <div class="space-y-3">
                            <ActivityItem name="Alice" action="created a new project" time="2 min ago" />
                            <ActivityItem name="Bob" action="pushed 3 commits" time="15 min ago" />
                            <ActivityItem name="Carol" action="joined the team" time="1 hour ago" />
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SidebarItemProps {
    children: Children,
    #[prop_or_default]
    active: bool,
    icon: &'static str,
}

#[function_component(SidebarItem)]
fn sidebar_item(props: &SidebarItemProps) -> Html {
    let class = if props.active {
        "flex items-center gap-2 px-3 py-2 rounded-lg bg-zinc-200 dark:bg-zinc-800 text-zinc-900 dark:text-white font-medium text-sm"
    } else {
        "flex items-center gap-2 px-3 py-2 rounded-lg text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-sm"
    };
    
    html! {
        <a href="#" class={class}>
            <span class="size-4 rounded bg-zinc-300 dark:bg-zinc-700"></span>
            { props.children.clone() }
        </a>
    }
}

#[derive(Properties, PartialEq)]
struct StatCardProps {
    label: &'static str,
    value: &'static str,
    change: &'static str,
}

#[function_component(StatCard)]
fn stat_card(props: &StatCardProps) -> Html {
    html! {
        <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 p-4">
            <p class="text-sm text-zinc-500 dark:text-zinc-400">{ props.label }</p>
            <p class="text-2xl font-bold text-zinc-900 dark:text-white">{ props.value }</p>
            <p class="text-sm text-green-600 dark:text-green-400">{ props.change }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ActivityItemProps {
    name: &'static str,
    action: &'static str,
    time: &'static str,
}

#[function_component(ActivityItem)]
fn activity_item(props: &ActivityItemProps) -> Html {
    html! {
        <div class="flex items-center gap-3">
            <Avatar alt={props.name.to_string()} />
            <div class="flex-1">
                <p class="text-sm text-zinc-900 dark:text-white">
                    <span class="font-medium">{ props.name }</span>
                    { " " }
                    { props.action }
                </p>
                <p class="text-xs text-zinc-500 dark:text-zinc-400">{ props.time }</p>
            </div>
        </div>
    }
}

// Stacked Shell
#[function_component(StackedShell)]
fn stacked_shell() -> Html {
    html! {
        <div class="h-[400px] flex flex-col">
            <header class="border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-900">
                <div class="px-4 py-3 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div class="size-8 rounded-lg bg-zinc-900 dark:bg-white"></div>
                        <nav class="hidden sm:flex items-center gap-1">
                            <a href="#" class="px-3 py-2 rounded-lg text-sm font-medium bg-zinc-200 dark:bg-zinc-800 text-zinc-900 dark:text-white">{ "Home" }</a>
                            <a href="#" class="px-3 py-2 rounded-lg text-sm text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-800">{ "Features" }</a>
                            <a href="#" class="px-3 py-2 rounded-lg text-sm text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-800">{ "Pricing" }</a>
                            <a href="#" class="px-3 py-2 rounded-lg text-sm text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-800">{ "About" }</a>
                        </nav>
                    </div>
                    <div class="flex items-center gap-2">
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Small}>{ "Sign In" }</Button>
                        <Button variant={ButtonVariant::Primary} size={ButtonSize::Small}>{ "Get Started" }</Button>
                    </div>
                </div>
            </header>
            <main class="flex-1 p-6 overflow-auto">
                <div class="max-w-2xl mx-auto text-center py-12">
                    <h1 class="text-3xl font-bold text-zinc-900 dark:text-white mb-4">
                        { "Welcome to Acme" }
                    </h1>
                    <p class="text-zinc-600 dark:text-zinc-400 mb-6">
                        { "Build better products faster with our platform." }
                    </p>
                    <div class="flex justify-center gap-3">
                        <Button variant={ButtonVariant::Primary}>{ "Get Started" }</Button>
                        <Button variant={ButtonVariant::Outline}>{ "Learn More" }</Button>
                    </div>
                </div>
            </main>
            <footer class="border-t border-zinc-200 dark:border-zinc-800 px-4 py-3 text-center text-sm text-zinc-500 dark:text-zinc-400">
                { "© 2024 Acme Inc. All rights reserved." }
            </footer>
        </div>
    }
}

// Centered Hero
#[function_component(CenteredHero)]
fn centered_hero() -> Html {
    html! {
        <div class="py-16 px-6 text-center bg-gradient-to-b from-zinc-50 to-white dark:from-zinc-900 dark:to-zinc-950">
            <Badge class="mb-4">{ "Just shipped v1.0" }</Badge>
            <h1 class="text-4xl font-bold text-zinc-900 dark:text-white mb-4 max-w-2xl mx-auto">
                { "Build beautiful apps with WonopUI" }
            </h1>
            <p class="text-lg text-zinc-600 dark:text-zinc-400 mb-8 max-w-xl mx-auto">
                { "A premium component library for Yew that brings shadcn/ui styling to Rust web applications." }
            </p>
            <div class="flex justify-center gap-3">
                <Button variant={ButtonVariant::Primary} size={ButtonSize::Large}>{ "Get Started" }</Button>
                <Button variant={ButtonVariant::Outline} size={ButtonSize::Large}>{ "View on GitHub" }</Button>
            </div>
        </div>
    }
}

// Split Hero
#[function_component(SplitHero)]
fn split_hero() -> Html {
    html! {
        <div class="flex flex-col md:flex-row">
            <div class="flex-1 p-8 flex flex-col justify-center">
                <h1 class="text-3xl font-bold text-zinc-900 dark:text-white mb-4">
                    { "The future of web development" }
                </h1>
                <p class="text-zinc-600 dark:text-zinc-400 mb-6">
                    { "Build type-safe, performant web applications with Rust and Yew. Get started in minutes with our component library." }
                </p>
                <div class="flex gap-3">
                    <Button variant={ButtonVariant::Primary}>{ "Start Building" }</Button>
                    <Button variant={ButtonVariant::Ghost}>{ "Watch Demo" }</Button>
                </div>
            </div>
            <div class="flex-1 bg-zinc-100 dark:bg-zinc-800 min-h-[300px] flex items-center justify-center">
                <div class="text-zinc-400 dark:text-zinc-600">{ "[Image Placeholder]" }</div>
            </div>
        </div>
    }
}

// Feature Grid
#[function_component(FeatureGrid)]
fn feature_grid() -> Html {
    html! {
        <div class="py-12 px-6">
            <div class="text-center mb-10">
                <h2 class="text-2xl font-bold text-zinc-900 dark:text-white mb-3">
                    { "Everything you need" }
                </h2>
                <p class="text-zinc-600 dark:text-zinc-400">
                    { "A complete toolkit for building modern web applications." }
                </p>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <FeatureCard
                    title="Type Safe"
                    description="Full Rust type safety for your UI components."
                />
                <FeatureCard
                    title="Fast"
                    description="Compile to WebAssembly for near-native performance."
                />
                <FeatureCard
                    title="Beautiful"
                    description="Premium shadcn/ui styling out of the box."
                />
                <FeatureCard
                    title="Accessible"
                    description="ARIA compliant and keyboard navigable."
                />
                <FeatureCard
                    title="Themeable"
                    description="Customize colors, spacing, and more."
                />
                <FeatureCard
                    title="Open Source"
                    description="MIT licensed and community driven."
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FeatureCardProps {
    title: &'static str,
    description: &'static str,
}

#[function_component(FeatureCard)]
fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div class="p-6 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors">
            <div class="size-10 rounded-lg bg-zinc-100 dark:bg-zinc-800 mb-4 flex items-center justify-center">
                <div class="size-5 rounded bg-zinc-300 dark:bg-zinc-600"></div>
            </div>
            <h3 class="font-semibold text-zinc-900 dark:text-white mb-2">{ props.title }</h3>
            <p class="text-sm text-zinc-600 dark:text-zinc-400">{ props.description }</p>
        </div>
    }
}

// Pricing Section
#[function_component(PricingSection)]
fn pricing_section() -> Html {
    html! {
        <div class="py-12 px-6">
            <div class="text-center mb-10">
                <h2 class="text-2xl font-bold text-zinc-900 dark:text-white mb-3">
                    { "Simple, transparent pricing" }
                </h2>
                <p class="text-zinc-600 dark:text-zinc-400">
                    { "Choose the plan that's right for you." }
                </p>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto">
                <PricingCard
                    name="Starter"
                    price="$0"
                    period="/month"
                    description="Perfect for side projects"
                    features={vec!["5 projects", "Basic analytics", "Community support"]}
                    highlighted=false
                />
                <PricingCard
                    name="Pro"
                    price="$29"
                    period="/month"
                    description="Best for growing teams"
                    features={vec!["Unlimited projects", "Advanced analytics", "Priority support", "Custom domains"]}
                    highlighted=true
                />
                <PricingCard
                    name="Enterprise"
                    price="$99"
                    period="/month"
                    description="For large organizations"
                    features={vec!["Everything in Pro", "SSO/SAML", "Dedicated support", "SLA guarantee"]}
                    highlighted=false
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PricingCardProps {
    name: &'static str,
    price: &'static str,
    period: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
    highlighted: bool,
}

#[function_component(PricingCard)]
fn pricing_card(props: &PricingCardProps) -> Html {
    let border_class = if props.highlighted {
        "border-2 border-zinc-900 dark:border-white"
    } else {
        "border border-zinc-200 dark:border-zinc-800"
    };
    
    html! {
        <div class={format!("p-6 rounded-xl {} relative", border_class)}>
            if props.highlighted {
                <div class="absolute -top-3 left-1/2 -translate-x-1/2">
                    <Badge>{ "Popular" }</Badge>
                </div>
            }
            <div class="text-center mb-6">
                <h3 class="font-semibold text-zinc-900 dark:text-white">{ props.name }</h3>
                <p class="text-sm text-zinc-500 dark:text-zinc-400 mb-3">{ props.description }</p>
                <div class="flex items-baseline justify-center gap-1">
                    <span class="text-3xl font-bold text-zinc-900 dark:text-white">{ props.price }</span>
                    <span class="text-zinc-500 dark:text-zinc-400">{ props.period }</span>
                </div>
            </div>
            <ul class="space-y-2 mb-6">
                { for props.features.iter().map(|feature| html! {
                    <li class="flex items-center gap-2 text-sm text-zinc-600 dark:text-zinc-400">
                        <span class="text-green-500">{ "✓" }</span>
                        { *feature }
                    </li>
                })}
            </ul>
            <Button 
                variant={if props.highlighted { ButtonVariant::Primary } else { ButtonVariant::Outline }}
                class="w-full"
            >
                { "Get Started" }
            </Button>
        </div>
    }
}

// Team Grid
#[function_component(TeamGrid)]
fn team_grid() -> Html {
    html! {
        <div class="py-12 px-6">
            <div class="text-center mb-10">
                <h2 class="text-2xl font-bold text-zinc-900 dark:text-white mb-3">
                    { "Meet our team" }
                </h2>
                <p class="text-zinc-600 dark:text-zinc-400">
                    { "The people behind the product." }
                </p>
            </div>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6 max-w-3xl mx-auto">
                <TeamMember name="Alice Johnson" role="CEO" />
                <TeamMember name="Bob Smith" role="CTO" />
                <TeamMember name="Carol White" role="Design Lead" />
                <TeamMember name="David Brown" role="Engineering" />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TeamMemberProps {
    name: &'static str,
    role: &'static str,
}

#[function_component(TeamMember)]
fn team_member(props: &TeamMemberProps) -> Html {
    html! {
        <div class="text-center">
            <Avatar alt={props.name.to_string()} size={AvatarSize::Large} class="mx-auto mb-3" />
            <h3 class="font-medium text-zinc-900 dark:text-white">{ props.name }</h3>
            <p class="text-sm text-zinc-500 dark:text-zinc-400">{ props.role }</p>
        </div>
    }
}

// Sign In Form
#[function_component(SignInForm)]
fn sign_in_form() -> Html {
    html! {
        <div class="p-8 max-w-sm mx-auto">
            <div class="text-center mb-6">
                <h2 class="text-xl font-bold text-zinc-900 dark:text-white">{ "Sign in" }</h2>
                <p class="text-sm text-zinc-500 dark:text-zinc-400">{ "Enter your credentials to continue" }</p>
            </div>
            <form class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-zinc-900 dark:text-white mb-1.5">
                        { "Email" }
                    </label>
                    <Input placeholder="you@example.com" kind="email" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-zinc-900 dark:text-white mb-1.5">
                        { "Password" }
                    </label>
                    <Input placeholder="••••••••" kind="password" />
                </div>
                <div class="flex items-center justify-between text-sm">
                    <label class="flex items-center gap-2 text-zinc-600 dark:text-zinc-400">
                        <input type="checkbox" class="rounded" />
                        { "Remember me" }
                    </label>
                    <a href="#" class="text-zinc-900 dark:text-white hover:underline">{ "Forgot password?" }</a>
                </div>
                <Button variant={ButtonVariant::Primary} class="w-full">{ "Sign in" }</Button>
            </form>
            <p class="text-center text-sm text-zinc-500 dark:text-zinc-400 mt-6">
                { "Don't have an account? " }
                <a href="#" class="text-zinc-900 dark:text-white hover:underline">{ "Sign up" }</a>
            </p>
        </div>
    }
}

// Contact Form
#[function_component(ContactForm)]
fn contact_form() -> Html {
    html! {
        <div class="p-8 max-w-sm mx-auto">
            <div class="text-center mb-6">
                <h2 class="text-xl font-bold text-zinc-900 dark:text-white">{ "Contact us" }</h2>
                <p class="text-sm text-zinc-500 dark:text-zinc-400">{ "We'd love to hear from you" }</p>
            </div>
            <form class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-zinc-900 dark:text-white mb-1.5">
                            { "First name" }
                        </label>
                        <Input placeholder="John" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-zinc-900 dark:text-white mb-1.5">
                            { "Last name" }
                        </label>
                        <Input placeholder="Doe" />
                    </div>
                </div>
                <div>
                    <label class="block text-sm font-medium text-zinc-900 dark:text-white mb-1.5">
                        { "Email" }
                    </label>
                    <Input placeholder="you@example.com" kind="email" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-zinc-900 dark:text-white mb-1.5">
                        { "Message" }
                    </label>
                    <textarea 
                        class="flex min-h-[120px] w-full rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent px-3.5 py-2.5 text-sm placeholder:text-zinc-500 dark:placeholder:text-zinc-400 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] outline-none text-zinc-900 dark:text-zinc-50"
                        placeholder="Your message..."
                    ></textarea>
                </div>
                <Button variant={ButtonVariant::Primary} class="w-full">{ "Send message" }</Button>
            </form>
        </div>
    }
}
