//! Browser Provider component for WonopUI.
//!
//! A context provider that provides safe access to browser APIs.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Browser context that provides safe access to browser APIs
#[derive(Clone, Debug, PartialEq, Default)]
pub struct BrowserContext {
    pub window: Option<web_sys::Window>,
    pub document: Option<web_sys::Document>,
    pub navigator: Option<web_sys::Navigator>,
    pub location: Option<web_sys::Location>,
    pub local_storage: Option<web_sys::Storage>,
    pub session_storage: Option<web_sys::Storage>,
}

impl BrowserContext {
    /// Initialize browser context (only works in browser environment)
    pub fn initialize() -> Self {
        if let Some(window) = web_sys::window() {
            let document = window.document();
            let navigator = Some(window.navigator());
            let location = Some(window.location());
            let local_storage = window.local_storage().ok().flatten();
            let session_storage = window.session_storage().ok().flatten();

            Self {
                window: Some(window),
                document,
                navigator,
                location,
                local_storage,
                session_storage,
            }
        } else {
            Self::default()
        }
    }
}

/// Properties for the BrowserProvider component
#[derive(Properties, PartialEq)]
pub struct BrowserProviderProps {
    pub children: Children,
}

/// Browser provider component that provides browser APIs to child components
#[function_component(BrowserProvider)]
pub fn browser_provider(props: &BrowserProviderProps) -> Html {
    let browser_ctx = use_state(BrowserContext::default);

    // Initialize browser context after mount
    {
        let browser_ctx = browser_ctx.clone();
        use_effect_with((), move |_| {
            browser_ctx.set(BrowserContext::initialize());
            || ()
        });
    }

    html! {
        <ContextProvider<BrowserContext> context={(*browser_ctx).clone()}>
            { for props.children.iter() }
        </ContextProvider<BrowserContext>>
    }
}

/// Hook to access the window object
#[hook]
pub fn use_window() -> Option<web_sys::Window> {
    let ctx = use_context::<BrowserContext>();
    ctx.and_then(|c| c.window.clone())
}

/// Hook to access the document object
#[hook]
pub fn use_document() -> Option<web_sys::Document> {
    let ctx = use_context::<BrowserContext>();
    ctx.and_then(|c| c.document.clone())
}

/// Hook to access the navigator object
#[hook]
pub fn use_window_navigator() -> Option<web_sys::Navigator> {
    let ctx = use_context::<BrowserContext>();
    ctx.and_then(|c| c.navigator.clone())
}

/// Hook to access the location object
#[hook]
pub fn use_location() -> Option<web_sys::Location> {
    let ctx = use_context::<BrowserContext>();
    ctx.and_then(|c| c.location.clone())
}

/// Hook to access localStorage
#[hook]
pub fn use_local_storage() -> Option<web_sys::Storage> {
    let ctx = use_context::<BrowserContext>();
    ctx.and_then(|c| c.local_storage.clone())
}

/// Hook to access sessionStorage
#[hook]
pub fn use_session_storage() -> Option<web_sys::Storage> {
    let ctx = use_context::<BrowserContext>();
    ctx.and_then(|c| c.session_storage.clone())
}

/// Hook for navigation operations
#[hook]
pub fn use_navigation() -> NavigationOps {
    let location = use_location();
    NavigationOps { location }
}

pub struct NavigationOps {
    location: Option<web_sys::Location>,
}

impl NavigationOps {
    /// Navigate to a URL
    pub fn navigate_to(&self, url: &str) {
        if let Some(location) = &self.location {
            let _ = location.set_href(url);
        }
    }

    /// Get the current URL
    pub fn current_url(&self) -> Option<String> {
        self.location.as_ref().and_then(|l| l.href().ok())
    }

    /// Get the current pathname
    pub fn pathname(&self) -> Option<String> {
        self.location.as_ref().and_then(|l| l.pathname().ok())
    }

    /// Get the current search params
    pub fn search(&self) -> Option<String> {
        self.location.as_ref().and_then(|l| l.search().ok())
    }

    /// Get the current hash
    pub fn hash(&self) -> Option<String> {
        self.location.as_ref().and_then(|l| l.hash().ok())
    }

    /// Reload the page
    pub fn reload(&self) {
        if let Some(location) = &self.location {
            let _ = location.reload();
        }
    }
}

/// Hook for clipboard operations
#[hook]
pub fn use_clipboard() -> ClipboardOps {
    let navigator = use_window_navigator();
    ClipboardOps { navigator }
}

#[derive(Clone)]
pub struct ClipboardOps {
    navigator: Option<web_sys::Navigator>,
}

impl ClipboardOps {
    /// Copy text to clipboard (returns a promise-like Future)
    pub fn copy_text(&self, text: &str) -> Option<js_sys::Promise> {
        self.navigator
            .as_ref()
            .map(|nav| nav.clipboard().write_text(text))
    }

    /// Read text from clipboard (returns a promise-like Future)
    pub fn read_text(&self) -> Option<js_sys::Promise> {
        self.navigator
            .as_ref()
            .map(|nav| nav.clipboard().read_text())
    }
}

/// Hook for viewport/window dimensions
#[hook]
pub fn use_viewport() -> ViewportInfo {
    let window = use_window();
    let dimensions = use_state(|| (0.0_f64, 0.0_f64));

    // Update dimensions on mount and resize
    {
        let window = window.clone();
        let dimensions = dimensions.clone();
        use_effect_with(window.clone(), move |window| {
            if let Some(window) = window {
                // Get initial dimensions
                if let (Ok(width), Ok(height)) = (window.inner_width(), window.inner_height()) {
                    if let (Some(w), Some(h)) = (width.as_f64(), height.as_f64()) {
                        dimensions.set((w, h));
                    }
                }

                // Set up resize listener
                let dimensions = dimensions.clone();
                let window_clone = window.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let (Ok(width), Ok(height)) =
                        (window_clone.inner_width(), window_clone.inner_height())
                    {
                        if let (Some(w), Some(h)) = (width.as_f64(), height.as_f64()) {
                            dimensions.set((w, h));
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = window
                    .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                closure.forget();
            }
            || ()
        });
    }

    ViewportInfo {
        width: dimensions.0,
        height: dimensions.1,
        window,
    }
}

pub struct ViewportInfo {
    pub width: f64,
    pub height: f64,
    window: Option<web_sys::Window>,
}

impl ViewportInfo {
    /// Scroll to top of page
    pub fn scroll_to_top(&self) {
        if let Some(window) = &self.window {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    }

    /// Scroll to specific position
    pub fn scroll_to(&self, x: f64, y: f64) {
        if let Some(window) = &self.window {
            window.scroll_to_with_x_and_y(x, y);
        }
    }

    /// Get current scroll position
    pub fn scroll_position(&self) -> (f64, f64) {
        if let Some(window) = &self.window {
            let x = window.scroll_x().unwrap_or(0.0);
            let y = window.scroll_y().unwrap_or(0.0);
            (x, y)
        } else {
            (0.0, 0.0)
        }
    }
}
