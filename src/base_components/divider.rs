use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DividerProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub text: String,
}

#[function_component(Divider)]
pub fn divider(props: &DividerProps) -> Html {
    html! {
        <div class={format!("relative mt-10 {}", props.class)}>
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <hr class="w-full border-t border-gray-200 dark:border-zinc-700" style={props.style.clone()} />
            </div>
            <div class="relative flex justify-center text-sm font-medium leading-6">
                <span class="bg-white dark:bg-zinc-900 px-6 text-gray-900 dark:text-zinc-100">{ &props.text }</span>
            </div>
        </div>
    }
}
