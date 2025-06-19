use crate::components::utils::drag_point::DragPoint;
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use std::rc::Rc;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::closure::Closure;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
#[cfg(not(feature = "ssr"))]
use web_sys::HtmlDivElement;
use yew::events::PointerEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ResizableProps {
    #[prop_or((0.,0.,600.,600.))]
    pub coordinates: (f64, f64, f64, f64), // (start_x, start_y, end_x, end_y)
    #[prop_or_default]
    pub on_coordinates_change: Callback<(f64, f64, f64, f64)>,
    #[prop_or_default]
    pub children: Children,

    #[prop_or(false)]
    pub north: bool,
    #[prop_or(false)]
    pub north_west: bool,
    #[prop_or(false)]
    pub north_east: bool,
    #[prop_or(true)]
    pub east: bool,
    #[prop_or(true)]
    pub south_east: bool,
    #[prop_or(true)]
    pub south: bool,
    #[prop_or(false)]
    pub south_west: bool,
    #[prop_or(false)]
    pub west: bool,
}

#[derive(Properties, PartialEq)]
pub struct ResizableInnerProps {
    pub brandguide: Rc<BrandGuideType>,
    pub props: ResizableProps,
}

#[derive(PartialEq, Clone, Debug)]
enum Mode {
    View,
    ResizeTopLeft,
    ResizeTopRight,
    ResizeBottomLeft,
    ResizeBottomRight,
    ResizeTop,
    ResizeBottom,
    ResizeLeft,
    ResizeRight,
}

#[cfg(not(feature = "ssr"))]
pub struct ResizableInner {
    container_ref: NodeRef,
    div_ref: NodeRef,

    mode: Mode,
    mouse_position: (i32, i32),
    coordinates: Option<(f64, f64, f64, f64)>,
    scale: f64,

    move_closure: Closure<dyn FnMut(PointerEvent)>,
}

#[cfg(feature = "ssr")]
pub struct ResizableInner {
    container_ref: NodeRef,
    div_ref: NodeRef,

    mode: Mode,
    mouse_position: (i32, i32),
    coordinates: Option<(f64, f64, f64, f64)>,
    scale: f64,
}

#[cfg(not(feature = "ssr"))]
impl Drop for ResizableInner {
    fn drop(&mut self) {
        let container = self.container_ref.cast::<HtmlDivElement>();
        if let Some(container) = container {
            if let Some(window) = web_sys::window() {
                window
                    .remove_event_listener_with_callback(
                        "pointermove",
                        self.move_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        }
    }
}

pub enum Msg {
    PointerMoveStart(Mode, PointerEvent),
    PointerMove(PointerEvent),
    PointerMoveEnd,
}

#[cfg(not(feature = "ssr"))]
impl Component for ResizableInner {
    type Message = Msg;
    type Properties = ResizableInnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            container_ref: NodeRef::default(),
            div_ref: NodeRef::default(),

            mode: Mode::View,
            mouse_position: (0, 0),
            coordinates: None,
            scale: 1.,
            move_closure: {
                let link = ctx.link().clone();

                Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                    link.send_message(Msg::PointerMove(event));
                }) as Box<dyn FnMut(_)>)
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut coordinates = self.coordinates.unwrap_or(ctx.props().props.coordinates);
        let rect = self
            .container_ref
            .cast::<HtmlDivElement>()
            .unwrap()
            .get_bounding_client_rect();
        match msg {
            Msg::PointerMoveStart(mode, event) => {
                if self.mode != Mode::View {
                    return false;
                }
                let x = event.client_x() - rect.x() as i32;
                let y = event.client_y() - rect.y() as i32;
                self.mode = mode.clone();
                self.mouse_position = (x, y);

                let window = web_sys::window().expect("no global `window` exists");

                window
                    .add_event_listener_with_callback(
                        "pointermove",
                        self.move_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }

            Msg::PointerMove(event) => {
                // event.prevent_default();
                // event.stop_propagation();

                let x = event.client_x() - rect.x() as i32;
                let y = event.client_y() - rect.y() as i32;
                let dx = x - self.mouse_position.0;
                let dy = y - self.mouse_position.1;

                self.mouse_position = (x, y);

                match self.mode {
                    Mode::ResizeTopLeft => {
                        coordinates.0 += dx as f64 / self.scale;
                        coordinates.1 += dy as f64 / self.scale;
                    }
                    Mode::ResizeTopRight => {
                        coordinates.2 += dx as f64 / self.scale;
                        coordinates.1 += dy as f64 / self.scale;
                    }
                    Mode::ResizeBottomLeft => {
                        coordinates.0 += dx as f64 / self.scale;
                        coordinates.3 += dy as f64 / self.scale;
                    }
                    Mode::ResizeBottomRight => {
                        coordinates.2 += dx as f64 / self.scale;
                        coordinates.3 += dy as f64 / self.scale;
                    }
                    Mode::ResizeTop => {
                        coordinates.1 += dy as f64 / self.scale;
                    }
                    Mode::ResizeBottom => {
                        coordinates.3 += dy as f64 / self.scale;
                    }
                    Mode::ResizeLeft => {
                        coordinates.0 += dx as f64 / self.scale;
                    }
                    Mode::ResizeRight => {
                        coordinates.2 += dx as f64 / self.scale;
                    }
                    Mode::View => {
                        return false;
                    }
                }

                self.coordinates = Some(coordinates.clone());
                ctx.props().props.on_coordinates_change.emit(coordinates);
            }
            Msg::PointerMoveEnd => {
                self.mode = Mode::View;

                let window = web_sys::window().expect("no global `window` exists");
                window
                    .remove_event_listener_with_callback(
                        "pointermove",
                        self.move_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        }

        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let brandguide = &ctx.props().brandguide;
        let props = &ctx.props().props;

        let (start_x, start_y, end_x, end_y) = self.coordinates.unwrap_or(props.coordinates);
        let start_x = (start_x * self.scale) as i32;
        let start_y = (start_y * self.scale) as i32;
        let end_x = (end_x * self.scale) as i32;
        let end_y = (end_y * self.scale) as i32;

        let x = std::cmp::min(start_x, end_x);
        let y = std::cmp::min(start_y, end_y);
        let width = std::cmp::max(start_x, end_x) - x;
        let height = std::cmp::max(start_y, end_y) - y;

        html! {
            <div
                ref={self.container_ref.clone()}
                class={classes!(&brandguide.resizable_container)}
            >
                <div
                    ref={self.div_ref.clone()}
                    class={classes!(&brandguide.resizable_box)}
                    style={format!("position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; ", x, y, width, height )}
                >
                    { for props.children.iter() }
                    <DragPoint
                        class={classes!(if props.north_west { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_nw)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeTopLeft, e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd )}
                    />
                    <DragPoint
                        class={classes!(if props.north_east { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_ne)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeTopRight, e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if props.south_west { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_sw)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeBottomLeft, e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if props.south_east { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_se)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeBottomRight,e ))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if props.north { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_n)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeTop,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if props.south { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_s)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeBottom,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if props.west { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_w)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeLeft,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if props.east { &brandguide.resizable_handle_visible } else { &brandguide.resizable_handle_hidden }, &brandguide.resizable_handle_e)}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeRight,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                </div>
            </div>
        }
    }
}

#[cfg(feature = "ssr")]
impl Component for ResizableInner {
    type Message = Msg;
    type Properties = ResizableInnerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container_ref: NodeRef::default(),
            div_ref: NodeRef::default(),
            mode: Mode::View,
            mouse_position: (0, 0),
            coordinates: None,
            scale: 1.,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let brandguide = &ctx.props().brandguide;
        let props = &ctx.props().props;

        html! {
            <div
                ref={self.container_ref.clone()}
                class={classes!(&brandguide.resizable_container)}
            >
                <div
                    ref={self.div_ref.clone()}
                    class={classes!(&brandguide.resizable_box)}
                >
                    { for props.children.iter() }
                </div>
            </div>
        }
    }
}

#[function_component(Resizable)]
pub fn resizable(props: &ResizableProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = Rc::new(get_brandguide());

    html! {
        <ResizableInner brandguide={brandguide} props={props.clone()} />
    }
}

// Snippets to update brandguide:
// ("resizable_container".to_string(), "container relative".to_string()),
// ("resizable_box".to_string(), "border-2 border-blue-500 border-dashed absolute bg-gray-500".to_string()),
// ("resizable_handle_visible".to_string(), "block".to_string()),
// ("resizable_handle_hidden".to_string(), "hidden".to_string()),
// ("resizable_handle_nw".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform top-0 left-0 -translate-x-2 -translate-y-2 cursor-nw-resize".to_string()),
// ("resizable_handle_ne".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform top-0 right-0 translate-x-2 -translate-y-2 cursor-ne-resize".to_string()),
// ("resizable_handle_sw".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform bottom-0 left-0 -translate-x-2 translate-y-2 cursor-sw-resize".to_string()),
// ("resizable_handle_se".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform bottom-0 right-0 translate-x-2 translate-y-2 cursor-se-resize".to_string()),
// ("resizable_handle_n".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform top-0 left-1/2 -translate-x-1/2 -translate-y-2 cursor-n-resize".to_string()),
// ("resizable_handle_s".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform bottom-0 left-1/2 -translate-x-1/2 translate-y-2 cursor-s-resize".to_string()),
// ("resizable_handle_w".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform left-0 top-1/2 -translate-y-1/2 -translate-x-2 cursor-w-resize".to_string()),
// ("resizable_handle_e".to_string(), "h-4 w-4 absolute rounded-full bg-blue-500 transform right-0 top-1/2 -translate-y-1/2 translate-x-2 cursor-e-resize".to_string()),
//
// pub resizable_container: ClassesContainer<T>,
// pub resizable_box: ClassesContainer<T>,
// pub resizable_handle_visible: ClassesContainer<T>,
// pub resizable_handle_hidden: ClassesContainer<T>,
// pub resizable_handle_nw: ClassesContainer<T>,
// pub resizable_handle_ne: ClassesContainer<T>,
// pub resizable_handle_sw: ClassesContainer<T>,
// pub resizable_handle_se: ClassesContainer<T>,
// pub resizable_handle_n: ClassesContainer<T>,
// pub resizable_handle_s: ClassesContainer<T>,
// pub resizable_handle_w: ClassesContainer<T>,
// pub resizable_handle_e: ClassesContainer<T>,
//
// resizable_container: self.resizable_container.to_owned(),
// resizable_box: self.resizable_box.to_owned(),
// resizable_handle_visible: self.resizable_handle_visible.to_owned(),
// resizable_handle_hidden: self.resizable_handle_hidden.to_owned(),
// resizable_handle_nw: self.resizable_handle_nw.to_owned(),
// resizable_handle_ne: self.resizable_handle_ne.to_owned(),
// resizable_handle_sw: self.resizable_handle_sw.to_owned(),
// resizable_handle_se: self.resizable_handle_se.to_owned(),
// resizable_handle_n: self.resizable_handle_n.to_owned(),
// resizable_handle_s: self.resizable_handle_s.to_owned(),
// resizable_handle_w: self.resizable_handle_w.to_owned(),
// resizable_handle_e: self.resizable_handle_e.to_owned(),
//
// resizable_container: default_config_hm
// .get("resizable_container")
// .expect("Template parameter missing")
// .clone(),
// resizable_box: default_config_hm
// .get("resizable_box")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_visible: default_config_hm
// .get("resizable_handle_visible")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_hidden: default_config_hm
// .get("resizable_handle_hidden")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_nw: default_config_hm
// .get("resizable_handle_nw")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_ne: default_config_hm
// .get("resizable_handle_ne")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_sw: default_config_hm
// .get("resizable_handle_sw")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_se: default_config_hm
// .get("resizable_handle_se")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_n: default_config_hm
// .get("resizable_handle_n")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_s: default_config_hm
// .get("resizable_handle_s")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_w: default_config_hm
// .get("resizable_handle_w")
// .expect("Template parameter missing")
// .clone(),
// resizable_handle_e: default_config_hm
// .get("resizable_handle_e")
// .expect("Template parameter missing")
// .clone(),
