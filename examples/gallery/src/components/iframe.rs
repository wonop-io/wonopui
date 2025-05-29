use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(IframeThemeEditor)]
pub fn iframe_theme_editor() -> Html {
    let fields = vec![
        (
            "iframe_container".to_string(),
            "Iframe Container".to_string(),
        ),
        ("iframe_element".to_string(), "Iframe Element".to_string()),
    ];

    let preview = html! {
        <Iframe
            class="w-full h-64"
            body_class="p-4"
            srcdoc={Some("<p>Hello, iframe content!</p>".to_string())}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(IframeDocumentation)]
pub fn iframe_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Iframe Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Iframe component allows embedding an iframe within a Yew application. It comes with support for injecting documents, event handling, and propagating events to the parent window, making it highly customizable and interactive." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <Iframe
                        class="w-full h-64"
                        body_class="p-4"
                        srcdoc={Some("<p>Hello, iframe content!</p>".to_string())}
                        onkeydown={Callback::from(|_e: web_sys::KeyboardEvent| {
                            log::info!("Key down event within iframe");
                        })}
                    >
                        { "Fallback content if no srcdoc is provided" }
                    </Iframe>
                }}
                customize={html! {
                    <IframeThemeEditor />
                }}
                code={r#"
<Iframe
    class="w-full h-64"
    body_class="p-4"
    srcdoc={Some("<p>Hello, iframe content!</p>".to_string())}
    onkeydown={Callback::from(|_e: web_sys::KeyboardEvent| {
        log::info!("Key down event within iframe");
    })}
>
    { "Fallback content if no srcdoc is provided" }
</Iframe>"#.to_string()}
            />

            <Features features={vec!["Iframe"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Iframe Props"
                description="Props for the Iframe component."
                props={vec![
                    ("children", "Children", "The content to be displayed inside the iframe if no srcdoc is provided."),
                    ("class", "Classes", "CSS class(es) for the iframe element."),
                    ("body_class", "String", "CSS class for the iframe body element."),
                    ("srcdoc", "Option<String>", "Optional HTML content to embed within the iframe."),
                    ("onkeydown", "Option<Callback<web_sys::KeyboardEvent>>", "Callback for handling 'keydown' events in the iframe."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The component uses internal state and effects to manage the iframe content and events.".to_string(),
                    "You can propagate events like mousemove, pointerup, and keydown from the iframe to the parent window.".to_string(),
                    "Make sure the iframe content is considered safe, as it runs within the same origin and could access parent window's JavaScript context.".to_string(),
                    "The Iframe component automatically injects necessary styles and scripts from the parent document into the iframe.".to_string(),
                    "Event propagation allows for seamless interaction between the iframe content and the parent application.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Iframe".to_string()}
                class_descriptions={vec![
                    ("iframe_container".to_string(), "For the main container of the iframe component".to_string()),
                    ("iframe_element".to_string(), "For the iframe element itself".to_string()),
                ]}
            />
        </Container>
    }
}
