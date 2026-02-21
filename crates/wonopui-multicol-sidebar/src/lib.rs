//! Multi-Column Sidebar component for WonopUI.
//!
//! A sidebar component that supports multiple columns with collapsible behavior.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the MultiColumnSidebar component
pub mod classes {
    pub const SIDEBAR: &str =
        "h-dvh inset-y-0 absolute bg-white text-zinc-700 dark:bg-zinc-900 dark:text-zinc-300";
    pub const SIDEBAR_LEFT: &str = "left-0";
    pub const SIDEBAR_RIGHT: &str = "right-0";
    pub const CURTAIN: &str = "inset-0 fixed lg:hidden";
    pub const CURTAIN_BACKDROP: &str = "absolute w-full h-full inset-0 opacity-50 bg-black";
    pub const CURTAIN_CONTENT: &str = "w-full h-full z-10 relative";
    pub const COLUMN: &str = "flex flex-col h-full w-full border-zinc-200 dark:border-zinc-800";
    pub const COLUMN_LEFT_BORDER: &str = "border-r";
    pub const COLUMN_RIGHT_BORDER: &str = "border-l";
    pub const COLUMN_CONTENT: &str = "flex-1 overflow-y-auto h-full";
}

#[derive(Clone, PartialEq, Default)]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
}

#[derive(Properties, PartialEq)]
pub struct SidebarColumnProps {
    #[prop_or_default]
    pub children: Children,
    /// Fixed width in pixels
    #[prop_or_default]
    pub width: Option<i32>,
    /// Hide this column when sidebar is folded
    #[prop_or_default]
    pub hide_when_folded: bool,
    #[prop_or_default]
    pub header: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
    /// Current folded state (passed down from parent)
    #[prop_or_default]
    pub folded: bool,
    /// Position for border styling
    #[prop_or_default]
    pub position: SidebarPosition,
}

#[function_component(SidebarColumn)]
pub fn sidebar_column(props: &SidebarColumnProps) -> Html {
    if props.folded && props.hide_when_folded {
        return html! {};
    }

    let (style_width, grow_class) = if let Some(w) = props.width {
        (format!("width: {}px;", w), "")
    } else {
        (String::new(), "flex-grow flex-1")
    };

    let border_class = match props.position {
        SidebarPosition::Left => classes::COLUMN_LEFT_BORDER,
        SidebarPosition::Right => classes::COLUMN_RIGHT_BORDER,
    };

    let column_class = merge_classes(&[
        classes::COLUMN,
        grow_class,
        border_class,
        &props.class.to_string(),
    ]);

    html! {
        <div class={column_class} style={style_width}>
            if let Some(header) = &props.header {
                { header.clone() }
            }
            <div class={classes::COLUMN_CONTENT}>
                { for props.children.iter() }
            </div>
            if let Some(footer) = &props.footer {
                { footer.clone() }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiColumnSidebarProps {
    #[prop_or_default]
    pub children: Children,
    /// Content to show behind the mobile curtain overlay
    #[prop_or_default]
    pub curtain_content: Html,
    /// Total width when expanded
    #[prop_or(280)]
    pub width: i32,
    /// Total width when folded
    #[prop_or(64)]
    pub folded_width: i32,
    /// Whether the sidebar is folded
    #[prop_or_default]
    pub folded: bool,
    /// Whether mobile menu is open
    #[prop_or_default]
    pub mobile_open: bool,
    /// Position of the sidebar
    #[prop_or_default]
    pub position: SidebarPosition,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(MultiColumnSidebar)]
pub fn multi_column_sidebar(props: &MultiColumnSidebarProps) -> Html {
    let total_width = if props.folded {
        props.folded_width
    } else {
        props.width
    };

    let sidebar_style = format!("width: {}px;", total_width);

    let (order_class, curtain_style) = match props.position {
        SidebarPosition::Left => ("", format!("padding-left: {}px;", total_width)),
        SidebarPosition::Right => (
            "flex-row-reverse",
            format!("padding-right: {}px;", total_width),
        ),
    };

    let show_mobile_class = if props.mobile_open {
        "block z-50"
    } else {
        "hidden"
    };

    let position_class = match props.position {
        SidebarPosition::Left => classes::SIDEBAR_LEFT,
        SidebarPosition::Right => classes::SIDEBAR_RIGHT,
    };

    let sidebar_class = merge_classes(&[
        order_class,
        "lg:flex",
        show_mobile_class,
        position_class,
        classes::SIDEBAR,
        &props.class.to_string(),
    ]);

    let curtain_class = merge_classes(&[show_mobile_class, classes::CURTAIN]);

    html! {
        <>
            // Mobile curtain/backdrop
            <div class={curtain_class} style={curtain_style}>
                <div class={classes::CURTAIN_BACKDROP} />
                <div class={classes::CURTAIN_CONTENT}>
                    { props.curtain_content.clone() }
                </div>
            </div>

            // Sidebar
            <div class={sidebar_class} style={sidebar_style}>
                { for props.children.iter() }
            </div>
        </>
    }
}
