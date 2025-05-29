use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(DialogThemeEditor)]
pub fn dialog_theme_editor() -> Html {
    let fields = vec![
        (
            "dialog_container".to_string(),
            "Dialog Container".to_string(),
        ),
        ("dialog_content".to_string(), "Dialog Content".to_string()),
        ("dialog_header".to_string(), "Dialog Header".to_string()),
        ("dialog_title".to_string(), "Dialog Title".to_string()),
        (
            "dialog_description".to_string(),
            "Dialog Description".to_string(),
        ),
        ("dialog_footer".to_string(), "Dialog Footer".to_string()),
    ];

    let preview = html! {
        <DialogProvider>
            <DialogTrigger id="example-dialog">
                <Button>{"Open Dialog"}</Button>
            </DialogTrigger>
            <Dialog id="example-dialog">
                <DialogHeader>
                    <DialogTitle>{"Dialog Title"}</DialogTitle>
                </DialogHeader>
                <DialogBody>{"This is a description inside the dialog."}</DialogBody>
                <DialogFooter>
                    <DialogClose>
                        <Button>{"Close"}</Button>
                    </DialogClose>
                </DialogFooter>
            </Dialog>
        </DialogProvider>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(DialogDocumentation)]
pub fn dialog_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Dialog Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Dialog component is a versatile UI element that displays content in a modal dialog. It is composed of several subcomponents to provide a flexible and customizable experience." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <DialogProvider>
                        <DialogTrigger id="example-dialog">
                            <Button>{"Open Dialog"}</Button>
                        </DialogTrigger>
                        <Dialog id="example-dialog">
                            <DialogHeader>
                                <DialogTitle>{"Dialog Title"}</DialogTitle>
                            </DialogHeader>
                            <DialogBody>{"This is a description inside the dialog."}</DialogBody>
                            <DialogFooter>
                                <DialogClose>
                                    <Button>{"Close"}</Button>
                                </DialogClose>
                            </DialogFooter>
                        </Dialog>
                    </DialogProvider>
                }}
                customize={html! {
                    <DialogThemeEditor />
                }}
                code={r#"
<DialogProvider>
    <DialogTrigger id="example-dialog">
        <Button>{"Open Dialog"}</Button>
    </DialogTrigger>
    <Dialog id="example-dialog">
        <DialogHeader>
            <DialogTitle>{"Dialog Title"}</DialogTitle>
        </DialogHeader>
        <DialogBody>{"This is a description inside the dialog."}</DialogBody>
        <DialogFooter>
            <DialogClose>
                <Button>{"Close"}</Button>
            </DialogClose>
        </DialogFooter>
    </Dialog>
</DialogProvider>"#.to_string()}
            />

            <Features features={vec!["Customizable", "Accessible", "Nested Dialogs", "Context-based State Management", "Keyboard Navigation"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Dialog"
                description="The main container for the dialog component."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the dialog component."),
                ]}
            />

            <ApiSection
                title="DialogTrigger"
                description="The element that triggers the opening and closing of the dialog."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the trigger element."),
                    ("id", "String", "A unique identifier for the dialog."),
                ]}
            />

            <ApiSection
                title="Dialog"
                description="The container for the dialog content."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the dialog content."),
                    ("id", "String", "A unique identifier for the dialog, matching the DialogTrigger id."),
                ]}
            />

            <ApiSection
                title="DialogHeader"
                description="The header section of the dialog."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the header."),
                ]}
            />

            <ApiSection
                title="DialogTitle"
                description="The title of the dialog."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the title."),
                ]}
            />

            <ApiSection
                title="DialogBody"
                description="The body section of the dialog."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the body."),
                ]}
            />

            <ApiSection
                title="DialogFooter"
                description="The footer section of the dialog."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the footer."),
                ]}
            />

            <ApiSection
                title="DialogClose"
                description="The element that triggers the closing of the dialog."
                props={vec![
                    ("children", "Children", "The child elements to be rendered inside the close element."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Dialog component uses a context (DialogContext) to manage its open/closed state.".to_string(),
                    "Multiple dialogs can be nested, with each dialog managed by its unique id.".to_string(),
                    "The DialogTrigger and DialogClose components handle opening and closing the dialog using the context's toggle callback.".to_string(),
                    "The dialog content is only rendered when the dialog is open, improving performance.".to_string(),
                    "For accessibility, ensure that the dialog traps focus when open and supports closing with the Escape key.".to_string(),
                    "Use semantic HTML within the dialog for better screen reader support.".to_string(),
                    "The Dialog component is flexible and can be used for various purposes such as alerts, confirmations, or complex forms.".to_string(),
                    "When nesting dialogs, be mindful of the user experience and avoid deep nesting when possible.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Dialog".to_string()}
                class_descriptions={vec![
                    ("dialog_container".to_string(), "For the main dialog container, typically includes a semi-transparent overlay".to_string()),
                    ("dialog_content".to_string(), "For the dialog content wrapper, usually a centered box with a background".to_string()),
                    ("dialog_header".to_string(), "For the dialog header, often contains the title and close button".to_string()),
                    ("dialog_title".to_string(), "For the dialog title, usually a larger, bold text".to_string()),
                    ("dialog_description".to_string(), "For the dialog body text, typically regular-sized text".to_string()),
                    ("dialog_footer".to_string(), "For the dialog footer, often contains action buttons".to_string()),
                ]}
            />

        </Container>
    }
}
