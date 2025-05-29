use crate::components::utils::media_query::use_media_query;
use crate::components::layout::layout_context::{LayoutContext, LayoutState, SidebarPosition};
use yew::context::ContextProvider;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum LayoutDirection {
    None,
    Horizontal,
    Vertical,
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub sidebar: Option<Html>, // TODO: Force to be Sidebar
    #[prop_or_default]
    pub topbar: Option<Html>, // TODO: Force to be Sidebar

    #[prop_or_default]
    pub footer: Option<Html>, // TODO: Force to be Sidebar

    #[prop_or_default]
    pub children: Children,

    #[prop_or(LayoutDirection::None)]
    pub direction: LayoutDirection,
}

#[hook]
pub fn use_layout() -> LayoutContext {
    use_context::<LayoutContext>().expect("LayoutContext not found")
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let layout_context = use_layout();
    let sidebar_size = if layout_context.sidebar_folded {
        layout_context.folded_menu_size
    } else {
        layout_context.standard_menu_size
    };

    let sidebar_position_style = match layout_context.sidebar_position {
        SidebarPosition::Left => format!("padding-left: {}px;", sidebar_size),
        SidebarPosition::Right => format!("padding-right: {}px;", sidebar_size),
    };

    let mobile_menu_only = layout_context.mobile_menu_only;
    let sidebar_style = if props.sidebar.is_some() && !mobile_menu_only {
        sidebar_position_style
    } else {
        String::new()
    };

    let is_large_screen = use_media_query("(min-width: 1024px)");

    let direction = match props.direction {
        LayoutDirection::None => classes!("overflow-y-auto"),
        LayoutDirection::Horizontal => classes!("flex", "flex-row", "overflow-hidden"),
        LayoutDirection::Vertical => classes!("flex", "flex-col", "overflow-hidden"),
    };

    html! {
      <div>
      {props.sidebar.clone().unwrap_or_default()}

      // Keep this comment to ensure that lg:pl-72 lg:pr-72 lg:pl-20 lg:pr-20 is added to the final CSS. Don't comma separate
      <div class="h-dvh w-screen flex flex-col bg-white text-black dark:bg-zinc-900 dark:text-zinc-100" style={if is_large_screen { sidebar_style.clone() } else { String::new() }}>
          if layout_context.show_topbar {
              {props.topbar.clone().unwrap_or_default()}
          }
          <div class={classes!("flex-1", direction)}>
            {props.children.clone()}
          </div>
          if layout_context.show_footer {
              {props.footer.clone().unwrap_or_default()}
          }
      </div>
    </div>
    }
}

#[function_component(LayoutProvider)]
pub fn layout_provider(props: &LayoutProviderProps) -> Html {
    let initial_state = props.initial_state.clone();
    let layout_context = use_reducer(|| initial_state);

    html! {
        <ContextProvider<LayoutContext> context={layout_context}>
            {props.children.clone()}
        </ContextProvider<LayoutContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProviderProps {
    #[prop_or_default]
    pub initial_state: LayoutState,
    #[prop_or_default]
    pub children: Children,
}