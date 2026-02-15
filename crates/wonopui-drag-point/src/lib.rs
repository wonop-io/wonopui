//! Drag Point component for WonopUI.
//!
//! A drag handle component that captures pointer events for drag operations.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the DragPoint component
pub mod classes {
    pub const DEFAULT: &str = "cursor-grab active:cursor-grabbing";
}

#[derive(Properties, PartialEq)]
pub struct DragPointProps {
    pub onstart: Callback<PointerEvent>,
    pub onstop: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DragPoint)]
pub fn drag_point(props: &DragPointProps) -> Html {
    let drag_point_ref = use_node_ref();
    let active_pointer = use_state(|| None::<i32>);

    let onpointerdown = {
        let active_pointer = active_pointer.clone();
        let drag_point_ref = drag_point_ref.clone();
        let onstart = props.onstart.clone();
        Callback::from(move |e: PointerEvent| {
            if let Some(element) = drag_point_ref.cast::<web_sys::Element>() {
                let _ = element.set_pointer_capture(e.pointer_id());
                active_pointer.set(Some(e.pointer_id()));
                onstart.emit(e);
            }
        })
    };

    // Set up global pointerup listener
    {
        let onstop = props.onstop.clone();
        let drag_point_ref = drag_point_ref.clone();
        let active_pointer = active_pointer.clone();

        use_effect_with((), move |_| {
            let active_pointer_clone = active_pointer.clone();
            let drag_point_ref_clone = drag_point_ref.clone();
            let onstop_clone = onstop.clone();

            let onpointerup = Closure::wrap(Box::new(move |e: PointerEvent| {
                if Some(e.pointer_id()) == *active_pointer_clone {
                    if let Some(element) = drag_point_ref_clone.cast::<web_sys::Element>() {
                        let _ = element.release_pointer_capture(e.pointer_id());
                    }
                    onstop_clone.emit(());
                    active_pointer_clone.set(None);
                }
            }) as Box<dyn FnMut(_)>);

            let window = web_sys::window().expect("no global `window` exists");
            let _ = window.add_event_listener_with_callback(
                "pointerup",
                onpointerup.as_ref().unchecked_ref(),
            );

            // Keep the closure alive and clean up on drop
            onpointerup.forget();

            || ()
        });
    }

    let combined_class = merge_classes(&[classes::DEFAULT, &props.class.to_string()]);

    // Use dynamic tag - for simplicity, we'll use a div but the tag prop could be used
    // to switch between different elements if needed
    html! {
        <div
            ref={drag_point_ref}
            class={combined_class}
            onpointerdown={onpointerdown}
        >
            { for props.children.iter() }
        </div>
    }
}
