//! Color Picker component for WonopUI.
//!
//! A color selection component with a small trigger swatch that opens a popover.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the ColorPicker component (shadcn v4)
pub mod classes {
    /// Wrapper for the trigger and popover
    pub const WRAPPER: &str = "relative inline-block";
    
    /// Trigger button (small color swatch)
    pub const TRIGGER: &str = "size-9 rounded-lg border border-zinc-200 shadow-xs cursor-pointer transition-all duration-200 hover:border-zinc-300 focus-visible:border-zinc-950 dark:border-zinc-700 dark:hover:border-zinc-600 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none";
    
    /// Popover container
    pub const POPOVER: &str = "absolute z-50 mt-2 flex flex-col gap-3 p-4 rounded-xl border border-zinc-200 bg-white shadow-lg dark:border-zinc-800 dark:bg-zinc-950 animate-in fade-in-0 zoom-in-95";
    
    /// Hidden when closed
    pub const POPOVER_CLOSED: &str = "hidden";
    
    /// Native color input styled as gradient picker
    pub const COLOR_INPUT: &str = "rounded-lg cursor-crosshair border border-zinc-200 dark:border-zinc-800 transition-all duration-200 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] focus-visible:outline-none";
    
    /// Row containing preview and hex input
    pub const INPUT_ROW: &str = "flex items-center gap-2";
    
    /// Color preview swatch in popover
    pub const COLOR_PREVIEW: &str = "size-8 rounded-md border border-zinc-200 shadow-xs dark:border-zinc-700 shrink-0";
    
    /// Hex color input
    pub const HEX_INPUT: &str = "flex h-9 w-full rounded-md border border-zinc-200 bg-transparent px-3 py-2 text-sm font-mono shadow-xs transition-all duration-200 placeholder:text-zinc-500 focus-visible:border-zinc-950 focus-visible:ring-zinc-950/50 focus-visible:ring-[3px] focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:border-zinc-800 dark:placeholder:text-zinc-400 dark:focus-visible:border-zinc-300 dark:focus-visible:ring-zinc-300/50 text-zinc-900 dark:text-zinc-50";
}

#[derive(Properties, PartialEq)]
pub struct ColorPickerProps {
    /// Current color value (hex format)
    #[prop_or_default]
    pub value: String,
    
    /// Callback when color changes
    #[prop_or_default]
    pub onchange: Callback<String>,
    
    /// Width of the color picker canvas in pixels
    #[prop_or(180)]
    pub width: u32,
    
    /// Height of the color picker canvas in pixels
    #[prop_or(120)]
    pub height: u32,
    
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Color picker component with a small trigger swatch that opens a popover.
///
/// # Example
///
/// ```rust
/// use wonopui_color_picker::ColorPicker;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let color = use_state(|| "#3b82f6".to_string());
///     let onchange = {
///         let color = color.clone();
///         Callback::from(move |new_color: String| color.set(new_color))
///     };
///     
///     html! {
///         <ColorPicker value={(*color).clone()} onchange={onchange} />
///     }
/// }
/// ```
#[function_component(ColorPicker)]
pub fn color_picker(props: &ColorPickerProps) -> Html {
    let color_input_ref = use_node_ref();
    let color = use_state(|| {
        if props.value.is_empty() {
            "#3b82f6".to_string()
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

    // Click trigger to open native color picker directly
    let on_trigger_click = {
        let color_input_ref = color_input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = color_input_ref.cast::<web_sys::HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_color_input = {
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

    let trigger_style = format!("background-color: {};", *color);

    html! {
        <div class={merge_classes(&[classes::WRAPPER, &props.class.to_string()])}>
            // Trigger swatch - clicking opens native color picker directly
            <button
                type="button"
                class={classes::TRIGGER}
                style={trigger_style}
                onclick={on_trigger_click}
                aria-haspopup="dialog"
                title={format!("Current color: {}", *color)}
            />
            
            // Hidden native color input
            <input
                ref={color_input_ref}
                type="color"
                value={(*color).clone()}
                oninput={on_color_input}
                class="sr-only"
            />
        </div>
    }
}
