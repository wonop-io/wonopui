#[cfg(not(feature = "ssr"))]
use gloo::events::EventListener;
use yew::prelude::*;

#[hook]
#[cfg(not(feature = "ssr"))]
pub fn use_media_query(query: &str) -> bool {
    let query_string = query.to_string();
    let state = use_state_eq(|| false);

    {
        let state = state.clone();
        let query = query_string.clone();
        use_effect_with(query, move |query| {
            // Store the listener in an Option so we can drop it in cleanup
            let mut listener_option: Option<EventListener> = None;

            // Only access window inside use_effect to ensure it runs on client
            if let Some(window) = web_sys::window() {
                if let Ok(Some(media_query_list)) = window.match_media(&query) {
                    // Set initial state
                    state.set(media_query_list.matches());

                    let state_clone = state.clone();
                    let media_query_list_clone = media_query_list.clone();

                    let listener = EventListener::new(&media_query_list, "change", move |_event| {
                        state_clone.set(media_query_list_clone.matches());
                    });

                    listener_option = Some(listener);
                }
            }

            // Return a single cleanup closure that handles the Option
            move || {
                drop(listener_option);
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
