use std::rc::Rc;

use crate::base_components::use_media_query;
use crate::components::layout_context::{LayoutContext, LayoutState, SidebarPosition};
use gloo_console as console;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::context::ContextProvider;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub sidebar: Option<Html>, // TODO: Force to be Sidebar
    #[prop_or_default]
    pub topbar: Option<Html>, // TODO: Force to be Sidebar

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    //let window = use_window();
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
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

    html! {
      <div>
      {props.sidebar.clone().unwrap_or_default()}

      // Keep this comment to ensure that lg:pl-72 lg:pr-72 lg:pl-20 lg:pr-20 is added to the final CSS. Don't comma separate
      <div class="h-screen w-screen flex flex-col" style={if is_large_screen { sidebar_style.clone() } else { String::new() }}>
          if layout_context.show_topbar {
              {props.topbar.clone().unwrap_or_default()}
          }
          <div class="flex-1 overflow-y-auto">
            {props.children.clone()}
          </div>
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