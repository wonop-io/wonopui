//! Tailwind Color Picker component for WonopUI.
//!
//! A color picker that shows Tailwind CSS color palette.

use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the TailwindColorPicker component
pub mod classes {
    pub const CONTAINER: &str = "p-4 bg-background border rounded-md";
    pub const COLOR_GROUP: &str = "mb-4";
    pub const COLOR_GROUP_LABEL: &str = "text-sm font-medium mb-2 capitalize";
    pub const SWATCHES: &str = "flex gap-1";
    pub const SWATCH: &str = "w-6 h-6 rounded cursor-pointer hover:ring-2 hover:ring-offset-2 hover:ring-gray-400 transition-all";
    pub const SWATCH_SELECTED: &str = "w-6 h-6 rounded cursor-pointer ring-2 ring-offset-2 ring-gray-800 dark:ring-white";
    pub const SELECTED_PREVIEW: &str = "mt-4 p-3 rounded-md flex items-center gap-3";
    pub const SELECTED_COLOR_BOX: &str = "w-10 h-10 rounded border";
    pub const SELECTED_INFO: &str = "flex flex-col";
    pub const SELECTED_NAME: &str = "font-medium text-sm";
    pub const SELECTED_VALUE: &str = "text-xs text-muted-foreground font-mono";
}

/// Tailwind color palette
pub const TAILWIND_COLORS: &[(&str, &[(&str, &str)])] = &[
    ("slate", &[
        ("50", "#f8fafc"), ("100", "#f1f5f9"), ("200", "#e2e8f0"), ("300", "#cbd5e1"),
        ("400", "#94a3b8"), ("500", "#64748b"), ("600", "#475569"), ("700", "#334155"),
        ("800", "#1e293b"), ("900", "#0f172a"), ("950", "#020617"),
    ]),
    ("gray", &[
        ("50", "#f9fafb"), ("100", "#f3f4f6"), ("200", "#e5e7eb"), ("300", "#d1d5db"),
        ("400", "#9ca3af"), ("500", "#6b7280"), ("600", "#4b5563"), ("700", "#374151"),
        ("800", "#1f2937"), ("900", "#111827"), ("950", "#030712"),
    ]),
    ("red", &[
        ("50", "#fef2f2"), ("100", "#fee2e2"), ("200", "#fecaca"), ("300", "#fca5a5"),
        ("400", "#f87171"), ("500", "#ef4444"), ("600", "#dc2626"), ("700", "#b91c1c"),
        ("800", "#991b1b"), ("900", "#7f1d1d"), ("950", "#450a0a"),
    ]),
    ("orange", &[
        ("50", "#fff7ed"), ("100", "#ffedd5"), ("200", "#fed7aa"), ("300", "#fdba74"),
        ("400", "#fb923c"), ("500", "#f97316"), ("600", "#ea580c"), ("700", "#c2410c"),
        ("800", "#9a3412"), ("900", "#7c2d12"), ("950", "#431407"),
    ]),
    ("yellow", &[
        ("50", "#fefce8"), ("100", "#fef9c3"), ("200", "#fef08a"), ("300", "#fde047"),
        ("400", "#facc15"), ("500", "#eab308"), ("600", "#ca8a04"), ("700", "#a16207"),
        ("800", "#854d0e"), ("900", "#713f12"), ("950", "#422006"),
    ]),
    ("green", &[
        ("50", "#f0fdf4"), ("100", "#dcfce7"), ("200", "#bbf7d0"), ("300", "#86efac"),
        ("400", "#4ade80"), ("500", "#22c55e"), ("600", "#16a34a"), ("700", "#15803d"),
        ("800", "#166534"), ("900", "#14532d"), ("950", "#052e16"),
    ]),
    ("blue", &[
        ("50", "#eff6ff"), ("100", "#dbeafe"), ("200", "#bfdbfe"), ("300", "#93c5fd"),
        ("400", "#60a5fa"), ("500", "#3b82f6"), ("600", "#2563eb"), ("700", "#1d4ed8"),
        ("800", "#1e40af"), ("900", "#1e3a8a"), ("950", "#172554"),
    ]),
    ("purple", &[
        ("50", "#faf5ff"), ("100", "#f3e8ff"), ("200", "#e9d5ff"), ("300", "#d8b4fe"),
        ("400", "#c084fc"), ("500", "#a855f7"), ("600", "#9333ea"), ("700", "#7e22ce"),
        ("800", "#6b21a8"), ("900", "#581c87"), ("950", "#3b0764"),
    ]),
    ("pink", &[
        ("50", "#fdf2f8"), ("100", "#fce7f3"), ("200", "#fbcfe8"), ("300", "#f9a8d4"),
        ("400", "#f472b6"), ("500", "#ec4899"), ("600", "#db2777"), ("700", "#be185d"),
        ("800", "#9d174d"), ("900", "#831843"), ("950", "#500724"),
    ]),
];

#[derive(Clone, PartialEq)]
pub struct SelectedColor {
    pub name: String,
    pub shade: String,
    pub hex: String,
}

#[derive(Properties, PartialEq)]
pub struct TailwindColorPickerProps {
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<SelectedColor>,
    #[prop_or_default]
    pub class: Classes,
    /// Which color groups to show (empty = all)
    #[prop_or_default]
    pub colors: Vec<String>,
    /// Current color name (e.g., "blue")
    #[prop_or_default]
    pub color: String,
    /// Current shade value (e.g., 500)
    #[prop_or_default]
    pub shade: u32,
    /// Callback when color changes with (color_name, shade)
    #[prop_or_default]
    pub oncolorchange: Callback<(String, u32)>,
}

#[function_component(TailwindColorPicker)]
pub fn tailwind_color_picker(props: &TailwindColorPickerProps) -> Html {
    let selected = use_state(|| props.value.clone());

    let on_color_click = {
        let selected = selected.clone();
        let onchange = props.onchange.clone();
        
        move |name: &str, shade: &str, hex: &str| {
            let selected = selected.clone();
            let onchange = onchange.clone();
            let name = name.to_string();
            let shade = shade.to_string();
            let hex = hex.to_string();
            
            Callback::from(move |_: MouseEvent| {
                selected.set(Some(hex.clone()));
                onchange.emit(SelectedColor {
                    name: name.clone(),
                    shade: shade.clone(),
                    hex: hex.clone(),
                });
            })
        }
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);
    
    // Filter colors if specified
    let colors_to_show: Vec<_> = if props.colors.is_empty() {
        TAILWIND_COLORS.to_vec()
    } else {
        TAILWIND_COLORS.iter()
            .filter(|(name, _)| props.colors.contains(&name.to_string()))
            .cloned()
            .collect()
    };

    // Find selected color info
    let selected_info = selected.as_ref().and_then(|hex| {
        for (name, shades) in TAILWIND_COLORS.iter() {
            for (shade, h) in shades.iter() {
                if h == hex {
                    return Some(SelectedColor {
                        name: name.to_string(),
                        shade: shade.to_string(),
                        hex: hex.clone(),
                    });
                }
            }
        }
        None
    });

    html! {
        <div class={container_class}>
            { for colors_to_show.iter().map(|(name, shades)| {
                html! {
                    <div class={classes::COLOR_GROUP}>
                        <div class={classes::COLOR_GROUP_LABEL}>{ name }</div>
                        <div class={classes::SWATCHES}>
                            { for shades.iter().map(|(shade, hex)| {
                                let is_selected = selected.as_ref() == Some(&hex.to_string());
                                let swatch_class = if is_selected {
                                    classes::SWATCH_SELECTED
                                } else {
                                    classes::SWATCH
                                };
                                let onclick = on_color_click(name, shade, hex);
                                html! {
                                    <button
                                        class={swatch_class}
                                        style={format!("background-color: {}", hex)}
                                        onclick={onclick}
                                        title={format!("{}-{}: {}", name, shade, hex)}
                                    />
                                }
                            }) }
                        </div>
                    </div>
                }
            }) }
            
            if let Some(info) = selected_info {
                <div class={classes::SELECTED_PREVIEW}>
                    <div class={classes::SELECTED_COLOR_BOX} style={format!("background-color: {}", info.hex)} />
                    <div class={classes::SELECTED_INFO}>
                        <span class={classes::SELECTED_NAME}>{ format!("{}-{}", info.name, info.shade) }</span>
                        <span class={classes::SELECTED_VALUE}>{ &info.hex }</span>
                    </div>
                </div>
            }
        </div>
    }
}
