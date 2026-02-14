//! Paint Canvas component for WonopUI.
//!
//! A simple drawing canvas component.

use wasm_bindgen::JsCast;
use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the PaintCanvas component
pub mod classes {
    pub const CONTAINER: &str = "relative border rounded";
    pub const CANVAS: &str = "touch-none";
    pub const TOOLBAR: &str = "flex gap-2 p-2 border-b";
    pub const COLOR_BUTTON: &str = "w-6 h-6 rounded-full border-2 border-transparent hover:border-gray-400";
    pub const COLOR_BUTTON_SELECTED: &str = "w-6 h-6 rounded-full border-2 border-gray-800 dark:border-white";
    pub const SIZE_BUTTON: &str = "px-2 py-1 rounded border hover:bg-accent text-sm";
    pub const CLEAR_BUTTON: &str = "px-2 py-1 rounded border hover:bg-accent text-sm ml-auto";
}

#[derive(Properties, PartialEq)]
pub struct PaintCanvasProps {
    #[prop_or(400)]
    pub width: u32,
    #[prop_or(300)]
    pub height: u32,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("#000000".to_string())]
    pub default_color: String,
    #[prop_or(3.0)]
    pub default_brush_size: f64,
    /// Whether to show the toolbar
    #[prop_or(true)]
    pub show_toolbar: bool,
    /// Callback when canvas is updated (returns data URL)
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
}

#[function_component(PaintCanvas)]
pub fn paint_canvas(props: &PaintCanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let is_drawing = use_state(|| false);
    let color = use_state(|| props.default_color.clone());
    let brush_size = use_state(|| props.default_brush_size);
    let last_pos = use_state(|| (0.0_f64, 0.0_f64));

    let colors = vec!["#000000", "#ffffff", "#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff"];

    let on_pointer_down = {
        let canvas_ref = canvas_ref.clone();
        let is_drawing = is_drawing.clone();
        let last_pos = last_pos.clone();
        
        Callback::from(move |e: PointerEvent| {
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                let rect = canvas.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();
                last_pos.set((x, y));
                is_drawing.set(true);
                
                // Set pointer capture
                if let Ok(element) = canvas.dyn_into::<web_sys::Element>() {
                    let _ = element.set_pointer_capture(e.pointer_id());
                }
            }
        })
    };

    let on_pointer_move = {
        let canvas_ref = canvas_ref.clone();
        let is_drawing = is_drawing.clone();
        let last_pos = last_pos.clone();
        let color = color.clone();
        let brush_size = brush_size.clone();
        
        Callback::from(move |e: PointerEvent| {
            if !*is_drawing {
                return;
            }
            
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                if let Some(ctx) = canvas.get_context("2d").ok().flatten() {
                    if let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                        let rect = canvas.get_bounding_client_rect();
                        let x = e.client_x() as f64 - rect.left();
                        let y = e.client_y() as f64 - rect.top();
                        
                        ctx.begin_path();
                        ctx.set_stroke_style_str(&color);
                        ctx.set_line_width(*brush_size);
                        ctx.set_line_cap("round");
                        ctx.set_line_join("round");
                        ctx.move_to(last_pos.0, last_pos.1);
                        ctx.line_to(x, y);
                        ctx.stroke();
                        
                        last_pos.set((x, y));
                    }
                }
            }
        })
    };

    let on_pointer_up = {
        let is_drawing = is_drawing.clone();
        let canvas_ref = canvas_ref.clone();
        let onchange = props.onchange.clone();
        
        Callback::from(move |_: PointerEvent| {
            is_drawing.set(false);
            
            // Emit canvas data URL if callback is provided
            if let Some(callback) = &onchange {
                if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                    if let Ok(data_url) = canvas.to_data_url() {
                        callback.emit(data_url);
                    }
                }
            }
        })
    };

    let on_clear = {
        let canvas_ref = canvas_ref.clone();
        let width = props.width;
        let height = props.height;
        
        Callback::from(move |_: MouseEvent| {
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                if let Some(ctx) = canvas.get_context("2d").ok().flatten() {
                    if let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                        ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
                    }
                }
            }
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            if props.show_toolbar {
                <div class={classes::TOOLBAR}>
                    { for colors.iter().map(|c| {
                        let color_state = color.clone();
                        let c = c.to_string();
                        let is_selected = *color_state == c;
                        let button_class = if is_selected {
                            classes::COLOR_BUTTON_SELECTED
                        } else {
                            classes::COLOR_BUTTON
                        };
                        html! {
                            <button
                                class={button_class}
                                style={format!("background-color: {}", c)}
                                onclick={Callback::from(move |_| color_state.set(c.clone()))}
                            />
                        }
                    }) }
                    <button class={classes::CLEAR_BUTTON} onclick={on_clear}>
                        {"Clear"}
                    </button>
                </div>
            }
            <canvas
                ref={canvas_ref}
                class={classes::CANVAS}
                width={props.width.to_string()}
                height={props.height.to_string()}
                onpointerdown={on_pointer_down}
                onpointermove={on_pointer_move}
                onpointerup={on_pointer_up}
            />
        </div>
    }
}
