use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(CarouselThemeEditor)]
pub fn carousel_theme_editor() -> Html {
    let fields = vec![
        (
            "carousel_container".to_string(),
            "Carousel Container".to_string(),
        ),
        ("carousel_inner".to_string(), "Carousel Inner".to_string()),
        ("carousel_item".to_string(), "Carousel Item".to_string()),
        (
            "carousel_item_active".to_string(),
            "Active Carousel Item".to_string(),
        ),
        (
            "carousel_controls".to_string(),
            "Carousel Controls".to_string(),
        ),
        (
            "carousel_control_prev".to_string(),
            "Previous Control".to_string(),
        ),
        (
            "carousel_control_next".to_string(),
            "Next Control".to_string(),
        ),
    ];

    let preview = html! {
        <Carousel interval={3000} class="h-96 w-96">
            <CarouselItem>
                <div style="background-color: #f0f0f0; padding: 20px; text-align: center;">{"Item 1"}</div>
            </CarouselItem>
            <CarouselItem>
                <div style="background-color: #e0e0e0; padding: 20px; text-align: center;">{"Item 2"}</div>
            </CarouselItem>
            <CarouselItem>
                <div style="background-color: #d0d0d0; padding: 20px; text-align: center;">{"Item 3"}</div>
            </CarouselItem>
        </Carousel>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CarouselDocumentation)]
pub fn carousel_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Carousel Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Carousel component is a dynamic slideshow for cycling through a series of content. It provides an engaging way to display multiple items in a limited space, with automatic cycling and manual navigation options." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Carousel interval={3000} class="h-96 w-96"
                        next={html! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3" />
                            </svg>
                        }}
                        prev={html! {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18" />
                            </svg>

                        }}
                    >
                        <CarouselItem>
                            <div class="h-full w-full flex justify-center items-center bg-amber-500" >{"Item 1"}</div>
                        </CarouselItem>
                        <CarouselItem>
                            <div class="h-full w-full flex justify-center items-center bg-indigo-500" >{"Item 2"}</div>
                        </CarouselItem>
                        <CarouselItem>
                            <div class="h-full w-full flex justify-center items-center bg-green-500" >{"Item 3"}</div>
                        </CarouselItem>
                    </Carousel>
                }}
                customize={html! {
                    <CarouselThemeEditor />
                }}
                code={r#"
<Carousel interval={3000}>
    <CarouselItem>
        <div style="background-color: #f0f0f0; padding: 20px; text-align: center;">{"Item 1"}</div>
    </CarouselItem>
    <CarouselItem>
        <div style="background-color: #e0e0e0; padding: 20px; text-align: center;">{"Item 2"}</div>
    </CarouselItem>
    <CarouselItem>
        <div style="background-color: #d0d0d0; padding: 20px; text-align: center;">{"Item 3"}</div>
    </CarouselItem>
</Carousel>"#.to_string()}
            />

            <Features features={vec![
                "Automatic cycling through items",
                "Customizable interval",
                "Manual navigation controls",
                "Responsive design",
                "Smooth transitions between items",
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Carousel"
                description="The main container for the carousel component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the carousel component."),
                    ("interval", "u32", "The interval in milliseconds for cycling through the items."),
                ]}
            />

            <ApiSection
                title="CarouselItem"
                description="An individual item within the carousel."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the carousel item."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Ensure that all CarouselItems have similar dimensions for consistent appearance.".to_string(),
                    "Consider using lazy loading for image-heavy carousels to improve performance.".to_string(),
                    "Provide alternative navigation methods (e.g., dots or thumbnails) for better user experience.".to_string(),
                    "Use aria-live attributes to make the carousel accessible to screen readers.".to_string(),
                    "Implement touch swipe functionality for better mobile experience.".to_string(),
                    "Consider pausing the auto-rotation when the user hovers over the carousel.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Carousel".to_string()}
                class_descriptions={vec![
                    ("carousel_container".to_string(), "For the main container of the carousel component".to_string()),
                    ("carousel_inner".to_string(), "For the inner container that holds all carousel items".to_string()),
                    ("carousel_item".to_string(), "For individual carousel items".to_string()),
                    ("carousel_item_active".to_string(), "For the currently active (visible) carousel item".to_string()),
                    ("carousel_controls".to_string(), "For the navigation controls container".to_string()),
                    ("carousel_control_prev".to_string(), "For the previous control button".to_string()),
                    ("carousel_control_next".to_string(), "For the next control button".to_string()),
                ]}
            />
        </Container>
    }
}
