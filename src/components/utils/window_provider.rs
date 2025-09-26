use std::rc::Rc;
use web_sys::{HtmlIFrameElement, Window};
use yew::context::ContextProvider;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct WindowState {
    pub window: Option<Window>,
}

impl WindowState {
    pub fn new() -> Self {
        Self { window: None }
    }
}

pub enum WindowAction {
    SetWindow(Option<Window>),
}

impl Reducible for WindowState {
    type Action = WindowAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        match action {
            WindowAction::SetWindow(window) => state.window = window,
        }
        state.into()
    }
}

pub type WindowContext = UseReducerHandle<WindowState>;

#[derive(Properties, PartialEq)]
pub struct WindowProviderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub iframe_ref: NodeRef,
}

#[function_component(WindowProvider)]
pub fn window_provider(props: &WindowProviderProps) -> Html {
    let window_context = use_reducer(WindowState::new);

    {
        let window_context = window_context.clone();
        let iframe_ref = props.iframe_ref.clone();
        use_effect_with((iframe_ref,), move |(iframe_ref,)| {
            // Only access window on client side
            let window = iframe_ref
                .cast::<HtmlIFrameElement>()
                .and_then(|iframe| iframe.content_window())
                .or_else(|| web_sys::window());
            window_context.dispatch(WindowAction::SetWindow(window));
            || {}
        });
    }

    html! {
        <ContextProvider<WindowContext> context={window_context}>
            {props.children.clone()}
        </ContextProvider<WindowContext>>
    }
}

#[hook]
pub fn use_window() -> Option<web_sys::Window> {
    let window_context = use_context::<WindowContext>();
    match window_context {
        Some(context) => context.window.clone(),
        None => None,
    }
}
