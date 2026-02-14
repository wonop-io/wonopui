//! Window Provider component for WonopUI.
//!
//! A context provider that provides access to the window object,
//! useful for iframe scenarios where child components need access to the iframe's window.

use std::rc::Rc;
use web_sys::{HtmlIFrameElement, Window};
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

impl Default for WindowState {
    fn default() -> Self {
        Self::new()
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
    /// Optional iframe ref - if provided, uses the iframe's content window
    #[prop_or_default]
    pub iframe_ref: NodeRef,
}

#[function_component(WindowProvider)]
pub fn window_provider(props: &WindowProviderProps) -> Html {
    let window_context = use_reducer(WindowState::new);

    {
        let window_context = window_context.clone();
        let iframe_ref = props.iframe_ref.clone();
        use_effect_with(iframe_ref.clone(), move |iframe_ref| {
            // Try to get window from iframe, or fall back to global window
            let window = iframe_ref
                .cast::<HtmlIFrameElement>()
                .and_then(|iframe| iframe.content_window())
                .or_else(web_sys::window);
            window_context.dispatch(WindowAction::SetWindow(window));
            || {}
        });
    }

    html! {
        <ContextProvider<WindowContext> context={window_context}>
            { for props.children.iter() }
        </ContextProvider<WindowContext>>
    }
}

/// Hook to access the window from the WindowProvider context
#[hook]
pub fn use_window_context() -> Option<Window> {
    let window_context = use_context::<WindowContext>();
    window_context.and_then(|ctx| ctx.window.clone())
}
