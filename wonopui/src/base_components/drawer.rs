use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DrawerContext<T: Clone + PartialEq + 'static> {
    pub is_open: bool,
    pub toggle: Callback<Option<T>>,
    pub open_drawer: Option<T>,
    pub side: DrawerSide,
    pub curtain: bool,
    pub curtain_content: Html,
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
pub struct DrawerProps<T: Clone + PartialEq + 'static> {
    pub children: Children,
    #[prop_or_default]
    pub side: DrawerSide,
    pub render: Callback<T, Html>,
    #[prop_or_default]
    pub curtain: bool,
    #[prop_or_default]
    pub curtain_content: Html,
}

#[function_component(Drawer)]
pub fn drawer<T: Clone + PartialEq + 'static>(props: &DrawerProps<T>) -> Html {
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

    html! {
        <ContextProvider<Rc<DrawerContext<T>>> context={context}>
            { for props.children.iter() }
            if *is_open {
                {props.render.emit((*open_drawer).clone().unwrap())}
            }
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
pub fn drawer_content<T: Clone + PartialEq + 'static>(props: &DrawerContentProps) -> Html {
    let context = use_context::<Rc<DrawerContext<T>>>().expect("no context found");

    let side_class = match context.side {
        DrawerSide::Right => "inset-y-0 right-0",
        DrawerSide::Top => "inset-x-0 top-0",
        DrawerSide::Bottom => "inset-x-0 bottom-0",
        DrawerSide::Left => "inset-y-0 left-0",
    };

    if context.curtain {
        html! {
            <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75">
                {context.curtain_content.clone()}
                <div class={format!("absolute bg-white rounded-lg shadow-lg max-w-md w-full {}", side_class)}>
                    { for props.children.iter() }
                </div>
            </div>
        }
    } else {
        html! {
            <div class={format!("absolute bg-white rounded-lg shadow-lg max-w-md w-full {}", side_class)}>
                { for props.children.iter() }
            </div>
        }        
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
pub fn drawer_close<T: Clone + PartialEq + 'static>(props: &DrawerCloseProps) -> Html {
    let context = use_context::<Rc<DrawerContext<T>>>().expect("no context found");

    let onclick = {
        let toggle = context.toggle.clone();
        Callback::from(move |_| toggle.emit(None))
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
