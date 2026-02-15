//! Layout component for wonopui
//!
//! Provides layout context and main layout components for building
//! application shells with sidebars, topbars, and content areas.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const LAYOUT_CONTAINER: &str =
        "h-dvh w-screen flex flex-col bg-white text-black dark:bg-zinc-900 dark:text-zinc-100";
    pub const LAYOUT_CONTENT: &str = "flex-1 overflow-y-auto";
    pub const LAYOUT_CONTENT_HORIZONTAL: &str = "flex-1 flex flex-row overflow-hidden";
    pub const LAYOUT_CONTENT_VERTICAL: &str = "flex-1 flex flex-col overflow-hidden";
}

// Layout Context

#[derive(Clone, PartialEq, Default)]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub struct LayoutState {
    pub mobile_menu_open: bool,
    pub mobile_menu_only: bool,
    pub folded_menu_size: i32,
    pub standard_menu_size: i32,
    pub show_topbar: bool,
    pub show_footer: bool,
    pub sidebar_folded: bool,
    pub sidebar_position: SidebarPosition,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            mobile_menu_open: false,
            mobile_menu_only: false,
            folded_menu_size: 72,
            standard_menu_size: 288,
            show_topbar: true,
            show_footer: true,
            sidebar_folded: false,
            sidebar_position: SidebarPosition::Left,
        }
    }
}

pub enum LayoutAction {
    SetMobileMenuOpen(bool),
    SetMenuSize(i32),
    SetShowTopbar(bool),
    SetShowFooter(bool),
    SetSidebarFolded(bool),
    SetSidebarPosition(SidebarPosition),
    ToggleMobileMenu,
    ToggleSidebarFolded,
}

impl Reducible for LayoutState {
    type Action = LayoutAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        match action {
            LayoutAction::SetMobileMenuOpen(value) => state.mobile_menu_open = value,
            LayoutAction::SetMenuSize(value) => state.standard_menu_size = value,
            LayoutAction::SetShowTopbar(value) => state.show_topbar = value,
            LayoutAction::SetShowFooter(value) => state.show_footer = value,
            LayoutAction::SetSidebarFolded(value) => state.sidebar_folded = value,
            LayoutAction::SetSidebarPosition(position) => state.sidebar_position = position,
            LayoutAction::ToggleMobileMenu => state.mobile_menu_open = !state.mobile_menu_open,
            LayoutAction::ToggleSidebarFolded => state.sidebar_folded = !state.sidebar_folded,
        }
        state.into()
    }
}

pub type LayoutContext = UseReducerHandle<LayoutState>;

/// Hook to access the layout context
#[hook]
pub fn use_layout() -> LayoutContext {
    use_context::<LayoutContext>()
        .expect("LayoutContext not found. Wrap your app in LayoutProvider.")
}

// Layout Provider

#[derive(Properties, PartialEq)]
pub struct LayoutProviderProps {
    #[prop_or_default]
    pub initial_state: LayoutState,
    #[prop_or_default]
    pub children: Children,
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

// Layout Direction

#[derive(Clone, PartialEq, Default)]
pub enum LayoutDirection {
    #[default]
    None,
    Horizontal,
    Vertical,
}

// Layout Component

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub sidebar: Option<Html>,
    #[prop_or_default]
    pub topbar: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub direction: LayoutDirection,
    #[prop_or_default]
    pub class: Classes,
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

    let sidebar_style = if props.sidebar.is_some() && !layout_context.mobile_menu_only {
        sidebar_position_style
    } else {
        String::new()
    };

    let direction_class = match props.direction {
        LayoutDirection::None => classes::LAYOUT_CONTENT,
        LayoutDirection::Horizontal => classes::LAYOUT_CONTENT_HORIZONTAL,
        LayoutDirection::Vertical => classes::LAYOUT_CONTENT_VERTICAL,
    };

    let container_class = merge_classes(&[classes::LAYOUT_CONTAINER, &props.class.to_string()]);

    html! {
        <div>
            { props.sidebar.clone().unwrap_or_default() }
            <div class={container_class} style={sidebar_style}>
                if layout_context.show_topbar {
                    { props.topbar.clone().unwrap_or_default() }
                }
                <div class={direction_class}>
                    { props.children.clone() }
                </div>
                if layout_context.show_footer {
                    { props.footer.clone().unwrap_or_default() }
                }
            </div>
        </div>
    }
}
