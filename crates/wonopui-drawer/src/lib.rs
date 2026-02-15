//! Drawer component for wonopui
//!
//! A slide-out panel that can appear from any edge of the screen.

use std::rc::Rc;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const DRAWER_OVERLAY: &str = "fixed inset-0 z-50 bg-black/50";
    pub const DRAWER_CONTAINER: &str = "fixed z-50 bg-white dark:bg-zinc-900 shadow-xl";
    pub const DRAWER_HEADER: &str = "p-4 border-b border-gray-200 dark:border-zinc-700";
    pub const DRAWER_TITLE: &str = "text-lg font-semibold text-gray-900 dark:text-zinc-100";
    pub const DRAWER_DESCRIPTION: &str = "text-sm text-gray-600 dark:text-zinc-400 mt-1";
    pub const DRAWER_CONTENT: &str = "p-4 flex-1 overflow-auto";
    pub const DRAWER_FOOTER: &str = "p-4 border-t border-gray-200 dark:border-zinc-700";

    // Side-specific classes
    pub const DRAWER_LEFT: &str = "inset-y-0 left-0 w-80 max-w-full";
    pub const DRAWER_RIGHT: &str = "inset-y-0 right-0 w-80 max-w-full";
    pub const DRAWER_TOP: &str = "inset-x-0 top-0 h-80 max-h-full";
    pub const DRAWER_BOTTOM: &str = "inset-x-0 bottom-0 h-80 max-h-full";
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
        <div class="cursor-pointer" {onclick}>
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
        <div class={classes::DRAWER_OVERLAY}>
            // Overlay click closes the drawer
            <div class="absolute inset-0" onclick={close_on_overlay}></div>
            if context.curtain {
                {context.curtain_content.clone()}
            }
            <div class={container_class}>
                { for props.children.iter() }
            </div>
        </div>
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
        <div class={class}>
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
        <h2 class={class}>
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
        <p class={class}>
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
        <div class={class}>
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
        <div class={class}>
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
        <div class="cursor-pointer" {onclick}>
            { for props.children.iter() }
        </div>
    }
}
