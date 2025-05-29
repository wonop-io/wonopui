use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ToggleThemeEditor)]
pub fn toggle_theme_editor() -> Html {
    let fields = vec![
        (
            "toggle_container".to_string(),
            "Toggle Container".to_string(),
        ),
        ("toggle_base".to_string(), "Toggle Base".to_string()),
        ("toggle_checked".to_string(), "Toggle Checked".to_string()),
        (
            "toggle_unchecked".to_string(),
            "Toggle Unchecked".to_string(),
        ),
        ("toggle_disabled".to_string(), "Toggle Disabled".to_string()),
        ("toggle_label".to_string(), "Toggle Label".to_string()),
        ("toggle_icon".to_string(), "Toggle Icon".to_string()),
    ];

    let preview = html! {
        <Toggle id="theme-toggle" checked=true>
            <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
                <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
            </svg>
        </Toggle>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ToggleDemo)]
pub fn toggle_demo() -> Html {
    let checked = use_state(|| false);
    let disabled = use_state(|| false);

    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    let on_disable_toggle = {
        let disabled = disabled.clone();
        Callback::from(move |_| {
            disabled.set(!*disabled);
        })
    };

    html! {
        <div class="space-y-4">
            <div class="flex items-center space-x-2">
                <Toggle id="bold-toggle" checked={*checked} on_toggle={on_toggle.clone()}>
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
                        <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
                    </svg>
                </Toggle>
                <Label for_id="bold-toggle">
                    { "Toggle bold" }
                </Label>
            </div>
            <div class="flex items-center space-x-2">
                <Toggle id="italic-toggle" checked={false} on_toggle={Callback::from(|_| {})}>
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <line x1="19" y1="4" x2="10" y2="4"></line>
                        <line x1="14" y1="20" x2="5" y2="20"></line>
                        <line x1="15" y1="4" x2="9" y2="20"></line>
                    </svg>
                </Toggle>
                <Label for_id="italic-toggle">
                    { "Toggle italic" }
                </Label>
            </div>
            <div class="flex items-center space-x-2">
                <Toggle id="disabled-toggle" checked={*checked} on_toggle={on_toggle} disabled={*disabled}>
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M18 6L6 18"></path>
                        <path d="M6 6l12 12"></path>
                    </svg>
                </Toggle>
                <Label for_id="disabled-toggle">
                    { "Disabled toggle" }
                </Label>
                <Button
                    onclick={on_disable_toggle}
                    variant={ButtonVariant::Default}
                    size={ButtonSize::Small}
                >
                    { if *disabled { "Enable" } else { "Disable" } }
                </Button>
            </div>
        </div>
    }
}

#[function_component(ToggleDocumentation)]
pub fn toggle_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Toggle Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Toggle component is a versatile switch component that allows users to toggle between two states. It supports customization through various props and can include icons or text within the toggle button." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! { <ToggleDemo /> }}
                customize={html! { <ToggleThemeEditor /> }}
                code={r#"
let checked = use_state(|| false);
let disabled = use_state(|| false);

let on_toggle = {
    let checked = checked.clone();
    Callback::from(move |_| {
        checked.set(!*checked);
    })
};

let on_disable_toggle = {
    let disabled = disabled.clone();
    Callback::from(move |_| {
        disabled.set(!*disabled);
    })
};

html! {
    <div class="space-y-4">
        <div class="flex items-center space-x-2">
            <Toggle id="bold-toggle" checked={*checked} on_toggle={on_toggle.clone()}>
                <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
                    <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
                </svg>
            </Toggle>
            <label for="bold-toggle" class={BRANDGUIDE.toggle_label}>
                { "Toggle bold" }
            </label>
        </div>
        <div class="flex items-center space-x-2">
            <Toggle id="italic-toggle" checked={false} on_toggle={Callback::from(|_| {})}>
                <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <line x1="19" y1="4" x2="10" y2="4"></line>
                    <line x1="14" y1="20" x2="5" y2="20"></line>
                    <line x1="15" y1="4" x2="9" y2="20"></line>
                </svg>
            </Toggle>
            <label for="italic-toggle" class={BRANDGUIDE.toggle_label}>
                { "Toggle italic" }
            </label>
        </div>
        <div class="flex items-center space-x-2">
            <Toggle id="disabled-toggle" checked={*checked} on_toggle={on_toggle} disabled={*disabled}>
                <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M18 6L6 18"></path>
                    <path d="M6 6l12 12"></path>
                </svg>
            </Toggle>
            <label for="disabled-toggle" class={BRANDGUIDE.toggle_label}>
                { "Disabled toggle" }
            </label>
            <button onclick={on_disable_toggle} class="ml-2 px-2 py-1 bg-gray-200 rounded">
                { if *disabled { "Enable" } else { "Disable" } }
            </button>
        </div>
    </div>
}"#.to_string()}
            />

            <Features features={vec!["Toggle"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Toggle"
                description="The main container for the toggle component."
                props={vec![
                    ("id", "String", "The id of the toggle component."),
                    ("checked", "bool", "The initial checked state of the toggle."),
                    ("on_toggle", "Callback<MouseEvent>", "The callback to be called when the toggle is clicked."),
                    ("disabled", "bool", "Whether the toggle is disabled."),
                    ("children", "Children", "The child elements to be rendered inside the toggle component."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Toggle component can be used with or without an icon.".to_string(),
                    "It's recommended to use a label with the Toggle for better accessibility.".to_string(),
                    "The component supports both controlled and uncontrolled usage.".to_string(),
                    "When disabled, the Toggle will not respond to user interactions.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Toggle".to_string()}
                class_descriptions={vec![
                    ("toggle_container".to_string(), "For the main container of the toggle component".to_string()),
                    ("toggle_base".to_string(), "For the base styling of the toggle".to_string()),
                    ("toggle_checked".to_string(), "For the toggle when it's in the checked state".to_string()),
                    ("toggle_unchecked".to_string(), "For the toggle when it's in the unchecked state".to_string()),
                    ("toggle_disabled".to_string(), "For the toggle when it's disabled".to_string()),
                    ("toggle_label".to_string(), "For the label associated with the toggle".to_string()),
                    ("toggle_icon".to_string(), "For the icon inside the toggle".to_string()),
                ]}
            />
        </Container>
    }
}
