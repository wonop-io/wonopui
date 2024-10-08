#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use gloo_console as console;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
pub fn tailwind_tailwind_color_picker(props: &ColorPickerProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

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
        <div class={classes!(&brandguide.tailwind_color_picker_container)}>
            <button onclick={toggle_palette.clone()} class={classes!(&brandguide.tailwind_color_picker_button)}>
                <div class={classes!(&brandguide.tailwind_color_picker_selected_color, format!("bg-{}-{}", props.color.to_lowercase(), props.shade))}></div>
            </button>
            if *palette_visible {
                <div ref={dropdown_ref} class={classes!(&brandguide.tailwind_color_picker_dropdown)} onblur={on_blur} tabindex="0" autofocus=true>
                    { for shades.iter().map(|shade| html! {
                        <div class={classes!(&brandguide.tailwind_color_picker_row)}>
                            { for colors.iter().map(|color_name| {
                                let color_class = format!("bg-{}-{}", color_name.to_lowercase(), shade);
                                let hover_class = format!("hover:bg-{}-{}", color_name.to_lowercase(), shade);
                                html! {
                                    <div class={classes!(&brandguide.tailwind_color_picker_cell, color_class, hover_class)} onclick={
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

// Snippets to update brandguide:
// ("tailwind_color_picker_container".to_string(), "relative".to_string()),
// ("tailwind_color_picker_button".to_string(), "h-8 w-8 mb-4 p-2 bg-gray-500 text-white rounded flex justify-center items-center".to_string()),
// ("tailwind_color_picker_selected_color".to_string(), "h-4 w-4".to_string()),
// ("tailwind_color_picker_dropdown".to_string(), "z-20 bg-white absolute top-12 left-0 flex-grow palettes overflow-auto flex flex-col".to_string()),
// ("tailwind_color_picker_row".to_string(), "flex flex-row".to_string()),
// ("tailwind_color_picker_cell".to_string(), "cursor-pointer border h-4 w-4".to_string()),
//
// pub tailwind_color_picker_container: ClassesContainer<T>,
// pub tailwind_color_picker_button: ClassesContainer<T>,
// pub tailwind_color_picker_selected_color: ClassesContainer<T>,
// pub tailwind_color_picker_dropdown: ClassesContainer<T>,
// pub tailwind_color_picker_row: ClassesContainer<T>,
// pub tailwind_color_picker_cell: ClassesContainer<T>,
//
// tailwind_color_picker_container: self.tailwind_color_picker_container.to_owned(),
// tailwind_color_picker_button: self.tailwind_color_picker_button.to_owned(),
// tailwind_color_picker_selected_color: self.tailwind_color_picker_selected_color.to_owned(),
// tailwind_color_picker_dropdown: self.tailwind_color_picker_dropdown.to_owned(),
// tailwind_color_picker_row: self.tailwind_color_picker_row.to_owned(),
// tailwind_color_picker_cell: self.tailwind_color_picker_cell.to_owned(),
//
// tailwind_color_picker_container: default_config_hm
// .get("tailwind_color_picker_container")
// .expect("Template parameter missing")
// .clone(),
// tailwind_color_picker_button: default_config_hm
// .get("tailwind_color_picker_button")
// .expect("Template parameter missing")
// .clone(),
// tailwind_color_picker_selected_color: default_config_hm
// .get("tailwind_color_picker_selected_color")
// .expect("Template parameter missing")
// .clone(),
// tailwind_color_picker_dropdown: default_config_hm
// .get("tailwind_color_picker_dropdown")
// .expect("Template parameter missing")
// .clone(),
// tailwind_color_picker_row: default_config_hm
// .get("tailwind_color_picker_row")
// .expect("Template parameter missing")
// .clone(),
// tailwind_color_picker_cell: default_config_hm
// .get("tailwind_color_picker_cell")
// .expect("Template parameter missing")
// .clone(),
