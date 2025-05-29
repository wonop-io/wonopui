#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
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
pub struct DrawerProps {
    pub children: Children,
}

#[function_component(Drawer)]
pub fn drawer<T: Clone + PartialEq + 'static>(props: &DrawerProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let context = use_context::<Rc<DrawerContext<T>>>().expect("no context found");

    let side_class = match context.side {
        DrawerSide::Right => &brandguide.drawer_right,
        DrawerSide::Top => &brandguide.drawer_top,
        DrawerSide::Bottom => &brandguide.drawer_bottom,
        DrawerSide::Left => &brandguide.drawer_left,
    };

    html! {
        <div class={&brandguide.drawer_provider}>
            if context.curtain {
                {context.curtain_content.clone()}
            }
            <div class={classes!(&brandguide.drawer_container, side_class)}>
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={&brandguide.drawer_header}>
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <h2 class={&brandguide.drawer_title}>
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <p class={&brandguide.drawer_description}>
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={&brandguide.drawer_footer}>
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

// Snippets to update brandguide:
// ("drawer_provider".to_string(), "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75".to_string()),
// ("drawer_container".to_string(), "bg-white rounded-lg shadow-lg max-w-md w-full".to_string()),
// ("drawer_header".to_string(), "p-4 border-b border-gray-200".to_string()),
// ("drawer_title".to_string(), "text-lg font-semibold text-gray-900".to_string()),
// ("drawer_description".to_string(), "text-sm text-gray-600".to_string()),
// ("drawer_footer".to_string(), "p-4 border-t border-gray-200".to_string()),
// ("drawer_right".to_string(), "inset-y-0 right-0".to_string()),
// ("drawer_top".to_string(), "inset-x-0 top-0".to_string()),
// ("drawer_bottom".to_string(), "inset-x-0 bottom-0".to_string()),
// ("drawer_left".to_string(), "inset-y-0 left-0".to_string()),
//
// pub drawer_provider: ClassesContainer<T>,
// pub drawer_container: ClassesContainer<T>,
// pub drawer_header: ClassesContainer<T>,
// pub drawer_title: ClassesContainer<T>,
// pub drawer_description: ClassesContainer<T>,
// pub drawer_footer: ClassesContainer<T>,
// pub drawer_right: ClassesContainer<T>,
// pub drawer_top: ClassesContainer<T>,
// pub drawer_bottom: ClassesContainer<T>,
// pub drawer_left: ClassesContainer<T>,
//
// drawer_provider: self.drawer_provider.to_owned(),
// drawer_container: self.drawer_container.to_owned(),
// drawer_header: self.drawer_header.to_owned(),
// drawer_title: self.drawer_title.to_owned(),
// drawer_description: self.drawer_description.to_owned(),
// drawer_footer: self.drawer_footer.to_owned(),
// drawer_right: self.drawer_right.to_owned(),
// drawer_top: self.drawer_top.to_owned(),
// drawer_bottom: self.drawer_bottom.to_owned(),
// drawer_left: self.drawer_left.to_owned(),
//
// drawer_provider: default_config_hm
// .get("drawer_provider")
// .expect("Template parameter missing")
// .clone(),
// drawer_container: default_config_hm
// .get("drawer_container")
// .expect("Template parameter missing")
// .clone(),
// drawer_header: default_config_hm
// .get("drawer_header")
// .expect("Template parameter missing")
// .clone(),
// drawer_title: default_config_hm
// .get("drawer_title")
// .expect("Template parameter missing")
// .clone(),
// drawer_description: default_config_hm
// .get("drawer_description")
// .expect("Template parameter missing")
// .clone(),
// drawer_footer: default_config_hm
// .get("drawer_footer")
// .expect("Template parameter missing")
// .clone(),
// drawer_right: default_config_hm
// .get("drawer_right")
// .expect("Template parameter missing")
// .clone(),
// drawer_top: default_config_hm
// .get("drawer_top")
// .expect("Template parameter missing")
// .clone(),
// drawer_bottom: default_config_hm
// .get("drawer_bottom")
// .expect("Template parameter missing")
// .clone(),
// drawer_left: default_config_hm
// .get("drawer_left")
// .expect("Template parameter missing")
// .clone(),
