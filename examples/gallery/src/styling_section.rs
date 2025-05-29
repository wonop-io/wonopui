use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StylingSectionProps {
    pub component_name: String,
    pub class_descriptions: Vec<(String, String)>,
}

#[function_component(StylingSection)]
pub fn styling_section(props: &StylingSectionProps) -> Html {
    html! {
        <div class="mt-8">
            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Styling" }</h2>
            <p class="mb-4 text-zinc-600 dark:text-zinc-400">
                { format!("The {} component uses classes defined in the BRANDGUIDE for consistent styling. These classes can be customized to match your application's design system.", props.component_name) }
            </p>
            <h3 class="text-xl font-semibold mt-4 mb-2 text-zinc-900 dark:text-white">{ "Key Classes" }</h3>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                {for props.class_descriptions.iter().map(|(class_name, description)| {
                    html! {
                        <li>
                            <span class="font-semibold">{ class_name }</span>
                            {": "}
                            { description }
                        </li>
                    }
                })}
            </ul>
            <p class="mt-4 text-zinc-600 dark:text-zinc-400">
                { format!("To customize the appearance of the {} component, you can override these classes in your CSS or use a styling solution that supports class overrides.", props.component_name) }
            </p>
        </div>
    }
}
