use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(BreadcrumbDocumentation)]
pub fn breadcrumb_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Breadcrumb Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Breadcrumb component is used to display a navigation trail for users. It helps users understand their location within the hierarchy of a website or application." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <Breadcrumb>
                        <BreadcrumbItem label="Home" />
                        <BreadcrumbItem label="Library" />
                        <BreadcrumbItem label="Data" />
                    </Breadcrumb>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Breadcrumb>
        <BreadcrumbItem label="Home" />
        <BreadcrumbItem label="Library" />
        <BreadcrumbItem label="Data" />
    </Breadcrumb>
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Breadcrumb" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the breadcrumb component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "children: Children - The child elements to be rendered inside the breadcrumb component." }</li>
                <li>{ "separator_icon: Option<Html> - An optional custom separator icon between breadcrumb items." }</li>
            </ul>

            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "BreadcrumbItem" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "An individual item within the breadcrumb." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "label: String - The text to be displayed for the breadcrumb item." }</li>
                <li>{ "href: Option<String> - An optional URL for the breadcrumb item. If provided, the item will be rendered as a link." }</li>
            </ul>
        </div>
    }
}
