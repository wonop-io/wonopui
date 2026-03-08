//! Marketing blocks - Hero sections, features, pricing, team displays

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_avatar::{Avatar, AvatarSize};
use wonopui::wonopui_badge::Badge;
use crate::blocks::BlockPreview;

/// Marketing category page
#[function_component(MarketingBlocks)]
pub fn marketing_blocks() -> Html {
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
                    {"Marketing"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Hero sections, feature grids, pricing tables, and team displays."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                <BlockPreview 
                    title="Hero - Centered"
                    description="Centered hero section with headline and call-to-action buttons."
                    code={HERO_CENTERED_CODE}
                    min_height={500}
                >
                    <HeroCentered />
                </BlockPreview>
                
                <BlockPreview 
                    title="Hero - Split"
                    description="Split hero with text on left and image on right."
                    code={HERO_SPLIT_CODE}
                    min_height={500}
                >
                    <HeroSplit />
                </BlockPreview>
                
                <BlockPreview 
                    title="Feature Grid"
                    description="Grid of features with icons and descriptions."
                    code={FEATURE_GRID_CODE}
                    min_height={400}
                >
                    <FeatureGrid />
                </BlockPreview>
                
                <BlockPreview 
                    title="Pricing"
                    description="Three-tier pricing comparison table."
                    code={PRICING_CODE}
                    min_height={600}
                >
                    <Pricing />
                </BlockPreview>
                
                <BlockPreview 
                    title="Team"
                    description="Team member grid with photos and roles."
                    code={TEAM_CODE}
                    min_height={400}
                >
                    <Team />
                </BlockPreview>
            </div>
        </Container>
    }
}

// =============================================================================
// Example Components
// =============================================================================

#[function_component(HeroCentered)]
fn hero_centered() -> Html {
    html! {
        <div class="flex min-h-[500px] flex-col items-center justify-center px-6 py-24 text-center">
            <Badge class="mb-4">{"New: AI-powered features"}</Badge>
            <h1 class="max-w-4xl text-4xl font-bold tracking-tight text-zinc-900 dark:text-white sm:text-5xl lg:text-6xl">
                {"Build beautiful products faster with our platform"}
            </h1>
            <p class="mt-6 max-w-2xl text-lg text-zinc-600 dark:text-zinc-400">
                {"Streamline your workflow, collaborate with your team, and ship products faster than ever before. Get started today with our comprehensive toolkit."}
            </p>
            <div class="mt-10 flex flex-col gap-4 sm:flex-row">
                <Button variant={ButtonVariant::Default} size={ButtonSize::Large}>
                    {"Get started"}
                </Button>
                <Button variant={ButtonVariant::Outline} size={ButtonSize::Large}>
                    {"Learn more"}
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="ml-2 h-4 w-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" />
                    </svg>
                </Button>
            </div>
            
            // Social proof
            <div class="mt-16 flex flex-col items-center gap-4">
                <div class="flex -space-x-2">
                    {for (1..6).map(|i| html! {
                        <Avatar 
                            src={format!("https://i.pravatar.cc/150?img={}", i + 20)} 
                            size={AvatarSize::Small}
                            class="border-2 border-white dark:border-zinc-950"
                        />
                    })}
                </div>
                <p class="text-sm text-zinc-500 dark:text-zinc-400">
                    {"Join "}
                    <span class="font-semibold text-zinc-900 dark:text-white">{"5,000+"}</span>
                    {" developers already using our platform"}
                </p>
            </div>
        </div>
    }
}

#[function_component(HeroSplit)]
fn hero_split() -> Html {
    html! {
        <div class="flex min-h-[500px] w-full flex-col bg-white dark:bg-zinc-950 lg:flex-row">
            // Left side - Content
            <div class="flex flex-1 flex-col justify-center px-6 py-16 lg:px-12">
                <Badge class="mb-6 w-fit">{"Introducing v2.0"}</Badge>
                <h1 class="text-4xl font-bold tracking-tight text-zinc-900 dark:text-white lg:text-5xl">
                    {"The modern platform for ambitious teams"}
                </h1>
                <p class="mt-6 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Build, deploy, and scale your applications with confidence. Our platform provides everything you need to go from idea to production."}
                </p>
                <div class="mt-8 flex flex-col gap-4 sm:flex-row">
                    <Button variant={ButtonVariant::Default} size={ButtonSize::Large}>
                        {"Start free trial"}
                    </Button>
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Large}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mr-2 h-5 w-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.91 11.672a.375.375 0 0 1 0 .656l-5.603 3.113a.375.375 0 0 1-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112Z" />
                        </svg>
                        {"Watch demo"}
                    </Button>
                </div>
                <div class="mt-12 flex items-center gap-8">
                    <div>
                        <p class="text-3xl font-bold text-zinc-900 dark:text-white">{"99.9%"}</p>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Uptime SLA"}</p>
                    </div>
                    <div class="h-10 w-px bg-zinc-200 dark:bg-zinc-800"></div>
                    <div>
                        <p class="text-3xl font-bold text-zinc-900 dark:text-white">{"24/7"}</p>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Support"}</p>
                    </div>
                    <div class="h-10 w-px bg-zinc-200 dark:bg-zinc-800"></div>
                    <div>
                        <p class="text-3xl font-bold text-zinc-900 dark:text-white">{"50+"}</p>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Integrations"}</p>
                    </div>
                </div>
            </div>
            
            // Right side - Image placeholder
            <div class="flex flex-1 items-center justify-center bg-zinc-100 dark:bg-zinc-900 lg:min-h-[500px]">
                <div class="relative h-80 w-full max-w-md lg:h-96">
                    <div class="absolute inset-0 rounded-xl bg-gradient-to-br from-zinc-300 to-zinc-400 dark:from-zinc-700 dark:to-zinc-800"></div>
                    <div class="absolute inset-4 rounded-lg bg-white shadow-2xl dark:bg-zinc-950">
                        <div class="flex items-center gap-2 border-b border-zinc-200 px-4 py-3 dark:border-zinc-800">
                            <div class="h-3 w-3 rounded-full bg-red-500"></div>
                            <div class="h-3 w-3 rounded-full bg-yellow-500"></div>
                            <div class="h-3 w-3 rounded-full bg-green-500"></div>
                        </div>
                        <div class="p-4 space-y-3">
                            <div class="h-2 w-24 rounded bg-zinc-200 dark:bg-zinc-700"></div>
                            <div class="h-2 w-32 rounded bg-zinc-200 dark:bg-zinc-700"></div>
                            <div class="h-2 w-20 rounded bg-zinc-200 dark:bg-zinc-700"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(FeatureGrid)]
fn feature_grid() -> Html {
    let features = vec![
        ("Lightning Fast", "Built for speed with optimized performance at every level.", "M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"),
        ("Secure by Default", "Enterprise-grade security with end-to-end encryption.", "M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z"),
        ("Easy Integration", "Connect with your favorite tools in minutes.", "M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244"),
        ("Analytics", "Deep insights into your application performance.", "M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 0 1 3 19.875v-6.75ZM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 0 1-1.125-1.125V8.625ZM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 0 1-1.125-1.125V4.125Z"),
        ("24/7 Support", "Our team is here to help you around the clock.", "M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"),
        ("Auto Scaling", "Scale automatically based on your traffic needs.", "M3.75 3v11.25A2.25 2.25 0 0 0 6 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0 1 18 16.5h-2.25m-7.5 0h7.5m-7.5 0-1 3m8.5-3 1 3m0 0 .5 1.5m-.5-1.5h-9.5m0 0-.5 1.5m.75-9 3-3 2.148 2.148A12.061 12.061 0 0 1 16.5 7.605"),
    ];
    
    html! {
        <div class="px-6 py-16">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold tracking-tight text-zinc-900 dark:text-white sm:text-4xl">
                    {"Everything you need to succeed"}
                </h2>
                <p class="mt-4 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Powerful features to help you build, deploy, and scale your applications."}
                </p>
            </div>
            
            <div class="mx-auto mt-16 max-w-5xl">
                <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    {for features.iter().map(|(title, desc, icon)| html! {
                        <div class="relative rounded-xl border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-950">
                            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-zinc-900 text-white dark:bg-zinc-100 dark:text-zinc-900">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d={*icon} />
                                </svg>
                            </div>
                            <h3 class="mt-4 text-lg font-semibold text-zinc-900 dark:text-white">{title}</h3>
                            <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">{desc}</p>
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}

#[function_component(Pricing)]
fn pricing() -> Html {
    html! {
        <div class="px-6 py-16">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold tracking-tight text-zinc-900 dark:text-white sm:text-4xl">
                    {"Simple, transparent pricing"}
                </h2>
                <p class="mt-4 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Choose the plan that's right for you and start building today."}
                </p>
            </div>
            
            <div class="mx-auto mt-16 grid max-w-5xl gap-8 lg:grid-cols-3">
                // Starter
                <div class="flex flex-col rounded-xl border border-zinc-200 bg-white p-8 dark:border-zinc-800 dark:bg-zinc-950">
                    <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">{"Starter"}</h3>
                    <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">{"Perfect for side projects"}</p>
                    <div class="mt-6">
                        <span class="text-4xl font-bold text-zinc-900 dark:text-white">{"$0"}</span>
                        <span class="text-zinc-500 dark:text-zinc-400">{"/month"}</span>
                    </div>
                    <ul class="mt-8 flex-1 space-y-4">
                        {for ["Up to 3 projects", "Basic analytics", "Community support", "1GB storage"].iter().map(|feature| html! {
                            <li class="flex items-center gap-3 text-sm text-zinc-600 dark:text-zinc-400">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="h-4 w-4 text-zinc-900 dark:text-white">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                                </svg>
                                {feature}
                            </li>
                        })}
                    </ul>
                    <Button variant={ButtonVariant::Outline} class="mt-8 w-full">
                        {"Get started"}
                    </Button>
                </div>
                
                // Pro - highlighted
                <div class="relative flex flex-col rounded-xl border-2 border-zinc-900 bg-white p-8 dark:border-zinc-100 dark:bg-zinc-950">
                    <div class="absolute -top-4 left-1/2 -translate-x-1/2">
                        <Badge>{"Most popular"}</Badge>
                    </div>
                    <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">{"Pro"}</h3>
                    <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">{"For growing teams"}</p>
                    <div class="mt-6">
                        <span class="text-4xl font-bold text-zinc-900 dark:text-white">{"$29"}</span>
                        <span class="text-zinc-500 dark:text-zinc-400">{"/month"}</span>
                    </div>
                    <ul class="mt-8 flex-1 space-y-4">
                        {for ["Unlimited projects", "Advanced analytics", "Priority support", "100GB storage", "Custom domains", "Team collaboration"].iter().map(|feature| html! {
                            <li class="flex items-center gap-3 text-sm text-zinc-600 dark:text-zinc-400">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="h-4 w-4 text-zinc-900 dark:text-white">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                                </svg>
                                {feature}
                            </li>
                        })}
                    </ul>
                    <Button variant={ButtonVariant::Default} class="mt-8 w-full">
                        {"Start free trial"}
                    </Button>
                </div>
                
                // Enterprise
                <div class="flex flex-col rounded-xl border border-zinc-200 bg-white p-8 dark:border-zinc-800 dark:bg-zinc-950">
                    <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">{"Enterprise"}</h3>
                    <p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">{"For large organizations"}</p>
                    <div class="mt-6">
                        <span class="text-4xl font-bold text-zinc-900 dark:text-white">{"Custom"}</span>
                    </div>
                    <ul class="mt-8 flex-1 space-y-4">
                        {for ["Everything in Pro", "Dedicated support", "Custom SLA", "Unlimited storage", "SSO & SAML", "Custom integrations"].iter().map(|feature| html! {
                            <li class="flex items-center gap-3 text-sm text-zinc-600 dark:text-zinc-400">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="h-4 w-4 text-zinc-900 dark:text-white">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                                </svg>
                                {feature}
                            </li>
                        })}
                    </ul>
                    <Button variant={ButtonVariant::Outline} class="mt-8 w-full">
                        {"Contact sales"}
                    </Button>
                </div>
            </div>
        </div>
    }
}

#[function_component(Team)]
fn team() -> Html {
    let team_members = vec![
        ("Sarah Johnson", "CEO & Co-founder", 21),
        ("Michael Chen", "CTO & Co-founder", 22),
        ("Emily Davis", "Head of Design", 23),
        ("James Wilson", "Head of Engineering", 24),
    ];
    
    html! {
        <div class="px-6 py-16">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold tracking-tight text-zinc-900 dark:text-white sm:text-4xl">
                    {"Meet our team"}
                </h2>
                <p class="mt-4 text-lg text-zinc-600 dark:text-zinc-400">
                    {"We're a passionate team dedicated to building the best product for our users."}
                </p>
            </div>
            
            <div class="mx-auto mt-16 grid max-w-4xl gap-8 sm:grid-cols-2 lg:grid-cols-4">
                {for team_members.iter().map(|(name, role, img)| html! {
                    <div class="text-center">
                        <Avatar 
                            src={format!("https://i.pravatar.cc/150?img={}", img)} 
                            size={AvatarSize::Large}
                            class="mx-auto"
                        />
                        <h3 class="mt-4 text-lg font-semibold text-zinc-900 dark:text-white">{name}</h3>
                        <p class="text-sm text-zinc-600 dark:text-zinc-400">{role}</p>
                        <div class="mt-4 flex justify-center gap-4">
                            <a href="#" class="text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-200">
                                <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84"/>
                                </svg>
                            </a>
                            <a href="#" class="text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-200">
                                <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" clip-rule="evenodd"/>
                                </svg>
                            </a>
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const HERO_CENTERED_CODE: &str = r##"use yew::prelude::*;

#[function_component(HeroCentered)]
pub fn hero_centered() -> Html {
    html! {
        <div class="flex min-h-screen flex-col items-center justify-center px-6 py-24 text-center">
            <Badge>{"New: AI-powered features"}</Badge>
            <h1 class="max-w-4xl text-4xl font-bold tracking-tight sm:text-5xl lg:text-6xl">
                {"Build beautiful products faster"}
            </h1>
            <p class="mt-6 max-w-2xl text-lg text-zinc-600">
                {"Streamline your workflow and ship products faster."}
            </p>
            <div class="mt-10 flex flex-col gap-4 sm:flex-row">
                <Button size={ButtonSize::Large}>{"Get started"}</Button>
                <Button variant={ButtonVariant::Outline} size={ButtonSize::Large}>
                    {"Learn more"}
                </Button>
            </div>
            
            // Social proof
            <div class="mt-16 flex flex-col items-center gap-4">
                <AvatarStack count={5} />
                <p class="text-sm text-zinc-500">
                    {"Join 5,000+ developers"}
                </p>
            </div>
        </div>
    }
}
"##;

const HERO_SPLIT_CODE: &str = r##"use yew::prelude::*;

#[function_component(HeroSplit)]
pub fn hero_split() -> Html {
    html! {
        <div class="flex min-h-screen flex-col lg:flex-row">
            // Left side - Content
            <div class="flex flex-1 flex-col justify-center px-6 py-16 lg:px-12">
                <Badge class="mb-6 w-fit">{"Introducing v2.0"}</Badge>
                <h1 class="text-4xl font-bold tracking-tight lg:text-5xl">
                    {"The modern platform for ambitious teams"}
                </h1>
                <p class="mt-6 text-lg text-zinc-600">
                    {"Build, deploy, and scale your applications."}
                </p>
                <div class="mt-8 flex flex-col gap-4 sm:flex-row">
                    <Button size={ButtonSize::Large}>{"Start free trial"}</Button>
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Large}>
                        {"Watch demo"}
                    </Button>
                </div>
                <Stats class="mt-12" />
            </div>
            
            // Right side - Image
            <div class="flex flex-1 items-center justify-center bg-zinc-100">
                <ProductImage />
            </div>
        </div>
    }
}
"##;

const FEATURE_GRID_CODE: &str = r##"use yew::prelude::*;

#[function_component(FeatureGrid)]
pub fn feature_grid() -> Html {
    let features = vec![
        ("Lightning Fast", "Built for speed.", "bolt-icon"),
        ("Secure", "Enterprise-grade security.", "shield-icon"),
        ("Easy Integration", "Connect in minutes.", "link-icon"),
    ];
    
    html! {
        <div class="px-6 py-16">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold tracking-tight sm:text-4xl">
                    {"Everything you need"}
                </h2>
                <p class="mt-4 text-lg text-zinc-600">
                    {"Powerful features for your applications."}
                </p>
            </div>
            
            <div class="mx-auto mt-16 max-w-5xl">
                <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    {for features.iter().map(|(title, desc, icon)| html! {
                        <FeatureCard 
                            title={title}
                            description={desc}
                            icon={icon}
                        />
                    })}
                </div>
            </div>
        </div>
    }
}
"##;

const PRICING_CODE: &str = r##"use yew::prelude::*;

#[function_component(Pricing)]
pub fn pricing() -> Html {
    html! {
        <div class="px-6 py-16">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold">{"Simple pricing"}</h2>
            </div>
            
            <div class="mx-auto mt-16 grid max-w-5xl gap-8 lg:grid-cols-3">
                <PricingCard 
                    name="Starter"
                    price="$0"
                    features={vec!["3 projects", "Basic analytics"]}
                />
                <PricingCard 
                    name="Pro"
                    price="$29"
                    features={vec!["Unlimited projects", "Advanced analytics"]}
                    highlighted={true}
                />
                <PricingCard 
                    name="Enterprise"
                    price="Custom"
                    features={vec!["Everything in Pro", "Dedicated support"]}
                />
            </div>
        </div>
    }
}
"##;

const TEAM_CODE: &str = r##"use yew::prelude::*;

#[function_component(Team)]
pub fn team() -> Html {
    let members = vec![
        ("Sarah Johnson", "CEO", "avatar1.jpg"),
        ("Michael Chen", "CTO", "avatar2.jpg"),
    ];
    
    html! {
        <div class="px-6 py-16">
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-3xl font-bold">{"Meet our team"}</h2>
            </div>
            
            <div class="mx-auto mt-16 grid max-w-4xl gap-8 sm:grid-cols-2 lg:grid-cols-4">
                {for members.iter().map(|(name, role, avatar)| html! {
                    <div class="text-center">
                        <Avatar src={avatar} size={AvatarSize::Large} />
                        <h3 class="mt-4 text-lg font-semibold">{name}</h3>
                        <p class="text-sm text-zinc-600">{role}</p>
                    </div>
                })}
            </div>
        </div>
    }
}
"##;
