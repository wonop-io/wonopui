use std::rc::Rc;
use web_sys::js_sys::DataView;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DrawerContext {
    pub is_open: bool,
    pub toggle: Callback<()>,
    pub side: DrawerSide,
}

#[derive(Clone, PartialEq)]
pub enum DrawerSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl Default for DrawerSide {
    fn default() -> Self {
        DrawerSide::Left
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub children: Children,
    #[prop_or_default]
    pub side: DrawerSide,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let is_open = use_state(|| false);
    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let context = Rc::new(DrawerContext {
        is_open: *is_open,
        toggle: toggle.clone(),
        side: props.side.clone(),
    });

    html! {
        <ContextProvider<Rc<DrawerContext>> context={context}>
            { for props.children.iter() }
        </ContextProvider<Rc<DrawerContext>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerTriggerProps {
    pub children: Children,
}

#[function_component(DrawerTrigger)]
pub fn drawer_trigger(props: &DrawerTriggerProps) -> Html {
    let context = use_context::<Rc<DrawerContext>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    html! {
        <div {onclick}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerContentProps {
    pub children: Children,
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProps) -> Html {
    let context = use_context::<Rc<DrawerContext>>().expect("no context found");

    if !context.is_open {
        return html! {};
    }

    let side_class = match context.side {
        DrawerSide::Right => "inset-y-0 right-0",
        DrawerSide::Top => "inset-x-0 top-0",
        DrawerSide::Bottom => "inset-x-0 bottom-0",
        DrawerSide::Left => "inset-y-0 left-0",
    };

    html! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75">
            <div class={format!("absolute bg-white rounded-lg shadow-lg max-w-md w-full {}", side_class)}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerHeaderProps {
    pub children: Children,
}

#[function_component(DrawerHeader)]
pub fn drawer_header(props: &DrawerHeaderProps) -> Html {
    html! {
        <div class="p-4 border-b border-gray-200">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerTitleProps {
    pub children: Children,
}

#[function_component(DrawerTitle)]
pub fn drawer_title(props: &DrawerTitleProps) -> Html {
    html! {
        <h2 class="text-lg font-semibold text-gray-900">
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerDescriptionProps {
    pub children: Children,
}

#[function_component(DrawerDescription)]
pub fn drawer_description(props: &DrawerDescriptionProps) -> Html {
    html! {
        <p class="text-sm text-gray-600">
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerFooterProps {
    pub children: Children,
}

#[function_component(DrawerFooter)]
pub fn drawer_footer(props: &DrawerFooterProps) -> Html {
    html! {
        <div class="p-4 border-t border-gray-200">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerCloseProps {
    pub children: Children,
}

#[function_component(DrawerClose)]
pub fn drawer_close(props: &DrawerCloseProps) -> Html {
    let context = use_context::<Rc<DrawerContext>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    html! {
        <div {onclick}>
            { for props.children.iter() }
        </div>
    }
}

// New entries in the brand guide (to be added in config.rs):
// drawer_container: "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75",
// drawer_content: "bg-white rounded-lg shadow-lg max-w-md w-full",
// drawer_header: "p-4 border-b border-gray-200",
// drawer_title: "text-lg font-semibold text-gray-900",
// drawer_description: "text-sm text-gray-600",
// drawer_footer: "p-4 border-t border-gray-200"
