use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaceholderScreenProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("Content placeholder".to_string())]
    pub text: String,
}

// TODO: Move classes to brandguide
#[function_component(Placeholder)]
pub fn placeholder(props: &PlaceholderProps) -> Html {
    html! {
        <div class={classes!("relative","overflow-hidden","rounded-md","border","w-full", "h-[100%]","text-zinc-7000","dark:text-zinc-300","dark:border-zinc-300","flex","justify-center","items-center", props.class.clone())}>
            <svg class="absolute inset-0 h-full w-full stroke-gray-900/10 dark:stroke-zinc-200/10" fill="none">
            <defs>
                <pattern id="dash" width="10" height="10" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
                    <line x1="0" y1="0" x2="0" y2="10" />
                </pattern>
            </defs>
            <rect stroke="none" fill="url(#dash)" width="100%" height="100%"></rect>
            </svg>
            <p class="p-2 z-10 bg-white dark:bg-zinc-800 rounded-md">{&props.text}</p>
        </div>
    }
}