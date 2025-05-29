use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(PopoverThemeEditor)]
pub fn popover_theme_editor() -> Html {
    let fields = vec![
        ("popover_container".to_string(), "Container".to_string()),
        ("popover_trigger".to_string(), "Trigger".to_string()),
        ("popover_content".to_string(), "Content".to_string()),
        (
            "popover_position_south_middle".to_string(),
            "Position: South Middle".to_string(),
        ),
    ];

    let preview = html! {
        <PopoverDemo />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(PopoverDemo)]
fn popover_demo() -> Html {
    html! {
        <div class="space-y-4">
            <Popover>
                <PopoverTrigger>
                    <Button>{"Default Popover"}</Button>
                </PopoverTrigger>
                <PopoverContent>
                    {"Default Popover Content"}
                </PopoverContent>
            </Popover>

            <Popover>
                <PopoverTrigger>
                    <Button variant={ButtonVariant::Secondary}>{"Custom Position"}</Button>
                </PopoverTrigger>
                <PopoverContent position={PopoverPosition::NorthStart}>
                    {"Popover Content (North Start)"}
                </PopoverContent>
            </Popover>

            <Popover>
                <PopoverTrigger>
                    <Button variant={ButtonVariant::Default}>{"With Close Button"}</Button>
                </PopoverTrigger>
                <PopoverContent>
                    <div class="flex justify-between items-center">
                        <span>{"Closable Popover"}</span>
                        <PopoverTrigger as_child=true>
                            <Button variant={ButtonVariant::Ghost} class="h-4 w-4 p-0">{"×"}</Button>
                        </PopoverTrigger>
                    </div>
                </PopoverContent>
            </Popover>
        </div>
    }
}

#[function_component(PopoverDocumentation)]
pub fn popover_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Popover Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Popover component is a versatile UI element that displays content in a floating container. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <PopoverDemo />
                }}
                customize={html! {
                    <PopoverThemeEditor />
                }}
                code={r#"
<Popover>
    <PopoverTrigger>
        <Button>{"Default Popover"}</Button>
    </PopoverTrigger>
    <PopoverContent>
        {"Default Popover Content"}
    </PopoverContent>
</Popover>

<Popover>
    <PopoverTrigger>
        <Button variant={ButtonVariant::Secondary}>{"Custom Position"}</Button>
    </PopoverTrigger>
    <PopoverContent position={PopoverPosition::NorthStart}>
        {"Popover Content (North Start)"}
    </PopoverContent>
</Popover>

<Popover>
    <PopoverTrigger>
        <Button variant={ButtonVariant::Outline}>{"With Close Button"}</Button>
    </PopoverTrigger>
    <PopoverContent>
        <div class="flex justify-between items-center">
            <span>{"Closable Popover"}</span>
            <PopoverTrigger as_child=true>
                <Button variant={ButtonVariant::Ghost} class="h-4 w-4 p-0">{"×"}</Button>
            </PopoverTrigger>
        </div>
    </PopoverContent>
</Popover>"#.to_string()}
            />

            <Features features={vec!["Popover"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Popover"
                description="The main container for the popover component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the popover component."),
                    ("class", "Classes", "Additional CSS classes for styling the popover container."),
                ]}
            />

            <ApiSection
                title="PopoverTrigger"
                description="The element that triggers the opening and closing of the popover."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the trigger element."),
                    ("as_child", "bool", "Whether the trigger should be rendered as a child element."),
                    ("class", "Classes", "Additional CSS classes for styling the trigger."),
                ]}
            />

            <ApiSection
                title="PopoverContent"
                description="The container for the popover content."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the popover content."),
                    ("class", "Classes", "Additional CSS classes for styling the content container."),
                    ("position", "PopoverPosition", "The position of the popover relative to its trigger."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Popover component uses a context to manage its state, allowing for nested components.".to_string(),
                    "The PopoverTrigger can be used multiple times within a Popover to create multiple trigger points.".to_string(),
                    "The PopoverContent is only rendered when the popover is open, improving performance.".to_string(),
                    "Use the 'position' prop on PopoverContent to control the placement of the popover relative to its trigger.".to_string(),
                    "The popover can be closed by clicking outside of it or by using the 'Esc' key.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Popover".to_string()}
                class_descriptions={vec![
                    ("popover_container".to_string(), "For the main container of the popover".to_string()),
                    ("popover_trigger".to_string(), "For the trigger element".to_string()),
                    ("popover_content".to_string(), "For the content container".to_string()),
                    ("popover_position_north_start".to_string(), "For positioning the popover to the north start of the trigger".to_string()),
                    ("popover_position_north_middle".to_string(), "For positioning the popover to the north middle of the trigger".to_string()),
                    ("popover_position_north_end".to_string(), "For positioning the popover to the north end of the trigger".to_string()),
                    ("popover_position_south_start".to_string(), "For positioning the popover to the south start of the trigger".to_string()),
                    ("popover_position_south_middle".to_string(), "For positioning the popover to the south middle of the trigger".to_string()),
                    ("popover_position_south_end".to_string(), "For positioning the popover to the south end of the trigger".to_string()),
                    ("popover_position_east_start".to_string(), "For positioning the popover to the east start of the trigger".to_string()),
                    ("popover_position_east_middle".to_string(), "For positioning the popover to the east middle of the trigger".to_string()),
                    ("popover_position_east_end".to_string(), "For positioning the popover to the east end of the trigger".to_string()),
                    ("popover_position_west_start".to_string(), "For positioning the popover to the west start of the trigger".to_string()),
                    ("popover_position_west_middle".to_string(), "For positioning the popover to the west middle of the trigger".to_string()),
                    ("popover_position_west_end".to_string(), "For positioning the popover to the west end of the trigger".to_string()),
                ]}
            />
        </Container>
    }
}
