use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo_console as console;

#[derive(Properties, PartialEq)]
pub struct ColorPickerProps {
    #[prop_or_default]
    pub oncolorchange: Callback<(String, u16)>,
    #[prop_or("blue".to_string())]
    pub color: String,
    #[prop_or(500)]
    pub shade: u16,
}

#[function_component(TailwindColorPicker)]
pub fn tailwind_color_picker(props: &ColorPickerProps) -> Html {
    let shades = vec![950, 900, 800, 700, 600, 500, 400, 300, 200, 100, 50];
    let colors = vec![
        "Red", "Orange", "Amber", "Yellow", "Lime", "Green", "Emerald", "Teal", "Cyan", "Sky",
        "Blue", "Indigo", "Violet", "Purple", "Fuchsia", "Pink", "Rose", "Stone", "Neutral",
        "Zinc", "Gray", "Slate",
    ];

    let palette_visible = use_state(|| false);
    let dropdown_ref = use_node_ref();

    {
        let palette_visible = *palette_visible;
        let dropdown_ref = dropdown_ref.clone();
        use_effect_with((palette_visible,), move |(palette_visible,)| {
            if *palette_visible {
                if let Some(dropdown) = dropdown_ref.cast::<web_sys::HtmlElement>() {
                    dropdown.focus().unwrap();
                }
            }
        });
    }

    let toggle_palette = {
        let palette_visible = palette_visible.clone();
        let dropdown_ref = dropdown_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let new_state = !*palette_visible;
            palette_visible.set(new_state);
            if new_state {
                if let Some(dropdown) = dropdown_ref.cast::<web_sys::HtmlElement>() {
                    console::log!("Focus!");
                    dropdown.focus().unwrap();
                }
            }
        })
    };

    let on_blur = {
        let palette_visible = palette_visible.clone();
        Callback::from(move |_| {
            console::log!("Blur!");
            palette_visible.set(false);
        })
    };

    let on_color_click = {
        let oncolorchange = props.oncolorchange.clone();
        move |color_name: &str, shade: u16| {
            oncolorchange.emit((color_name.to_lowercase(), shade));
        }
    };

    html! {
        <div class="relative">
            <button onclick={toggle_palette.clone()} class="h-8 w-8 mb-4 p-2 bg-gray-500 text-white rounded flex justify-center items-center">
                <div class={format!("h-4 w-4 bg-{}-{}", props.color.to_lowercase(), props.shade)}></div>
            </button>
            if *palette_visible {
                <div ref={dropdown_ref} class="z-20 bg-white absolute top-12 left-0 flex-grow palettes overflow-auto flex flex-col" onblur={on_blur} tabindex="0" autofocus=true>
                    { for shades.iter().map(|shade| html! {
                        <div class="flex flex-row">
                            { for colors.iter().map(|color_name| {
                                let color_class = format!("bg-{}-{}", color_name.to_lowercase(), shade);
                                let hover_class = format!("hover:bg-{}-{}", color_name.to_lowercase(), shade);
                                html! {
                                    <div class={classes!("cursor-pointer", "border", color_class, hover_class, "h-4", "w-4")} onclick={
                                        let shade = shade.clone();
                                        let color_name = color_name.clone();
                                        let on_color_click = on_color_click.clone();
                                        Callback::from(move |_| on_color_click(color_name.clone(), shade.clone()))}>
                                    </div>
                                }
                            }) }
                        </div>
                    }) }
                </div>
            }
        </div>
    }
}
