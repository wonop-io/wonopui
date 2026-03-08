//! Iframe component for WonopUI.
//!
//! An iframe wrapper component that enables rendering Yew components inside an iframe
//! with proper style injection and event propagation.

use gloo_utils::document;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlIFrameElement;
use js_sys;
pub use wonopui_core::merge_classes;
use yew::prelude::*;
use yew::virtual_dom::VNode;

/// CSS classes for the Iframe component
pub mod classes {
    pub const CONTAINER: &str = "w-full border-0";
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
    /// If true, the iframe will automatically adjust its height to fit content
    #[prop_or(false)]
    pub auto_height: bool,
    /// Callback fired when the content height changes (only when auto_height is true)
    #[prop_or_default]
    pub on_height_change: Option<Callback<f64>>,
    /// Minimum height for the iframe (default: 100)
    #[prop_or(100)]
    pub min_height: u32,
}

#[function_component(Iframe)]
pub fn iframe(props: &IframeProps) -> Html {
    let iframe_ref = use_node_ref();
    let body_ref = use_state(|| None::<web_sys::Element>);
    let head_ref = use_state(|| None::<web_sys::Element>);
    let preamble = use_state(String::new);
    let iframe_height = use_state(|| None::<f64>);

    // Set up iframe document and event listeners
    {
        let iframe_ref = iframe_ref.clone();
        let body_ref = body_ref.clone();
        let head_ref = head_ref.clone();
        let body_class = props.body_class.clone();
        let onkeydown = props.onkeydown.clone();
        let auto_height = props.auto_height;
        let on_height_change = props.on_height_change.clone();
        let iframe_height = iframe_height.clone();
        let min_height = props.min_height as f64;

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

                    // Set up auto-height using ResizeObserver and MutationObserver
                    if auto_height {
                        let iframe = iframe.clone();
                        let iframe_height = iframe_height.clone();
                        let on_height_change = on_height_change.clone();
                        let body_el: web_sys::Element = body.clone().into();
                        
                        // Function to update height - needs to be Rc<RefCell> for sharing across closures
                        // If on_height_change callback is provided, only report height (parent controls size)
                        // Otherwise, directly set the iframe's style
                        let has_callback = on_height_change.is_some();
                        let update_height = std::rc::Rc::new({
                            let iframe = iframe.clone();
                            let iframe_height = iframe_height.clone();
                            let on_height_change = on_height_change.clone();
                            let body_el = body_el.clone();
                            move || {
                                let scroll_height = body_el.scroll_height() as f64;
                                let height = scroll_height.max(min_height);
                                iframe_height.set(Some(height));
                                // Only directly set iframe height if no callback provided
                                // When callback is provided, parent is responsible for sizing
                                if !has_callback {
                                    if let Ok(html_el) = iframe.clone().dyn_into::<web_sys::HtmlElement>() {
                                        let _ = html_el.style().set_property("height", &format!("{}px", height));
                                    }
                                }
                                if let Some(ref cb) = on_height_change {
                                    cb.emit(height);
                                }
                            }
                        });
                        
                        // Use ResizeObserver to watch for content size changes (more reliable than MutationObserver)
                        let update_height_resize = update_height.clone();
                        let resize_callback = Closure::wrap(Box::new(move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                            update_height_resize();
                        }) as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);
                        
                        if let Ok(resize_observer) = web_sys::ResizeObserver::new(resize_callback.as_ref().unchecked_ref()) {
                            resize_observer.observe(&body_el);
                        }
                        resize_callback.forget();
                        
                        // Use MutationObserver for DOM structure changes
                        let update_height_mutation = update_height.clone();
                        let mutation_callback = Closure::wrap(Box::new(move |_mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
                            update_height_mutation();
                        }) as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
                        
                        if let Ok(observer) = web_sys::MutationObserver::new(mutation_callback.as_ref().unchecked_ref()) {
                            let config = web_sys::MutationObserverInit::new();
                            config.set_child_list(true);
                            config.set_subtree(true);
                            config.set_attributes(true);
                            let _ = observer.observe_with_options(&body, &config);
                        }
                        mutation_callback.forget();
                        
                        // Initial height calculation after a short delay to allow content to render
                        // Also set up multiple retries to catch async content (images, fonts, etc.)
                        for delay in [50, 150, 300, 500] {
                            let update_height_timeout = update_height.clone();
                            let timeout_callback = Closure::wrap(Box::new(move || {
                                update_height_timeout();
                            }) as Box<dyn FnMut()>);
                            let _ = web_sys::window()
                                .unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    timeout_callback.as_ref().unchecked_ref(),
                                    delay,
                                );
                            timeout_callback.forget();
                        }
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

            // Copy inline style tags (e.g., inlined Tailwind CSS from Trunk)
            if let Ok(styles) = master_document.query_selector_all("style") {
                for i in 0..styles.length() {
                    if let Some(style_node) = styles.get(i) {
                        if let Ok(style) = style_node.dyn_into::<web_sys::HtmlStyleElement>() {
                            let css_text = style.inner_html();
                            // Only copy non-empty styles
                            if !css_text.trim().is_empty() {
                                preamble_value.push_str(&format!("<style>{}</style>", css_text));
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
        let mut style_elements = Vec::new();

        // Copy link tags for CSS
        if let Ok(links) = document().query_selector_all("link") {
            for i in 0..links.length() {
                if let Some(link_node) = links.get(i) {
                    if let Ok(link) = link_node.dyn_into::<web_sys::HtmlLinkElement>() {
                        let rel = link.get_attribute("rel").unwrap_or_default();
                        let href = link.get_attribute("href").unwrap_or_default();
                        if href.ends_with(".css") {
                            style_elements.push(html! { <link rel={rel} href={href} /> });
                        }
                    }
                }
            }
        }

        // Copy inline style tags (e.g., inlined Tailwind CSS from Trunk)
        if let Ok(styles) = document().query_selector_all("style") {
            for i in 0..styles.length() {
                if let Some(style_node) = styles.get(i) {
                    if let Ok(style) = style_node.dyn_into::<web_sys::HtmlStyleElement>() {
                        let css_text = style.inner_html();
                        // Only copy non-empty styles
                        if !css_text.trim().is_empty() {
                            style_elements.push(html! {
                                <style>{css_text}</style>
                            });
                        }
                    }
                }
            }
        }

        if !style_elements.is_empty() {
            create_portal(html! { <>{ for style_elements }</> }, head.clone())
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

    // Build class - add h-full when not using auto_height, OR when parent controls size via callback
    let parent_controls_size = props.auto_height && props.on_height_change.is_some();
    let height_class = if !props.auto_height || parent_controls_size { "h-full" } else { "" };
    let iframe_class = merge_classes(&[classes::CONTAINER, height_class, &props.class.to_string()]);
    
    // Build style for auto_height (only when no callback - i.e. iframe controls its own size)
    let style = if props.auto_height && props.on_height_change.is_none() {
        if let Some(h) = *iframe_height {
            format!("min-height: {}px; height: {}px;", props.min_height, h)
        } else {
            format!("min-height: {}px;", props.min_height)
        }
    } else {
        String::new()
    };

    html! {
        <iframe
            class={iframe_class}
            ref={iframe_ref}
            srcdoc={srcdoc}
            id={props.id.clone()}
            title={props.title.clone().unwrap_or_else(|| "Embedded content".to_string())}
            style={style}
        >
            { head_portal }
            { body_portal }
        </iframe>
    }
}
