#[cfg(not(feature = "ssr"))]
use gloo_console as console;
#[cfg(not(feature = "ssr"))]
use gloo_utils::window;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
#[cfg(not(feature = "ssr"))]
use web_sys::wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::ResizeObserver;
#[cfg(not(feature = "ssr"))]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};
use yew::events::PointerEvent;
use yew::prelude::*;

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
    #[cfg(not(feature = "ssr"))]
    context: Option<CanvasRenderingContext2d>,
    active_pointer: Option<i32>,
    last_props_hue: f64,
    current_hue: f64,
    indicator_position: f64,
    #[cfg(not(feature = "ssr"))]
    move_closure: Option<Closure<dyn FnMut(PointerEvent)>>,
    #[cfg(not(feature = "ssr"))]
    up_closure: Option<Closure<dyn FnMut(PointerEvent)>>,
    #[cfg(not(feature = "ssr"))]
    resize_observer: Option<ResizeObserver>,
    #[cfg(not(feature = "ssr"))]
    resize_callback: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {
    PointerDown(PointerEvent),
    PointerMove(PointerEvent),
    PointerUp(PointerEvent),
    UpdateHue(f64),
    UpdateIndicatorPosition(f64),
    Resize,
}

#[cfg(not(feature = "ssr"))]
impl Drop for ColorPickerHue {
    fn drop(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");
        if let Some(move_closure) = &self.move_closure {
            window
                .remove_event_listener_with_callback(
                    "pointermove",
                    move_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
        }
        if let Some(up_closure) = &self.up_closure {
            window
                .remove_event_listener_with_callback(
                    "pointerup",
                    up_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
        }
        if let Some(resize_observer) = &self.resize_observer {
            resize_observer.disconnect();
        }
        if let Some(resize_callback) = &self.resize_callback {
            window
                .remove_event_listener_with_callback(
                    "resize",
                    resize_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
        }
    }
}

#[cfg(feature = "ssr")]
impl Drop for ColorPickerHue {
    fn drop(&mut self) {
        // No-op for SSR
    }
}

impl Component for ColorPickerHue {
    type Message = Msg;
    type Properties = ColorPickerHueProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            #[cfg(not(feature = "ssr"))]
            context: None,
            active_pointer: None,
            last_props_hue: ctx.props().value,
            current_hue: ctx.props().value,
            indicator_position: ctx.props().value,
            #[cfg(not(feature = "ssr"))]
            move_closure: None,
            #[cfg(not(feature = "ssr"))]
            up_closure: None,
            #[cfg(not(feature = "ssr"))]
            resize_observer: None,
            #[cfg(not(feature = "ssr"))]
            resize_callback: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PointerDown(event) => {
                #[cfg(not(feature = "ssr"))]
                {
                    if let Some(element) = self.canvas_ref.cast::<web_sys::Element>() {
                        element.set_pointer_capture(event.pointer_id()).unwrap();
                        self.active_pointer = Some(event.pointer_id());

                        let window = web_sys::window().expect("no global `window` exists");
                        let link = ctx.link().clone();
                        self.move_closure = {
                            let link = link.clone();
                            Some(Closure::wrap(
                                Box::new(move |event: web_sys::PointerEvent| {
                                    link.send_message(Msg::PointerMove(event));
                                }) as Box<dyn FnMut(_)>,
                            ))
                        };
                        self.up_closure = {
                            let link = link.clone();
                            Some(Closure::wrap(
                                Box::new(move |event: web_sys::PointerEvent| {
                                    link.send_message(Msg::PointerUp(event));
                                }) as Box<dyn FnMut(_)>,
                            ))
                        };

                        window
                            .add_event_listener_with_callback(
                                "pointermove",
                                self.move_closure.as_ref().unwrap().as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        window
                            .add_event_listener_with_callback(
                                "pointerup",
                                self.up_closure.as_ref().unwrap().as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                    self.update_hue(ctx, event.offset_x() as f64);
                }

                #[cfg(feature = "ssr")]
                {
                    self.active_pointer = Some(event.pointer_id());
                }

                true
            }
            Msg::PointerMove(event) => {
                if Some(event.pointer_id()) == self.active_pointer {
                    #[cfg(not(feature = "ssr"))]
                    self.update_hue(ctx, event.offset_x() as f64);
                }
                true
            }
            Msg::PointerUp(event) => {
                if Some(event.pointer_id()) == self.active_pointer {
                    #[cfg(not(feature = "ssr"))]
                    {
                        if let Some(element) = self.canvas_ref.cast::<web_sys::Element>() {
                            element.release_pointer_capture(event.pointer_id()).unwrap();
                        }

                        let window = web_sys::window().expect("no global `window` exists");
                        if let Some(move_closure) = &self.move_closure {
                            window
                                .remove_event_listener_with_callback(
                                    "pointermove",
                                    move_closure.as_ref().unchecked_ref(),
                                )
                                .unwrap();
                        }
                        if let Some(up_closure) = &self.up_closure {
                            window
                                .remove_event_listener_with_callback(
                                    "pointerup",
                                    up_closure.as_ref().unchecked_ref(),
                                )
                                .unwrap();
                        }
                        self.move_closure = None;
                        self.up_closure = None;
                    }

                    self.active_pointer = None;
                    ctx.props().onchange.emit(self.current_hue);
                }
                false
            }
            Msg::UpdateHue(hue) => {
                let update = (self.current_hue - hue).abs() > f64::EPSILON;
                self.current_hue = hue;
                update
            }
            Msg::UpdateIndicatorPosition(x) => {
                let update = (self.indicator_position - x).abs() > f64::EPSILON;
                self.indicator_position = x;
                update
            }
            Msg::Resize => {
                #[cfg(not(feature = "ssr"))]
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
        #[cfg(not(feature = "ssr"))]
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
                let link = ctx.link().clone();
                let cb = {
                    let link = link.clone();

                    Closure::wrap(Box::new(move || {
                        link.send_message(Msg::Resize);
                    }) as Box<dyn FnMut()>)
                };

                self.resize_callback = Some(Closure::wrap(Box::new(move || {
                    let window = web_sys::window().expect("no global `window` exists");
                    window.request_animation_frame(cb.as_ref().unchecked_ref());
                }) as Box<dyn FnMut()>));

                let resize_observer = ResizeObserver::new(
                    self.resize_callback
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
                resize_observer.observe(&canvas);

                self.resize_observer = Some(resize_observer);
            }
        }

        // Update current_hue if value prop changes
        if (self.last_props_hue - ctx.props().value).abs() > f64::EPSILON {
            self.current_hue = ctx.props().value;
            self.last_props_hue = ctx.props().value;
            ctx.link()
                .send_message(Msg::UpdateIndicatorPosition(self.current_hue));

            #[cfg(not(feature = "ssr"))]
            self.draw_gradient();
        }
    }
}

impl ColorPickerHue {
    #[cfg(not(feature = "ssr"))]
    fn update_hue(&mut self, ctx: &Context<Self>, x: f64) {
        if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
            let width = canvas.client_width() as f64;
            let hue = (x / width).clamp(0.0, 1.0);
            ctx.link().send_message(Msg::UpdateIndicatorPosition(hue));
            ctx.link().send_message(Msg::UpdateHue(hue));
        }
    }

    #[cfg(feature = "ssr")]
    fn update_hue(&mut self, ctx: &Context<Self>, x: f64) {
        // Simplified version for SSR
        let hue = (x / 100.0).clamp(0.0, 1.0); // Assume some default width
        ctx.link().send_message(Msg::UpdateIndicatorPosition(hue));
        ctx.link().send_message(Msg::UpdateHue(hue));
    }

    #[cfg(not(feature = "ssr"))]
    fn draw_gradient(&self) {
        if let (Some(canvas), Some(context)) =
            (self.canvas_ref.cast::<HtmlCanvasElement>(), &self.context)
        {
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

    #[cfg(feature = "ssr")]
    fn draw_gradient(&self) {
        // No-op for SSR
    }
}
