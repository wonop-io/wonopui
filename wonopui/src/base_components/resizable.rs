use crate::base_components::drag_point::DragPoint;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use yew::events::PointerEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
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

pub struct Resizable {
    container_ref: NodeRef,
    div_ref: NodeRef,

    mode: Mode,
    mouse_position: (i32, i32),
    coordinates: Option<(f64, f64, f64, f64)>,
    scale: f64,

    move_closure: Closure<dyn FnMut(PointerEvent)>,
}

impl Drop for Resizable {
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

impl Component for Resizable {
    type Message = Msg;
    type Properties = ResizableProps;

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
        let mut coordinates = self.coordinates.unwrap_or(ctx.props().coordinates);
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
                ctx.props().on_coordinates_change.emit(coordinates);
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
        let bg_color = "bg-blue-500";
        let (start_x, start_y, end_x, end_y) = self.coordinates.unwrap_or(ctx.props().coordinates);
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
                class="container relative"
            >
                <div
                    ref={self.div_ref.clone()}
                    class="border-2 border-blue-500 border-dashed absolute bg-gray-500"
                    style={format!("position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; ", x, y, width, height )}
                >
                    { for ctx.props().children.iter() }
                    <DragPoint
                        class={classes!(if ctx.props().north_west { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "top-0","left-0","-translate-x-2","-translate-y-2", "cursor-nw-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeTopLeft, e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd )}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().north_east { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "top-0","right-0","translate-x-2","-translate-y-2", "cursor-ne-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeTopRight, e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().south_west { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "bottom-0","left-0","-translate-x-2","translate-y-2", "cursor-sw-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeBottomLeft, e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().south_east { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "bottom-0","right-0","translate-x-2","translate-y-2", "cursor-se-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeBottomRight,e ))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().north { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "top-0","left-1/2","-translate-x-1/2","-translate-y-2", "cursor-n-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeTop,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().south { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "bottom-0","left-1/2","-translate-x-1/2","translate-y-2", "cursor-s-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeBottom,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().west { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "left-0","top-1/2","-translate-y-1/2","-translate-x-2", "cursor-w-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeLeft,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                    <DragPoint
                        class={classes!(if ctx.props().east { "block" } else { "hidden" }, "h-4","w-4","absolute","rounded-full",bg_color,"transform", "right-0","top-1/2","-translate-y-1/2","translate-x-2", "cursor-e-resize")}
                        onstart={ctx.link().callback(|e| Msg::PointerMoveStart(Mode::ResizeRight,e))}
                        onstop={ctx.link().callback(|_| Msg::PointerMoveEnd)}
                    />
                </div>
            </div>
        }
    }
}
