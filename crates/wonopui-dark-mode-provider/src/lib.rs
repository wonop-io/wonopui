//! Dark mode provider for wonopui
//!
//! Provides context for managing dark/light mode across the application.

use std::rc::Rc;
use wonopui_media_query::use_media_query;
use yew::prelude::*;

/// The color mode setting
#[derive(Clone, PartialEq, Default)]
pub enum ColorMode {
    /// Always use light mode
    Light,
    /// Always use dark mode
    Dark,
    /// Follow system preference
    #[default]
    System,
}

/// Context for dark mode state and control
#[derive(Clone, PartialEq)]
pub struct DarkModeContext {
    /// Current color mode setting
    pub mode: ColorMode,
    /// Whether dark mode is currently active (resolved from mode + system preference)
    pub is_dark: bool,
    /// Callback to change the color mode
    pub set_mode: Callback<ColorMode>,
}

#[derive(Properties, PartialEq)]
pub struct DarkModeProviderProps {
    #[prop_or_default]
    pub children: Children,
    /// Initial color mode (defaults to System)
    #[prop_or_default]
    pub initial_mode: ColorMode,
    /// CSS class to add to body when dark mode is active (defaults to "dark")
    #[prop_or("dark".to_string())]
    pub dark_class: String,
}

#[function_component(DarkModeProvider)]
pub fn dark_mode_provider(props: &DarkModeProviderProps) -> Html {
    let mode = use_state(|| props.initial_mode.clone());
    let system_prefers_dark = use_media_query("(prefers-color-scheme: dark)");

    // Compute whether dark mode is active
    let is_dark = match *mode {
        ColorMode::Light => false,
        ColorMode::Dark => true,
        ColorMode::System => system_prefers_dark,
    };

    // Update body class when dark mode changes
    {
        let is_dark = is_dark;
        let dark_class = props.dark_class.clone();
        use_effect_with((is_dark, dark_class), move |(is_dark, dark_class)| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let class_list = body.class_list();
                        if *is_dark {
                            let _ = class_list.add_1(dark_class);
                        } else {
                            let _ = class_list.remove_1(dark_class);
                        }
                    }
                }
            }
            || {}
        });
    }

    let set_mode = {
        let mode = mode.clone();
        Callback::from(move |new_mode: ColorMode| {
            mode.set(new_mode);
        })
    };

    let context = Rc::new(DarkModeContext {
        mode: (*mode).clone(),
        is_dark,
        set_mode,
    });

    html! {
        <ContextProvider<Rc<DarkModeContext>> context={context}>
            { for props.children.iter() }
        </ContextProvider<Rc<DarkModeContext>>>
    }
}

/// Hook to access the dark mode context
#[hook]
pub fn use_dark_mode() -> Rc<DarkModeContext> {
    use_context::<Rc<DarkModeContext>>().expect("DarkModeContext not found. Wrap your app in DarkModeProvider.")
}

/// Hook to check if dark mode is currently active
#[hook]
pub fn use_is_dark() -> bool {
    use_dark_mode().is_dark
}

/// Hook to toggle between light and dark mode
#[hook]
pub fn use_toggle_dark_mode() -> Callback<()> {
    let ctx = use_dark_mode();
    let set_mode = ctx.set_mode.clone();
    let current_is_dark = ctx.is_dark;
    
    Callback::from(move |_| {
        if current_is_dark {
            set_mode.emit(ColorMode::Light);
        } else {
            set_mode.emit(ColorMode::Dark);
        }
    })
}
