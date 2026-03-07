//! Resizable component for WonopUI.
//!
//! A component that allows resizing its content by dragging handles.

use wasm_bindgen::JsCast;
use web_sys::PointerEvent;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Resizable component (shadcn v4)
pub mod classes {
    /// Container for resizable panels
    pub const CONTAINER: &str = "relative bg-white dark:bg-zinc-950";
    /// Base handle style - subtle with hover effect
    pub const HANDLE: &str = "absolute bg-transparent transition-all duration-200 hover:bg-zinc-500/10 dark:hover:bg-zinc-400/10 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-950 dark:focus-visible:ring-zinc-300 z-10";
    /// East (right) handle with grip indicator
    pub const HANDLE_E: &str = "right-0 top-0 h-full w-2 cursor-ew-resize after:absolute after:left-1/2 after:top-1/2 after:-translate-x-1/2 after:-translate-y-1/2 after:h-8 after:w-1 after:rounded-full after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// South (bottom) handle with grip indicator
    pub const HANDLE_S: &str = "bottom-0 left-0 w-full h-2 cursor-ns-resize after:absolute after:left-1/2 after:top-1/2 after:-translate-x-1/2 after:-translate-y-1/2 after:h-1 after:w-8 after:rounded-full after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// South-east (bottom-right) corner handle
    pub const HANDLE_SE: &str = "right-0 bottom-0 w-5 h-5 cursor-nwse-resize after:absolute after:right-1 after:bottom-1 after:h-3 after:w-3 after:rounded-sm after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// West (left) handle
    pub const HANDLE_W: &str = "left-0 top-0 h-full w-2 cursor-ew-resize after:absolute after:left-1/2 after:top-1/2 after:-translate-x-1/2 after:-translate-y-1/2 after:h-8 after:w-1 after:rounded-full after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// North (top) handle
    pub const HANDLE_N: &str = "top-0 left-0 w-full h-2 cursor-ns-resize after:absolute after:left-1/2 after:top-1/2 after:-translate-x-1/2 after:-translate-y-1/2 after:h-1 after:w-8 after:rounded-full after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// North-west corner handle
    pub const HANDLE_NW: &str = "left-0 top-0 w-5 h-5 cursor-nwse-resize after:absolute after:left-1 after:top-1 after:h-3 after:w-3 after:rounded-sm after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// North-east corner handle
    pub const HANDLE_NE: &str = "right-0 top-0 w-5 h-5 cursor-nesw-resize after:absolute after:right-1 after:top-1 after:h-3 after:w-3 after:rounded-sm after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
    /// South-west corner handle
    pub const HANDLE_SW: &str = "left-0 bottom-0 w-5 h-5 cursor-nesw-resize after:absolute after:left-1 after:bottom-1 after:h-3 after:w-3 after:rounded-sm after:bg-zinc-300 after:transition-colors hover:after:bg-zinc-400 dark:after:bg-zinc-700 dark:hover:after:bg-zinc-500";
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

#[derive(Clone, Copy, PartialEq, Debug)]
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

/// Shared state for resize operations using interior mutability
#[derive(Clone, Default)]
struct ResizeState {
    mode: ResizeMode,
    start_x: f64,
    start_y: f64,
    start_coords: Coordinates,
}

impl Default for ResizeMode {
    fn default() -> Self {
        ResizeMode::None
    }
}

#[function_component(Resizable)]
pub fn resizable(props: &ResizableProps) -> Html {
    let container_ref = use_node_ref();
    let coordinates = use_state(|| props.coordinates);
    
    // Use Rc<RefCell<>> for shared mutable state that closures can access
    let resize_state = use_mut_ref(ResizeState::default);

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
        let resize_state = resize_state.clone();
        let coordinates = coordinates.clone();
        move |resize_mode: ResizeMode| {
            let resize_state = resize_state.clone();
            let coordinates = coordinates.clone();
            Callback::from(move |e: PointerEvent| {
                e.prevent_default();
                // Capture the pointer to ensure we get all events
                if let Some(target) = e.target() {
                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                        let _ = element.set_pointer_capture(e.pointer_id());
                    }
                }
                let mut state = resize_state.borrow_mut();
                state.mode = resize_mode;
                state.start_x = e.client_x() as f64;
                state.start_y = e.client_y() as f64;
                state.start_coords = *coordinates;
            })
        }
    };

    // Handle pointer move during resize
    let on_pointer_move = {
        let resize_state = resize_state.clone();
        let coordinates = coordinates.clone();
        let on_coordinates_change = props.on_coordinates_change.clone();
        let min_width = props.min_width;
        let min_height = props.min_height;
        
        Callback::from(move |e: PointerEvent| {
            let state = resize_state.borrow();
            let current_mode = state.mode;
            if current_mode == ResizeMode::None {
                return;
            }

            let dx = e.client_x() as f64 - state.start_x;
            let dy = e.client_y() as f64 - state.start_y;
            let (sx, sy, ex, ey) = state.start_coords;

            let mut new_coords = match current_mode {
                ResizeMode::North => (sx, sy + dy, ex, ey),
                ResizeMode::South => (sx, sy, ex, ey + dy),
                ResizeMode::East => (sx, sy, ex + dx, ey),
                ResizeMode::West => (sx + dx, sy, ex, ey),
                ResizeMode::NorthWest => (sx + dx, sy + dy, ex, ey),
                ResizeMode::NorthEast => (sx, sy + dy, ex + dx, ey),
                ResizeMode::SouthWest => (sx + dx, sy, ex, ey + dy),
                ResizeMode::SouthEast => (sx, sy, ex + dx, ey + dy),
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
        })
    };

    // Handle pointer up to end resize
    let on_pointer_up = {
        let resize_state = resize_state.clone();
        Callback::from(move |e: PointerEvent| {
            // Release pointer capture
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                    let _ = element.release_pointer_capture(e.pointer_id());
                }
            }
            resize_state.borrow_mut().mode = ResizeMode::None;
        })
    };

    let (sx, sy, ex, ey) = *coordinates;
    let width = ex - sx;
    let height = ey - sy;
    let style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        sx, sy, width, height
    );

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    // Create handle with pointer events
    let create_handle = |mode: ResizeMode, direction: &str, handle_class: &str, orientation: Option<&str>| {
        let class = merge_classes(&[classes::HANDLE, handle_class]);
        let on_start = on_resize_start(mode);
        let on_move = on_pointer_move.clone();
        let on_up = on_pointer_up.clone();
        
        html! {
            <div
                data-slot="resizable-handle"
                data-direction={direction.to_string()}
                class={class}
                onpointerdown={on_start}
                onpointermove={on_move}
                onpointerup={on_up}
                role={orientation.map(|_| "separator")}
                aria-orientation={orientation.map(|s| s.to_string())}
                tabindex="0"
            />
        }
    };

    html! {
        <div data-slot="resizable" ref={container_ref} class={container_class} style={style}>
            { for props.children.iter() }

            // Resize handles
            if props.north {
                { create_handle(ResizeMode::North, "north", classes::HANDLE_N, Some("horizontal")) }
            }
            if props.south {
                { create_handle(ResizeMode::South, "south", classes::HANDLE_S, Some("horizontal")) }
            }
            if props.east {
                { create_handle(ResizeMode::East, "east", classes::HANDLE_E, Some("vertical")) }
            }
            if props.west {
                { create_handle(ResizeMode::West, "west", classes::HANDLE_W, Some("vertical")) }
            }
            if props.north_west {
                { create_handle(ResizeMode::NorthWest, "north-west", classes::HANDLE_NW, None) }
            }
            if props.north_east {
                { create_handle(ResizeMode::NorthEast, "north-east", classes::HANDLE_NE, None) }
            }
            if props.south_west {
                { create_handle(ResizeMode::SouthWest, "south-west", classes::HANDLE_SW, None) }
            }
            if props.south_east {
                { create_handle(ResizeMode::SouthEast, "south-east", classes::HANDLE_SE, None) }
            }
        </div>
    }
}
