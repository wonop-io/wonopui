use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}
// TODO: Get from brand guide
#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <div class="flex justify-between items-end mb-8">
            <span class="text-2xl font-bold">{&props.title}</span>
            <div class="flex space-x-2">
                { for props.children.iter() }
            </div>
        </div>
    }
}
