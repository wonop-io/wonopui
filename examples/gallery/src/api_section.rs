use wonopui::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ApiSectionProps {
    pub title: &'static str,
    pub description: &'static str,
    pub props: Vec<(&'static str, &'static str, &'static str)>,
    #[prop_or_default]
    pub template_params: Option<Vec<(&'static str, &'static str)>>,
}

#[function_component(ApiSection)]
pub fn api_section(props: &ApiSectionProps) -> Html {
    html! {
        <>
            <h3 class="text-xl font-semibold mt-6 mb-2 text-zinc-900 dark:text-white">
                { props.title }
            </h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">
                { props.description }
            </p>
            {if let Some(template_params) = &props.template_params {
                html! {
                    <div class="mb-4">
                        <h4 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">
                            { "Template Parameters" }
                        </h4>
                        <ul class="list-disc list-inside text-zinc-600 dark:text-zinc-400">
                            {for template_params.iter().map(|(name, description)| {
                                html! {
                                    <li>
                                        <span class="font-semibold">{ name }</span>
                                        {" - "}
                                        { description }
                                    </li>
                                }
                            })}
                        </ul>
                    </div>
                }
            } else {
                html! {}
            }}
            <h4 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">
                { "Properties" }
            </h4>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                {for props.props.iter().map(|(name, type_, description)| {
                    html! {
                        <li>
                            <span class="font-semibold">{ name }</span>
                            {": "}
                            <span class="italic">{ type_ }</span>
                            {" - "}
                            { description }
                        </li>
                    }
                })}
            </ul>
        </>
    }
}
