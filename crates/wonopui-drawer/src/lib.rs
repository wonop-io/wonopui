//! Drawer component for wonopui
//!
//! A slide-out panel that can appear from any edge of the screen.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for drawer styling (shadcn v4 style).
pub mod classes {
    /// Overlay styles - shadcn v4 drawer/sheet overlay with animations.
    pub const DRAWER_OVERLAY: &str = "fixed inset-0 z-50 bg-black/50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0";
    
    /// Container/Content styles - shadcn v4 drawer content base.
    pub const DRAWER_CONTAINER: &str = "group/drawer-content fixed z-50 flex h-auto flex-col gap-4 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 p-6 shadow-lg outline-none";
    
    /// Header styles - shadcn v4 drawer header.
    pub const DRAWER_HEADER: &str = "flex flex-col gap-1.5 text-center sm:text-left";
    
    /// Title styles - shadcn v4 drawer title.
    pub const DRAWER_TITLE: &str = "font-semibold text-zinc-950 dark:text-zinc-50";
    
    /// Description styles - shadcn v4 drawer description.
    pub const DRAWER_DESCRIPTION: &str = "text-sm text-zinc-500 dark:text-zinc-400";
    
    /// Content area styles.
    pub const DRAWER_CONTENT: &str = "flex-1 overflow-auto px-0";
    
    /// Footer styles - shadcn v4 drawer footer.
    pub const DRAWER_FOOTER: &str = "mt-auto flex flex-col gap-2";
    
    /// Handle/grabber indicator for bottom drawer.
    pub const DRAWER_HANDLE: &str = "mx-auto h-2 w-[100px] shrink-0 rounded-full bg-zinc-200 dark:bg-zinc-700";

    // Side-specific classes - shadcn v4 drawer sides with rounded corners
    /// Left side drawer.
    pub const DRAWER_LEFT: &str = "inset-y-0 left-0 h-full w-3/4 border-r border-zinc-200 dark:border-zinc-800 sm:max-w-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left";
    
    /// Right side drawer.
    pub const DRAWER_RIGHT: &str = "inset-y-0 right-0 h-full w-3/4 border-l border-zinc-200 dark:border-zinc-800 sm:max-w-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right";
    
    /// Top side drawer.
    pub const DRAWER_TOP: &str = "inset-x-0 top-0 max-h-[80vh] rounded-b-lg border-b border-zinc-200 dark:border-zinc-800 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top";
    
    /// Bottom side drawer.
    pub const DRAWER_BOTTOM: &str = "inset-x-0 bottom-0 max-h-[80vh] rounded-t-lg border-t border-zinc-200 dark:border-zinc-800 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom";
    
    /// Close button styles - shadcn v4 style.
    pub const DRAWER_CLOSE: &str = "absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:outline-none focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:pointer-events-none text-zinc-500 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
}

#[derive(Clone, PartialEq, Default)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

impl DrawerSide {
    pub fn to_class(&self) -> &'static str {
        match self {
            DrawerSide::Left => classes::DRAWER_LEFT,
            DrawerSide::Right => classes::DRAWER_RIGHT,
            DrawerSide::Top => classes::DRAWER_TOP,
            DrawerSide::Bottom => classes::DRAWER_BOTTOM,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawerContext<T: Clone + PartialEq + 'static> {
    pub is_open: bool,
    pub toggle: Callback<Option<T>>,
    pub open_drawer: Option<T>,
    pub side: DrawerSide,
    pub curtain: bool,
    pub curtain_content: Html,
}

#[derive(Properties, PartialEq)]
pub struct DrawerProviderProps<T: Clone + PartialEq + 'static> {
    pub children: Children,
    #[prop_or_default]
    pub side: DrawerSide,
    pub render: Callback<T, Html>,
    #[prop_or_default]
    pub curtain: bool,
    #[prop_or_default]
    pub curtain_content: Html,
}

#[function_component(DrawerProvider)]
pub fn drawer_provider<T: Clone + PartialEq + 'static>(props: &DrawerProviderProps<T>) -> Html {
    let is_open = use_state(|| false);
    let open_drawer = use_state(|| None);

    let toggle = {
        let is_open = is_open.clone();
        let open_drawer = open_drawer.clone();
        Callback::from(move |drawer: Option<T>| {
            if let Some(d) = drawer {
                open_drawer.set(Some(d));
                is_open.set(true);
            } else {
                open_drawer.set(None);
                is_open.set(false);
            }
        })
    };

    let context = Rc::new(DrawerContext {
        is_open: *is_open,
        toggle: toggle.clone(),
        open_drawer: (*open_drawer).clone(),
        side: props.side.clone(),
        curtain: props.curtain,
        curtain_content: props.curtain_content.clone(),
    });

    let is_open_html = if *is_open {
        props.render.emit((*open_drawer).clone().unwrap())
    } else {
        html! {}
    };

    html! {
        <ContextProvider<Rc<DrawerContext<T>>> context={context}>
            { for props.children.iter() }
            {is_open_html}
        </ContextProvider<Rc<DrawerContext<T>>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerTriggerProps<T: Clone + PartialEq + 'static> {
    pub children: Children,
    pub drawer: T,
}

#[function_component(DrawerTrigger)]
pub fn drawer_trigger<T: Clone + PartialEq + 'static>(props: &DrawerTriggerProps<T>) -> Html {
    let context = use_context::<Rc<DrawerContext<T>>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        let drawer = props.drawer.clone();
        Callback::from(move |_| toggle.emit(Some(drawer.clone())))
    };

    html! {
        <div data-slot="drawer-trigger" class="cursor-pointer" {onclick}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Drawer)]
pub fn drawer<T: Clone + PartialEq + 'static>(props: &DrawerProps) -> Html {
    let context = use_context::<Rc<DrawerContext<T>>>().expect("no context found");

    let side_class = context.side.to_class();
    let is_bottom = matches!(context.side, DrawerSide::Bottom);

    let close_on_overlay = {
        let toggle = context.toggle.clone();
        Callback::from(move |_| toggle.emit(None))
    };

    let container_class = merge_classes(&[
        classes::DRAWER_CONTAINER,
        side_class,
        &props.class.to_string(),
    ]);

    html! {
        <>
            <div data-slot="drawer-overlay" data-state="open" class={classes::DRAWER_OVERLAY} onclick={close_on_overlay.clone()}></div>
            <div data-slot="drawer-portal">
                if context.curtain {
                    {context.curtain_content.clone()}
                }
                <div data-slot="drawer-content" data-state="open" class={container_class}>
                    if is_bottom {
                        <div class={classes::DRAWER_HANDLE}></div>
                    }
                    { for props.children.iter() }
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerHeaderProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DrawerHeader)]
pub fn drawer_header(props: &DrawerHeaderProps) -> Html {
    let class = merge_classes(&[classes::DRAWER_HEADER, &props.class.to_string()]);

    html! {
        <div data-slot="drawer-header" class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerTitleProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DrawerTitle)]
pub fn drawer_title(props: &DrawerTitleProps) -> Html {
    let class = merge_classes(&[classes::DRAWER_TITLE, &props.class.to_string()]);

    html! {
        <h2 data-slot="drawer-title" class={class}>
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerDescriptionProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DrawerDescription)]
pub fn drawer_description(props: &DrawerDescriptionProps) -> Html {
    let class = merge_classes(&[classes::DRAWER_DESCRIPTION, &props.class.to_string()]);

    html! {
        <p data-slot="drawer-description" class={class}>
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerContentProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProps) -> Html {
    let class = merge_classes(&[classes::DRAWER_CONTENT, &props.class.to_string()]);

    html! {
        <div data-slot="drawer-body" class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerFooterProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DrawerFooter)]
pub fn drawer_footer(props: &DrawerFooterProps) -> Html {
    let class = merge_classes(&[classes::DRAWER_FOOTER, &props.class.to_string()]);

    html! {
        <div data-slot="drawer-footer" class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerCloseProps {
    pub children: Children,
}

#[function_component(DrawerClose)]
pub fn drawer_close<T: Clone + PartialEq + 'static>(props: &DrawerCloseProps) -> Html {
    let context = use_context::<Rc<DrawerContext<T>>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        Callback::from(move |_| toggle.emit(None))
    };

    html! {
        <button data-slot="drawer-close" type="button" class={classes::DRAWER_CLOSE} {onclick}>
            { for props.children.iter() }
        </button>
    }
}
