use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(DrawerThemeEditor)]
pub fn drawer_theme_editor() -> Html {
    let fields = vec![
        (
            "drawer_provider".to_string(),
            "Drawer Container".to_string(),
        ),
        ("drawer_container".to_string(), "Drawer Content".to_string()),
        ("drawer_header".to_string(), "Drawer Header".to_string()),
        ("drawer_title".to_string(), "Drawer Title".to_string()),
        (
            "drawer_description".to_string(),
            "Drawer Description".to_string(),
        ),
        ("drawer_footer".to_string(), "Drawer Footer".to_string()),
        ("drawer_right".to_string(), "Drawer Right".to_string()),
        ("drawer_top".to_string(), "Drawer Top".to_string()),
        ("drawer_bottom".to_string(), "Drawer Bottom".to_string()),
        ("drawer_left".to_string(), "Drawer Left".to_string()),
    ];

    let preview = html! {
        <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
            <Drawer<String>>
                <DrawerHeader>
                    <DrawerTitle>{"Drawer Title"}</DrawerTitle>
                    <DrawerClose<String>>{"X"}</DrawerClose<String>>
                </DrawerHeader>
                <p>{"Drawer Content"}</p>
                <DrawerFooter>
                    <DrawerClose<String>>
                        <Button>{"Close"}</Button>
                    </DrawerClose<String>>
                </DrawerFooter>
            </Drawer<String>>
        })}>
            <DrawerTrigger<String> drawer={"example".to_string()}>
                <Button>{"Open Drawer"}</Button>
            </DrawerTrigger<String>>
        </DrawerProvider<String>>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(DrawerDocumentation)]
pub fn drawer_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Drawer Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Drawer component is a versatile UI element that slides in from the side of the screen. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
                        <Drawer<String>>
                            <DrawerHeader>
                                <DrawerTitle>{"Drawer Title"}</DrawerTitle>
                                <DrawerClose<String>>{"X"}</DrawerClose<String>>
                            </DrawerHeader>
                            <p>{"Drawer Content"}</p>
                            <DrawerFooter>
                                <DrawerClose<String>>
                                    <Button>{"Close"}</Button>
                                </DrawerClose<String>>
                            </DrawerFooter>
                        </Drawer<String>>
                    })}>
                        <DrawerTrigger<String> drawer={"example".to_string()}>
                            <Button>{"Open Drawer"}</Button>
                        </DrawerTrigger<String>>
                    </DrawerProvider<String>>
                }}
                customize={html! {
                    <DrawerThemeEditor />
                }}
                code={r#"
<DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
    <Drawer<String>>
        <DrawerHeader>
            <DrawerTitle>{"Drawer Title"}</DrawerTitle>
            <DrawerClose<String>>{"X"}</DrawerClose<String>>
        </DrawerHeader>
        <p>{"Drawer Content"}</p>
        <DrawerFooter>
            <DrawerClose<String>>
                <Button>{"Close"}</Button>
            </DrawerClose<String>>
        </DrawerFooter>
    </Drawer<String>>
})}>
    <DrawerTrigger<String> drawer="example".to_string()>
        <Button>{"Open Drawer"}</Button>
    </DrawerTrigger<String>>
</DrawerProvider<String>>"#.to_string()}
            />

            <Features features={vec!["Drawer"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="DrawerProvider<T>"
                description="The main container for the drawer component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the drawer component."),
                    ("side", "DrawerSide", "The side from which the drawer will slide in. Options are Left, Right, Top, Bottom. Default is Left."),
                    ("render", "Callback<T, Html>", "A callback to render the drawer content."),
                    ("curtain", "bool", "Whether to show a curtain behind the drawer. Default is false."),
                    ("curtain_content", "Html", "Content to show in the curtain. Default is empty."),
                ]}
                template_params={Some(vec![
                    ("T", "The type of the drawer identifier. Must implement Clone, PartialEq, and 'static."),
                ])}
            />

            <ApiSection
                title="DrawerTrigger<T>"
                description="The element that triggers the opening and closing of the drawer."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the trigger element."),
                    ("drawer", "T", "The identifier for the drawer to be opened."),
                ]}
                template_params={Some(vec![
                    ("T", "The type of the drawer identifier."),
                ])}
            />

            <ApiSection
                title="Drawer<T>"
                description="The container for the drawer content."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the drawer content."),
                ]}
                template_params={Some(vec![
                    ("T", "The type of the drawer identifier."),
                ])}
            />

            <ApiSection
                title="DrawerClose<T>"
                description="The element that triggers the closing of the drawer."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the close element."),
                ]}
                template_params={Some(vec![
                    ("T", "The type of the drawer identifier."),
                ])}
            />

            <ApiSection
                title="DrawerHeader"
                description="The header section of the drawer."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the header."),
                ]}
            />

            <ApiSection
                title="DrawerTitle"
                description="The title of the drawer."
                props={vec![
                    ("children", "Children", "The child elements to be rendered as the title."),
                ]}
            />

            <ApiSection
                title="DrawerFooter"
                description="The footer section of the drawer."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the footer."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Drawer component uses a generic type T for flexibility in identifying different drawers.".to_string(),
                    "The side prop determines which side the drawer slides in from.".to_string(),
                    "Use the curtain prop to add a background overlay when the drawer is open.".to_string(),
                    "The DrawerTrigger component is used to open the drawer, while DrawerClose is used to close it.".to_string(),
                    "Drawer content is rendered using the render callback, allowing for dynamic content based on the drawer identifier.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Drawer".to_string()}
                class_descriptions={vec![
                    ("drawer_provider".to_string(), "For the main container of the drawer component".to_string()),
                    ("drawer_container".to_string(), "For the content area of the drawer".to_string()),
                    ("drawer_header".to_string(), "For the header section of the drawer".to_string()),
                    ("drawer_title".to_string(), "For the title within the drawer header".to_string()),
                    ("drawer_description".to_string(), "For the description text in the drawer".to_string()),
                    ("drawer_footer".to_string(), "For the footer section of the drawer".to_string()),
                    ("drawer_right".to_string(), "Applied when the drawer slides in from the right".to_string()),
                    ("drawer_top".to_string(), "Applied when the drawer slides in from the top".to_string()),
                    ("drawer_bottom".to_string(), "Applied when the drawer slides in from the bottom".to_string()),
                    ("drawer_left".to_string(), "Applied when the drawer slides in from the left".to_string()),
                ]}
            />
        </Container>
    }
}
