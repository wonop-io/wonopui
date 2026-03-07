use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(ErrorBoundaryDocumentation)]
pub fn error_boundary_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "ErrorBoundary Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "A wrapper component that catches errors in child components and displays a fallback UI. Useful for preventing entire app crashes from component errors." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="space-y-4">
                        <ErrorBoundary>
                            <div class="p-4 border rounded">
                                {"This content is protected by an error boundary"}
                            </div>
                        </ErrorBoundary>
                    </div>
                }}
                code={r#"
<ErrorBoundary>
    <RiskyComponent />
</ErrorBoundary>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Default Fallback" }</h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">{ "When an error occurs, the ErrorBoundary displays a default fallback UI with an error message and retry button." }</p>
            <ExampleCode
                preview={html! {
                    <div class="p-4 border border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20 rounded-lg">
                        <h3 class="text-lg font-semibold text-red-800 dark:text-red-200 mb-2">{"Something went wrong"}</h3>
                        <p class="text-sm text-red-600 dark:text-red-400">{"Example error message"}</p>
                        <button class="mt-4 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-md text-sm font-medium">
                            {"Try again"}
                        </button>
                    </div>
                }}
                code={r#"
// Default fallback when an error is caught
<div class="error-container">
    <h3>Something went wrong</h3>
    <p>{error_message}</p>
    <button onclick={reset}>Try again</button>
</div>"#.to_string()}
            />

            <Features features={vec![
                "Catches rendering errors in child components",
                "Provides default fallback UI",
                "Supports custom fallback rendering",
                "Reset/retry functionality",
                "Optional error callback"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="ErrorBoundary"
                description="Props for the ErrorBoundary component."
                props={vec![
                    ("children", "Children", "Child components to wrap"),
                    ("fallback", "Option<Callback<(String, Callback<()>), Html>>", "Custom fallback renderer"),
                    ("on_error", "Option<Callback<String>>", "Callback when an error is caught"),
                    ("class", "Classes", "Additional CSS classes"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Wrap components that might fail with ErrorBoundary to prevent app crashes.".to_string(),
                    "Use the on_error callback to log errors to a monitoring service.".to_string(),
                    "Consider using multiple ErrorBoundary components for different sections.".to_string(),
                    "Note: Yew's error boundary support is limited; this provides a context-based pattern.".to_string(),
                ]}
            />
        </Container>
    }
}
