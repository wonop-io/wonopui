use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::events::PointerEvent;
use yew::prelude::*;

#[function_component(DragPoint)]
pub fn drag_point(props: &DragPointProps) -> Html {
    let DragPointProps {
        onstart,
        onstop,
        class,
        tag
    } = props;
    let drag_point_ref = use_node_ref();
    let active_pointer = use_state(|| None);
    let onpointerdown = {
        let active_pointer = active_pointer.clone();
        let drag_point_ref = drag_point_ref.clone();
        let onstart = onstart.clone();
        Callback::from(move |e: PointerEvent| {
            if let Some(element) = drag_point_ref.cast::<web_sys::Element>() {
                element
                    .set_pointer_capture(e.pointer_id())
                    .expect("Failed to set pointer capture");
                active_pointer.set(Some(e.pointer_id()));
                onstart.emit(e);
            }
        })
    };

    {
        let onstop = onstop.clone();
        let drag_point_ref = drag_point_ref.clone();
        let active_pointer = active_pointer.clone();
        let onpointerup = {
            Closure::wrap(Box::new(move |e: PointerEvent| {
                if Some(e.pointer_id()) == *active_pointer {
                    if let Some(element) = drag_point_ref.cast::<web_sys::Element>() {
                        element
                            .release_pointer_capture(e.pointer_id())
                            .expect("Failed to release pointer capture");
                    }
                }
                onstop.emit(());
                active_pointer.set(None);
            }) as Box<dyn FnMut(_)>)
        };
        use_effect_with((), move |_| {
            let window = web_sys::window().expect("no global `window` exists");

            window
                .add_event_listener_with_callback("pointerup", onpointerup.as_ref().unchecked_ref())
                .expect("Failed to add pointerup event listener to window");

            move || {
                window
                    .remove_event_listener_with_callback(
                        "pointerup",
                        onpointerup.as_ref().unchecked_ref(),
                    )
                    .expect("Failed to remove pointerup event listener from window");
                onpointerup.forget();
            }
        });
    }

    html! {
        <@{tag.clone()}
            ref={drag_point_ref}
            class={class.clone()}
            onpointerdown={onpointerdown}

        />
    }
}

#[derive(Properties, PartialEq)]
pub struct DragPointProps {
    pub onstart: Callback<PointerEvent>,
    pub onstop: Callback<()>,
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
}
