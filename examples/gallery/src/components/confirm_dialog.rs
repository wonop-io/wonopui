use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(ConfirmDialogDocumentation)]
pub fn confirm_dialog_documentation() -> Html {
    let default_visible = use_state(|| false);
    let destructive_visible = use_state(|| false);

    let show_default = {
        let visible = default_visible.clone();
        Callback::from(move |_| visible.set(true))
    };
    let hide_default = {
        let visible = default_visible.clone();
        Callback::from(move |_| visible.set(false))
    };

    let show_destructive = {
        let visible = destructive_visible.clone();
        Callback::from(move |_| visible.set(true))
    };
    let hide_destructive = {
        let visible = destructive_visible.clone();
        Callback::from(move |_| visible.set(false))
    };

    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "ConfirmDialog Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "A modal dialog that asks the user to confirm an action. Supports default and destructive variants for different use cases." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Default Variant" }</h2>
            <ExampleCode
                preview={html! {
                    <div>
                        <Button onclick={show_default.clone()}>{"Open Default Dialog"}</Button>
                        <ConfirmDialog
                            visible={*default_visible}
                            title="Confirm Action"
                            message="Are you sure you want to proceed with this action?"
                            on_confirm={hide_default.clone()}
                            on_cancel={hide_default.clone()}
                        />
                    </div>
                }}
                code={r#"
let visible = use_state(|| false);

<Button onclick={show}>{"Open Dialog"}</Button>
<ConfirmDialog
    visible={*visible}
    title="Confirm Action"
    message="Are you sure you want to proceed?"
    on_confirm={on_confirm}
    on_cancel={on_cancel}
/>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Destructive Variant" }</h2>
            <ExampleCode
                preview={html! {
                    <div>
                        <Button variant={ButtonVariant::Danger} onclick={show_destructive.clone()}>{"Delete Item"}</Button>
                        <ConfirmDialog
                            visible={*destructive_visible}
                            title="Delete Item"
                            message="Are you sure you want to delete this item? This action cannot be undone."
                            confirm_text="Delete"
                            cancel_text="Cancel"
                            variant={ConfirmDialogVariant::Destructive}
                            on_confirm={hide_destructive.clone()}
                            on_cancel={hide_destructive.clone()}
                        />
                    </div>
                }}
                code={r#"
<ConfirmDialog
    visible={*visible}
    title="Delete Item"
    message="Are you sure? This cannot be undone."
    confirm_text="Delete"
    variant={ConfirmDialogVariant::Destructive}
    on_confirm={on_confirm}
    on_cancel={on_cancel}
/>"#.to_string()}
            />

            <Features features={vec![
                "Two variants: Default and Destructive",
                "Customizable title, message, and button text",
                "Backdrop click to dismiss",
                "Accessible modal behavior"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="ConfirmDialog"
                description="Props for the ConfirmDialog component."
                props={vec![
                    ("visible", "bool", "Whether the dialog is visible"),
                    ("title", "String", "Dialog title"),
                    ("message", "String", "Dialog message/description"),
                    ("confirm_text", "String", "Text for confirm button. Default: 'Confirm'"),
                    ("cancel_text", "String", "Text for cancel button. Default: 'Cancel'"),
                    ("variant", "ConfirmDialogVariant", "Dialog variant: Default or Destructive"),
                    ("on_confirm", "Callback<()>", "Callback when confirm is clicked"),
                    ("on_cancel", "Callback<()>", "Callback when cancel is clicked or dialog is dismissed"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Use the destructive variant for actions that cannot be undone.".to_string(),
                    "Keep the message concise and clear about what will happen.".to_string(),
                    "The dialog closes when clicking the backdrop.".to_string(),
                ]}
            />
        </Container>
    }
}
