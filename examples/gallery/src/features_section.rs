use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FeaturesProps {
    pub features: Vec<&'static str>,
}

#[function_component(Features)]
pub fn features(props: &FeaturesProps) -> Html {
    html! {
        <div class="mt-8">
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">
                { "Required Features" }
            </h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">
                { "This component requires the following feature(s) to be enabled:" }
            </p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                {for props.features.iter().map(|feature| {
                    html! {
                        <li>{ feature }</li>
                    }
                })}
            </ul>
        </div>
    }
}
