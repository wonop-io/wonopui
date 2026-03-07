use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(BadgeDocumentation)]
pub fn badge_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Badge Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Badge component is used to display a small badge with a label. It supports different types of badges such as success, warning, error, info, and default." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <>
                        <Badge label="New" badge_type={BadgeType::Success} />
                        <Badge label="Warning" badge_type={BadgeType::Warning} />
                        <Badge label="Error" badge_type={BadgeType::Error} />
                        <Badge label="Info" badge_type={BadgeType::Info} />
                        <Badge label="Default" badge_type={BadgeType::Default} />
                    </>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Badge label="New" badge_type={BadgeType::Success} />
    <Badge label="Warning" badge_type={BadgeType::Warning} />
    <Badge label="Error" badge_type={BadgeType::Error} />
    <Badge label="Info" badge_type={BadgeType::Info} />
    <Badge label="Default" badge_type={BadgeType::Default} />
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Badge" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the badge component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "label: String - The text to be displayed inside the badge." }</li>
                <li>{ "badge_type: BadgeType - The type of badge to be displayed. It can be one of the following: Success, Warning, Error, Info, Default." }</li>
            </ul>
        </div>
    }
}
