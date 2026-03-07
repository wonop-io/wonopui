use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(StatusIndicatorDocumentation)]
pub fn status_indicator_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "StatusIndicator Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "A status indicator that combines a colored dot with an optional icon and text label. Perfect for displaying user status, service health, or any labeled state." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex flex-col gap-4">
                        <StatusIndicator variant={StatusDotVariant::Success} label="Online" />
                        <StatusIndicator variant={StatusDotVariant::Warning} label="Away" />
                        <StatusIndicator variant={StatusDotVariant::Error} label="Offline" />
                        <StatusIndicator variant={StatusDotVariant::Info} label="Busy" />
                        <StatusIndicator variant={StatusDotVariant::Neutral} label="Unknown" />
                    </div>
                }}
                code={r#"
<StatusIndicator variant={StatusDotVariant::Success} label="Online" />
<StatusIndicator variant={StatusDotVariant::Warning} label="Away" />
<StatusIndicator variant={StatusDotVariant::Error} label="Offline" />
<StatusIndicator variant={StatusDotVariant::Info} label="Busy" />
<StatusIndicator variant={StatusDotVariant::Neutral} label="Unknown" />"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "With Pulse Animation" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex flex-col gap-4">
                        <StatusIndicator variant={StatusDotVariant::Success} label="Live" pulse={true} />
                        <StatusIndicator variant={StatusDotVariant::Error} label="Alert" pulse={true} />
                    </div>
                }}
                code={r#"
<StatusIndicator variant={StatusDotVariant::Success} label="Live" pulse={true} />
<StatusIndicator variant={StatusDotVariant::Error} label="Alert" pulse={true} />"#.to_string()}
            />

            <Features features={vec![
                "Combines StatusDot with optional label",
                "Supports optional icon before the dot",
                "All StatusDot variants and sizes available",
                "Optional pulse animation"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="StatusIndicator"
                description="Props for the StatusIndicator component."
                props={vec![
                    ("variant", "StatusDotVariant", "Color variant for the dot"),
                    ("size", "StatusDotSize", "Size of the status dot"),
                    ("pulse", "bool", "Whether to show pulse animation"),
                    ("label", "Option<String>", "Text label to display"),
                    ("icon", "Option<Html>", "Optional icon to render before the dot"),
                    ("class", "Classes", "Additional CSS classes"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Use StatusIndicator when you need to show status with a text label.".to_string(),
                    "For simple dot-only indicators, use StatusDot directly.".to_string(),
                    "The icon prop can be used for custom icons alongside the status.".to_string(),
                ]}
            />
        </Container>
    }
}
