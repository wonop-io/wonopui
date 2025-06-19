#[cfg(not(feature = "ssr"))]
use crate::components::utils::window_provider::use_window;
#[cfg(not(feature = "ssr"))]
use gloo::events::EventListener;
use yew::prelude::*;

#[hook]
#[cfg(not(feature = "ssr"))]
pub fn use_media_query(query: &str) -> bool {
    let window = use_window();
    let query = query.to_string();
    let match_media = {
        let query = query.clone();
        move || {
            window
                .match_media(&query)
                .expect("Failed to query media")
                .expect("Media query not supported")
        }
    };

    let state = use_state_eq(|| match_media().matches());
    let state_clone = state.clone();

    {
        let query = query.clone();
        use_effect_with(query, move |_| {
            let match_media = match_media();
            let match_media_clone = match_media.clone();

            let listener = EventListener::new(&match_media, "change", move |_event| {
                state_clone.set(match_media_clone.matches());
            });

            move || {
                drop(listener);
            }
        });
    }

    *state
}

#[cfg(feature = "ssr")]
#[hook]
pub fn use_media_query(_query: &str) -> bool {
    false
}
