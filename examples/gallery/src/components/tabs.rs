use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(TabsThemeEditor)]
pub fn tabs_theme_editor() -> Html {
    let fields = vec![
        ("tabs_container".to_string(), "Tabs Container".to_string()),
        ("tabs_list".to_string(), "Tabs List".to_string()),
        ("tabs_trigger".to_string(), "Tabs Trigger".to_string()),
        (
            "tabs_trigger_active".to_string(),
            "Active Trigger".to_string(),
        ),
        (
            "tabs_trigger_inactive".to_string(),
            "Inactive Trigger".to_string(),
        ),
        ("tabs_content".to_string(), "Tabs Content".to_string()),
    ];

    let preview = html! {
        <Tabs default_value="tab1">
            <TabsList>
                <TabsTrigger value="tab1">{ "Tab 1" }</TabsTrigger>
                <TabsTrigger value="tab2">{ "Tab 2" }</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">{ "Content 1" }</TabsContent>
            <TabsContent value="tab2">{ "Content 2" }</TabsContent>
        </Tabs>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(TabsDemo)]
pub fn tabs_demo() -> Html {
    html! {
        <Tabs default_value="tab2">
            <TabsList>
                <TabsTrigger value="tab1">{ "Tab 1" }</TabsTrigger>
                <TabsTrigger value="tab2">{ "Tab 2" }</TabsTrigger>
                <TabsTrigger value="tab3">{ "Tab 3" }</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">
                { "Tab 1 Content" }
            </TabsContent>
            <TabsContent value="tab2">
                { "Tab 2 Content" }
            </TabsContent>
            <TabsContent value="tab3">
                { "Tab 3 Content" }
            </TabsContent>
        </Tabs>
    }
}

#[function_component(TabsDocumentation)]
pub fn tabs_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Tabs Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Tabs component is a versatile UI element that allows users to switch between different views or sections within the same page. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! { <TabsDemo /> }}
                customize={html! { <TabsThemeEditor /> }}
                code={r#"
fn tabs_demo() -> Html {
    html! {
        <Tabs default_value={(*active_tab).clone()}>
            <TabsList>
                <TabsTrigger value="tab1">{ "Tab 1" }</TabsTrigger>
                <TabsTrigger value="tab2">{ "Tab 2" }</TabsTrigger>
                <TabsTrigger value="tab3">{ "Tab 3" }</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">
                { "Tab 1 Content" }
            </TabsContent>
            <TabsContent value="tab2">
                { "Tab 2 Content" }
            </TabsContent>
            <TabsContent value="tab3">
                { "Tab 3 Content" }
            </TabsContent>
        </Tabs>
    }
}
                "#.to_string()}
            />

            <Features features={vec!["Tabs", "TabsList", "TabsTrigger", "TabsContent"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Tabs"
                description="The main container for the tabs component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the tabs component."),
                    ("default_value", "String", "The default active tab value."),
                    ("class", "Classes", "Additional CSS classes for styling the tabs container."),
                ]}
            />

            <ApiSection
                title="TabsList"
                description="The container for the list of tab triggers."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the tabs list."),
                    ("class", "Classes", "Additional CSS classes for styling the tabs list."),
                    ("direction", "FlexDirection", "The direction of the tabs list. Defaults to Row."),
                ]}
            />

            <ApiSection
                title="TabsTrigger"
                description="The button that triggers the switching of tabs."
                props={vec![
                    ("value", "String", "The value of the tab to be activated when the trigger is clicked."),
                    ("children", "Children", "The child elements to be rendered inside the tabs trigger."),
                    ("class", "Classes", "Additional CSS classes for styling the tabs trigger."),
                ]}
            />

            <ApiSection
                title="TabsContent"
                description="The container for the content of a specific tab."
                props={vec![
                    ("value", "String", "The value of the tab that this content belongs to."),
                    ("children", "Children", "The child elements to be rendered inside the tabs content."),
                    ("class", "Classes", "Additional CSS classes for styling the tabs content."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Tabs component uses internal state to manage the active tab.".to_string(),
                    "Each TabsTrigger should have a unique value that corresponds to a TabsContent.".to_string(),
                    "The TabsList component can be customized to display tabs horizontally or vertically.".to_string(),
                    "For accessibility, the component supports keyboard navigation and ARIA attributes.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Tabs".to_string()}
                class_descriptions={vec![
                    ("tabs_container".to_string(), "For the main container of the tabs component".to_string()),
                    ("tabs_list".to_string(), "For the list of tab triggers".to_string()),
                    ("tabs_trigger".to_string(), "For individual tab triggers".to_string()),
                    ("tabs_trigger_active".to_string(), "For the active tab trigger".to_string()),
                    ("tabs_trigger_inactive".to_string(), "For inactive tab triggers".to_string()),
                    ("tabs_content".to_string(), "For the content of each tab".to_string()),
                ]}
            />
        </Container>
    }
}
