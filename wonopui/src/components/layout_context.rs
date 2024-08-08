use std::rc::Rc;
use crate::components::multicol_sidebar::SidebarColumn;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SidebarPosition {
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub struct LayoutState {
    pub mobile_menu_open: bool,
    pub mobile_menu_only: bool,
    pub folded_menu_size: i32,
    pub standard_menu_size: i32,
    pub base_size: i32,
    pub show_topbar: bool,
    pub show_footer: bool,
    pub sidebar_folded: bool,
    pub sidebar_position: SidebarPosition,
    pub extra_sidebar: Option<Html>,
}

impl LayoutState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            mobile_menu_open: false,
            mobile_menu_only: false,
            folded_menu_size: 72,
            standard_menu_size: 288,
            base_size: 288,
            show_topbar: true,
            show_footer: true,
            sidebar_folded: false,
            sidebar_position: SidebarPosition::Left,
            extra_sidebar: None,
        }
    }
}

pub enum LayoutAction {
    SetMobileMenuOpen(bool),
    SetSizeMenuSize(i32), // Corrected type from String to i32
    SetShowTopbar(bool),
    SetShowFooter(bool),
    SetSidebarFolded(bool),
    SetSidebarPosition(SidebarPosition), // Added action for setting sidebar position
    SetExtraSidebar(Option<Html>),
}

impl Reducible for LayoutState {
    type Action = LayoutAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        match action {
            LayoutAction::SetMobileMenuOpen(value) => state.mobile_menu_open = value,
            LayoutAction::SetSizeMenuSize(value) => state.standard_menu_size = value,
            LayoutAction::SetShowTopbar(value) => state.show_topbar = value,
            LayoutAction::SetShowFooter(value) => state.show_footer = value,
            LayoutAction::SetSidebarFolded(value) => state.sidebar_folded = value,
            LayoutAction::SetSidebarPosition(position) => state.sidebar_position = position, // Handling new action
            LayoutAction::SetExtraSidebar(value) => state.extra_sidebar = value,
        }

        state.into()
    }
}

pub type LayoutContext = UseReducerHandle<LayoutState>;
