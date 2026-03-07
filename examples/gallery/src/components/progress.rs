use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(ProgressDocumentation)]
pub fn progress_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Progress Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "A progress bar component for displaying completion status, with optional labels and multiple color variants." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Basic Usage" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="w-full max-w-md space-y-4">
                        <Progress value={25.0} />
                        <Progress value={50.0} />
                        <Progress value={75.0} />
                        <Progress value={100.0} />
                    </div>
                }}
                code={r#"
<Progress value={25.0} />
<Progress value={50.0} />
<Progress value={75.0} />
<Progress value={100.0} />"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "With Labels" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="w-full max-w-md space-y-4">
                        <Progress value={65.0} label="Upload Progress" show_value={true} />
                        <Progress value={42.0} label="Download" show_value={true} />
                    </div>
                }}
                code={r#"
<Progress value={65.0} label="Upload Progress" show_value={true} />
<Progress value={42.0} label="Download" show_value={true} />"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Variants" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="w-full max-w-md space-y-4">
                        <Progress value={60.0} variant={ProgressVariant::Default} label="Default" show_value={true} />
                        <Progress value={80.0} variant={ProgressVariant::Success} label="Success" show_value={true} />
                        <Progress value={45.0} variant={ProgressVariant::Warning} label="Warning" show_value={true} />
                        <Progress value={30.0} variant={ProgressVariant::Error} label="Error" show_value={true} />
                    </div>
                }}
                code={r#"
<Progress value={60.0} variant={ProgressVariant::Default} />
<Progress value={80.0} variant={ProgressVariant::Success} />
<Progress value={45.0} variant={ProgressVariant::Warning} />
<Progress value={30.0} variant={ProgressVariant::Error} />"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Sizes" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="w-full max-w-md space-y-4">
                        <Progress value={50.0} size={ProgressSize::Sm} />
                        <Progress value={50.0} size={ProgressSize::Md} />
                        <Progress value={50.0} size={ProgressSize::Lg} />
                    </div>
                }}
                code={r#"
<Progress value={50.0} size={ProgressSize::Sm} />
<Progress value={50.0} size={ProgressSize::Md} />
<Progress value={50.0} size={ProgressSize::Lg} />"#.to_string()}
            />

            <Features features={vec![
                "Multiple size options (Sm, Md, Lg)",
                "Four color variants (Default, Success, Warning, Error)",
                "Optional label and value display",
                "Custom value formatting support",
                "Accessible with ARIA progressbar role"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Progress"
                description="Props for the Progress component."
                props={vec![
                    ("value", "f64", "Current progress value"),
                    ("max", "f64", "Maximum value. Default: 100.0"),
                    ("size", "ProgressSize", "Size: Sm, Md, or Lg. Default: Md"),
                    ("variant", "ProgressVariant", "Color: Default, Success, Warning, or Error"),
                    ("label", "Option<String>", "Optional label text"),
                    ("show_value", "bool", "Whether to show the value. Default: false"),
                    ("value_format", "Option<Callback<(f64, f64), String>>", "Custom value format function"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Use Progress for determinate progress (known completion percentage).".to_string(),
                    "For indeterminate loading, use the Spinner component instead.".to_string(),
                    "Color variants can indicate status - green for success, red for errors.".to_string(),
                ]}
            />
        </Container>
    }
}
