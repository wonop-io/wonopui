//! Iframe component for WonopUI.
//!
//! An iframe wrapper component that enables rendering Yew components inside an iframe
//! with proper style injection and event propagation.

use gloo_utils::document;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlIFrameElement;
pub use wonopui_core::merge_classes;
use yew::prelude::*;
use yew::virtual_dom::VNode;

/// CSS classes for the Iframe component
pub mod classes {
    pub const CONTAINER: &str = "w-full h-full border-0";
}

#[derive(Properties, PartialEq)]
pub struct IframeProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub body_class: String,
    /// If provided, use srcdoc instead of rendering children
    #[prop_or_default]
    pub srcdoc: Option<String>,
    #[prop_or_default]
    pub onkeydown: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Iframe)]
pub fn iframe(props: &IframeProps) -> Html {
    let iframe_ref = use_node_ref();
    let body_ref = use_state(|| None::<web_sys::Element>);
    let head_ref = use_state(|| None::<web_sys::Element>);
    let preamble = use_state(String::new);

    // Set up iframe document and event listeners
    {
        let iframe_ref = iframe_ref.clone();
        let body_ref = body_ref.clone();
        let head_ref = head_ref.clone();
        let body_class = props.body_class.clone();
        let onkeydown = props.onkeydown.clone();

        use_effect_with(body_class.clone(), move |body_class| {
            if let Some(iframe) = iframe_ref.cast::<HtmlIFrameElement>() {
                if let Some(doc) = iframe.content_document() {
                    let head = doc.head().expect("iframe should have a head");
                    let body = doc.body().expect("iframe should have a body");

                    // Set body class
                    body.set_class_name(body_class);

                    // Set up event propagation for mousemove
                    {
                        let iframe = iframe.clone();
                        let onmousemove =
                            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                                let options = web_sys::CustomEventInit::new();
                                options.set_bubbles(true);
                                options.set_cancelable(false);
                                if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict(
                                    "mousemove",
                                    &options,
                                ) {
                                    let _ = iframe.dispatch_event(&evt);
                                }
                            }) as Box<dyn FnMut(_)>);
                        let _ = body.add_event_listener_with_callback(
                            "mousemove",
                            onmousemove.as_ref().unchecked_ref(),
                        );
                        onmousemove.forget();
                    }

                    // Set up event propagation for pointerup
                    {
                        let iframe = iframe.clone();
                        let onpointerup =
                            Closure::wrap(Box::new(move |_event: web_sys::PointerEvent| {
                                let options = web_sys::CustomEventInit::new();
                                options.set_bubbles(true);
                                options.set_cancelable(false);
                                if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict(
                                    "pointerup",
                                    &options,
                                ) {
                                    let _ = iframe.dispatch_event(&evt);
                                }
                            }) as Box<dyn FnMut(_)>);
                        let _ = body.add_event_listener_with_callback(
                            "pointerup",
                            onpointerup.as_ref().unchecked_ref(),
                        );
                        onpointerup.forget();
                    }

                    // Set up keydown callback if provided
                    if let Some(onkeydown_cb) = onkeydown {
                        let onkeydown =
                            Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                                onkeydown_cb.emit(event);
                            }) as Box<dyn FnMut(_)>);
                        let _ = body.add_event_listener_with_callback(
                            "keydown",
                            onkeydown.as_ref().unchecked_ref(),
                        );
                        onkeydown.forget();
                    }

                    // Store references
                    body_ref.set(Some(body.into()));
                    head_ref.set(Some(head.into()));
                }
            }
            || ()
        });
    }

    // Collect stylesheets from parent document
    {
        let preamble = preamble.clone();

        use_effect_with((), move |_| {
            let master_document = document();
            let mut preamble_value = String::new();

            // Copy link tags for CSS
            if let Ok(links) = master_document.query_selector_all("link") {
                for i in 0..links.length() {
                    if let Some(link_node) = links.get(i) {
                        if let Ok(link) = link_node.dyn_into::<web_sys::HtmlLinkElement>() {
                            let rel = link.get_attribute("rel").unwrap_or_default();
                            let href = link.get_attribute("href").unwrap_or_default();

                            if href.ends_with(".css") {
                                preamble_value.push_str(&format!(
                                    "<link rel=\"{}\" href=\"{}\" />",
                                    rel, href
                                ));
                            }
                        }
                    }
                }
            }

            preamble.set(preamble_value);
            || ()
        });
    }

    // Create portals for head content
    let head_portal: VNode = if let Some(ref head) = *head_ref {
        if let Ok(links) = document().query_selector_all("link") {
            let mut link_html = Vec::new();
            for i in 0..links.length() {
                if let Some(link_node) = links.get(i) {
                    if let Ok(link) = link_node.dyn_into::<web_sys::HtmlLinkElement>() {
                        let rel = link.get_attribute("rel").unwrap_or_default();
                        let href = link.get_attribute("href").unwrap_or_default();
                        if href.ends_with(".css") {
                            link_html.push(html! { <link rel={rel} href={href} /> });
                        }
                    }
                }
            }
            create_portal(html! { <>{ for link_html }</> }, head.clone())
        } else {
            html! {}
        }
    } else {
        html! {}
    };

    // Create portal for body content
    let body_portal: VNode = if let Some(ref body) = *body_ref {
        if props.srcdoc.is_none() {
            create_portal(props.children.clone().into(), body.clone())
        } else {
            html! {}
        }
    } else {
        html! {}
    };

    // Build srcdoc if provided
    let srcdoc = props.srcdoc.as_ref().map(|content| {
        format!(
            "<html><head>{}</head><body>{}</body></html>",
            (*preamble).clone(),
            content
        )
    });

    let iframe_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <iframe
            class={iframe_class}
            ref={iframe_ref}
            srcdoc={srcdoc}
            id={props.id.clone()}
            title={props.title.clone().unwrap_or_else(|| "Embedded content".to_string())}
        >
            { head_portal }
            { body_portal }
        </iframe>
    }
}
