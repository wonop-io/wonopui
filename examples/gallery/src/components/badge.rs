use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(BadgeThemeEditor)]
pub fn badge_theme_editor() -> Html {
    let fields = vec![
        ("badge_base".to_string(), "Base Badge".to_string()),
        ("badge_success".to_string(), "Success Badge".to_string()),
        ("badge_warning".to_string(), "Warning Badge".to_string()),
        ("badge_error".to_string(), "Error Badge".to_string()),
        ("badge_info".to_string(), "Info Badge".to_string()),
        ("badge_default".to_string(), "Default Badge".to_string()),
    ];

    let preview = html! {
        <div class="flex gap-2">
            <Badge label="New" badge_type={BadgeType::Success} />
            <Badge label="Warning" badge_type={BadgeType::Warning} />
            <Badge label="Error" badge_type={BadgeType::Error} />
            <Badge label="Info" badge_type={BadgeType::Info} />
            <Badge label="Default" badge_type={BadgeType::Default} />
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(BadgeDocumentation)]
pub fn badge_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Badge Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Badge component is used to display a small, inline element with a short label. It's often used to highlight status, counts, or categories in a compact and visually distinct way." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex gap-2">
                        <Badge label="New" badge_type={BadgeType::Success} />
                        <Badge label="Warning" badge_type={BadgeType::Warning} />
                        <Badge label="Error" badge_type={BadgeType::Error} />
                        <Badge label="Info" badge_type={BadgeType::Info} />
                        <Badge label="Default" badge_type={BadgeType::Default} />
                    </div>
                }}
                customize={html! {
                    <BadgeThemeEditor />
                }}
                code={r#"
<Badge label="New" badge_type={BadgeType::Success} />
<Badge label="Warning" badge_type={BadgeType::Warning} />
<Badge label="Error" badge_type={BadgeType::Error} />
<Badge label="Info" badge_type={BadgeType::Info} />
<Badge label="Default" badge_type={BadgeType::Default} />"#.to_string()}
            />

            <Features features={vec![
                "Multiple badge types (Success, Warning, Error, Info, Default)",
                "Customizable styling through theme editor",
                "Compact and inline design",
                "Accessible with high contrast options"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Badge"
                description="Props for the Badge component."
                props={vec![
                    ("label", "String", "The text to be displayed inside the badge."),
                    ("badge_type", "BadgeType", "The type of badge to be displayed. Can be Success, Warning, Error, Info, or Default."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Use badges sparingly to avoid cluttering the UI.".to_string(),
                    "Choose the appropriate badge type to convey the right context (e.g., Success for positive outcomes, Warning for cautions).".to_string(),
                    "Keep the label text short and concise for better readability.".to_string(),
                    "Ensure sufficient color contrast between the badge and its background for accessibility.".to_string(),
                    "Consider using badges in lists, tables, or next to headings to provide additional context.".to_string(),
                    "For dynamic content, update the badge label or type programmatically as needed.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Badge".to_string()}
                class_descriptions={vec![
                    ("badge_base".to_string(), "Base styling applied to all badges".to_string()),
                    ("badge_success".to_string(), "Styling for success-type badges".to_string()),
                    ("badge_warning".to_string(), "Styling for warning-type badges".to_string()),
                    ("badge_error".to_string(), "Styling for error-type badges".to_string()),
                    ("badge_info".to_string(), "Styling for info-type badges".to_string()),
                    ("badge_default".to_string(), "Styling for default-type badges".to_string()),
                ]}
            />
        </Container>
    }
}
