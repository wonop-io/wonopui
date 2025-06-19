#[cfg(not(feature = "ssr"))]
use crate::components::utils::window_provider::WindowProvider;
#[cfg(not(feature = "ssr"))]
use gloo_console as console;
#[cfg(not(feature = "ssr"))]
use gloo_utils::document;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::closure::Closure;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::HtmlIFrameElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IframeProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub body_class: String,
    #[prop_or_default]
    pub srcdoc: Option<String>,
    #[prop_or_default]
    pub onkeydown: Option<Callback<web_sys::KeyboardEvent>>,
}

#[function_component(Iframe)]
pub fn iframe(props: &IframeProps) -> Html {
    #[cfg(not(feature = "ssr"))]
    {
        let script = "".to_string();
        let iframe_ref = use_node_ref();
        let body_ref = use_state(|| None);
        let head_ref = use_state(|| None);
        let script_state = use_state(|| script.clone());
        let onkeydown = props.onkeydown.clone();
        {
            let iframe_ref = iframe_ref.clone();
            let body_ref = body_ref.clone();
            let head_ref = head_ref.clone();
            let body_class = props.body_class.clone();
            use_effect_with((body_class,), move |(body_class,)| {
                if let Some(iframe) = iframe_ref.cast::<HtmlIFrameElement>() {
                    if let Some(document) = iframe.content_document() {
                        let head = document.head().expect("iframe should have a head");
                        let body = document.body().expect("iframe should have a body");

                        // Set the body class
                        body.set_class_name(body_class);

                        // Propagate mousemove events to parent
                        let onmousemove = {
                            let iframe = iframe.clone();
                            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                                //let bounding_client_rect = iframe.get_bounding_client_rect();
                                let mut options = web_sys::CustomEventInit::new();
                                options.bubbles(true).cancelable(false);
                                let evt = web_sys::CustomEvent::new_with_event_init_dict(
                                    "mousemove",
                                    &options,
                                )
                                .expect("Failed to create event");
                                // evt.set_client_x(event.client_x() + bounding_client_rect.left() as i32);
                                // evt.set_client_y(event.client_y() + bounding_client_rect.top() as i32);
                                iframe
                                    .dispatch_event(&evt)
                                    .expect("Failed to dispatch event");
                            }) as Box<dyn FnMut(_)>)
                        };
                        body.add_event_listener_with_callback(
                            "mousemove",
                            onmousemove.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to add mousemove event listener");
                        onmousemove.forget();

                        // Propagate pointerup events to parent
                        let onpointerup = {
                            let iframe = iframe.clone();
                            Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                                console::log!("Bubbling pointerevent.");
                                //let bounding_client_rect = iframe.get_bounding_client_rect();
                                let mut options = web_sys::CustomEventInit::new();
                                options.bubbles(true).cancelable(false);
                                let evt = web_sys::CustomEvent::new_with_event_init_dict(
                                    "pointerup",
                                    &options,
                                )
                                .expect("Failed to create event");
                                // evt.set_client_x(event.client_x() + bounding_client_rect.left() as i32);
                                // evt.set_client_y(event.client_y() + bounding_client_rect.top() as i32);
                                iframe
                                    .dispatch_event(&evt)
                                    .expect("Failed to dispatch event");
                            }) as Box<dyn FnMut(_)>)
                        };
                        body.add_event_listener_with_callback(
                            "pointerup",
                            onpointerup.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to add mousemove event listener");
                        onpointerup.forget();

                        let onkeydown = {
                            let iframe = iframe.clone();
                            Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                                match onkeydown.clone() {
                                    Some(onkeydown) => {
                                        onkeydown.emit(event.clone());

                                        let window =
                                            web_sys::window().expect("should have a Window");
                                        let key = event.key().clone();
                                        let mut init_dict = web_sys::KeyboardEventInit::new();
                                        init_dict
                                            .bubbles(true)
                                            .cancelable(true)
                                            .view(Some(&window))
                                            .key(&key)
                                            .location(event.location())
                                            .ctrl_key(event.ctrl_key())
                                            .alt_key(event.alt_key())
                                            .shift_key(event.shift_key())
                                            .meta_key(event.meta_key());

                                        let evt = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
                                                "keydown",
                                                &init_dict
                                            ).expect("Failed to create KeyboardEvent");

                                        window
                                            .dispatch_event(&evt)
                                            .expect("Failed to dispatch event");
                                    }
                                    None => {}
                                }
                            }) as Box<dyn FnMut(_)>)
                        };
                        body.add_event_listener_with_callback(
                            "keydown",
                            onkeydown.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to add keydown event listener");
                        onkeydown.forget();

                        // Propagate mousemove events to parent
                        let onmouseup = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                            //let bounding_client_rect = iframe.get_bounding_client_rect();
                            let mut options = web_sys::CustomEventInit::new();
                            options.bubbles(true).cancelable(false);
                            let evt = web_sys::CustomEvent::new_with_event_init_dict(
                                "mousemove",
                                &options,
                            )
                            .expect("Failed to create event");
                            // evt.set_client_x(event.client_x() + bounding_client_rect.left() as i32);
                            // evt.set_client_y(event.client_y() + bounding_client_rect.top() as i32);
                            iframe
                                .dispatch_event(&evt)
                                .expect("Failed to dispatch event");
                        })
                            as Box<dyn FnMut(_)>);
                        body.add_event_listener_with_callback(
                            "mouseup",
                            onmouseup.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to add mousemove event listener");
                        onmouseup.forget();

                        body_ref.set(Some(body));
                        head_ref.set(Some(head));
                    }
                }
                || ()
            });
        }

        // Effect to update script state
        {
            let script = script.clone();
            use_effect_with((script,), move |(script,)| {
                script_state.set(script.clone());
                || ()
            });
        }

        let master_document = document();

        // JIT
        let style = "".to_string(); // TODO: Add JIT compiler
                                    // console::log!( style);

        // Creating link list
        let links = master_document.query_selector_all("link").unwrap();
        let mut link_list: Vec<Html> = Vec::new();

        let mut preamble = format!("<style>{}</style>", style,);

        for i in 0..links.length() {
            let link = links
                .get(i)
                .unwrap()
                .dyn_into::<web_sys::HtmlLinkElement>()
                .unwrap();
            let rel = &link.get_attribute("rel").unwrap_or_default();
            let href = &link.get_attribute("href").unwrap_or_default();

            if href.ends_with(".css") {
                link_list.push(html! { <link rel={rel.clone()} href={href.clone()} /> });

                preamble = format!(
                    "{}<link rel=\"{}\" href=\"{}\" />",
                    preamble,
                    rel.clone(),
                    href.clone()
                );
            }
        }

        let link_portal = if let Some(ref head) = *head_ref {
            create_portal(
                html! {
                    <>
                        // <script src="https://cdn.tailwindcss.com" />
                        <style>{ style }</style>
                        { for link_list.into_iter() }
                    </>
                },
                head.clone().into(),
            )
        } else {
            html! { <></> }
        };

        let contents = if let Some(ref body) = *body_ref {
            if props.srcdoc.is_none() {
                create_portal(props.children.clone().into(), body.clone().into())
            } else {
                html! { <></> }
            }
        } else {
            html! { <></> }
        };

        let srcdoc = match props.srcdoc.clone() {
            Some(srcdoc) => Some(format!(
                "<html><head>{}</head><body>{}</body></html>",
                preamble, srcdoc
            )),
            None => None,
        };

        html! {
            <WindowProvider iframe_ref={iframe_ref.clone()}>
            <iframe
                class={props.class.clone()}
                ref={iframe_ref}
                onload={Callback::from(move |_e| {
                    console::log!("---->> Iframe loaded <<< -----");
                })}
                srcdoc={srcdoc}
                id="frameViewport"
                title="Editor Iframe"
            >
                { link_portal }
                { contents }
            </iframe>
            </WindowProvider>
        }
    }

    #[cfg(feature = "ssr")]
    {
        html! {
            <div class={props.class.clone()}>
                { "Iframe not available in SSR mode" }
            </div>
        }
    }
}
