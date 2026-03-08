//! Enhanced BlockPreview component with Iframe isolation, Resizable viewport, and dark mode toggle
//!
//! This component properly isolates block previews using an iframe, which allows:
//! - Full-page layouts to be previewed without breaking the gallery
//! - Proper dark mode theming (iframe body gets the `dark` class)
//! - Accurate responsive testing with resizable viewports

use yew::prelude::*;
use wonopui::wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
use wonopui::wonopui_switch::SwitchButton;
use wonopui::Resizable;
use wonopui::Iframe;

/// Viewport size options for block preview
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Viewport {
    Desktop,
    Tablet,
    Mobile,
}

impl Viewport {
    fn width(&self) -> f64 {
        match self {
            Viewport::Desktop => 1200.,
            Viewport::Tablet => 768.,
            Viewport::Mobile => 375.,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BlockPreviewProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    pub children: Children,
    #[prop_or_default]
    pub code: Option<AttrValue>,
    /// Minimum height for the preview container (default: 100)
    #[prop_or(100)]
    pub min_height: u32,
    /// Whether to show viewport controls
    #[prop_or(true)]
    pub show_viewport_controls: bool,
    /// Whether to show dark mode toggle
    #[prop_or(true)]
    pub show_dark_mode_toggle: bool,
    /// Whether the block content should be centered (for small components)
    /// When true, wraps content in a centered flex container
    #[prop_or(false)]
    pub isolate: bool,
}

#[function_component(BlockPreview)]
pub fn block_preview(props: &BlockPreviewProps) -> Html {
    let min_height = props.min_height;
    let dark_mode = use_state(|| false);
    let iframe_height = use_state(|| min_height as f64);
    // Coordinates only control width - height is determined by iframe content
    let viewport_width = use_state(|| 1200.0_f64);
    
    let on_viewport_change = |vp: Viewport| {
        let viewport_width = viewport_width.clone();
        Callback::from(move |_: MouseEvent| {
            viewport_width.set(vp.width());
        })
    };
    
    let on_height_change = {
        let iframe_height = iframe_height.clone();
        Callback::from(move |height: f64| {
            iframe_height.set(height);
        })
    };
    
    // Resizable coordinates derived from viewport_width and iframe_height
    let coordinates = (0., 0., *viewport_width, *iframe_height);
    
    let on_dark_mode_toggle = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            dark_mode.set(!*dark_mode);
        })
    };
    
    let on_coordinates_change = {
        let viewport_width = viewport_width.clone();
        Callback::from(move |new_coordinates: (f64, f64, f64, f64)| {
            // Only extract width from resize - height is controlled by iframe content
            viewport_width.set(new_coordinates.2);
        })
    };
    
    // Viewport button classes
    let viewport_btn_class = |vp: Viewport| -> Classes {
        let base = "inline-flex items-center justify-center rounded-md px-3 py-1.5 text-sm font-medium transition-colors";
        let current_width = *viewport_width;
        let is_active = (current_width - vp.width()).abs() < 10.;
        if is_active {
            classes!(base, "bg-zinc-900", "text-white", "dark:bg-zinc-100", "dark:text-zinc-900")
        } else {
            classes!(base, "text-zinc-600", "hover:bg-zinc-100", "dark:text-zinc-400", "dark:hover:bg-zinc-800")
        }
    };
    
    // Build the content to render in the iframe
    // Note: We don't use h-full here since auto-height measures the content
    let iframe_content = if props.isolate {
        html! {
            <div class="w-full min-h-full bg-white dark:bg-zinc-950 flex items-center justify-center p-8">
                {props.children.clone()}
            </div>
        }
    } else {
        html! {
            <div class="w-full bg-white dark:bg-zinc-950">
                {props.children.clone()}
            </div>
        }
    };
    
    let viewport_icons = html! {
        <div class="flex items-center gap-1 rounded-lg bg-zinc-100 p-1 dark:bg-zinc-800">
            // Desktop icon
            <button 
                class={viewport_btn_class(Viewport::Desktop)}
                onclick={on_viewport_change(Viewport::Desktop)}
                title="Desktop (1200px)"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25" />
                </svg>
            </button>
            // Tablet icon
            <button
                class={viewport_btn_class(Viewport::Tablet)}
                onclick={on_viewport_change(Viewport::Tablet)}
                title="Tablet (768px)"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5h3m-6.75 2.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-15a2.25 2.25 0 0 0-2.25-2.25H6.75A2.25 2.25 0 0 0 4.5 4.5v15a2.25 2.25 0 0 0 2.25 2.25Z" />
                </svg>
            </button>
            // Mobile icon
            <button
                class={viewport_btn_class(Viewport::Mobile)}
                onclick={on_viewport_change(Viewport::Mobile)}
                title="Mobile (375px)"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 1.5H8.25A2.25 2.25 0 0 0 6 3.75v16.5a2.25 2.25 0 0 0 2.25 2.25h7.5A2.25 2.25 0 0 0 18 20.25V3.75a2.25 2.25 0 0 0-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3" />
                </svg>
            </button>
        </div>
    };
    
    let dark_mode_toggle = html! {
        <div class="flex items-center gap-2">
            <span class="text-sm text-zinc-500 dark:text-zinc-400">{"Dark"}</span>
            <SwitchButton checked={*dark_mode} on_toggle={on_dark_mode_toggle} />
        </div>
    };
    
    // The preview uses Iframe for proper isolation with auto-height
    // Container adapts to iframe content height dynamically
    let preview_content = html! {
        <div 
            class="relative w-full rounded-lg border border-zinc-200 bg-zinc-100 dark:border-zinc-800 dark:bg-zinc-900 overflow-hidden"
            style={format!("min-height: {}px;", min_height)}
        >
            <Resizable 
                east={true} 
                south={false}
                south_east={false}
                coordinates={coordinates} 
                on_coordinates_change={on_coordinates_change}
            >
                <Iframe 
                    body_class={if *dark_mode { "dark" } else { "" }} 
                    class="w-full rounded-lg"
                    auto_height={true}
                    min_height={min_height}
                    on_height_change={on_height_change.clone()}
                >
                    {iframe_content}
                </Iframe>
            </Resizable>
        </div>
    };
    
    let code_content = if let Some(code) = &props.code {
        html! {
            <div class="relative">
                <pre class="overflow-x-auto rounded-lg bg-zinc-950 p-4 text-sm text-zinc-100 max-h-[500px] overflow-y-auto">
                    <code>{code}</code>
                </pre>
            </div>
        }
    } else {
        html! {
            <div class="flex items-center justify-center h-32 text-zinc-500 dark:text-zinc-400">
                {"Code not available for this block"}
            </div>
        }
    };

    html! {
        <div class="space-y-4">
            // Header
            <div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                <div>
                    <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">
                        {&props.title}
                    </h3>
                    if let Some(desc) = &props.description {
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">{desc}</p>
                    }
                </div>
                <div class="flex items-center gap-4">
                    if props.show_viewport_controls {
                        {viewport_icons}
                    }
                    if props.show_dark_mode_toggle {
                        {dark_mode_toggle}
                    }
                </div>
            </div>
            
            // Tabs for Preview/Code
            <Tabs default_value="preview">
                <TabsList class="inline-flex h-9 items-center justify-center rounded-lg bg-zinc-100 p-1 text-zinc-500 dark:bg-zinc-800 dark:text-zinc-400">
                    <TabsTrigger value="preview" class="rounded-md px-3 py-1 text-sm font-medium">
                        {"Preview"}
                    </TabsTrigger>
                    <TabsTrigger value="code" class="rounded-md px-3 py-1 text-sm font-medium">
                        {"Code"}
                    </TabsTrigger>
                </TabsList>
                
                <TabsContent value="preview" class="mt-4">
                    {preview_content}
                </TabsContent>
                
                <TabsContent value="code" class="mt-4">
                    {code_content}
                </TabsContent>
            </Tabs>
        </div>
    }
}

/// A category card for the blocks index page
#[derive(Properties, PartialEq)]
pub struct BlockCategoryCardProps {
    pub title: AttrValue,
    pub description: AttrValue,
    pub href: AttrValue,
    pub count: usize,
    #[prop_or_default]
    pub icon: Option<Html>,
}

#[function_component(BlockCategoryCard)]
pub fn block_category_card(props: &BlockCategoryCardProps) -> Html {
    html! {
        <a 
            href={props.href.clone()}
            class="group flex flex-col rounded-xl border border-zinc-200 bg-white p-6 transition-all hover:border-zinc-300 hover:shadow-md dark:border-zinc-800 dark:bg-zinc-900 dark:hover:border-zinc-700"
        >
            <div class="flex items-start justify-between">
                <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-zinc-100 text-zinc-600 dark:bg-zinc-800 dark:text-zinc-400">
                    if let Some(icon) = &props.icon {
                        {icon.clone()}
                    } else {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6.429 9.75 2.25 12l4.179 2.25m0-4.5 5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0 4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0-5.571 3-5.571-3" />
                        </svg>
                    }
                </div>
                <span class="rounded-full bg-zinc-100 px-2.5 py-0.5 text-xs font-medium text-zinc-600 dark:bg-zinc-800 dark:text-zinc-400">
                    {props.count} {" blocks"}
                </span>
            </div>
            <h3 class="mt-4 text-lg font-semibold text-zinc-900 group-hover:text-zinc-700 dark:text-white dark:group-hover:text-zinc-200">
                {&props.title}
            </h3>
            <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
                {&props.description}
            </p>
            <div class="mt-4 flex items-center text-sm font-medium text-zinc-600 group-hover:text-zinc-900 dark:text-zinc-400 dark:group-hover:text-white">
                {"Browse blocks"}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="ml-1 h-4 w-4 transition-transform group-hover:translate-x-1">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" />
                </svg>
            </div>
        </a>
    }
}
