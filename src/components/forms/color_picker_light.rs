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
pub struct ColorPickerLightProps {
    #[prop_or_default]
    pub value: f64,
    #[prop_or_default]
    pub hue: f64,
    #[prop_or_default]
    pub saturation: f64,
    #[prop_or_default]
    pub onchange: Callback<f64>,
    #[prop_or(30)]
    pub height: u32,
}

pub struct ColorPickerLight {
    canvas_ref: NodeRef,
    #[cfg(not(feature = "ssr"))]
    context: Option<CanvasRenderingContext2d>,
    active_pointer: Option<i32>,
    current_light: f64,
    indicator_position: f64,
    #[cfg(not(feature = "ssr"))]
    move_closure: Option<Closure<dyn FnMut(PointerEvent)>>,
    #[cfg(not(feature = "ssr"))]
    up_closure: Option<Closure<dyn FnMut(PointerEvent)>>,
    #[cfg(not(feature = "ssr"))]
    resize_observer: Option<ResizeObserver>,
    #[cfg(not(feature = "ssr"))]
    resize_callback: Option<Closure<dyn FnMut()>>,
    hue: f64,
    saturation: f64,
}

pub enum Msg {
    PointerDown(PointerEvent),
    PointerMove(PointerEvent),
    PointerUp(PointerEvent),
    UpdateLight(f64),
    UpdateIndicatorPosition(f64),
    Resize,
}

#[cfg(not(feature = "ssr"))]
impl Drop for ColorPickerLight {
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
impl Drop for ColorPickerLight {
    fn drop(&mut self) {
        // Empty implementation for SSR
    }
}

impl Component for ColorPickerLight {
    type Message = Msg;
    type Properties = ColorPickerLightProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            #[cfg(not(feature = "ssr"))]
            context: None,
            active_pointer: None,
            current_light: ctx.props().value,
            indicator_position: ctx.props().value,
            #[cfg(not(feature = "ssr"))]
            move_closure: None,
            #[cfg(not(feature = "ssr"))]
            up_closure: None,
            #[cfg(not(feature = "ssr"))]
            resize_observer: None,
            #[cfg(not(feature = "ssr"))]
            resize_callback: None,
            hue: ctx.props().hue,
            saturation: ctx.props().saturation,
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
                    self.update_light(ctx, event.offset_x() as f64);
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
                    {
                        self.update_light(ctx, event.offset_x() as f64);
                    }
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
                    ctx.props().onchange.emit(self.current_light);
                }
                false
            }
            Msg::UpdateLight(light) => {
                let update = self.current_light != light;
                self.current_light = light;
                update
            }
            Msg::UpdateIndicatorPosition(x) => {
                let update = (self.indicator_position - x).abs() > f64::EPSILON;
                self.indicator_position = x;
                update
            }
            Msg::Resize => {
                #[cfg(not(feature = "ssr"))]
                {
                    self.draw_gradient(ctx);
                }
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
                    class="absolute top-0 w-1 h-full bg-black pointer-events-none"
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
                self.hue = ctx.props().hue;
                self.saturation = ctx.props().saturation;

                self.draw_gradient(ctx);

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
        } else {
            #[cfg(not(feature = "ssr"))]
            {
                // Check if hue or saturation has changed
                if self.hue != ctx.props().hue || self.saturation != ctx.props().saturation {
                    self.hue = ctx.props().hue;
                    self.saturation = ctx.props().saturation;
                    self.draw_gradient(ctx);
                }
            }

            #[cfg(feature = "ssr")]
            {
                self.hue = ctx.props().hue;
                self.saturation = ctx.props().saturation;
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Redraw gradient if hue or saturation has changed
        #[cfg(not(feature = "ssr"))]
        {
            self.draw_gradient(ctx);
        }
        true
    }
}

impl ColorPickerLight {
    #[cfg(not(feature = "ssr"))]
    fn update_light(&mut self, ctx: &Context<Self>, x: f64) {
        if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
            let width = canvas.client_width() as f64;
            let light = (x / width).clamp(0.0, 1.0);
            let position = x / width;
            ctx.link()
                .send_message(Msg::UpdateIndicatorPosition(position));
            ctx.link().send_message(Msg::UpdateLight(light));
        }
    }

    #[cfg(feature = "ssr")]
    fn update_light(&mut self, _ctx: &Context<Self>, _x: f64) {
        // Empty implementation for SSR
    }

    #[cfg(not(feature = "ssr"))]
    fn draw_gradient(&self, ctx: &Context<Self>) {
        if let (Some(canvas), Some(context)) =
            (self.canvas_ref.cast::<HtmlCanvasElement>(), &self.context)
        {
            let width = canvas.client_width() as f64;
            let height = canvas.client_height() as f64;
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            let gradient = context.create_linear_gradient(0.0, 0.0, width, 0.0);
            gradient
                .add_color_stop(
                    0.0,
                    &format!(
                        "hsl({}, {}%, 0%)",
                        ctx.props().hue,
                        ctx.props().saturation * 100.0
                    ),
                )
                .unwrap();
            gradient
                .add_color_stop(
                    1.0,
                    &format!(
                        "hsl({}, {}%, 100%)",
                        ctx.props().hue,
                        ctx.props().saturation * 100.0
                    ),
                )
                .unwrap();

            context.set_fill_style(&gradient);
            context.fill_rect(0.0, 0.0, width, height);
        }
    }

    #[cfg(feature = "ssr")]
    fn draw_gradient(&self, _ctx: &Context<Self>) {
        // Empty implementation for SSR
    }
}
