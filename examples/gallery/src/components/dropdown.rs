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
        (
            "dropdown_heading".to_string(),
            "Dropdown Item Headfing".to_string(),
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
                    <div class="grid grid-cols-3 gap-8 py-8 min-h-[300px]">
                        // Top row
                        <div class="flex justify-start items-start">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Heading { label: "Account".into() },
                                    DropdownItem::Action { label: "Profile".into(), icon: None, onclick: Callback::from(|_| log::info!("Profile")), disabled: false },
                                    DropdownItem::Action { label: "Settings".into(), icon: None, onclick: Callback::from(|_| log::info!("Settings")), disabled: false },
                                    DropdownItem::Separator,
                                    DropdownItem::Action { label: "Logout".into(), icon: None, onclick: Callback::from(|_| log::info!("Logout")), disabled: false },
                                ]}
                                position={PopoverPosition::SouthStart}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Top Left ↓" }</Button>
                            </Dropdown>
                        </div>
                        <div class="flex justify-center items-start">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Edit".into(), icon: Some(html! { <svg class="size-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg> }), onclick: Callback::from(|_| log::info!("Edit")), disabled: false },
                                    DropdownItem::Action { label: "Duplicate".into(), icon: Some(html! { <svg class="size-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg> }), onclick: Callback::from(|_| log::info!("Duplicate")), disabled: false },
                                    DropdownItem::Separator,
                                    DropdownItem::Action { label: "Delete".into(), icon: Some(html! { <svg class="size-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg> }), onclick: Callback::from(|_| log::info!("Delete")), disabled: false },
                                ]}
                                position={PopoverPosition::SouthMiddle}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Top Center ↓" }</Button>
                            </Dropdown>
                        </div>
                        <div class="flex justify-end items-start">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Share".into(), icon: None, onclick: Callback::from(|_| log::info!("Share")), disabled: false },
                                    DropdownItem::Action { label: "Export".into(), icon: None, onclick: Callback::from(|_| log::info!("Export")), disabled: false },
                                ]}
                                position={PopoverPosition::SouthEnd}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Top Right ↓" }</Button>
                            </Dropdown>
                        </div>
                        // Middle row
                        <div class="flex justify-start items-center">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Option A".into(), icon: None, onclick: Callback::from(|_| log::info!("A")), disabled: false },
                                    DropdownItem::Action { label: "Option B".into(), icon: None, onclick: Callback::from(|_| log::info!("B")), disabled: false },
                                    DropdownItem::Action { label: "Option C".into(), icon: None, onclick: Callback::from(|_| log::info!("C")), disabled: true },
                                ]}
                                position={PopoverPosition::EastStart}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Left →" }</Button>
                            </Dropdown>
                        </div>
                        <div class="flex justify-center items-center">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Heading { label: "Quick Actions".into() },
                                    DropdownItem::Action { label: "New File".into(), icon: None, onclick: Callback::from(|_| log::info!("New")), disabled: false },
                                    DropdownItem::Action { label: "Open...".into(), icon: None, onclick: Callback::from(|_| log::info!("Open")), disabled: false },
                                    DropdownItem::Action { label: "Save".into(), icon: None, onclick: Callback::from(|_| log::info!("Save")), disabled: false },
                                    DropdownItem::Separator,
                                    DropdownItem::Action { label: "Preferences".into(), icon: None, onclick: Callback::from(|_| log::info!("Prefs")), disabled: false },
                                ]}
                            >
                                <Button size={ButtonSize::Small}>{ "Center (Default)" }</Button>
                            </Dropdown>
                        </div>
                        <div class="flex justify-end items-center">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Item 1".into(), icon: None, onclick: Callback::from(|_| log::info!("1")), disabled: false },
                                    DropdownItem::Action { label: "Item 2".into(), icon: None, onclick: Callback::from(|_| log::info!("2")), disabled: false },
                                ]}
                                position={PopoverPosition::WestStart}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "← Right" }</Button>
                            </Dropdown>
                        </div>
                        // Bottom row
                        <div class="flex justify-start items-end">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Action".into(), icon: None, onclick: Callback::from(|_| log::info!("Act")), disabled: false },
                                ]}
                                position={PopoverPosition::NorthStart}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Bottom Left ↑" }</Button>
                            </Dropdown>
                        </div>
                        <div class="flex justify-center items-end">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Up Action".into(), icon: None, onclick: Callback::from(|_| log::info!("Up")), disabled: false },
                                ]}
                                position={PopoverPosition::NorthMiddle}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Bottom Center ↑" }</Button>
                            </Dropdown>
                        </div>
                        <div class="flex justify-end items-end">
                            <Dropdown
                                items={vec![
                                    DropdownItem::Action { label: "Final Action".into(), icon: None, onclick: Callback::from(|_| log::info!("Final")), disabled: false },
                                ]}
                                position={PopoverPosition::NorthEnd}
                            >
                                <Button variant={ButtonVariant::Outline} size={ButtonSize::Small}>{ "Bottom Right ↑" }</Button>
                            </Dropdown>
                        </div>
                    </div>
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
                    ("dropdown_heading".to_string(), "Applied to custom widget items. Use this to style the container of headings within the dropdown.".to_string()),

                ]}
            />
        </Container>
    }
}
