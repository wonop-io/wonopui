use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(InputThemeEditor)]
pub fn input_theme_editor() -> Html {
    let fields = vec![("input_base".to_string(), "Base Input Style".to_string())];

    let preview = html! {
        <Input
            value="Hello, World!"
            placeholder="Enter text..."
            class="w-full"
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(InputDocumentation)]
pub fn input_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Input Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Input component is a versatile text input field, supporting multiple event handlers and customizable properties. It provides a controlled input experience with extensive callback support for various events." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Examples" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="space-y-4">
                        <Input
                            value="Hello, World!"
                            placeholder="Basic input"
                            class="w-full"
                        />
                        <Input
                            value=""
                            placeholder="Password input"
                            kind="password"
                            class="w-full"
                        />
                        <Input
                            value="5"
                            kind="number"
                            min="0"
                            max="10"
                            step="1"
                            class="w-full"
                        />
                    </div>
                }}
                customize={html! {
                    <InputThemeEditor />
                }}
                code={r#"
<Input
    value="Hello, World!"
    placeholder="Basic input"
    class="w-full"
/>

<Input
    value=""
    placeholder="Password input"
    kind="password"
    class="w-full"
/>

<Input
    value="5"
    kind="number"
    min="0"
    max="10"
    step="1"
    class="w-full"
/>"#.to_string()}
            />
            <Features features={vec!["Input"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="InputProps"
                description="Props for the Input component."
                props={vec![
                    ("value", "String", "The current value of the input."),
                    ("oninput", "Callback<InputEvent>", "Callback to handle the oninput event."),
                    ("ontext", "Callback<String>", "Callback to handle text input changes."),
                    ("onchange", "Callback<Event>", "Callback to handle the onchange event."),
                    ("onkeypress", "Callback<KeyboardEvent>", "Callback to handle keypress events."),
                    ("onkeydown", "Callback<KeyboardEvent>", "Callback to handle keydown events."),
                    ("onkeyup", "Callback<KeyboardEvent>", "Callback to handle keyup events."),
                    ("placeholder", "String", "Placeholder text for the input."),
                    ("class", "Classes", "CSS classes to apply to the input."),
                    ("id", "String", "HTML id attribute for the input."),
                    ("name", "String", "HTML name attribute for the input."),
                    ("kind", "String", "Type of the input (e.g., text, password, number, etc.). Default is 'text'."),
                    ("maxlength", "Option<i32>", "Maximum length of the input value."),
                    ("readonly", "bool", "Whether the input is read-only."),
                    ("min", "Option<String>", "Minimum value for the input (for number inputs)."),
                    ("max", "Option<String>", "Maximum value for the input (for number inputs)."),
                    ("step", "Option<String>", "Step value for the input (for number inputs)."),
                    ("node_ref", "NodeRef", "Reference to the input node."),
                ]}
            />

            <NotesSection
                title="Usage Notes"
                notes={vec![
                    "The Input component supports various event handlers such as oninput, ontext, onchange, onkeypress, onkeydown, and onkeyup.".to_string(),
                    "Most props have default values, allowing you to use the component with minimal configuration.".to_string(),
                    "The ontext callback is triggered whenever the text in the input field changes, providing a convenient way to handle input changes.".to_string(),
                    "For number inputs, use the min, max, and step props to control the range and increment of values.".to_string(),
                    "The readonly prop can be used to create non-editable inputs, useful for displaying static information.".to_string(),
                    "Use the node_ref prop if you need to access the underlying DOM element for advanced manipulations.".to_string(),
                ]}
            />

            <StylingSection
                component_name="Input"
                class_descriptions={vec![
                    ("input_base".to_string(), "Base styling for the input. Default: 'border rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-600'".to_string()),
                ]}
            />
        </Container>
    }
}
