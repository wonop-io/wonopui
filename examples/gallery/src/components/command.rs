use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(CommandThemeEditor)]
pub fn command_theme_editor() -> Html {
    let fields = vec![
        (
            "command_container".to_string(),
            "Command Container".to_string(),
        ),
        (
            "command_input_wrapper".to_string(),
            "Input Wrapper".to_string(),
        ),
        ("command_icon".to_string(), "Icon".to_string()),
        ("command_input".to_string(), "Input".to_string()),
        ("command_list".to_string(), "List".to_string()),
        ("command_item".to_string(), "Item".to_string()),
        ("command_item_icon".to_string(), "Item Icon".to_string()),
        (
            "command_selected_item".to_string(),
            "Selected Item".to_string(),
        ),
    ];

    let preview = html! {
        <Command<String>
            placeholder="Search..."
            options={vec![
                ("1".to_string(), "coffee, one".to_string(), "Option 1".to_string(), None),
                ("2".to_string(), "cake, two".to_string(), "Option 2".to_string(), None),
                ("3".to_string(), "ice, three".to_string(), "Option 3".to_string(), None)
            ]}
            on_select={Callback::from(|_| {})}
        />
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CommandDocumentation)]
pub fn command_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Command Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Command component is a versatile input component that combines search functionality with option selection. It provides an intuitive interface for users to quickly find and choose from a list of options, supporting both keyboard and mouse interactions." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Command<String>
                        placeholder="Search commands..."
                        options={vec![
                            ("1".to_string(), "find, search, files, one".to_string(), "Search files".to_string(), Some(html! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-search"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
                            })),
                            ("2".to_string(), "new, create, document, two".to_string(), "Create new document".to_string(), Some(html! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-plus"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="12" x2="12" y1="18" y2="12"/><line x1="9" x2="15" y1="15" y2="15"/></svg>
                            })),
                            ("3".to_string(), "open, settings, options, three".to_string(), "Open settings".to_string(), Some(html! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
                            }))
                        ]}
                        on_select={Callback::from(|_| {})}
                    />
                }}
                customize={html! {
                    <CommandThemeEditor />
                }}
                code={r#"
<Command<String>
    placeholder="Search commands..."
    options={vec![
        ("1".to_string(), "search, files".to_string(), "Search files".to_string(), Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-search"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
        })),
        ("2".to_string(), "create, document".to_string(), "Create new document".to_string(), Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-plus"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="12" x2="12" y1="18" y2="12"/><line x1="9" x2="15" y1="15" y2="15"/></svg>
        })),
        ("3".to_string(), "open, settings".to_string(), "Open settings".to_string(), Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
        }))
    ]}
    on_select={Callback::from(|selected| {
        // Handle the selection
    })}
/>"#.to_string()}
            />

            <Features features={vec!["Command", "Search", "Keyboard Navigation"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Command"
                description="Props for the Command component."
                props={vec![
                    ("placeholder", "String", "The placeholder text to be displayed in the input field."),
                    ("options", "Vec<(T, String, String, Option<Html>)>", "A list of options where each option is a tuple of (value, keywords, label, icon)."),
                    ("on_select", "Callback<T>", "The callback to be called when an option is selected."),
                    ("class", "Classes", "Additional CSS classes to apply to the command container."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The component uses internal state to manage the open/closed state of the dropdown.".to_string(),
                    "The dropdown list is only rendered when the command is open.".to_string(),
                    "The Command component supports icons for each option, which can be customized as needed.".to_string(),
                    "For accessibility, the component supports keyboard navigation (Arrow Up/Down, Enter, Escape) and screen reader announcements.".to_string(),
                    "The search functionality is handled internally, filtering options based on user input.".to_string(),
                    "Use the Command component when you need a powerful search and select interface, especially for applications with many commands or options.".to_string(),
                    "The component is ideal for implementing features like command palettes or advanced search functionalities.".to_string(),
                    "Consider using keyboard shortcuts to quickly open the Command component for improved user efficiency.".to_string(),
                    "The component automatically closes when focus is lost, ensuring a clean user interface.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Command".to_string()}
                class_descriptions={vec![
                    ("command_container".to_string(), "Styles the main container of the command component. Use this to control the overall layout and appearance.".to_string()),
                    ("command_input_wrapper".to_string(), "Styles the wrapper around the input and search icon. This can be used to add borders or background colors to the input area.".to_string()),
                    ("command_icon".to_string(), "Styles the search icon. Adjust this to change the icon's size, color, or position.".to_string()),
                    ("command_input".to_string(), "Styles the input field. Use this to customize the text input's appearance.".to_string()),
                    ("command_list".to_string(), "Styles the list of options. This affects the dropdown's appearance when it's open.".to_string()),
                    ("command_item".to_string(), "Styles individual items in the list. Use this to customize how each option appears.".to_string()),
                    ("command_item_icon".to_string(), "Styles icons within list items. Adjust this to control the appearance of option icons.".to_string()),
                    ("command_selected_item".to_string(), "Styles the currently selected item in the list. Use this to highlight the active option.".to_string()),
                ]}
            />
        </Container>
    }
}
