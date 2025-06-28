use crate::components::utils::media_query::use_media_query;
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum DarkModeColor {
    Light,
    Dark,
    System,
}

#[derive(Clone, PartialEq)]
pub struct DarkModeContext {
    pub mode: DarkModeColor,
    pub set_mode: Callback<DarkModeColor>,
}

#[derive(Properties, PartialEq)]
pub struct DarkModeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DarkModeProvider)]
pub fn dark_mode_provider(props: &DarkModeProviderProps) -> Html {
    let mode = use_state(|| DarkModeColor::System);
    let mode_preference = use_media_query("(prefers-color-scheme: dark)");

    use_effect_with(
        (mode.clone(), mode_preference.clone()),
        move |(mode, mode_preference)| {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();

            match **mode {
                DarkModeColor::Light => {
                    body.class_list().remove_1("dark").unwrap();
                }
                DarkModeColor::Dark => {
                    body.class_list().add_1("dark").unwrap();
                }
                DarkModeColor::System => {
                    if *mode_preference {
                        body.class_list().add_1("dark").unwrap();
                    } else {
                        body.class_list().remove_1("dark").unwrap();
                    }
                }
            }
        },
    );

    let context = DarkModeContext {
        mode: (*mode).clone(),
        set_mode: {
            let mode = mode.clone();
            Callback::from(move |new_mode| {
                mode.set(new_mode);
            })
        },
    };

    html! {
        <ContextProvider<Rc<DarkModeContext>> context={Rc::new(context)}>
            { for props.children.iter() }
        </ContextProvider<Rc<DarkModeContext>>>
    }
}

#[hook]
pub fn use_dark_mode() -> Rc<DarkModeContext> {
    use_context::<Rc<DarkModeContext>>().expect("DarkModeContext not found")
}
