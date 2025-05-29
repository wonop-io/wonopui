use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use gloo_console as console;
use wonopui::prelude::*;
use yew::prelude::*;

#[function_component(ButtonThemeEditor)]
pub fn button_theme_editor() -> Html {
    let fields = vec![
        ("button_base".to_string(), "Base Button".to_string()),
        ("button_primary".to_string(), "Primary Button".to_string()),
        (
            "button_secondary".to_string(),
            "Secondary Button".to_string(),
        ),
        ("button_success".to_string(), "Success Button".to_string()),
        ("button_warning".to_string(), "Warning Button".to_string()),
        ("button_danger".to_string(), "Danger Button".to_string()),
        ("button_ghost".to_string(), "Ghost Button".to_string()),
        ("button_default".to_string(), "Default Button".to_string()),
        ("button_small".to_string(), "Small Button".to_string()),
        ("button_medium".to_string(), "Medium Button".to_string()),
        ("button_large".to_string(), "Large Button".to_string()),
    ];

    let preview = html! {
        <div class="flex flex-col space-y-2">
            <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| console::log!("Primary clicked!"))}>
                {"Primary"}
            </Button>
            <Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| console::log!("Secondary clicked!"))}>
                {"Secondary"}
            </Button>
            <Button variant={ButtonVariant::Success} onclick={Callback::from(|_| console::log!("Success clicked!"))}>
                {"Success"}
            </Button>
            <Button variant={ButtonVariant::Warning} onclick={Callback::from(|_| console::log!("Warning clicked!"))}>
                {"Warning"}
            </Button>
            <Button variant={ButtonVariant::Danger} onclick={Callback::from(|_| console::log!("Danger clicked!"))}>
                {"Danger"}
            </Button>
            <Button variant={ButtonVariant::Ghost} onclick={Callback::from(|_| console::log!("Ghost clicked!"))}>
                {"Ghost"}
            </Button>
            <Button variant={ButtonVariant::Default} onclick={Callback::from(|_| console::log!("Default clicked!"))}>
                {"Default"}
            </Button>
            <div class="flex space-x-2">
                <Button variant={ButtonVariant::Primary} size={ButtonSize::Small} onclick={Callback::from(|_| console::log!("Small clicked!"))}>
                    {"Small"}
                </Button>
                <Button variant={ButtonVariant::Primary} size={ButtonSize::Medium} onclick={Callback::from(|_| console::log!("Medium clicked!"))}>
                    {"Medium"}
                </Button>
                <Button variant={ButtonVariant::Primary} size={ButtonSize::Large} onclick={Callback::from(|_| console::log!("Large clicked!"))}>
                    {"Large"}
                </Button>
            </div>
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ButtonDocumentation)]
pub fn button_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Button Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Button component is a versatile and customizable button element that can be used in various parts of your application. It supports different variants, sizes, and can handle click events." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex flex-col space-y-2">
                        <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| console::log!("Primary clicked!"))}>
                            {"Primary"}
                        </Button>
                        <Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| console::log!("Secondary clicked!"))}>
                            {"Secondary"}
                        </Button>
                        <Button variant={ButtonVariant::Success} onclick={Callback::from(|_| console::log!("Success clicked!"))}>
                            {"Success"}
                        </Button>
                        <Button variant={ButtonVariant::Warning} onclick={Callback::from(|_| console::log!("Warning clicked!"))}>
                            {"Warning"}
                        </Button>
                        <Button variant={ButtonVariant::Danger} onclick={Callback::from(|_| console::log!("Danger clicked!"))}>
                            {"Danger"}
                        </Button>
                        <Button variant={ButtonVariant::Ghost} onclick={Callback::from(|_| console::log!("Ghost clicked!"))}>
                            {"Ghost"}
                        </Button>
                        <Button variant={ButtonVariant::Default} onclick={Callback::from(|_| console::log!("Default clicked!"))}>
                            {"Default"}
                        </Button>
                        <div class="flex space-x-2">
                            <Button variant={ButtonVariant::Primary} size={ButtonSize::Small} onclick={Callback::from(|_| console::log!("Small clicked!"))}>
                                {"Small"}
                            </Button>
                            <Button variant={ButtonVariant::Primary} size={ButtonSize::Medium} onclick={Callback::from(|_| console::log!("Medium clicked!"))}>
                                {"Medium"}
                            </Button>
                            <Button variant={ButtonVariant::Primary} size={ButtonSize::Large} onclick={Callback::from(|_| console::log!("Large clicked!"))}>
                                {"Large"}
                            </Button>
                        </div>
                    </div>
                }}
                customize={html! {
                    <ButtonThemeEditor />
                }}
                code={r#"
<Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| console::log!("Primary clicked!"))}>
    {"Primary"}
</Button>
<Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| console::log!("Secondary clicked!"))}>
    {"Secondary"}
</Button>
<Button variant={ButtonVariant::Success} onclick={Callback::from(|_| console::log!("Success clicked!"))}>
    {"Success"}
</Button>
<Button variant={ButtonVariant::Warning} onclick={Callback::from(|_| console::log!("Warning clicked!"))}>
    {"Warning"}
</Button>
<Button variant={ButtonVariant::Danger} onclick={Callback::from(|_| console::log!("Danger clicked!"))}>
    {"Danger"}
</Button>
<Button variant={ButtonVariant::Ghost} onclick={Callback::from(|_| console::log!("Ghost clicked!"))}>
    {"Ghost"}
</Button>
<Button variant={ButtonVariant::Default} onclick={Callback::from(|_| console::log!("Default clicked!"))}>
    {"Default"}
</Button>
<Button variant={ButtonVariant::Primary} size={ButtonSize::Small} onclick={Callback::from(|_| console::log!("Small clicked!"))}>
    {"Small"}
</Button>
<Button variant={ButtonVariant::Primary} size={ButtonSize::Medium} onclick={Callback::from(|_| console::log!("Medium clicked!"))}>
    {"Medium"}
</Button>
<Button variant={ButtonVariant::Primary} size={ButtonSize::Large} onclick={Callback::from(|_| console::log!("Large clicked!"))}>
    {"Large"}
</Button>"#.to_string()}
            />

            <Features features={vec![
                "Multiple button variants (Primary, Secondary, Success, Warning, Danger, Ghost, Default)",
                "Three button sizes (Small, Medium, Large)",
                "Customizable styling through theme editor",
                "Support for click event handling",
                "Disabled state support",
                "Custom CSS class support",
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Button"
                description="Props for the Button component."
                props={vec![
                    ("onclick", "Callback<MouseEvent>", "The callback to be executed when the button is clicked."),
                    ("variant", "ButtonVariant", "The variant of the button, which can be Primary, Secondary, Success, Warning, Danger, Ghost, or Default."),
                    ("size", "ButtonSize", "The size of the button, which can be Small, Medium, or Large."),
                    ("class", "Classes", "Additional CSS classes to be applied to the button."),
                    ("children", "Children", "The child elements to be rendered inside the button."),
                    ("disabled", "bool", "Whether the button is disabled or not."),
                    ("kind", "Option<String>", "The type of the button, defaults to 'button' if not specified."),
                ]}
            />

            <ApiSection
                title="ButtonVariant"
                description="An enum representing the different variants of buttons."
                props={vec![
                    ("Primary", "", "A primary button, usually used for main actions."),
                    ("Secondary", "", "A secondary button, usually used for less important actions."),
                    ("Success", "", "A success button, usually used for positive actions."),
                    ("Warning", "", "A warning button, usually used for actions that require caution."),
                    ("Danger", "", "A danger button, usually used for destructive actions."),
                    ("Ghost", "", "A ghost button, usually used for subtle actions."),
                    ("Default", "", "A default button, used for general purposes."),
                ]}
            />

            <ApiSection
                title="ButtonSize"
                description="An enum representing the different sizes of buttons."
                props={vec![
                    ("Small", "", "A small-sized button."),
                    ("Medium", "", "A medium-sized button (default)."),
                    ("Large", "", "A large-sized button."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Button component is highly customizable through its props.".to_string(),
                    "Use different variants to convey the importance or nature of the action.".to_string(),
                    "The 'disabled' prop can be used to prevent user interaction when needed.".to_string(),
                    "The 'kind' prop can be used to specify the HTML button type (e.g., 'submit', 'reset').".to_string(),
                    "Custom CSS classes can be added using the 'class' prop for further styling.".to_string(),
                    "The Button component is designed to be accessible, with proper focus management and keyboard interaction.".to_string(),
                    "When using icons within buttons, ensure they have proper aria-labels for screen readers.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Button".to_string()}
                class_descriptions={vec![
                    ("button_base".to_string(), "Base styles applied to all buttons".to_string()),
                    ("button_primary".to_string(), "Styles for primary buttons".to_string()),
                    ("button_secondary".to_string(), "Styles for secondary buttons".to_string()),
                    ("button_success".to_string(), "Styles for success buttons".to_string()),
                    ("button_warning".to_string(), "Styles for warning buttons".to_string()),
                    ("button_danger".to_string(), "Styles for danger buttons".to_string()),
                    ("button_ghost".to_string(), "Styles for ghost buttons".to_string()),
                    ("button_default".to_string(), "Styles for default buttons".to_string()),
                    ("button_small".to_string(), "Styles for small-sized buttons".to_string()),
                    ("button_medium".to_string(), "Styles for medium-sized buttons".to_string()),
                    ("button_large".to_string(), "Styles for large-sized buttons".to_string()),
                ]}
            />
        </Container>
    }
}
