use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(CollapsibleDemo)]
pub fn collapsible_demo() -> Html {
    let is_open = use_state(|| false);

    let on_open_change = {
        let is_open = is_open.clone();
        Callback::from(move |new_state: bool| {
            is_open.set(new_state);
        })
    };

    let toggle_open = {
        let is_open = is_open.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |_| {
            let new_state = !*is_open;
            is_open.set(new_state);
            on_open_change.emit(new_state);
        })
    };

    html! {
        <Collapsible open={*is_open} on_open_change={on_open_change}>
            <CollapsibleHeader>
                <CollapsibleTitle>{"@peduarte starred 3 repositories"}</CollapsibleTitle>
                <CollapsibleTrigger as_child=true onclick={toggle_open}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevrons-up-down h-4 w-4">
                        <path d="m7 15 5 5 5-5"></path>
                        <path d="m7 9 5-5 5 5"></path>
                    </svg>
                    <span class="sr-only">{"Toggle"}</span>
                </CollapsibleTrigger>
            </CollapsibleHeader>
            <CollapsibleContent is_open={*is_open}>
                <CollapsibleItem>
                    {"@radix-ui/primitives"}
                </CollapsibleItem>
                <CollapsibleItem>{"@radix-ui/colors"}</CollapsibleItem>
                <CollapsibleItem>{"@stitches/react"}</CollapsibleItem>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[function_component(CollapsibleThemeEditor)]
pub fn collapsible_theme_editor() -> Html {
    let fields = vec![
        (
            "collapsible_container".to_string(),
            "Collapsible Container".to_string(),
        ),
        (
            "collapsible_button".to_string(),
            "Collapsible Button".to_string(),
        ),
        (
            "collapsible_content".to_string(),
            "Collapsible Content".to_string(),
        ),
        (
            "collapsible_header".to_string(),
            "Collapsible Header".to_string(),
        ),
        (
            "collapsible_title".to_string(),
            "Collapsible Title".to_string(),
        ),
        (
            "collapsible_item".to_string(),
            "Collapsible Item".to_string(),
        ),
    ];

    let preview = html! {
        <CollapsibleDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CollapsibleDocumentation)]
pub fn collapsible_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Collapsible Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Collapsible component provides an expandable and collapsible container for content. It's useful for creating accordion-like structures or hiding content that doesn't need to be visible all the time." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! { <CollapsibleDemo /> }}
                customize={html! { <CollapsibleThemeEditor /> }}
                code={r#"
<Collapsible open={is_open} on_open_change={on_open_change}>
    <div class="collapsible_header">
        <h4 class="collapsible_title">{"@peduarte starred 3 repositories"}</h4>
        <CollapsibleTrigger as_child=true onclick={toggle_open}>
            <svg>/*<!-- ... -->*/</svg>
            <span class="sr-only">{"Toggle"}</span>
        </CollapsibleTrigger>
    </div>
    <div class="collapsible_item">{"@radix-ui/primitives"}</div>
    <CollapsibleContent is_open={is_open}>
        <div class="collapsible_item">{"@radix-ui/colors"}</div>
        <div class="collapsible_item">{"@stitches/react"}</div>
    </CollapsibleContent>
</Collapsible>
"#.to_string()}
            />

            <Features features={vec!["Collapsible", "CollapsibleTrigger", "CollapsibleContent"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Collapsible"
                description="Props for the Collapsible component."
                props={vec![
                    ("open", "bool", "Whether the collapsible is open or closed."),
                    ("on_open_change", "Callback<bool>", "Callback function called when the open state changes."),
                    ("class", "String", "Additional CSS classes to apply to the collapsible container."),
                    ("children", "Children", "Child components to be rendered inside the collapsible."),
                ]}
            />

            <ApiSection
                title="CollapsibleTrigger"
                description="Props for the CollapsibleTrigger component."
                props={vec![
                    ("as_child", "bool", "Whether to render the trigger as a child component."),
                    ("children", "Children", "Child components to be rendered inside the trigger."),
                    ("onclick", "Callback<MouseEvent>", "Callback function called when the trigger is clicked."),
                ]}
            />

            <ApiSection
                title="CollapsibleContent"
                description="Props for the CollapsibleContent component."
                props={vec![
                    ("class", "String", "Additional CSS classes to apply to the content container."),
                    ("children", "Children", "Child components to be rendered inside the content area."),
                    ("is_open", "bool", "Whether the content should be visible."),
                ]}
            />

            <NotesSection
                title="Usage Notes"
                notes={vec![
                    "The Collapsible component manages its own open/closed state internally.".to_string(),
                    "Use CollapsibleTrigger to create a clickable element that toggles the collapsible state.".to_string(),
                    "Wrap content that should be hidden/shown in the CollapsibleContent component.".to_string(),
                    "The component supports keyboard navigation and screen reader announcements for accessibility.".to_string(),
                ]}
            />

            <StylingSection
                component_name="Collapsible"
                class_descriptions={vec![
                    ("collapsible_container".to_string(), "Styles for the main container of the collapsible component".to_string()),
                    ("collapsible_button".to_string(), "Styles for the trigger button that toggles the collapsible state".to_string()),
                    ("collapsible_content".to_string(), "Styles for the content container that appears when the collapsible is open".to_string()),
                    ("collapsible_header".to_string(), "Styles for the header section of the collapsible".to_string()),
                    ("collapsible_title".to_string(), "Styles for the title text within the collapsible header".to_string()),
                    ("collapsible_item".to_string(), "Styles for individual items within the collapsible content".to_string()),
                ]}
            />
        </Container>
    }
}
