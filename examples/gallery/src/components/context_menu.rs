use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(ContextMenuThemeEditor)]
pub fn context_menu_theme_editor() -> Html {
    let fields = vec![
        ("context_menu_trigger".to_string(), "Trigger".to_string()),
        ("context_menu_content".to_string(), "Content".to_string()),
        ("context_menu_item".to_string(), "Item".to_string()),
        ("context_menu_sub".to_string(), "Submenu".to_string()),
        (
            "context_menu_sub_trigger".to_string(),
            "Submenu Trigger".to_string(),
        ),
        (
            "context_menu_sub_content".to_string(),
            "Submenu Content".to_string(),
        ),
        (
            "context_menu_separator".to_string(),
            "Separator".to_string(),
        ),
        (
            "context_menu_checkbox_item".to_string(),
            "Checkbox Item".to_string(),
        ),
        (
            "context_menu_radio_group".to_string(),
            "Radio Group".to_string(),
        ),
        ("context_menu_label".to_string(), "Label".to_string()),
        (
            "context_menu_radio_item".to_string(),
            "Radio Item".to_string(),
        ),
        ("context_menu_shortcut".to_string(), "Shortcut".to_string()),
    ];

    let preview = html! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                { "Right click here" }
            </ContextMenuTrigger>
            <ContextMenuContent class="w-64">
                <ContextMenuItem inset=true>
                    { "Back" }
                    <ContextMenuShortcut>{ "⌘[" }</ContextMenuShortcut>
                </ContextMenuItem>
                <ContextMenuItem inset=true disabled=true>
                    { "Forward" }
                    <ContextMenuShortcut>{ "⌘]" }</ContextMenuShortcut>
                </ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuCheckboxItem checked=true>
                    { "Show Bookmarks Bar" }
                </ContextMenuCheckboxItem>
                <ContextMenuRadioGroup value="pedro">
                    <ContextMenuLabel inset=true>{ "People" }</ContextMenuLabel>
                    <ContextMenuRadioItem value="pedro">
                        { "Pedro Duarte" }
                    </ContextMenuRadioItem>
                    <ContextMenuRadioItem value="colm">{ "Colm Tuite" }</ContextMenuRadioItem>
                </ContextMenuRadioGroup>
            </ContextMenuContent>
        </ContextMenu>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(ContextMenuDocumentation)]
pub fn context_menu_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "ContextMenu Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The ContextMenu component presents a contextual menu that appears upon a right-click action. It is a versatile component suitable for providing additional options or actions related to specific content or interactions." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <ContextMenu>
                        <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                            { "Right click here" }
                        </ContextMenuTrigger>
                        <ContextMenuContent class="w-64">
                            <ContextMenuItem inset=true>
                                { "Back" }
                                <ContextMenuShortcut>{ "⌘[" }</ContextMenuShortcut>
                            </ContextMenuItem>
                            <ContextMenuItem inset=true disabled=true>
                                { "Forward" }
                                <ContextMenuShortcut>{ "⌘]" }</ContextMenuShortcut>
                            </ContextMenuItem>
                            <ContextMenuItem inset=true>
                                { "Reload" }
                                <ContextMenuShortcut>{ "⌘R" }</ContextMenuShortcut>
                            </ContextMenuItem>
                            <ContextMenuSub>
                                <ContextMenuSubTrigger inset=true>{ "More Tools" }</ContextMenuSubTrigger>
                                <ContextMenuSubContent class="w-48">
                                    <ContextMenuItem>
                                        { "Save Page As..." }
                                        <ContextMenuShortcut>{ "⇧⌘S" }</ContextMenuShortcut>
                                    </ContextMenuItem>
                                    <ContextMenuItem>{ "Create Shortcut..." }</ContextMenuItem>
                                    <ContextMenuItem>{ "Name Window..." }</ContextMenuItem>
                                    <ContextMenuSeparator />
                                    <ContextMenuItem>{ "Developer Tools" }</ContextMenuItem>
                                </ContextMenuSubContent>
                            </ContextMenuSub>
                            <ContextMenuSeparator />
                            <ContextMenuCheckboxItem checked=true>
                                { "Show Bookmarks Bar" }
                                <ContextMenuShortcut>{ "⌘⇧B" }</ContextMenuShortcut>
                            </ContextMenuCheckboxItem>
                            <ContextMenuCheckboxItem>{ "Show Full URLs" }</ContextMenuCheckboxItem>
                            <ContextMenuSeparator />
                            <ContextMenuRadioGroup value="pedro">
                                <ContextMenuLabel inset=true>{ "People" }</ContextMenuLabel>
                                <ContextMenuSeparator />
                                <ContextMenuRadioItem value="pedro">
                                    { "Pedro Duarte" }
                                </ContextMenuRadioItem>
                                <ContextMenuRadioItem value="colm">{ "Colm Tuite" }</ContextMenuRadioItem>
                            </ContextMenuRadioGroup>
                        </ContextMenuContent>
                    </ContextMenu>
                }}
                customize={html! {
                    <ContextMenuThemeEditor />
                }}
                code={r#"
<ContextMenu>
    <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
        { "Right click here" }
    </ContextMenuTrigger>
    <ContextMenuContent class="w-64">
        <ContextMenuItem inset=true>
            { "Back" }
            <ContextMenuShortcut>{ "⌘[" }</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuItem inset=true disabled=true>
            { "Forward" }
            <ContextMenuShortcut>{ "⌘]" }</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuItem inset=true>
            { "Reload" }
            <ContextMenuShortcut>{ "⌘R" }</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuSub>
            <ContextMenuSubTrigger inset=true>{ "More Tools" }</ContextMenuSubTrigger>
            <ContextMenuSubContent class="w-48">
                <ContextMenuItem>
                    { "Save Page As..." }
                    <ContextMenuShortcut>{ "⇧⌘S" }</ContextMenuShortcut>
                </ContextMenuItem>
                <ContextMenuItem>{ "Create Shortcut..." }</ContextMenuItem>
                <ContextMenuItem>{ "Name Window..." }</ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuItem>{ "Developer Tools" }</ContextMenuItem>
            </ContextMenuSubContent>
        </ContextMenuSub>
        <ContextMenuSeparator />
        <ContextMenuCheckboxItem checked=true>
            { "Show Bookmarks Bar" }
            <ContextMenuShortcut>{ "⌘⇧B" }</ContextMenuShortcut>
        </ContextMenuCheckboxItem>
        <ContextMenuCheckboxItem>{ "Show Full URLs" }</ContextMenuCheckboxItem>
        <ContextMenuSeparator />
        <ContextMenuRadioGroup value="pedro">
            <ContextMenuLabel inset=true>{ "People" }</ContextMenuLabel>
            <ContextMenuSeparator />
            <ContextMenuRadioItem value="pedro">
                { "Pedro Duarte" }
            </ContextMenuRadioItem>
            <ContextMenuRadioItem value="colm">{ "Colm Tuite" }</ContextMenuRadioItem>
        </ContextMenuRadioGroup>
    </ContextMenuContent>
</ContextMenu>"#.to_string()}
            />

            <Features features={vec!["ContextMenu", "ContextMenuTrigger", "ContextMenuContent", "ContextMenuItem", "ContextMenuSub", "ContextMenuSubTrigger", "ContextMenuSubContent", "ContextMenuSeparator", "ContextMenuCheckboxItem", "ContextMenuRadioGroup", "ContextMenuLabel", "ContextMenuRadioItem", "ContextMenuShortcut"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="ContextMenu"
                description="The main container for the context menu component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the context menu component."),
                ]}
            />

            <ApiSection
                title="ContextMenuTrigger"
                description="The element that triggers the opening of the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the trigger element."),
                    ("class", "Classes", "Additional CSS classes to apply to the trigger."),
                ]}
            />

            <ApiSection
                title="ContextMenuContent"
                description="The container for the context menu content."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the context menu content."),
                    ("class", "Classes", "Additional CSS classes to apply to the content."),
                ]}
            />

            <ApiSection
                title="ContextMenuItem"
                description="Individual items within the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the context menu item."),
                    ("inset", "bool", "Whether the item should be inset."),
                    ("disabled", "bool", "Whether the item is disabled."),
                ]}
            />

            <ApiSection
                title="ContextMenuSub"
                description="A submenu within the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the submenu."),
                ]}
            />

            <ApiSection
                title="ContextMenuSubTrigger"
                description="The trigger for a submenu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the submenu trigger."),
                    ("inset", "bool", "Whether the trigger should be inset."),
                ]}
            />

            <ApiSection
                title="ContextMenuSubContent"
                description="The content of a submenu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the submenu content."),
                    ("class", "Classes", "Additional CSS classes to apply to the submenu content."),
                ]}
            />

            <ApiSection
                title="ContextMenuSeparator"
                description="A separator line in the context menu."
                props={vec![]}
            />

            <ApiSection
                title="ContextMenuCheckboxItem"
                description="A checkbox item in the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the checkbox item."),
                    ("checked", "bool", "Whether the checkbox is checked."),
                ]}
            />

            <ApiSection
                title="ContextMenuRadioGroup"
                description="A group of radio items in the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the radio group."),
                    ("value", "String", "The currently selected value in the radio group."),
                ]}
            />

            <ApiSection
                title="ContextMenuLabel"
                description="A label for a group of items in the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the label."),
                    ("inset", "bool", "Whether the label should be inset."),
                ]}
            />

            <ApiSection
                title="ContextMenuRadioItem"
                description="A radio item within a radio group in the context menu."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the radio item."),
                    ("value", "String", "The value associated with this radio item."),
                ]}
            />

            <ApiSection
                title="ContextMenuShortcut"
                description="A keyboard shortcut displayed next to a menu item."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the shortcut."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The ContextMenu component uses context to manage its open/closed state.".to_string(),
                    "Right-clicking on the ContextMenuTrigger opens the context menu.".to_string(),
                    "The context menu is automatically hidden when clicking outside the menu.".to_string(),
                    "Use ContextMenuSub for nested submenus.".to_string(),
                    "ContextMenuCheckboxItem and ContextMenuRadioItem can be used for selectable options.".to_string(),
                    "ContextMenuShortcut can be used to display keyboard shortcuts for menu items.".to_string(),
                    "The component supports keyboard navigation for improved accessibility.".to_string(),
                    "Ensure that the context menu is positioned within the viewport to avoid overflow issues.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"ContextMenu".to_string()}
                class_descriptions={vec![
                    ("context_menu_trigger".to_string(), "For the trigger element that opens the context menu".to_string()),
                    ("context_menu_content".to_string(), "For the main content wrapper of the context menu".to_string()),
                    ("context_menu_item".to_string(), "For individual menu items".to_string()),
                    ("context_menu_sub".to_string(), "For submenu containers".to_string()),
                    ("context_menu_sub_trigger".to_string(), "For submenu trigger elements".to_string()),
                    ("context_menu_sub_content".to_string(), "For submenu content wrappers".to_string()),
                    ("context_menu_separator".to_string(), "For separator lines in the menu".to_string()),
                    ("context_menu_checkbox_item".to_string(), "For checkbox items in the menu".to_string()),
                    ("context_menu_radio_group".to_string(), "For radio group containers".to_string()),
                    ("context_menu_label".to_string(), "For labels in the menu".to_string()),
                    ("context_menu_radio_item".to_string(), "For radio items in the menu".to_string()),
                    ("context_menu_shortcut".to_string(), "For keyboard shortcut displays".to_string()),
                ]}
            />
        </Container>
    }
}
