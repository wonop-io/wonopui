use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(MediaQueryDemo)]
fn media_query_demo() -> Html {
    let is_small_screen = use_media_query("(max-width: 600px)");
    let is_dark_mode = use_media_query("(prefers-color-scheme: dark)");

    html! {
        <div>
            <p class="mb-2">
                { "Current screen size: " }
                {
                    if is_small_screen {
                        html! { <span class="font-bold text-green-600 dark:text-green-400">{ "Small (≤600px)" }</span> }
                    } else {
                        html! { <span class="font-bold text-blue-600 dark:text-blue-400">{ "Large (>600px)" }</span> }
                    }
                }
            </p>
            <p>
                { "Preferred color scheme: " }
                {
                    if is_dark_mode {
                        html! { <span class="font-bold text-purple-600 dark:text-purple-400">{ "Dark" }</span> }
                    } else {
                        html! { <span class="font-bold text-yellow-600 dark:text-yellow-400">{ "Light" }</span> }
                    }
                }
            </p>
        </div>
    }
}

#[function_component(MediaQueryDocumentation)]
pub fn media_query_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "use_media_query Hook" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The use_media_query hook allows you to monitor and react to changes in the result of a CSS media query. This is particularly useful for creating responsive designs or adapting to user preferences." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <MediaQueryDemo />
                }}
                code={r#"
use wonopui::*;
use yew::prelude::*;

#[function_component(MediaQueryDemo)]
fn media_query_demo() -> Html {
    let is_small_screen = use_media_query("(max-width: 600px)");
    let is_dark_mode = use_media_query("(prefers-color-scheme: dark)");

    html! {
        <div>
            <p class="mb-2">
                { "Current screen size: " }
                {
                    if *is_small_screen {
                        html! { <span class="font-bold text-green-600 dark:text-green-400">{ "Small (≤600px)" }</span> }
                    } else {
                        html! { <span class="font-bold text-blue-600 dark:text-blue-400">{ "Large (>600px)" }</span> }
                    }
                }
            </p>
            <p>
                { "Preferred color scheme: " }
                {
                    if *is_dark_mode {
                        html! { <span class="font-bold text-purple-600 dark:text-purple-400">{ "Dark" }</span> }
                    } else {
                        html! { <span class="font-bold text-yellow-600 dark:text-yellow-400">{ "Light" }</span> }
                    }
                }
            </p>
        </div>
    }
}
"#
                .to_string()}
            />

            <Features features={vec!["use_media_query"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="use_media_query"
                description="A custom hook to monitor and react to CSS media query changes."
                props={vec![
                    ("query", "&str", "The CSS media query to match against."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The hook returns a boolean indicating whether the media query currently matches.".to_string(),
                    "It uses the browser's `matchMedia` API to observe changes to the media query.".to_string(),
                    "The hook automatically updates when the media query result changes, triggering a re-render.".to_string(),
                    "Ensure that the media query string passed to the hook is valid and supported by the browser.".to_string(),
                    "This hook is particularly useful for creating responsive layouts or adapting to user preferences like dark mode.".to_string(),
                    "Remember that media queries can impact performance if overused, so use them judiciously.".to_string(),
                ]}
            />

        </Container>
    }
}
