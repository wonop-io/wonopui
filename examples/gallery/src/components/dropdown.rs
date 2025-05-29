use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(DropdownThemeEditor)]
pub fn dropdown_theme_editor() -> Html {
    let fields = vec![
        (
            "dropdown_content".to_string(),
            "Dropdown Content".to_string(),
        ),
        ("dropdown_item".to_string(), "Dropdown Item".to_string()),
        (
            "dropdown_item_icon".to_string(),
            "Dropdown Item Icon".to_string(),
        ),
        (
            "dropdown_separator".to_string(),
            "Dropdown Separator".to_string(),
        ),
        (
            "dropdown_item_disabled".to_string(),
            "Dropdown Item Disabled".to_string(),
        ),
        (
            "dropdown_item_widget".to_string(),
            "Dropdown Item Widget".to_string(),
        ),
    ];

    let preview = html! {
        <Dropdown
            items={vec![
                DropdownItem::Action {
                    label: "Item 1".into(),
                    icon: Some(html! { <i class="fas fa-user"></i> }),
                    onclick: Callback::from(|_| log::info!("Item 1 clicked")),
                    disabled: false,
                },
                DropdownItem::Action {
                    label: "Item 2".into(),
                    icon: Some(html! { <i class="fas fa-cog"></i> }),
                    onclick: Callback::from(|_| log::info!("Item 2 clicked")),
                    disabled: false,
                },
                DropdownItem::Separator,
                DropdownItem::Action {
                    label: "Item 3".into(),
                    icon: Some(html! { <i class="fas fa-sign-out-alt"></i> }),
                    onclick: Callback::from(|_| log::info!("Item 3 clicked")),
                    disabled: false,
                },
                DropdownItem::Widget(html! { <input type="text" placeholder="Custom widget" /> }),
            ]}
            full_width = {false}
        >
            <Button>{ "Open Dropdown" }</Button>
        </Dropdown>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(DropdownDocumentation)]
pub fn dropdown_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Dropdown Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Dropdown component provides a versatile dropdown menu with customizable items, optional icons, separators, and custom widgets. It integrates with the Popover component for positioning and display control." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Dropdown
                        items={vec![
                            DropdownItem::Action {
                                label: "Item 1".into(),
                                icon: Some(html! { <i class="fas fa-user"></i> }),
                                onclick: Callback::from(|_| log::info!("Item 1 clicked")),
                                disabled: false,
                            },
                            DropdownItem::Action {
                                label: "Item 2".into(),
                                icon: Some(html! { <i class="fas fa-cog"></i> }),
                                onclick: Callback::from(|_| log::info!("Item 2 clicked")),
                                disabled: false,
                            },
                            DropdownItem::Separator,
                            DropdownItem::Action {
                                label: "Item 3".into(),
                                icon: Some(html! { <i class="fas fa-sign-out-alt"></i> }),
                                onclick: Callback::from(|_| log::info!("Item 3 clicked")),
                                disabled: false,
                            },
                            DropdownItem::Widget(html! { <input type="text" placeholder="Custom widget" /> }),
                        ]}
                        full_width = {false}
                    >
                        <Button>{ "Open Dropdown" }</Button>
                    </Dropdown>
                }}
                customize={html! {
                    <DropdownThemeEditor />
                }}
                code={r#"
<Dropdown
    items={vec![
        DropdownItem::Action {
            label: "Item 1".into(),
            icon: Some(html! { <i class="fas fa-user"></i> }),
            onclick: Callback::from(|_| log::info!("Item 1 clicked")),
            disabled: false,
        },
        DropdownItem::Action {
            label: "Item 2".into(),
            icon: Some(html! { <i class="fas fa-cog"></i> }),
            onclick: Callback::from(|_| log::info!("Item 2 clicked")),
            disabled: false,
        },
        DropdownItem::Separator,
        DropdownItem::Action {
            label: "Item 3".into(),
            icon: Some(html! { <i class="fas fa-sign-out-alt"></i> }),
            onclick: Callback::from(|_| log::info!("Item 3 clicked")),
            disabled: false,
        },
        DropdownItem::Widget(html! { <input type="text" placeholder="Custom widget" /> }),
    ]}
    full_width = {false}
>
    <Button>{ "Open Dropdown" }</Button>
</Dropdown>"#.to_string()}
            />
            <Features features={vec!["Customizable items", "Optional icons", "Separators", "Custom widgets", "Disabled items", "Flexible positioning", "Full-width option"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>

            <ApiSection
                title="Dropdown"
                description="Props for the Dropdown component."
                props={vec![
                    ("items", "Vec<DropdownItem>", "A vector of dropdown items."),
                    ("children", "Children", "The trigger for the dropdown, generally a button or link."),
                    ("position", "PopoverPosition", "The position of the dropdown relative to its trigger. Default is SouthMiddle."),
                    ("class", "Classes", "Additional CSS classes for styling the dropdown."),
                    ("full_width", "bool", "Whether the dropdown should take the full width of its container. Default is false."),
                ]}
                template_params={None}
            />

            <ApiSection
                title="DropdownItem"
                description="Enum representing different types of dropdown items."
                props={vec![
                    ("Action", "{ label: String, icon: Option<Html>, onclick: Callback<MouseEvent>, disabled: bool }", "An actionable item in the dropdown."),
                    ("Widget", "Html", "A custom widget to be inserted in the dropdown."),
                    ("Separator", "", "A separator line between items."),
                ]}
                template_params={None}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Dropdown component uses the Popover component internally for positioning and display control.".to_string(),
                    "Dropdown items can be actions, custom widgets, or separators.".to_string(),
                    "Action items can be disabled, preventing user interaction.".to_string(),
                    "The `full_width` prop allows the dropdown content to match the width of its trigger, useful for responsive designs.".to_string(),
                    "Icons can be added to action items to provide visual cues and improve usability.".to_string(),
                    "Custom widgets allow for complex interactions within the dropdown.".to_string(),
                    "The component supports keyboard navigation for accessibility.".to_string(),
                    "When using icons, ensure they are properly loaded and accessible in your project.".to_string(),
                    "Consider the positioning of the dropdown carefully to avoid overflow issues on smaller screens.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Dropdown".to_string()}
                class_descriptions={vec![
                    ("dropdown_content".to_string(), "Applied to the main dropdown content container. Use this to style the overall dropdown appearance.".to_string()),
                    ("dropdown_item".to_string(), "Applied to individual dropdown items. Use this to style the appearance of each item.".to_string()),
                    ("dropdown_item_icon".to_string(), "Applied to the icon container within a dropdown item. Use this to style the icon's appearance and positioning.".to_string()),
                    ("dropdown_separator".to_string(), "Applied to separator items. Use this to style the appearance of separators between groups of items.".to_string()),
                    ("dropdown_item_disabled".to_string(), "Applied to disabled dropdown items. Use this to style the appearance of items that cannot be interacted with.".to_string()),
                    ("dropdown_item_widget".to_string(), "Applied to custom widget items. Use this to style the container of custom widgets within the dropdown.".to_string()),
                ]}
            />
        </Container>
    }
}
