use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(CheckboxThemeEditor)]
pub fn checkbox_theme_editor() -> Html {
    let fields = vec![
        ("checkbox_base".to_string(), "Checkbox Base".to_string()),
        (
            "checkbox_checked".to_string(),
            "Checkbox Checked".to_string(),
        ),
        (
            "checkbox_unchecked".to_string(),
            "Checkbox Unchecked".to_string(),
        ),
        (
            "checkbox_disabled".to_string(),
            "Checkbox Disabled".to_string(),
        ),
        ("checkbox_label".to_string(), "Checkbox Label".to_string()),
    ];

    let preview = html! {
        <div class="flex items-center space-x-2">
            <Checkbox id="terms" checked={false} on_toggle={Callback::from(|_| {})} />
            <label
                for="terms"
                class={BRANDGUIDE.checkbox_label}
            >
                { "Accept terms and conditions" }
            </label>
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CheckboxDocumentation)]
pub fn checkbox_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Checkbox Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Checkbox component is a versatile input element that allows users to toggle between checked and unchecked states. It can be used in forms, settings, and anywhere a binary choice is needed." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center space-x-2">
                        <Checkbox id="terms" checked={false} on_toggle={Callback::from(|_| {})} />
                        <label
                            for="terms"
                            class={BRANDGUIDE.checkbox_label}
                        >
                            { "Accept terms and conditions" }
                        </label>
                    </div>
                }}
                customize={html! {
                    <CheckboxThemeEditor />
                }}
                code={r#"
<div class="flex items-center space-x-2">
    <Checkbox
        id="terms"
        checked={false}
        on_toggle={Callback::from(|_| {})}
    />
    <label
        for="terms"
        class={BRANDGUIDE.checkbox_label}
    >
        { "Accept terms and conditions" }
    </label>
</div>"#.to_string()}
            />

            <Features features={vec!["Checkbox", "Customizable appearance", "Disabled state support", "Accessible design"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Checkbox"
                description="Props for the Checkbox component."
                props={vec![
                    ("id", "String", "The unique identifier for the checkbox."),
                    ("checked", "bool", "The checked state of the checkbox."),
                    ("on_toggle", "Callback<MouseEvent>", "The callback to be called when the checkbox is toggled."),
                    ("disabled", "bool", "Whether the checkbox is disabled."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Always use the Checkbox component with an associated label for better accessibility.".to_string(),
                    "The 'on_toggle' callback is triggered when the checkbox is clicked, allowing you to update the checked state in your application.".to_string(),
                    "When disabled, the checkbox will not respond to user interactions and will have a visual indication of being disabled.".to_string(),
                    "For groups of related checkboxes, consider using a fieldset and legend to provide a description for the entire group.".to_string(),
                    "Ensure sufficient contrast between the checkbox, its label, and the background for better visibility.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Checkbox".to_string()}
                class_descriptions={vec![
                    ("checkbox_base".to_string(), "Base styles for the checkbox, including size, border, and background".to_string()),
                    ("checkbox_checked".to_string(), "Styles applied when the checkbox is checked, typically including a checkmark or fill color".to_string()),
                    ("checkbox_unchecked".to_string(), "Styles applied when the checkbox is unchecked".to_string()),
                    ("checkbox_disabled".to_string(), "Styles applied when the checkbox is disabled, usually reducing opacity and changing the cursor".to_string()),
                    ("checkbox_label".to_string(), "Styles for the checkbox label, including font size, color, and spacing".to_string()),
                ]}
            />
        </Container>
    }
}
