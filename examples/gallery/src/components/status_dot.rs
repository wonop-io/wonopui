use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(StatusDotDocumentation)]
pub fn status_dot_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "StatusDot Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "A simple colored dot indicator for displaying status. Useful for showing online/offline states, health status, or any binary/categorical state." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Variants" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center gap-4">
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Success} />
                            <span class="text-sm">{"Success"}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Warning} />
                            <span class="text-sm">{"Warning"}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Error} />
                            <span class="text-sm">{"Error"}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Info} />
                            <span class="text-sm">{"Info"}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Neutral} />
                            <span class="text-sm">{"Neutral"}</span>
                        </div>
                    </div>
                }}
                code={r#"
<StatusDot variant={StatusDotVariant::Success} />
<StatusDot variant={StatusDotVariant::Warning} />
<StatusDot variant={StatusDotVariant::Error} />
<StatusDot variant={StatusDotVariant::Info} />
<StatusDot variant={StatusDotVariant::Neutral} />"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Sizes" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center gap-4">
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Success} size={StatusDotSize::Sm} />
                            <span class="text-sm">{"Small"}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Success} size={StatusDotSize::Md} />
                            <span class="text-sm">{"Medium"}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <StatusDot variant={StatusDotVariant::Success} size={StatusDotSize::Lg} />
                            <span class="text-sm">{"Large"}</span>
                        </div>
                    </div>
                }}
                code={r#"
<StatusDot variant={StatusDotVariant::Success} size={StatusDotSize::Sm} />
<StatusDot variant={StatusDotVariant::Success} size={StatusDotSize::Md} />
<StatusDot variant={StatusDotVariant::Success} size={StatusDotSize::Lg} />"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Pulse Animation" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center gap-4">
                        <StatusDot variant={StatusDotVariant::Success} pulse={true} />
                        <StatusDot variant={StatusDotVariant::Error} pulse={true} />
                    </div>
                }}
                code={r#"
<StatusDot variant={StatusDotVariant::Success} pulse={true} />
<StatusDot variant={StatusDotVariant::Error} pulse={true} />"#.to_string()}
            />

            <Features features={vec![
                "Five color variants (Success, Warning, Error, Info, Neutral)",
                "Three size options (Sm, Md, Lg)",
                "Optional pulse animation",
                "Accessible with ARIA attributes"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="StatusDot"
                description="Props for the StatusDot component."
                props={vec![
                    ("variant", "StatusDotVariant", "Color variant: Success, Warning, Error, Info, or Neutral. Default: Neutral"),
                    ("size", "StatusDotSize", "Size: Sm, Md, or Lg. Default: Md"),
                    ("pulse", "bool", "Whether to show pulse animation. Default: false"),
                    ("class", "Classes", "Additional CSS classes to apply"),
                    ("label", "Option<String>", "Accessible label for screen readers"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Use StatusDot for simple status indicators without text.".to_string(),
                    "Use the pulse effect sparingly to draw attention to important status changes.".to_string(),
                    "For status with labels, consider using StatusIndicator instead.".to_string(),
                ]}
            />
        </Container>
    }
}
