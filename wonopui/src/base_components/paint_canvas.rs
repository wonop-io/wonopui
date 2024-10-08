use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PaintCanvasProps {
    #[prop_or_default]
    pub image_src: Option<String>,
}

#[function_component(PaintCanvas)]
pub fn paint_canvas(props: &PaintCanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let image_ref = use_node_ref();
    let container_ref = use_node_ref();
    let context_ref = use_mut_ref(|| None::<CanvasRenderingContext2d>);
    let drawing = use_state(|| false);
    let image_loaded = use_state(|| false);

    let on_mouse_down = {
        let drawing = drawing.clone();
        let canvas_ref = canvas_ref.clone();
        let context_ref = context_ref.clone();
        Callback::from(move |event: MouseEvent| {
            drawing.set(true);
            if let Some(context) = &*context_ref.borrow() {
                let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
                let rect = canvas.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                context.move_to(x, y);
            }
        })
    };

    let on_mouse_up = {
        let drawing = drawing.clone();
        Callback::from(move |_| drawing.set(false))
    };

    let on_mouse_move = {
        let canvas_ref = canvas_ref.clone();
        let context_ref = context_ref.clone();
        let drawing = drawing.clone();
        Callback::from(move |event: MouseEvent| {
            if *drawing {
                if let Some(context) = &*context_ref.borrow() {
                    let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
                    let rect = canvas.get_bounding_client_rect();
                    let x = event.client_x() as f64 - rect.left();
                    let y = event.client_y() as f64 - rect.top();

                    context.line_to(x, y);
                    context.stroke();
                }
            }
        })
    };

    let on_image_load = {
        let canvas_ref = canvas_ref.clone();
        let image_ref = image_ref.clone();
        let container_ref = container_ref.clone();
        let context_ref = context_ref.clone();
        let image_loaded = image_loaded.clone();
        Callback::from(move |_| {
            let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
            let image: HtmlImageElement = image_ref.cast().unwrap();
            let container = container_ref.cast::<web_sys::HtmlElement>().unwrap();

            let container_width = container.client_width() as u32;
            let container_height = container.client_height() as u32;

            canvas.set_width(container_width);
            canvas.set_height(container_height);
            image.set_width(container_width);
            image.set_height(container_height);

            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context.set_stroke_style(&"#000000".into());
            context.set_line_width(10.0);
            *context_ref.borrow_mut() = Some(context);

            image_loaded.set(true);
        })
    };

    use_effect_with(
        (
            canvas_ref.clone(),
            context_ref.clone(),
            props.clone(),
            container_ref.clone(),
        ),
        move |(canvas_ref, context_ref, props, container_ref)| {
            if props.image_src.is_none() {
                let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
                let container = container_ref.cast::<web_sys::HtmlElement>().unwrap();

                let container_width = container.client_width() as u32;
                let container_height = container.client_height() as u32;

                canvas.set_width(container_width);
                canvas.set_height(container_height);

                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                context.set_stroke_style(&"#000000".into());
                context.set_line_width(2.0);
                *context_ref.borrow_mut() = Some(context);
            }
            || ()
        },
    );

    html! {
        <div ref={container_ref} class="relative mx-auto w-full h-full">
            if let Some(src) = &props.image_src {
                <img
                    ref={image_ref}
                    src={src.clone()}
                    onload={on_image_load}
                    class="w-full h-full object-contain"
                />
            }
            <canvas
                ref={canvas_ref}
                class={format!("absolute top-0 left-0 w-full h-full {}", if props.image_src.is_some() && !*image_loaded { "hidden" } else { "" })}
                onmousedown={on_mouse_down}
                onmouseup={on_mouse_up}
                onmousemove={on_mouse_move}
            />
        </div>
    }
}
