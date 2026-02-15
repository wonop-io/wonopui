//! Resizable component for WonopUI.
//!
//! A component that allows resizing its content by dragging handles.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Resizable component
pub mod classes {
    pub const CONTAINER: &str = "relative";
    pub const HANDLE: &str = "absolute bg-transparent";
    pub const HANDLE_E: &str = "right-0 top-0 h-full w-2 cursor-ew-resize";
    pub const HANDLE_S: &str = "bottom-0 left-0 w-full h-2 cursor-ns-resize";
    pub const HANDLE_SE: &str = "right-0 bottom-0 w-4 h-4 cursor-nwse-resize";
    pub const HANDLE_W: &str = "left-0 top-0 h-full w-2 cursor-ew-resize";
    pub const HANDLE_N: &str = "top-0 left-0 w-full h-2 cursor-ns-resize";
    pub const HANDLE_NW: &str = "left-0 top-0 w-4 h-4 cursor-nwse-resize";
    pub const HANDLE_NE: &str = "right-0 top-0 w-4 h-4 cursor-nesw-resize";
    pub const HANDLE_SW: &str = "left-0 bottom-0 w-4 h-4 cursor-nesw-resize";
}

/// Coordinates for the resizable element (start_x, start_y, end_x, end_y)
pub type Coordinates = (f64, f64, f64, f64);

#[derive(Properties, PartialEq, Clone)]
pub struct ResizableProps {
    /// Coordinates as (start_x, start_y, end_x, end_y)
    #[prop_or((0., 0., 300., 200.))]
    pub coordinates: Coordinates,
    #[prop_or_default]
    pub on_coordinates_change: Callback<Coordinates>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    /// Enable north (top) resize handle
    #[prop_or(false)]
    pub north: bool,
    /// Enable north-west (top-left) resize handle
    #[prop_or(false)]
    pub north_west: bool,
    /// Enable north-east (top-right) resize handle
    #[prop_or(false)]
    pub north_east: bool,
    /// Enable east (right) resize handle
    #[prop_or(true)]
    pub east: bool,
    /// Enable south-east (bottom-right) resize handle
    #[prop_or(true)]
    pub south_east: bool,
    /// Enable south (bottom) resize handle
    #[prop_or(true)]
    pub south: bool,
    /// Enable south-west (bottom-left) resize handle
    #[prop_or(false)]
    pub south_west: bool,
    /// Enable west (left) resize handle
    #[prop_or(false)]
    pub west: bool,
    /// Minimum width
    #[prop_or(50.)]
    pub min_width: f64,
    /// Minimum height
    #[prop_or(50.)]
    pub min_height: f64,
}

#[derive(Clone, Copy, PartialEq)]
enum ResizeMode {
    None,
    North,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

#[function_component(Resizable)]
pub fn resizable(props: &ResizableProps) -> Html {
    let container_ref = use_node_ref();
    let coordinates = use_state(|| props.coordinates);
    let mode = use_state(|| ResizeMode::None);
    let start_pos = use_state(|| (0_i32, 0_i32));
    let start_coords = use_state(|| props.coordinates);

    // Sync with prop changes
    {
        let coordinates = coordinates.clone();
        let prop_coords = props.coordinates;
        use_effect_with(prop_coords, move |prop_coords| {
            coordinates.set(*prop_coords);
            || ()
        });
    }

    // Handle resize start
    let on_resize_start = {
        let mode = mode.clone();
        let start_pos = start_pos.clone();
        let start_coords = start_coords.clone();
        let coordinates = coordinates.clone();
        move |resize_mode: ResizeMode| {
            let mode = mode.clone();
            let start_pos = start_pos.clone();
            let start_coords = start_coords.clone();
            let coordinates = coordinates.clone();
            Callback::from(move |e: PointerEvent| {
                e.prevent_default();
                mode.set(resize_mode);
                start_pos.set((e.client_x(), e.client_y()));
                start_coords.set(*coordinates);
            })
        }
    };

    // Set up pointer move and up handlers
    {
        let mode_for_move = mode.clone();
        let mode_for_up = mode.clone();
        let start_pos = start_pos.clone();
        let start_coords = start_coords.clone();
        let coordinates = coordinates.clone();
        let on_coordinates_change = props.on_coordinates_change.clone();
        let min_width = props.min_width;
        let min_height = props.min_height;

        use_effect_with((), move |_| {
            let onpointermove = Closure::wrap(Box::new(move |e: PointerEvent| {
                let current_mode = *mode_for_move;
                if current_mode == ResizeMode::None {
                    return;
                }

                let dx = e.client_x() - start_pos.0;
                let dy = e.client_y() - start_pos.1;
                let (sx, sy, ex, ey) = *start_coords;

                let mut new_coords = match current_mode {
                    ResizeMode::North => (sx, sy + dy as f64, ex, ey),
                    ResizeMode::South => (sx, sy, ex, ey + dy as f64),
                    ResizeMode::East => (sx, sy, ex + dx as f64, ey),
                    ResizeMode::West => (sx + dx as f64, sy, ex, ey),
                    ResizeMode::NorthWest => (sx + dx as f64, sy + dy as f64, ex, ey),
                    ResizeMode::NorthEast => (sx, sy + dy as f64, ex + dx as f64, ey),
                    ResizeMode::SouthWest => (sx + dx as f64, sy, ex, ey + dy as f64),
                    ResizeMode::SouthEast => (sx, sy, ex + dx as f64, ey + dy as f64),
                    ResizeMode::None => (sx, sy, ex, ey),
                };

                // Enforce minimum dimensions
                if new_coords.2 - new_coords.0 < min_width {
                    new_coords.2 = new_coords.0 + min_width;
                }
                if new_coords.3 - new_coords.1 < min_height {
                    new_coords.3 = new_coords.1 + min_height;
                }

                coordinates.set(new_coords);
                on_coordinates_change.emit(new_coords);
            }) as Box<dyn FnMut(_)>);

            let onpointerup = Closure::wrap(Box::new(move |_: PointerEvent| {
                mode_for_up.set(ResizeMode::None);
            }) as Box<dyn FnMut(_)>);

            let window = web_sys::window().expect("no global window");
            let _ = window.add_event_listener_with_callback(
                "pointermove",
                onpointermove.as_ref().unchecked_ref(),
            );
            let _ = window.add_event_listener_with_callback(
                "pointerup",
                onpointerup.as_ref().unchecked_ref(),
            );

            onpointermove.forget();
            onpointerup.forget();

            || ()
        });
    }

    let (sx, sy, ex, ey) = *coordinates;
    let width = ex - sx;
    let height = ey - sy;
    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        sx, sy, width, height
    );

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div ref={container_ref} class={container_class} style={style}>
            { for props.children.iter() }

            // Resize handles
            if props.north {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_N])}
                    onpointerdown={on_resize_start(ResizeMode::North)}
                />
            }
            if props.south {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_S])}
                    onpointerdown={on_resize_start(ResizeMode::South)}
                />
            }
            if props.east {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_E])}
                    onpointerdown={on_resize_start(ResizeMode::East)}
                />
            }
            if props.west {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_W])}
                    onpointerdown={on_resize_start(ResizeMode::West)}
                />
            }
            if props.north_west {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_NW])}
                    onpointerdown={on_resize_start(ResizeMode::NorthWest)}
                />
            }
            if props.north_east {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_NE])}
                    onpointerdown={on_resize_start(ResizeMode::NorthEast)}
                />
            }
            if props.south_west {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_SW])}
                    onpointerdown={on_resize_start(ResizeMode::SouthWest)}
                />
            }
            if props.south_east {
                <div
                    class={merge_classes(&[classes::HANDLE, classes::HANDLE_SE])}
                    onpointerdown={on_resize_start(ResizeMode::SouthEast)}
                />
            }
        </div>
    }
}
