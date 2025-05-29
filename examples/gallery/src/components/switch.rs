use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(SwitchThemeEditor)]
pub fn switch_theme_editor() -> Html {
    let fields = vec![
        ("switch_base".to_string(), "Base".to_string()),
        ("switch_thumb".to_string(), "Thumb".to_string()),
        ("switch_checked".to_string(), "Checked".to_string()),
        ("switch_unchecked".to_string(), "Unchecked".to_string()),
        (
            "switch_translate_checked".to_string(),
            "Translate Checked".to_string(),
        ),
        (
            "switch_translate_unchecked".to_string(),
            "Translate Unchecked".to_string(),
        ),
        ("switch_disabled".to_string(), "Disabled".to_string()),
        ("switch_label".to_string(), "Label".to_string()),
    ];

    let preview = html! {
        <SwitchDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(SwitchDemo)]
fn switch_demo() -> Html {
    let checked = use_state(|| false);
    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    html! {
        <div class="flex flex-col space-y-4">
            <SwitchButton
                id="switch-1"
                checked={*checked}
                on_toggle={on_toggle.clone()}
            />
            <SwitchButton
                id="switch-2"
                checked={true}
                on_toggle={Callback::from(|_| {})}
                on_icon={Some(html! { <i class="fas fa-check"></i> })}
                off_icon={Some(html! { <i class="fas fa-times"></i> })}
            />
            <SwitchButton
                id="switch-3"
                checked={false}
                on_toggle={Callback::from(|_| {})}
                disabled={true}
            />
        </div>
    }
}

#[function_component(SwitchDocumentation)]
pub fn switch_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "SwitchButton Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The SwitchButton component is a customizable switch that can be toggled on or off. It includes support for custom icons and is designed with accessibility in mind." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <SwitchDemo />
                }}
                customize={html! {
                    <SwitchThemeEditor />
                }}
                code={r#"
let checked = use_state(|| false);
let on_toggle = {
    let checked = checked.clone();
    Callback::from(move |_| {
        checked.set(!*checked);
    })
};

html! {
    <div class="flex flex-col space-y-4">
        <SwitchButton
            id="switch-1"
            checked={*checked}
            on_toggle={on_toggle.clone()}
        />
        <SwitchButton
            id="switch-2"
            checked={true}
            on_toggle={Callback::from(|_| {})}
            on_icon={Some(html! { <i class="fas fa-check"></i> })}
            off_icon={Some(html! { <i class="fas fa-times"></i> })}
        />
        <SwitchButton
            id="switch-3"
            checked={false}
            on_toggle={Callback::from(|_| {})}
            disabled={true}
        />
    </div>
}"#.to_string()}
            />
            <Features features={vec!["SwitchButton"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="SwitchButton"
                description="Props for the SwitchButton component."
                props={vec![
                    ("id", "String", "A unique identifier for the switch button."),
                    ("checked", "bool", "The initial checked state of the switch button."),
                    ("on_toggle", "Callback<MouseEvent>", "A callback function that is called when the switch is toggled."),
                    ("disabled", "bool", "Disables the switch button if set to true."),
                    ("on_icon", "Option<Html>", "An optional icon to display when the switch is in the 'on' state."),
                    ("off_icon", "Option<Html>", "An optional icon to display when the switch is in the 'off' state."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The SwitchButton component uses internal state to manage its toggle state.".to_string(),
                    "Custom icons can be displayed based on the toggle state using the on_icon and off_icon props.".to_string(),
                    "The component is accessible, with proper ARIA attributes for screen readers.".to_string(),
                    "The switch can be disabled, which applies appropriate styles and prevents user interaction.".to_string(),
                    "Transitions are applied for smooth visual feedback when toggling the switch.".to_string(),
                    "The component is responsive and works well on different screen sizes.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"SwitchButton".to_string()}
                class_descriptions={vec![
                    ("switch_base".to_string(), "Base styling for the switch button container.".to_string()),
                    ("switch_thumb".to_string(), "Styling for the switch button thumb (the moving part).".to_string()),
                    ("switch_checked".to_string(), "Styling applied when the switch is in the checked state.".to_string()),
                    ("switch_unchecked".to_string(), "Styling applied when the switch is in the unchecked state.".to_string()),
                    ("switch_translate_checked".to_string(), "Controls the position of the thumb when checked.".to_string()),
                    ("switch_translate_unchecked".to_string(), "Controls the position of the thumb when unchecked.".to_string()),
                    ("switch_disabled".to_string(), "Styling applied when the switch is disabled.".to_string()),
                    ("switch_label".to_string(), "Styling for any associated label (visually hidden by default).".to_string()),
                ]}
            />

        </Container>
    }
}
