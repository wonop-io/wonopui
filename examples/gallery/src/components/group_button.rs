use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(GroupButtonThemeEditor)]
pub fn group_button_theme_editor() -> Html {
    let fields = vec![
        (
            "group_button_container".to_string(),
            "Container".to_string(),
        ),
        ("group_button_list".to_string(), "Button List".to_string()),
        (
            "group_button_trigger".to_string(),
            "Button Trigger".to_string(),
        ),
        (
            "group_button_trigger_active".to_string(),
            "Active Trigger".to_string(),
        ),
        (
            "group_button_trigger_inactive".to_string(),
            "Inactive Trigger".to_string(),
        ),
    ];

    let preview = html! {
        <GroupButton default_value="option1" class="w-[400px]" direction={FlexDirection::Row} on_change={Callback::from(|_| {})}>
            <GroupButtonTrigger value="option1" onclick={Callback::noop()}>{"Option 1"}</GroupButtonTrigger>
            <GroupButtonTrigger value="option2" onclick={Callback::noop()}>{"Option 2"}</GroupButtonTrigger>
        </GroupButton>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(GroupButtonDocumentation)]
pub fn group_button_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "GroupButton Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The GroupButton component is designed to group multiple buttons together, allowing for quick toggling between different options. It is highly customizable with flex direction, default active value, and more." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <GroupButton default_value="option1" class="w-[400px]" direction={FlexDirection::Row} on_change={Callback::from(|_| {})}>
                        <GroupButtonTrigger value="option1" onclick={Callback::noop()}>{"Option 1"}</GroupButtonTrigger>
                        <GroupButtonTrigger value="option2" onclick={Callback::noop()}>{"Option 2"}</GroupButtonTrigger>
                    </GroupButton>
                }}
                customize={html! {
                    <GroupButtonThemeEditor />
                }}
                code={r#"
<GroupButton default_value="option1" class="w-[400px]" direction={FlexDirection::Row} on_change={Callback::from(|value: String| {
    // Handle value change
})}>
    <GroupButtonTrigger value="option1" onclick={Callback::noop()}>{"Option 1"}</GroupButtonTrigger>
    <GroupButtonTrigger value="option2" onclick={Callback::noop()}>{"Option 2"}</GroupButtonTrigger>
</GroupButton>
                "#.to_string()}
            />

            <Features features={vec!["GroupButton", "GroupButtonTrigger"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="GroupButton"
                description="Props for the GroupButton component."
                props={vec![
                    ("children", "Children", "The child elements, typically GroupButtonTrigger components."),
                    ("default_value", "String", "The default active button value."),
                    ("class", "Classes", "Additional CSS classes for the container."),
                    ("direction", "FlexDirection", "The direction of the flex container (Row or Column)."),
                    ("on_change", "Callback<String>", "Callback function that is called when an option is selected."),
                ]}
            />

            <ApiSection
                title="GroupButtonTrigger"
                description="Props for the GroupButtonTrigger component."
                props={vec![
                    ("value", "String", "The value for the button."),
                    ("children", "Children", "Content to be displayed inside the button."),
                    ("class", "Classes", "Additional CSS classes for the button."),
                    ("onclick", "Callback<MouseEvent>", "Callback function for the click event."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The component uses a context to manage the active button state.".to_string(),
                    "FlexDirection determines the layout direction of the buttons (either Row or Column).".to_string(),
                    "The active button is styled differently for better user distinction.".to_string(),
                    "Ensure that each GroupButtonTrigger has a unique value.".to_string(),
                    "The GroupButton component is responsive and adapts to different screen sizes.".to_string(),
                    "For accessibility, the component supports keyboard navigation.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"GroupButton".to_string()}
                class_descriptions={vec![
                    ("group_button_container".to_string(), "For the outermost container of the group button".to_string()),
                    ("group_button_list".to_string(), "For the container that wraps the group buttons".to_string()),
                    ("group_button_trigger".to_string(), "For individual button triggers".to_string()),
                    ("group_button_trigger_active".to_string(), "For the active button trigger".to_string()),
                    ("group_button_trigger_inactive".to_string(), "For inactive button triggers".to_string()),
                ]}
            />
        </Container>
    }
}
