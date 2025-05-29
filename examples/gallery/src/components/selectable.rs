use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use std::rc::Rc;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(SelectableThemeEditor)]
pub fn selectable_theme_editor() -> Html {
    let fields = vec![
        ("selectable_indicator".to_string(), "Indicator".to_string()),
        ("selectable_hover".to_string(), "Hover".to_string()),
        ("selectable_selected".to_string(), "Selected".to_string()),
        ("selectable_cursor".to_string(), "Cursor".to_string()),
    ];

    let preview = html! {
        <SelectableArea select_mode={true}>
            <Selectable id="item1">
                { "Selectable Item 1" }
            </Selectable>
            <Selectable id="item2">
                { "Selectable Item 2" }
            </Selectable>
            <SelectableIndicator />
        </SelectableArea>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(SelectableDocumentation)]
pub fn selectable_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Selectable Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Selectable component provides functionality for selecting and highlighting elements within a selectable area. It uses internal state management to maintain the selection and hover states." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <SelectableArea select_mode={true}>
                        <Selectable id="item1">
                            { "Selectable Item 1" }
                        </Selectable>
                        <Selectable id="item2">
                            { "Selectable Item 2" }
                        </Selectable>
                        <SelectableIndicator />
                    </SelectableArea>
                }}
                customize={html! {
                    <SelectableThemeEditor />
                }}
                code={r#"
<SelectableArea select_mode={true}>
    <Selectable id="item1">
        { "Selectable Item 1" }
    </Selectable>
    <Selectable id="item2">
        { "Selectable Item 2" }
    </Selectable>
    <SelectableIndicator />
</SelectableArea>"#.to_string()}
            />
            <Features features={vec!["Selectable", "SelectableArea", "SelectableIndicator", "SelectableVTag"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>

            <ApiSection
                title="SelectableProps"
                description="Props for the Selectable component."
                props={vec![
                    ("id", "String", "The unique identifier for the selectable item."),
                    ("children", "VNode", "The child elements to be rendered inside the selectable component."),
                    ("class", "Classes", "CSS classes to apply to the component."),
                    ("tag", "String", "The HTML tag to be used for this component."),
                    ("select_mode", "bool", "Indicates whether the selectable component is in selection mode."),
                    ("style", "String", "Inline styles to apply to the component."),
                ]}
            />

            <ApiSection
                title="SelectableAreaProps"
                description="Props for the SelectableArea component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the selectable area."),
                    ("onselect", "Callback<Option<String>>", "Callback function to handle selection changes."),
                    ("select_mode", "bool", "Indicates whether the selectable area is in selection mode."),
                ]}
            />

            <ApiSection
                title="SelectableIndicatorProps"
                description="Props for the SelectableIndicator component."
                props={vec![
                    ("class", "Classes", "CSS classes to apply to the indicator."),
                ]}
            />

            <ApiSection
                title="SelectableVTagProps"
                description="Props for the SelectableVTag component."
                props={vec![
                    ("node", "Box<VTag>", "The VTag node to be wrapped by the selectable functionality."),
                    ("node_ref", "NodeRef", "A reference to the DOM node."),
                    ("onclick", "Callback<MouseEvent>", "Callback function for handling click events."),
                    ("id", "String", "The unique identifier for the VTag."),
                    ("onmouseenter", "Callback<MouseEvent>", "Callback function for handling mouse enter events."),
                    ("onmouseleave", "Callback<MouseEvent>", "Callback function for handling mouse leave events."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Selectable component uses context to manage its state.".to_string(),
                    "SelectableIndicator highlights the selected area visually.".to_string(),
                    "The components need to be nested within a SelectableArea to function correctly.".to_string(),
                    "The component uses MutationObserver to track changes in the DOM and update the selected area accordingly.".to_string(),
                    "Selection mode can be toggled using the select_mode prop on the SelectableArea component.".to_string(),
                    "The component supports both mouse and touch interactions for selection.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Selectable".to_string()}
                class_descriptions={vec![
                    ("selectable_indicator".to_string(), "For the visual indicator of the selected area.".to_string()),
                    ("selectable_hover".to_string(), "For the hover effect on selectable items.".to_string()),
                    ("selectable_selected".to_string(), "For the selected state of items.".to_string()),
                    ("selectable_cursor".to_string(), "For the cursor style on selectable items.".to_string()),
                ]}
            />

        </Container>
    }
}
