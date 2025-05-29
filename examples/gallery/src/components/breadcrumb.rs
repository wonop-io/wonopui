use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(BreadcrumbThemeEditor)]
pub fn breadcrumb_theme_editor() -> Html {
    let fields = vec![
        ("breadcrumb_nav".to_string(), "Breadcrumb Nav".to_string()),
        ("breadcrumb_list".to_string(), "Breadcrumb List".to_string()),
        ("breadcrumb_item".to_string(), "Breadcrumb Item".to_string()),
        (
            "breadcrumb_separator".to_string(),
            "Breadcrumb Separator".to_string(),
        ),
    ];

    let preview = html! {
        <Breadcrumb>
            <BreadcrumbItem label="Home" href="#" />
            <BreadcrumbItem label="Library" href="#" />
            <BreadcrumbItem label="Data" />
        </Breadcrumb>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(BreadcrumbDocumentation)]
pub fn breadcrumb_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Breadcrumb Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Breadcrumb component is used to display a navigation trail for users. It helps users understand their location within the hierarchy of a website or application and provides easy navigation to parent pages." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Breadcrumb>
                        <BreadcrumbItem label="Home" href="#" />
                        <BreadcrumbItem label="Library" href="#" />
                        <BreadcrumbItem label="Data" />
                    </Breadcrumb>
                }}
                customize={html! {
                    <BreadcrumbThemeEditor />
                }}
                code={r#"
<Breadcrumb>
    <BreadcrumbItem label="Home" href="\#" />
    <BreadcrumbItem label="Library" href="\#" />
    <BreadcrumbItem label="Data" />
</Breadcrumb>"#.to_string()}
            />

            <Features features={vec![
                "Customizable separator icon",
                "Automatic styling for the current page",
                "Accessible navigation with proper ARIA attributes",
                "Responsive design",
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Breadcrumb"
                description="Props for the Breadcrumb component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the breadcrumb component."),
                    ("separator_icon", "Option<Html>", "An optional custom separator icon between breadcrumb items."),
                ]}
            />

            <ApiSection
                title="BreadcrumbItem"
                description="Props for the BreadcrumbItem component."
                props={vec![
                    ("label", "String", "The text to be displayed for the breadcrumb item."),
                    ("href", "Option<String>", "An optional URL for the breadcrumb item. If provided, the item will be rendered as a link."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Breadcrumb component automatically adds separators between items.".to_string(),
                    "The last BreadcrumbItem is automatically styled differently to indicate the current page.".to_string(),
                    "You can customize the separator icon by passing a custom HTML to the separator_icon prop of the Breadcrumb component.".to_string(),
                    "For accessibility, the component uses appropriate ARIA attributes.".to_string(),
                    "Keep breadcrumbs concise and use clear, descriptive labels for each item.".to_string(),
                    "Consider using truncation for very long breadcrumb trails on smaller screens.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Breadcrumb".to_string()}
                class_descriptions={vec![
                    ("breadcrumb_nav".to_string(), "For the main navigation container of the breadcrumb".to_string()),
                    ("breadcrumb_list".to_string(), "For the ordered list containing breadcrumb items".to_string()),
                    ("breadcrumb_item".to_string(), "For individual breadcrumb items".to_string()),
                    ("breadcrumb_separator".to_string(), "For the separator between breadcrumb items".to_string()),
                ]}
            />
        </Container>
    }
}
