use super::input::Input;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::closure::Closure;
#[cfg(not(feature = "ssr"))]
use web_sys::wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::events::PointerEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ColorPickerProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or(200)]
    pub width: u32,
    #[prop_or(200)]
    pub height: u32,
}

pub struct ColorPicker {
    canvas_ref: NodeRef,
    #[cfg(not(feature = "ssr"))]
    context: Option<CanvasRenderingContext2d>,
    active_pointer: Option<i32>,
    current_color: String,
    indicator_position: (i32, i32),
    #[cfg(not(feature = "ssr"))]
    move_closure: Closure<dyn FnMut(PointerEvent)>,
    #[cfg(not(feature = "ssr"))]
    up_closure: Closure<dyn FnMut(PointerEvent)>,
}

pub enum Msg {
    PointerDown(PointerEvent),
    PointerMove(PointerEvent),
    PointerUp(PointerEvent),
    UpdateColor(String),
    UpdateIndicatorPosition(i32, i32),
    UpdateHexInput(String),
}

impl Component for ColorPicker {
    type Message = Msg;
    type Properties = ColorPickerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        #[cfg(not(feature = "ssr"))]
        let move_closure = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                link.send_message(Msg::PointerMove(event));
            }) as Box<dyn FnMut(_)>)
        };

        #[cfg(not(feature = "ssr"))]
        let up_closure = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                link.send_message(Msg::PointerUp(event));
            }) as Box<dyn FnMut(_)>)
        };

        Self {
            canvas_ref: NodeRef::default(),
            #[cfg(not(feature = "ssr"))]
            context: None,
            active_pointer: None,
            current_color: ctx.props().value.clone(),
            indicator_position: (0, 0),
            #[cfg(not(feature = "ssr"))]
            move_closure,
            #[cfg(not(feature = "ssr"))]
            up_closure,
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
                        window
                            .add_event_listener_with_callback(
                                "pointermove",
                                self.move_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        window
                            .add_event_listener_with_callback(
                                "pointerup",
                                self.up_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                    self.update_color(ctx, event.offset_x() as f64, event.offset_y() as f64);
                }
                #[cfg(feature = "ssr")]
                {
                    self.active_pointer = Some(event.pointer_id());
                }
                false
            }
            Msg::PointerMove(event) => {
                if Some(event.pointer_id()) == self.active_pointer {
                    #[cfg(not(feature = "ssr"))]
                    self.update_color(ctx, event.offset_x() as f64, event.offset_y() as f64);
                }
                false
            }
            Msg::PointerUp(event) => {
                if Some(event.pointer_id()) == self.active_pointer {
                    #[cfg(not(feature = "ssr"))]
                    {
                        if let Some(element) = self.canvas_ref.cast::<web_sys::Element>() {
                            element.release_pointer_capture(event.pointer_id()).unwrap();
                        }

                        let window = web_sys::window().expect("no global `window` exists");
                        window
                            .remove_event_listener_with_callback(
                                "pointermove",
                                self.move_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        window
                            .remove_event_listener_with_callback(
                                "pointerup",
                                self.up_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                    self.active_pointer = None;
                }
                false
            }
            Msg::UpdateColor(color) => {
                let update = self.current_color != color;
                self.current_color = color;
                ctx.props().onchange.emit(self.current_color.clone());
                update
            }
            Msg::UpdateIndicatorPosition(x, y) => {
                let update = self.indicator_position != (x, y);
                self.indicator_position = (x, y);
                update
            }
            Msg::UpdateHexInput(hex) => {
                if hex.len() == 7 && hex.starts_with('#') {
                    self.current_color = hex.clone();
                    ctx.props().onchange.emit(self.current_color.clone());
                    #[cfg(not(feature = "ssr"))]
                    self.update_indicator_from_hex(&hex);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="relative">
                <canvas
                    ref={self.canvas_ref.clone()}
                    width={ctx.props().width.to_string()}
                    height={ctx.props().height.to_string()}
                    onpointerdown={ctx.link().callback(Msg::PointerDown)}
                    style="cursor: crosshair;"
                />
                <div
                    class="absolute w-4 h-4 border-2 border-white rounded-full pointer-events-none"
                    style={format!("left: {}px; top: {}px; transform: translate(-50%, -50%);", self.indicator_position.0, self.indicator_position.1)}
                />
                <div class="mt-2 flex items-center">
                    <div class="w-8 h-8 inline-block mr-2" style={format!("background-color: {}", self.current_color)} />
                    <Input
                        kind="text"
                        value={self.current_color.clone()}
                        ontext={ctx.link().callback(|v: String| Msg::UpdateHexInput(v))}
                    />
                </div>
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

                self.context = Some(context.clone());

                // Draw 2D color gradient
                let width = ctx.props().width as f64;
                let height = ctx.props().height as f64;

                for x in 0..ctx.props().width {
                    for y in 0..ctx.props().height {
                        let hue = (x as f64 / width) * 360.0;
                        let saturation = 1.0;
                        let lightness = 1.0 - (y as f64 / height);

                        let color = format!(
                            "hsl({}, {}%, {}%)",
                            hue,
                            saturation * 100.0,
                            lightness * 100.0
                        );
                        context.set_fill_style(&color.into());
                        context.fill_rect(x as f64, y as f64, 1.0, 1.0);
                    }
                }
                // TODO: Not working
                let current_color = self.current_color.clone();
                self.update_indicator_from_hex(&current_color);
            }
        }
    }
}

impl ColorPicker {
    #[cfg(not(feature = "ssr"))]
    fn update_color(&mut self, ctx: &Context<Self>, x: f64, y: f64) {
        ctx.link()
            .send_message(Msg::UpdateIndicatorPosition(x as i32, y as i32));
        if let Some(context) = &self.context {
            if let Ok(image_data) = context.get_image_data(x, y, 1.0, 1.0) {
                let data = image_data.data();
                let color = if data.len() >= 3 {
                    format!("#{:02X}{:02X}{:02X}", data[0], data[1], data[2])
                } else {
                    "#000000".to_string()
                };
                ctx.link().send_message(Msg::UpdateColor(color));
            } else {
                ctx.link()
                    .send_message(Msg::UpdateColor("#000000".to_string()));
            }
        }
    }

    #[cfg(feature = "ssr")]
    fn update_color(&mut self, _ctx: &Context<Self>, _x: f64, _y: f64) {
        // Empty implementation for SSR
    }

    #[cfg(not(feature = "ssr"))]
    fn update_indicator_from_hex(&mut self, hex: &str) {
        if hex.len() < 7 {
            return;
        }
        if let (Some(context), Ok(r), Ok(g), Ok(b)) = (
            &self.context,
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            let width = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap().width() as f64;
            let height = self
                .canvas_ref
                .cast::<HtmlCanvasElement>()
                .unwrap()
                .height() as f64;
            let mut closest_distance = f64::MAX;
            let mut closest_position = (0, 0);

            for x in 0..width as u32 {
                for y in 0..height as u32 {
                    let image_data = context
                        .get_image_data(x as f64, y as f64, 1.0, 1.0)
                        .unwrap();
                    let data = image_data.data();
                    if data.len() >= 3 {
                        let distance = ((data[0] as i32 - r as i32).pow(2)
                            + (data[1] as i32 - g as i32).pow(2)
                            + (data[2] as i32 - b as i32).pow(2))
                            as f64;

                        if distance < closest_distance {
                            closest_distance = distance;
                            closest_position = (x as i32, y as i32);
                        }
                    }
                }
            }

            self.indicator_position = closest_position;
        }
    }

    #[cfg(feature = "ssr")]
    fn update_indicator_from_hex(&mut self, _hex: &str) {
        // Empty implementation for SSR
    }
}
