use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(WindowProviderDocumentation)]
pub fn window_provider_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "WindowProvider Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The WindowProvider component offers a context provider for managing the window object associated with an iframe. This enables descendant components to access and interact with the global window object effectively." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <WindowProvider iframe_ref={NodeRef::default()}>
                        <p>{ "Content inside WindowProvider" }</p>
                    </WindowProvider>
                }}
                code={r#"
<WindowProvider iframe_ref={NodeRef::default()}>
    <p>{ "Content inside WindowProvider" }</p>
</WindowProvider>
                "#.to_string()}
            />
            <Features features={vec!["WindowProvider"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>

            <ApiSection
                title="WindowProviderProps"
                description="Props for the WindowProvider component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the provider."),
                    ("iframe_ref", "NodeRef", "A node reference to the iframe for which the window object is provided."),
                ]}
            />

            <NotesSection
                title={"Notes".to_string()}
                notes={vec![
                    "The WindowProvider component uses the UseReducerHandle to manage the window state.".to_string(),
                    "It utilizes the use_effect_with hook to set the window context from the iframe reference.".to_string(),
                    "The provided window context can be accessed in descendant components using the use_window hook.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"WindowProvider".to_string()}
                class_descriptions={vec![
                    ("window_provider_container".to_string(), "For the main container".to_string()),
                ]}
            />

        </Container>
    }
}
