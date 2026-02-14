//! Color Picker component for WonopUI.
//!
//! A color selection component with canvas-based gradient picker.

use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the ColorPicker component
pub mod classes {
    pub const CONTAINER: &str = "flex flex-col gap-2 p-4 rounded-md border bg-background";
    pub const CANVAS: &str = "rounded cursor-crosshair";
    pub const INPUT_ROW: &str = "flex items-center gap-2";
    pub const COLOR_PREVIEW: &str = "w-8 h-8 rounded border";
    pub const HEX_INPUT: &str = "flex-1 px-2 py-1 border rounded text-sm";
}

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
    #[prop_or_default]
    pub class: Classes,
}

/// Simple color picker component
/// Note: A full implementation would include canvas gradient rendering and pointer events
#[function_component(ColorPicker)]
pub fn color_picker(props: &ColorPickerProps) -> Html {
    let color = use_state(|| {
        if props.value.is_empty() {
            "#000000".to_string()
        } else {
            props.value.clone()
        }
    });

    // Sync with prop changes
    {
        let color = color.clone();
        let prop_value = props.value.clone();
        use_effect_with(prop_value, move |value| {
            if !value.is_empty() {
                color.set(value.clone());
            }
            || ()
        });
    }

    let oninput = {
        let color = color.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let new_color = input.value();
                color.set(new_color.clone());
                onchange.emit(new_color);
            }
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);
    let preview_style = format!("background-color: {};", *color);

    html! {
        <div class={container_class}>
            // Native color input as fallback
            <input
                type="color"
                value={(*color).clone()}
                oninput={oninput.clone()}
                class={classes::CANVAS}
                style={format!("width: {}px; height: {}px;", props.width, props.height)}
            />
            <div class={classes::INPUT_ROW}>
                <div class={classes::COLOR_PREVIEW} style={preview_style} />
                <input
                    type="text"
                    class={classes::HEX_INPUT}
                    value={(*color).clone()}
                    oninput={oninput}
                    placeholder="#000000"
                />
            </div>
        </div>
    }
}
