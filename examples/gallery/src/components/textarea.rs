use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(TextareaThemeEditor)]
pub fn textarea_theme_editor() -> Html {
    let fields = vec![("textarea_base".to_string(), "Textarea Base".to_string())];

    let preview = html! {
        <Textarea
            value="Sample text"
            placeholder="Enter text here..."
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(TextareaDemo)]
pub fn textarea_demo() -> Html {
    let value = use_state(|| "".to_string());
    let oninput = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            value.set(target.value());
        })
    };

    html! {
        <div>
            <Textarea
                value={(*value).clone()}
                oninput={oninput.clone()}
                placeholder="Enter some text..."
            />
            <p>{"Current value: "}{&*value}</p>
        </div>
    }
}

#[function_component(TextareaDocumentation)]
pub fn textarea_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Textarea Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Textarea component is a versatile input field for multi-line text. It supports various properties to customize its appearance and behavior." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <>
                        <Textarea
                            value="Initial text"
                            placeholder="Enter some text..."
                            class={classes!("mb-4")}
                        />
                        <Textarea
                            value=""
                            placeholder="Disabled textarea"
                            class={classes!("mb-4")}
                            disabled=true
                        />
                        <TextareaDemo />
                    </>
                }}
                customize={html! {
                    <TextareaThemeEditor />
                }}
                code={r#"
// Static Textarea
<Textarea
    value="Initial text"
    placeholder="Enter some text..."
    class={classes!("mb-4")}
/>

// Disabled Textarea
<Textarea
    value=""
    placeholder="Disabled textarea"
    class={classes!("mb-4")}
    disabled=true
/>

// Dynamic Textarea
let value = use_state(|| "".to_string());
let oninput = {
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
        let target = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
        value.set(target.value());
    })
};

html! {
    <div>
        <Textarea
            value={(*value).clone()}
            oninput={oninput}
            placeholder="Enter some text..."
        />
        <p>{"Current value: "}{&*value}</p>
    </div>
}
"#.to_string()}
            />

            <Features features={vec!["Textarea"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="TextareaProps"
                description="Props for the Textarea component."
                props={vec![
                    ("value", "String", "The current value of the textarea."),
                    ("oninput", "Callback<InputEvent>", "A callback function that is called when the input value changes."),
                    ("placeholder", "String", "Placeholder text to display when the textarea is empty."),
                    ("class", "Classes", "CSS classes to be applied to the textarea."),
                    ("id", "String", "The id attribute of the textarea."),
                    ("name", "String", "The name attribute of the textarea."),
                    ("disabled", "bool", "If true, the textarea will be disabled."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Ensure to manage the state of the `value` prop correctly to make the component controlled.".to_string(),
                    "Use the `oninput` callback to handle changes to the textarea's content.".to_string(),
                    "The `class` prop allows for additional customization via CSS classes.".to_string(),
                    "For accessibility, consider using the `id` prop in conjunction with a `<label>` element.".to_string(),
                    "The `disabled` prop can be used to prevent user interaction with the textarea.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Textarea".to_string()}
                class_descriptions={vec![
                    ("textarea_base".to_string(), "The base styling for the textarea, including border, padding, and focus effects.".to_string()),
                ]}
            />
        </Container>
    }
}
