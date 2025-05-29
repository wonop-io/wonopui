use gloo_console as console;
use gloo_utils::window;
use std::f64::consts::PI;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::wasm_bindgen::JsCast;
use web_sys::ImageData;
use web_sys::ResizeObserver;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};
use yew::events::PointerEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ColorPickerWheelProps {
    #[prop_or_default]
    pub onchange: Callback<Vec<(f64, f64, f64)>>, // Vec of (hue, saturation, value)
    #[prop_or(1024)]
    pub size: u32,
    #[prop_or(1.0)]
    pub value: f64,
    #[prop_or_default]
    pub indicators: Vec<(f64, f64, f64)>, // Vec of (hue, saturation, value)
    #[prop_or(64.0)]
    pub indicator_radius: f64,
}

pub struct ColorPickerWheel {
    wheel_canvas_ref: NodeRef,
    indicator_canvas_ref: NodeRef,
    wheel_context: Option<CanvasRenderingContext2d>,
    indicator_context: Option<CanvasRenderingContext2d>,
    active_pointer: Option<(i32, usize)>, // (pointer_id, indicator_index)
    indicators: Vec<(f64, f64, f64)>,
    resize_observer: Option<ResizeObserver>,
    resize_callback: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {
    PointerDown(PointerEvent),
    PointerMove(PointerEvent),
    PointerUp(PointerEvent),
    UpdateColor(usize, f64, f64, f64),
    Resize,
}

impl Drop for ColorPickerWheel {
    fn drop(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");

        if let Some(resize_observer) = self.resize_observer.take() {
            resize_observer.disconnect();
        }
        if let Some(resize_callback) = self.resize_callback.take() {
            window
                .remove_event_listener_with_callback(
                    "resize",
                    resize_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
        }
    }
}

impl Component for ColorPickerWheel {
    type Message = Msg;
    type Properties = ColorPickerWheelProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            wheel_canvas_ref: NodeRef::default(),
            indicator_canvas_ref: NodeRef::default(),
            wheel_context: None,
            indicator_context: None,
            active_pointer: None,
            indicators: ctx.props().indicators.clone(),
            resize_observer: None,
            resize_callback: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PointerDown(event) => {
                if let Some(element) = self.indicator_canvas_ref.cast::<web_sys::Element>() {
                    let (index, distance) = self
                        .find_closest_indicator(event.offset_x() as f64, event.offset_y() as f64);

                    if distance <= ctx.props().indicator_radius {
                        element.set_pointer_capture(event.pointer_id()).unwrap();
                        self.active_pointer = Some((event.pointer_id(), index));

                        let window = web_sys::window().expect("no global `window` exists");
                        let link = ctx.link().clone();
                        let move_closure = {
                            let link = link.clone();
                            Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                                link.send_message(Msg::PointerMove(event));
                            }) as Box<dyn FnMut(_)>)
                        };
                        let up_closure = {
                            let link = link.clone();
                            Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                                link.send_message(Msg::PointerUp(event));
                            }) as Box<dyn FnMut(_)>)
                        };

                        window
                            .add_event_listener_with_callback(
                                "pointermove",
                                move_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        window
                            .add_event_listener_with_callback(
                                "pointerup",
                                up_closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();

                        // TODO: cause memory leak
                        move_closure.forget();
                        up_closure.forget();

                        self.update_color(ctx, event.offset_x() as f64, event.offset_y() as f64);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::PointerMove(event) => {
                if let Some((pointer_id, _)) = self.active_pointer {
                    if event.pointer_id() == pointer_id {
                        self.update_color(ctx, event.offset_x() as f64, event.offset_y() as f64);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::PointerUp(event) => {
                if let Some((pointer_id, _)) = self.active_pointer {
                    if event.pointer_id() == pointer_id {
                        if let Some(element) = self.indicator_canvas_ref.cast::<web_sys::Element>()
                        {
                            element.release_pointer_capture(event.pointer_id()).unwrap();
                        }
                        self.active_pointer = None;

                        let window = web_sys::window().expect("no global `window` exists");

                        // Trigger onchange here
                        self.update_color(ctx, event.offset_x() as f64, event.offset_y() as f64);
                        let colors: Vec<(f64, f64, f64)> = self.indicators.clone();
                        ctx.props().onchange.emit(colors);
                    }
                }
                false
            }
            Msg::UpdateColor(index, hue, saturation, value) => {
                let old_hue = self.indicators[index].0;
                self.indicators[index] = (hue, saturation, value);

                // Rotate other indicators if hue changed
                if (hue - old_hue).abs() > 1e-6 {
                    let hue_diff = hue - old_hue;
                    for (i, indicator) in self.indicators.iter_mut().enumerate() {
                        if i != index {
                            indicator.0 = (indicator.0 + hue_diff) % 1.0;
                            if indicator.0 < 0.0 {
                                indicator.0 += 1.0;
                            }
                        }
                    }
                }

                self.draw_indicators(ctx);
                true
            }
            Msg::Resize => {
                self.draw_wheel(ctx);
                self.draw_indicators(ctx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style={format!("position:relative")} class="w-full h-full aspect-square" >
                <canvas
                    class="w-full h-full aspect-square"
                    style={format!("margin: {}px;", ctx.props().indicator_radius.to_string())}
                    ref={self.wheel_canvas_ref.clone()}
                    width={ctx.props().size.to_string()}
                    height={ctx.props().size.to_string()}
                    style="position: absolute; top: 0; left: 0;"
                />
                <canvas
                    class="w-full h-full aspect-square"
                    style={format!("margin: {}px;", ctx.props().indicator_radius.to_string())}
                    ref={self.indicator_canvas_ref.clone()}
                    width={ctx.props().size.to_string()}
                    height={ctx.props().size.to_string()}
                    onpointerdown={ctx.link().callback(Msg::PointerDown)}
                    style="position: absolute; top: 0; left: 0; cursor: pointer;"
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let (Some(wheel_canvas), Some(indicator_canvas)) = (
                self.wheel_canvas_ref.cast::<HtmlCanvasElement>(),
                self.indicator_canvas_ref.cast::<HtmlCanvasElement>(),
            ) {
                let wheel_context = wheel_canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let indicator_context = indicator_canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                self.wheel_context = Some(wheel_context);
                self.indicator_context = Some(indicator_context);
                self.draw_wheel(ctx);
                self.draw_indicators(ctx);

                // Set up ResizeObserver
                let link = ctx.link().clone();
                let resize_callback = Closure::wrap(Box::new(move || {
                    link.send_message(Msg::Resize);
                }) as Box<dyn FnMut()>);

                let resize_observer =
                    ResizeObserver::new(resize_callback.as_ref().unchecked_ref()).unwrap();
                resize_observer.observe(&wheel_canvas);

                self.resize_observer = Some(resize_observer);
                self.resize_callback = Some(resize_callback);
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().size != old_props.size
            || ctx.props().value != old_props.value
            || ctx.props().indicators != old_props.indicators
            || ctx.props().indicator_radius != old_props.indicator_radius
        {
            self.indicators = ctx.props().indicators.clone();
            self.draw_wheel(ctx);
            self.draw_indicators(ctx);
            true
        } else {
            false
        }
    }
}

impl ColorPickerWheel {
    fn update_color(&mut self, ctx: &Context<Self>, x: f64, y: f64) {
        if let (Some(canvas), Some((_, index))) = (
            self.wheel_canvas_ref.cast::<HtmlCanvasElement>(),
            self.active_pointer,
        ) {
            let rect = canvas.get_bounding_client_rect();
            let center_x = rect.width() / 2.0;
            let center_y = rect.height() / 2.0;
            let dx = x - center_x;
            let dy = y - center_y;

            let radius = (dx * dx + dy * dy).sqrt();
            let max_radius = rect.width().min(rect.height()) / 2.0;

            let mut hue = (dy.atan2(dx) + PI) / (2.0 * PI);
            let saturation = (radius / max_radius).min(1.0);
            let value = self.indicators[index].2;

            // Adjust hue to maintain relative positions
            let hue_diff = hue - self.indicators[index].0;
            for indicator in self.indicators.iter_mut() {
                indicator.0 = (indicator.0 + hue_diff) % 1.0;
                if indicator.0 < 0.0 {
                    indicator.0 += 1.0;
                }
            }

            ctx.link()
                .send_message(Msg::UpdateColor(index, hue, saturation, value));
        }
    }

    fn draw_wheel(&self, ctx: &Context<Self>) {
        if let (Some(canvas), Some(context)) = (
            self.wheel_canvas_ref.cast::<HtmlCanvasElement>(),
            &self.wheel_context,
        ) {
            let width = canvas.width() as u32;
            let height = canvas.height() as u32;
            let center_x = width as f64 / 2.0;
            let center_y = height as f64 / 2.0;
            let radius = (width.min(height) / 2) as f64;

            let mut image_data = context
                .get_image_data(0.0, 0.0, width as f64, height as f64)
                .unwrap();
            let mut data = image_data.data();

            for x in 0..width {
                for y in 0..height {
                    let dx = x as f64 - center_x;
                    let dy = y as f64 - center_y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance <= radius {
                        let hue = (dy.atan2(dx) + PI) / (2.0 * PI);
                        let saturation = distance / radius;
                        let (r, g, b) = hsv_to_rgb(hue * 360.0, saturation, ctx.props().value);

                        let index = ((y * width + x) * 4) as usize;
                        data[index] = r;
                        data[index + 1] = g;
                        data[index + 2] = b;
                        data[index + 3] = 255; // Full opacity
                    }
                }
            }
            let imdata = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(data.as_slice()),
                width,
                height,
            )
            .unwrap();
            context.put_image_data(&imdata, 0.0, 0.0).unwrap();
        }
    }

    fn draw_indicators(&self, ctx: &Context<Self>) {
        if let (Some(canvas), Some(context)) = (
            self.indicator_canvas_ref.cast::<HtmlCanvasElement>(),
            &self.indicator_context,
        ) {
            let width = canvas.width() as f64;
            let height = canvas.height() as f64;
            let center_x = width / 2.0;
            let center_y = height / 2.0;
            let radius = width.min(height) / 2.0;

            context.clear_rect(0.0, 0.0, width, height);

            for (hue, saturation, value) in &self.indicators {
                let indicator_angle = hue * 2.0 * PI - PI;
                let indicator_distance = saturation * radius;
                let indicator_x = center_x + indicator_distance * indicator_angle.cos();
                let indicator_y = center_y + indicator_distance * indicator_angle.sin();

                // Draw white line from center to indicator
                context.begin_path();
                context.move_to(center_x, center_y);
                context.line_to(indicator_x, indicator_y);
                context.set_stroke_style(&"#FFFFFF".into());
                context.set_line_width(2.0);
                context.stroke();

                let (r, g, b) = hsv_to_rgb(hue * 360.0, *saturation, *value);
                let color = format!("rgb({}, {}, {})", r, g, b);

                context.begin_path();
                context
                    .arc(
                        indicator_x,
                        indicator_y,
                        ctx.props().indicator_radius,
                        0.0,
                        2.0 * PI,
                    )
                    .unwrap();
                context.set_fill_style(&color.into());
                context.fill();
                context.set_stroke_style(&"#FFFFFF".into());
                context.set_line_width(2.0);
                context.stroke();

                context.begin_path();
                context
                    .arc(indicator_x, indicator_y, 6.0, 0.0, 2.0 * PI)
                    .unwrap();
                context.set_stroke_style(&"#000000".into());
                context.set_line_width(1.0);
                context.stroke();
            }
        }
    }

    fn find_closest_indicator(&self, x: f64, y: f64) -> (usize, f64) {
        if let Some(canvas) = self.wheel_canvas_ref.cast::<HtmlCanvasElement>() {
            let rect = canvas.get_bounding_client_rect();
            let center_x = rect.width() / 2.0;
            let center_y = rect.height() / 2.0;
            let radius = rect.width().min(rect.height()) / 2.0;

            self.indicators
                .iter()
                .enumerate()
                .map(|(index, &(h, s, _))| {
                    let indicator_angle = h * 2.0 * PI - PI;
                    let indicator_distance = s * radius;
                    let indicator_x = center_x + indicator_distance * indicator_angle.cos();
                    let indicator_y = center_y + indicator_distance * indicator_angle.sin();

                    let dx = x - indicator_x;
                    let dy = y - indicator_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    (index, distance)
                })
                .min_by(|&(_, d1), &(_, d2)| d1.partial_cmp(&d2).unwrap())
                .unwrap_or((0, f64::MAX))
        } else {
            (0, f64::MAX)
        }
    }
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
