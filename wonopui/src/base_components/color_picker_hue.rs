use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, HtmlElement};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use yew::events::PointerEvent;
use gloo_utils::window;
use wasm_bindgen::prelude::*;
use web_sys::ResizeObserver;

#[derive(Properties, PartialEq)]
pub struct ColorPickerHueProps {
    #[prop_or_default]
    pub value: f64,
    #[prop_or_default]
    pub onchange: Callback<f64>,
    #[prop_or(30)]
    pub height: u32,
}

pub struct ColorPickerHue {
    canvas_ref: NodeRef,
    context: Option<CanvasRenderingContext2d>,
    active_pointer: Option<i32>,
    current_hue: f64,
    indicator_position: f64,
    move_closure: Closure<dyn FnMut(PointerEvent)>,
    up_closure: Closure<dyn FnMut(PointerEvent)>,
    resize_observer: Option<ResizeObserver>,
    resize_callback: Closure<dyn FnMut()>,
}

pub enum Msg {
    PointerDown(PointerEvent),
    PointerMove(PointerEvent),
    PointerUp(PointerEvent),
    UpdateHue(f64),
    UpdateIndicatorPosition(f64),
    Resize,
}

impl Component for ColorPickerHue {
    type Message = Msg;
    type Properties = ColorPickerHueProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Self {
            canvas_ref: NodeRef::default(),
            context: None,
            active_pointer: None,
            current_hue: ctx.props().value,
            indicator_position: ctx.props().value,
            move_closure: {
                let link = link.clone();
                Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                    link.send_message(Msg::PointerMove(event));
                }) as Box<dyn FnMut(_)>)
            },
            up_closure: {
                let link = link.clone();
                Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                    link.send_message(Msg::PointerUp(event));
                }) as Box<dyn FnMut(_)>)
            },
            resize_observer: None,
            resize_callback: {
                let link = link.clone();
                Closure::wrap(Box::new(move || {
                    link.send_message(Msg::Resize);
                }) as Box<dyn FnMut()>)
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PointerDown(event) => {
                if let Some(element) = self.canvas_ref.cast::<web_sys::Element>() {
                    element.set_pointer_capture(event.pointer_id()).unwrap();
                    self.active_pointer = Some(event.pointer_id());

                    let window = web_sys::window().expect("no global `window` exists");
                    window.add_event_listener_with_callback("pointermove", self.move_closure.as_ref().unchecked_ref()).unwrap();
                    window.add_event_listener_with_callback("pointerup", self.up_closure.as_ref().unchecked_ref()).unwrap();
                }
                self.update_hue(ctx, event.offset_x() as f64);
                true
            }
            Msg::PointerMove(event) => {
                if Some(event.pointer_id()) == self.active_pointer {
                    self.update_hue(ctx, event.offset_x() as f64);
                }
                true
            }
            Msg::PointerUp(event) => {
                if Some(event.pointer_id()) == self.active_pointer {
                    if let Some(element) = self.canvas_ref.cast::<web_sys::Element>() {
                        element.release_pointer_capture(event.pointer_id()).unwrap();
                    }
                    self.active_pointer = None;

                    let window = web_sys::window().expect("no global `window` exists");
                    window.remove_event_listener_with_callback("pointermove", self.move_closure.as_ref().unchecked_ref()).unwrap();
                    window.remove_event_listener_with_callback("pointerup", self.up_closure.as_ref().unchecked_ref()).unwrap();
                }
                false
            }
            Msg::UpdateHue(hue) => {
                let update = (self.current_hue - hue).abs() > f64::EPSILON;
                self.current_hue = hue;
                ctx.props().onchange.emit(self.current_hue);
                update
            }
            Msg::UpdateIndicatorPosition(x) => {
                let update = (self.indicator_position - x).abs() > f64::EPSILON;
                self.indicator_position = x;
                update
            }
            Msg::Resize => {
                self.draw_gradient();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="relative w-full">
                <canvas
                    ref={self.canvas_ref.clone()}
                    height={ctx.props().height.to_string()}
                    onpointerdown={ctx.link().callback(Msg::PointerDown)}
                    style="cursor: pointer; width: 100%;"
                />
                <div 
                    class="absolute top-0 w-1 h-full bg-white pointer-events-none"
                    style={format!("left: {}%; transform: translateX(-50%);", self.indicator_position * 100.0)}
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                self.context = Some(context);

                self.draw_gradient();

                // Set up ResizeObserver
                let resize_observer = ResizeObserver::new(self.resize_callback.as_ref().unchecked_ref()).unwrap();
                resize_observer.observe(&canvas);

                self.resize_observer = Some(resize_observer);
            }
        }
    }
}

impl ColorPickerHue {
    fn update_hue(&mut self, ctx: &Context<Self>, x: f64) {
        if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
            let width = canvas.client_width() as f64;
            let hue = (x / width).clamp(0.0, 1.0);
            ctx.link().send_message(Msg::UpdateIndicatorPosition(hue));
            ctx.link().send_message(Msg::UpdateHue(hue));
        }
    }

    fn draw_gradient(&self) {
        if let (Some(canvas), Some(context)) = (self.canvas_ref.cast::<HtmlCanvasElement>(), &self.context) {
            let width = canvas.client_width() as f64;
            let height = canvas.client_height() as f64;
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            for x in 0..width as u32 {
                let hue = x as f64 / width;
                let color = format!("hsl({}, 100%, 50%)", hue * 360.0);
                context.set_fill_style(&color.into());
                context.fill_rect(x as f64, 0.0, 1.0, height);
            }
        }
    }
}
