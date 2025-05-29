use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(LabelThemeEditor)]
pub fn label_theme_editor() -> Html {
    let fields = vec![("label_base".to_string(), "Base Label Style".to_string())];

    let preview = html! {
        <div class="space-y-2">
            <Label for_id="example-input">{ "Default Label" }</Label>
            <Label for_id="example-input" class="text-blue-600">{ "Custom Colored Label" }</Label>
        </div>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(LabelDocumentation)]
pub fn label_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Label Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Label component is used to provide labels for form elements such as inputs and selects. It supports custom classes and an optional 'for' attribute to link it to a form element by its id." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Examples" }
            </h2>
            <ExampleCode
                preview={html! {
                    <div class="space-y-4">
                        <div>
                            <Label for_id="example-input">{ "Default Label" }</Label>
                            <Input id="example-input" placeholder="Enter text..." />
                        </div>
                        <div>
                            <Label for_id="custom-input" class="text-blue-600 font-bold">{ "Custom Styled Label" }</Label>
                            <Input id="custom-input" placeholder="Enter text..." />
                        </div>
                        <div>
                            <Label for_id="required-input" class="flex items-center">
                                { "Required Field " }
                                <span class="text-red-500 ml-1">{ "*" }</span>
                            </Label>
                            <Input id="required-input" placeholder="Enter required text..." />
                        </div>
                    </div>
                }}
                customize={html! {
                    <LabelThemeEditor />
                }}
                code={r#"
<Label for_id="example-input">{ "Default Label" }</Label>
<Input id="example-input" placeholder="Enter text..." />

<Label for_id="custom-input" class="text-blue-600 font-bold">{ "Custom Styled Label" }</Label>
<Input id="custom-input" placeholder="Enter text..." />

<Label for_id="required-input" class="flex items-center">
    { "Required Field " }
    <span class="text-red-500 ml-1">{ "*" }</span>
</Label>
<Input id="required-input" placeholder="Enter required text..." />"#.to_string()}
            />

            <Features features={vec!["Label"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="LabelProps"
                description="Props for the Label component."
                props={vec![
                    ("children", "Children", "The content to be displayed inside the label."),
                    ("class", "Classes", "Optional additional CSS classes to style the label."),
                    ("for_id", "String", "An optional id of a form element to link the label to."),
                ]}
            />

            <NotesSection
                title="Usage Notes"
                notes={vec![
                    "The default styling for the label is defined in the BRANDGUIDE.label_base class.".to_string(),
                    "You can customize the label using the 'class' prop to add your own classes.".to_string(),
                    "If 'for_id' is provided, it should match the id of the form element the label is associated with.".to_string(),
                    "Labels improve accessibility by providing context for screen readers and allowing users to click on the label to focus the associated input.".to_string(),
                    "For required fields, you can add visual indicators (like an asterisk) within the label component.".to_string(),
                ]}
            />

            <StylingSection
                component_name="Label"
                class_descriptions={vec![
                    ("label_base".to_string(), "Base class for the label styling. Default: 'px-4 py-2'".to_string()),
                ]}
            />
        </Container>
    }
}
