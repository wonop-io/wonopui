use crate::components::layout::layout_context::{LayoutAction, LayoutContext, SidebarPosition};
use yew::prelude::*; // Import LayoutContext and SidebarPosition

#[derive(Properties, PartialEq)]
pub struct SidebarColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub width: Option<i32>,
    #[prop_or_default]
    pub hide_when_folded: bool,
    #[prop_or_default]
    pub header: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SidebarColumn)]
pub fn sidebar_column(props: &SidebarColumnProps) -> Html {
    let SidebarColumnProps {
        children,
        header,
        footer,
        width,
        hide_when_folded,
        class,
    } = props;

    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;
    if folded && *hide_when_folded {
        return html! { <></> };
    }
    let (style_width, grow_class) = if width.is_some() {
        (format!("width: {}px;", width.unwrap()), "".to_string())
    } else {
        ("".to_string(), "flex-grow flex-1".to_string())
    };

    let sidebar_border_class = match layout_context.sidebar_position {
        SidebarPosition::Left => "border-r",
        SidebarPosition::Right => "border-l",
    };

    html! {
        <div class={classes!("flex","flex-col","h-full","w-full","border-zinc-200", "dark:border-zinc-800",  grow_class, sidebar_border_class, class.clone())} style={style_width}>
            if let Some(header_content) = header {
                {header_content.clone()}
            }
            <div class="flex-1 overflow-y-auto h-full">
                {children}
            </div>
            if let Some(footer_content) = footer {
                {footer_content.clone()}
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiColumnSidebarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub curtain_content: Html,
}

#[function_component(MultiColumnSidebar)]
pub fn multi_column_sidebar(props: &MultiColumnSidebarProps) -> Html {
    let MultiColumnSidebarProps {
        children,
        curtain_content,
    } = props;
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let folded = layout_context.sidebar_folded;

    {
        let count = if layout_context.extra_sidebar.is_some() {
            props.children.len() + 1
        } else {
            props.children.len()
        };
        let full_size = layout_context.base_size * (count as i32);
        let layout_context = layout_context.clone();
        use_effect_with((full_size,), move |(full_size,)| {
            layout_context.dispatch(LayoutAction::SetSizeMenuSize(*full_size));
        });
    }

    let size = if folded {
        layout_context.folded_menu_size
    } else {
        layout_context.standard_menu_size
    };
    let total_width = size;
    let sidebar_style = format!("width: {}px;", total_width);
    let (order, curtain_style) = match layout_context.sidebar_position {
        SidebarPosition::Left => ("", format!("padding-left: {}px;", total_width)),
        SidebarPosition::Right => (
            "flex-row-reverse",
            format!("padding-right: {}px;", total_width),
        ),
    };

    let show_mobile_class = if layout_context.mobile_menu_open {
        "block z-50"
    } else {
        "hidden"
    };

    let sidebar_position_class = match layout_context.sidebar_position {
        SidebarPosition::Left => "left-0",
        SidebarPosition::Right => "right-0",
    };

    let mobile_menu_only_class = if layout_context.mobile_menu_only {
        "lg:hidden"
    } else {
        "lg:flex"
    };

    html! {
      <>
         <div key="curtain" class={classes!(show_mobile_class, "inset-0", "fixed", "lg:hidden")} style={curtain_style}>
            <div class="absolute w-full h-full inset-0 opacity-50 bg-black">
            </div>
            <div class="w-full h-full z-10 relative">
               {curtain_content.clone()}
            </div>
         </div>
         <div key="sidebar" class={classes!(order, mobile_menu_only_class, show_mobile_class,  sidebar_position_class,"h-dvh", "inset-y-0", "absolute",  "bg-white",  "text-zinc-700", "dark:bg-zinc-900", "dark:text-zinc-300")} style={sidebar_style}>
            {for children.iter().map(|child| html! {
                {child}
            })}
            if let Some(extra_sidebar) = &layout_context.extra_sidebar {
                {extra_sidebar.clone()}
            }
         </div>
       </>
    }
}
