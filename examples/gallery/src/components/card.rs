use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(CardThemeEditor)]
pub fn card_theme_editor() -> Html {
    let fields = vec![
        ("card_container".to_string(), "Card Container".to_string()),
        ("card_header".to_string(), "Card Header".to_string()),
        ("card_title".to_string(), "Card Title".to_string()),
        ("card_body".to_string(), "Card Body".to_string()),
    ];

    let preview = html! {
        <Card>
            <CardHeader>
                <CardTitle>{"Card Title"}</CardTitle>
            </CardHeader>
            <CardContent>
                {"This is the card content."}
            </CardContent>
        </Card>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(CardDocumentation)]
pub fn card_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Card Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Card component is a versatile container that can hold various types of content. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Card Title"}</CardTitle>
                        </CardHeader>
                        <CardContent>
                            {"This is the card content."}
                        </CardContent>
                    </Card>
                }}
                customize={html! {
                    <CardThemeEditor />
                }}
                code={r#"
<Card>
    <CardHeader>
        <CardTitle>{"Card Title"}</CardTitle>
    </CardHeader>
    <CardContent>
        {"This is the card content."}
    </CardContent>
</Card>"#.to_string()}
            />

            <Features features={vec![
                "Customizable appearance through theming",
                "Composable structure with CardHeader, CardTitle, and CardContent",
                "Optional click event handler",
                "Flexible content support",
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Card"
                description="The main container for the card component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the card component."),
                    ("class", "Classes", "Additional CSS classes to apply to the card."),
                    ("onclick", "Option<Callback<MouseEvent>>", "Optional callback for click events on the card."),
                ]}
            />

            <ApiSection
                title="CardHeader"
                description="The header section of the card."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the card header."),
                    ("class", "Classes", "Additional CSS classes to apply to the card header."),
                ]}
            />

            <ApiSection
                title="CardTitle"
                description="The title of the card."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the card title."),
                ]}
            />

            <ApiSection
                title="CardContent"
                description="The content section of the card."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the card content."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Card component is designed to be flexible and can contain various types of content.".to_string(),
                    "Use CardHeader for consistent styling of the card's header section.".to_string(),
                    "CardTitle is optimized for rendering the card's main title.".to_string(),
                    "CardContent is ideal for the main body of the card, supporting any type of content.".to_string(),
                    "You can nest Card components to create more complex layouts.".to_string(),
                    "The Card component can be made interactive by adding an onclick handler.".to_string(),
                    "For accessibility, ensure that interactive cards have appropriate ARIA attributes.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Card".to_string()}
                class_descriptions={vec![
                    ("card_container".to_string(), "Styles for the main card container. Consider using rounded corners, shadows, and background colors.".to_string()),
                    ("card_header".to_string(), "Styles for the card header section. Often includes padding and a different background color.".to_string()),
                    ("card_title".to_string(), "Styles for the card title. Usually involves font size, weight, and color adjustments.".to_string()),
                    ("card_body".to_string(), "Styles for the card content area. Typically includes padding and potentially a different background color.".to_string()),
                ]}
            />
        </Container>
    }
}
