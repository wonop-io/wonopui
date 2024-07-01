use crate::base_components::container::Container;
use crate::components::layout_context::{LayoutContext, LayoutState, SidebarPosition};
use yew::prelude::*;
// <div class="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-4 border-b border-gray-200 bg-white px-4 shadow-sm sm:gap-x-6 sm:px-6 lg:px-8">

#[derive(Clone, PartialEq)]
pub enum TopbarStyle {
    Sticky,
    Relative,
}

#[derive(Properties, PartialEq)]
pub struct TopbarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(TopbarStyle::Sticky)]
    pub style: TopbarStyle,
}

#[function_component(Topbar)]
pub fn topbar(props: &TopbarProps) -> Html {
    let style = match props.style {
        TopbarStyle::Sticky => "sticky",
        TopbarStyle::Relative => "relative",
    };

    html!(
      <div class={format!("{style} top-0 z-10 flex h-16 border-b border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900")}>
        {for props.children.iter()}
      </div>
    )
}
