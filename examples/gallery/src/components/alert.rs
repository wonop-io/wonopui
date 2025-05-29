use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(AlertThemeEditor)]
pub fn alert_theme_editor() -> Html {
    let fields = vec![
        ("alert_base".to_string(), "Base Alert".to_string()),
        ("alert_success".to_string(), "Success Alert".to_string()),
        ("alert_warning".to_string(), "Warning Alert".to_string()),
        ("alert_error".to_string(), "Error Alert".to_string()),
        ("alert_info".to_string(), "Info Alert".to_string()),
    ];

    let preview = html! {
        <div class="flex flex-col gap-2">
            <Alert alert_type={AlertType::Info}>
                <p>{"This is an info alert"}</p>
            </Alert>
            <Alert alert_type={AlertType::Success}>
                <p>{"This is a success alert"}</p>
            </Alert>
            <Alert alert_type={AlertType::Warning}>
                <p>{"This is a warning alert"}</p>
            </Alert>
            <Alert alert_type={AlertType::Error}>
                <p>{"This is an error alert"}</p>
            </Alert>
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(AlertDocumentation)]
pub fn alert_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Alert Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Alert component is used to display important messages to the user. It supports different types of alerts such as info, success, warning, and error." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex flex-col gap-2">
                        <Alert alert_type={AlertType::Info}>
                            <p>{"This is an info alert"}</p>
                        </Alert>
                        <Alert alert_type={AlertType::Success}>
                            <p>{"This is a success alert"}</p>
                        </Alert>
                        <Alert alert_type={AlertType::Warning}>
                            <p>{"This is a warning alert"}</p>
                        </Alert>
                        <Alert alert_type={AlertType::Error}>
                            <p>{"This is an error alert"}</p>
                        </Alert>
                    </div>
                }}
                customize={html! {
                    <AlertThemeEditor />
                }}
                code={r#"
<Alert alert_type={AlertType::Info}>
    <p>{"This is an info alert"}</p>
</Alert>
<Alert alert_type={AlertType::Success}>
    <p>{"This is a success alert"}</p>
</Alert>
<Alert alert_type={AlertType::Warning}>
    <p>{"This is a warning alert"}</p>
</Alert>
<Alert alert_type={AlertType::Error}>
    <p>{"This is an error alert"}</p>
</Alert>"#.to_string()}
            />

            <Features features={vec!["Alert"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Alert"
                description="Props for the Alert component."
                props={vec![
                    ("alert_type", "AlertType", "The type of alert to be displayed. It can be one of the following: Info, Success, Warning, Error."),
                    ("icon", "Option<Html>", "An optional icon to be displayed in the alert."),
                    ("children", "Children", "Child elements to be rendered inside the alert."),
                    ("class", "Classes", "Additional CSS classes to apply to the alert container."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Alert component uses children for its content, allowing for more complex and flexible alert messages.".to_string(),
                    "The alert_type prop determines the styling and icon of the alert.".to_string(),
                    "Custom icons can be provided using the icon prop.".to_string(),
                    "The component is responsive and will adjust to the width of its container.".to_string(),
                    "For accessibility, the component uses appropriate ARIA attributes.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Alert".to_string()}
                class_descriptions={vec![
                    ("alert_base".to_string(), "Base styles for all alert types".to_string()),
                    ("alert_success".to_string(), "Styles specific to success alerts".to_string()),
                    ("alert_warning".to_string(), "Styles specific to warning alerts".to_string()),
                    ("alert_error".to_string(), "Styles specific to error alerts".to_string()),
                    ("alert_info".to_string(), "Styles specific to info alerts".to_string()),
                ]}
            />
        </Container>
    }
}
