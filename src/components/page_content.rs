use crate::base_components::MainContent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PageContentProps {
    pub children: Children,
}

#[function_component(PageContent)]
pub fn page_content(props: &PageContentProps) -> Html {
    html! {
        <MainContent class="bg-white dark:bg-zinc-900 overflow-y-auto h-full  min-h-full">
            {props.children.clone()}
        </MainContent>
    }
}
