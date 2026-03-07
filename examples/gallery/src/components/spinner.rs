use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(SpinnerDocumentation)]
pub fn spinner_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Spinner Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Spinner component provides a loading indicator with multiple size variants. Use it to indicate that content is loading or an action is in progress." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center gap-4">
                        <Spinner size={SpinnerSize::Sm} />
                        <Spinner size={SpinnerSize::Md} />
                        <Spinner size={SpinnerSize::Lg} />
                        <Spinner size={SpinnerSize::Xl} />
                    </div>
                }}
                code={r#"
<Spinner size={SpinnerSize::Sm} />
<Spinner size={SpinnerSize::Md} />
<Spinner size={SpinnerSize::Lg} />
<Spinner size={SpinnerSize::Xl} />"#.to_string()}
            />

            <Features features={vec![
                "Multiple size variants (Sm, Md, Lg, Xl)",
                "CSS animation with reduced motion support",
                "Accessible with ARIA attributes",
                "Customizable label for screen readers"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Spinner"
                description="Props for the Spinner component."
                props={vec![
                    ("size", "SpinnerSize", "Size variant: Sm (16px), Md (24px), Lg (32px), or Xl (48px). Default: Md"),
                    ("class", "Classes", "Additional CSS classes to apply"),
                    ("label", "String", "Accessible label for screen readers. Default: 'Loading...'"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Use the Spinner to indicate loading states in buttons, cards, or page sections.".to_string(),
                    "Choose an appropriate size based on the context - smaller for inline use, larger for page-level loading.".to_string(),
                    "The spinner respects reduced motion preferences with a slower animation.".to_string(),
                    "Always provide a meaningful label for accessibility.".to_string(),
                ]}
            />
        </Container>
    }
}
