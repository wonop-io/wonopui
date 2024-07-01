// build.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tera::{Context, Result, Tera};
use std::fs;
use std::path::Path;
use regex::Regex;

fn is_valid_tailwind_class(class: &str) -> bool {
    // Define a comprehensive list of Tailwind CSS prefixes and standalone classes
    let valid_prefixes = [
        // Layout
        "container", "box-", "block", "inline", "flex", "grid", "table", "hidden",
        // Flexbox & Grid
        "flex-", "grid-", "order-", "col-", "row-", "gap-", "justify-", "items-", "content-", "place-",
        // Spacing
        "p-", "px-", "py-", "pt-", "pr-", "pb-", "pl-", 
        "m-", "mx-", "my-", "mt-", "mr-", "mb-", "ml-",
        "space-",
        // Sizing
        "w-", "min-w-", "max-w-", "h-", "min-h-", "max-h-",
        // Typography
        "font-", "text-", "leading-", "tracking-", "whitespace-", "break-", "truncate", "indent-",
        "list-", "align-", "uppercase", "lowercase", "capitalize", "normal-case",
        // Backgrounds
        "bg-", "from-", "via-", "to-", "gradient-",
        // Borders
        "border", "border-", "rounded", "rounded-", "divide-", "ring-", "ring-offset-",
        // Effects
        "shadow-", "opacity-", "mix-blend-", "blur-", "brightness-", "contrast-", "grayscale-", "hue-rotate-", "invert-", "saturate-", "sepia-",
        // Filters
        "filter", "backdrop-",
        // Tables
        "table-",
        // Transitions & Animation
        "transition-", "duration-", "ease-", "delay-", "animate-",
        // Transforms
        "scale-", "rotate-", "translate-", "skew-", "origin-", "transform",
        // Interactivity
        "cursor-", "select-", "resize-", "scroll-", "snap-", "touch-", "user-", "pointer-events-",
        "appearance-", "outline-", "caret-",
        // SVG
        "fill-", "stroke-",
        // Accessibility
        "sr-", "not-sr-",
        // Variants
        "hover:", "focus:", "active:", "group-hover:", "focus-within:", "focus-visible:", "disabled:",
        "dark:", "sm:", "md:", "lg:", "xl:", "2xl:", "first:", "last:", "odd:", "even:", "visited:",
        "checked:", "indeterminate:", "default:", "required:", "valid:", "invalid:", "in-range:", "out-of-range:",
        "placeholder-shown:", "autofill:", "read-only:",
        // Display
        "inline-", "flow-",
        // Position
        "static", "fixed", "absolute", "relative", "sticky", "top-", "right-", "bottom-", "left-", "inset-",
        // Visibility
        "visible", "invisible",
        // Z-index
        "z-",
        // Overflow
        "overflow-",
        // Float
        "float-", "clear-",
        // Object fit
        "object-",
        // Aspect ratio
        "aspect-",
        // Columns
        "columns-",
        // Break
        "break-",
        // Additional cases
        "prose", "prose-", "underline", "overline", "line-through", "no-underline",
        "antialiased", "subpixel-antialiased",
        "italic", "not-italic",
        "ordinal", "slashed-zero", "lining-nums", "oldstyle-nums", "proportional-nums", "tabular-nums",
        "diagonal-fractions", "stacked-fractions",
        "overscroll-", "scroll-",
        "hyphens-",
        "write-",
        "accent-",
        "decoration-",
        "placeholder-",
        "backdrop-",
        "will-change-",
        "content-",
        // Additional prefixes to cover all cases
        "group", "peer", "motion-", "print:", "rtl:", "ltr:", "open:", "closed:", "file:", "dir:", "before:", "after:",
        "marker:", "selection:", "first-of-type:", "last-of-type:", "only-of-type:", "only-child:", "empty:", "target:",
        "enabled:", "focus-visible:", "optional:", "placeholder:", "read-write:", "landscape:", "portrait:", "motion-safe:",
        "motion-reduce:", "contrast-more:", "contrast-less:", "3xl:", "4xl:", "5xl:", "6xl:", "7xl:", "8xl:", "9xl:",
        "2xs:", "xs:", "supports-", "not-", "group-", "peer-", "all:", "children:", "siblings:", "sibling:",
    ];

    // Check if the class starts with any valid prefix or is a valid standalone class
    valid_prefixes.iter().any(|&prefix| class.starts_with(prefix) || class == prefix.trim_end_matches('-'))
}

fn create_baseclasses() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = out_dir.split("target").next().expect("Failed to determine target directory");
    let target_dir = Path::new(target_dir).join("target");
    let baseclasses_path = target_dir.join("tailwindcss.txt");

    let mut classes = vec![];

    // Recursively find all .rs files and extract Tailwind CSS classes
    visit_dirs(Path::new("."), &mut |file_path| {
        if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let content = fs::read_to_string(file_path).expect("Unable to read file");
            let re = Regex::new(r#""([^"]*?)([^"\s]+:[^"\s]+)([^"]*?)""#).unwrap();
            for cap in re.captures_iter(&content) {
                let class_str = &cap[0];
                let class_re = Regex::new(r"\b[a-zA-Z0-9_-]+\b").unwrap();
                for class in class_re.find_iter(class_str) {
                    let class_str = class.as_str().to_string();
                    if is_valid_tailwind_class(&class_str) {
                        classes.push(class_str);
                    }
                }


            }
        }
    }).expect("Error visiting directories");

    classes.sort();
    classes.dedup();

    let mut file = fs::File::create(baseclasses_path).expect("Unable to create file");
    for class in &classes {
        writeln!(file, "{}", class).expect("Unable to write to file");
    }

}

// Function to recursively visit directories and apply a callback to each file
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&path);
            }
        }
    }
    Ok(())
}




























fn render_vec_to_hashmap(input: Vec<(String, String)>) -> Result<HashMap<String, String>> {
    let mut hashmap = HashMap::new();
    let mut tera = Tera::default();

    for (key, template) in input {
        let mut context = Context::new();
        for (k, v) in &hashmap {
            context.insert(k, v);
        }

        let rendered = tera.render_str(&template, &context)?;
        hashmap.insert(key, rendered);
    }

    Ok(hashmap)
}

fn get_default_config() -> Result<HashMap<String, String>> {
    let elements = vec![
        // Base styles
        ("border_light".to_string(), "border-gray-200".to_string()),
        ("border_dark".to_string(), "border-zinc-700".to_string()),
        ("border".to_string(), "border {{ border_light }} dark:{{ border_dark }}".to_string()),

        ("bg_light".to_string(), "bg-white".to_string()),
        ("bg_dark".to_string(), "bg-zinc-900".to_string()),
        ("background".to_string(), "{{ bg_light }} dark:{{ bg_dark }}".to_string()),

        ("text_light".to_string(), "text-gray-800".to_string()),
        ("text_dark".to_string(), "text-gray-100".to_string()),
        ("text".to_string(), "{{ text_light }} dark:{{ text_dark }}".to_string()),
        ("text_inverted".to_string(), "dark:{{ text_light }} {{ text_dark }}".to_string()),
        ("text_muted".to_string(), "text-gray-500 dark:text-gray-400".to_string()),
                
        ("text_container_small_padding".to_string(), "px-2 py-1".to_string()),
        ("text_container_medium_padding".to_string(), "px-3 py-2".to_string()),
        ("text_container_large_padding".to_string(), "px-4 py-3".to_string()),
        ("text_container_content_padding".to_string(), "px-6 py-4".to_string()),

        ("default_opacity_addon".to_string(), "/90".to_string()),
        ("default_opacity_addon_hover".to_string(), "/100".to_string()),        
        ("default_shade".to_string(), "500".to_string()),
        ("default_shade_lighter".to_string(), "400".to_string()),
        ("default_shade_darker".to_string(), "600".to_string()),

        ("primary_light".to_string(), "blue-{{default_shade}}".to_string()),
        ("primary_dark".to_string(), "blue-{{default_shade_darker}}".to_string()),
        ("primary_darker".to_string(), "blue-700".to_string()),
        ("primary_background".to_string(), "bg-{{ primary_light }}{{ default_opacity_addon }} dark:bg-{{ primary_dark }}".to_string()),
        ("primary_background_hover".to_string(), "hover:bg-{{ primary_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ primary_darker }}".to_string()),
        ("text_primary".to_string(), "text-white".to_string()),
        ("border_primary".to_string(), "border border-{{ primary_light }} dark:border-{{ primary_dark }}".to_string()),

        ("secondary_light".to_string(), "gray-{{default_shade}}".to_string()),
        ("secondary_dark".to_string(), "gray-{{default_shade_darker}}".to_string()),
        ("secondary_darker".to_string(), "gray-700".to_string()),
        ("secondary_background".to_string(), "bg-{{ secondary_light }}{{ default_opacity_addon }} dark:bg-{{ secondary_dark }}".to_string()),
        ("secondary_background_hover".to_string(), "hover:bg-{{ secondary_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ secondary_darker }}".to_string()),
        ("text_secondary".to_string(), "text-white".to_string()),
        ("border_secondary".to_string(), "border border-{{ secondary_light }} dark:border-{{ secondary_dark }}".to_string()),
        
        ("default_light".to_string(), "gray-100".to_string()),
        ("default_dark".to_string(), "zinc-800".to_string()),
        ("default_darker".to_string(), "zinc-700".to_string()),
        ("default_background".to_string(), "bg-{{ default_light }} dark:bg-{{ default_dark }}".to_string()),
        ("default_background_hover".to_string(), "hover:bg-{{ default_darker }} dark:hover:bg-{{ default_darker }}".to_string()),
        ("text_default".to_string(), "text-gray-700 dark:text-gray-300".to_string()),
        ("border_default".to_string(), "border border-gray-300 dark:border-gray-600".to_string()),
        
        ("error_light".to_string(), "red-{{default_shade}}".to_string()),
        ("error_dark".to_string(), "red-{{default_shade_darker}}".to_string()),
        ("error_darker".to_string(), "red-700".to_string()),
        ("error_background".to_string(), "bg-{{ error_light }}{{ default_opacity_addon }} dark:bg-{{ error_dark }}".to_string()),
        ("error_background_hover".to_string(), "hover:bg-{{ error_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ error_darker }}".to_string()),
        ("text_danger".to_string(), "text-white".to_string()),
        ("border_danger".to_string(), "border border-{{ error_light }} dark:border-{{ error_dark }}".to_string()),
        
        ("success_light".to_string(), "green-{{default_shade}}".to_string()),
        ("success_dark".to_string(), "green-{{default_shade_darker}}".to_string()),
        ("success_darker".to_string(), "green-700".to_string()),
        ("success_background".to_string(), "bg-{{ success_light }}{{ default_opacity_addon }} dark:bg-{{ success_dark }}".to_string()),
        ("success_background_hover".to_string(), "hover:bg-{{ success_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ success_darker }}".to_string()),
        ("text_success".to_string(), "text-white".to_string()),
        ("border_success".to_string(), "border border-{{ success_light }} dark:border-{{ success_dark }}".to_string()),
        
        ("warning_light".to_string(), "yellow-{{default_shade}}".to_string()),
        ("warning_dark".to_string(), "yellow-{{default_shade_darker}}".to_string()),
        ("warning_darker".to_string(), "yellow-700".to_string()),
        ("warning_background".to_string(), "bg-{{ warning_light }}{{ default_opacity_addon }} dark:bg-{{ warning_dark }}".to_string()),
        ("warning_background_hover".to_string(), "hover:bg-{{ warning_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ warning_darker }}".to_string()),
        ("text_warning".to_string(), "text-white".to_string()),
        ("border_warning".to_string(), "border border-{{ warning_light }} dark:border-{{ warning_dark }}".to_string()),
        
        ("success_all".to_string(), "{{ success_background }} {{ success_darker }} {{ text_success }} {{ border_success }}".to_string()),
        ("warning_all".to_string(), "{{ warning_background }} {{ warning_darker }} {{ text_warning }} {{ border_warning }}".to_string()),
        ("error_all".to_string(), "{{ error_background }} {{ error_darker }} {{ text_danger }} {{ border_danger }}".to_string()),
        ("info_all".to_string(), "{{ primary_background }} {{ primary_darker }} {{ text_primary }} {{ border_primary }}".to_string()),
        ("default_all".to_string(), "{{ default_background }} {{ secondary_darker }} {{ text_secondary }} {{ border_secondary }}".to_string()),
        ("primary_all".to_string(), "{{ primary_background }} {{ primary_darker }} {{ text_primary }} {{ border_primary }}".to_string()),
        ("secondary_all".to_string(), "{{ secondary_background }} {{ secondary_darker }} {{ text_secondary }} {{ border_secondary }}".to_string()),

        ("success_all_hover".to_string(), "{{ success_all }} {{ success_background_hover }}".to_string()),
        ("warning_all_hover".to_string(), "{{ warning_all }} {{ warning_background_hover }}".to_string()),
        ("error_all_hover".to_string(), "{{ error_all }} {{ error_background_hover }}".to_string()),
        ("info_all_hover".to_string(), "{{ info_all }} {{ primary_background_hover }}".to_string()),
        ("default_all_hover".to_string(), "{{ default_all }} {{ default_background_hover }}".to_string()),
        ("primary_all_hover".to_string(), "{{ primary_all }} {{ primary_background_hover }}".to_string()),
        ("secondary_all_hover".to_string(), "{{ secondary_all }} {{ secondary_background_hover }}".to_string()),

        // Sizing
        ("padding_4".to_string(), "p-4".to_string()),
        ("default_rounding_smaller".to_string(), "rounded".to_string()),
        ("default_rounding".to_string(), "rounded-md".to_string()),
        ("default_rounding_larger".to_string(), "rounded-lg".to_string()),
        ("default_shadow".to_string(), "shadow-sm".to_string()),
        ("default_shadow_larger".to_string(), "shadow-md".to_string()),

        // Typography
        ("font_bold".to_string(), "font-bold".to_string()),
        ("font_semibold".to_string(), "font-semibold".to_string()),
        ("font_medium".to_string(), "font-medium".to_string()),
        ("font_normal".to_string(), "font-normal".to_string()),
        ("font_light".to_string(), "font-light".to_string()),
        ("font_thin".to_string(), "font-thin".to_string()),

        ("text_4xl".to_string(), "text-4xl".to_string()),
        ("text_3xl".to_string(), "text-3xl".to_string()),
        ("text_2xl".to_string(), "text-2xl".to_string()),
        ("text_xl".to_string(), "text-xl".to_string()),
        ("text_lg".to_string(), "text-lg".to_string()),
        ("text_base".to_string(), "text-base".to_string()),

        ("typography_h1".to_string(), "mt-4 mb-8 {{ text }} {{ text_4xl }} {{ font_bold }} tracking-tight mb-4".to_string()),
        ("typography_h2".to_string(), "mt-3 mb-6 {{ text }} {{ text_3xl }} {{ font_semibold }} tracking-tight mb-3".to_string()),
        ("typography_h3".to_string(), "mt-4 mb-3 {{ text }} {{ text_2xl }} {{ font_semibold }} tracking-tight mb-2".to_string()),
        ("typography_h4".to_string(), "mt-2 mb-1.5 {{ text }} {{ text_xl }} {{ font_semibold }} mb-2".to_string()),
        ("typography_h5".to_string(), "mt-1 mb-1 {{ text }} {{ text_lg }} {{ font_medium }} mb-2".to_string()),
        ("typography_h6".to_string(), "mt-1 mb-0.5 {{ text }} {{ text_base }} {{ font_medium }} mb-2".to_string()),
        ("typography_p".to_string(), "my-2 {{ text }} {{ text_base }} {{ font_normal }} mb-4".to_string()),

        ("input_base".to_string(), "{{ default_rounding }} {{ border }} {{ default_shadow }} {{ background }} focus:ring-2 focus:ring-blue-500 focus:border-blue-500 w-full px-3 py-2 transition duration-150 ease-in-out".to_string()),
        ("label_base".to_string(), "block text-sm font-medium {{ text }} mb-1".to_string()),
        ("textarea_base".to_string(), "{{ default_rounding }} {{ border }} {{ default_shadow }} {{ background }} focus:ring-2 focus:ring-blue-500 focus:border-blue-500 w-full px-3 py-2 transition duration-150 ease-in-out".to_string()),

        ("default_separator".to_string(), "{{ border }}".to_string()),
        ("limit_width_content_size".to_string(), "max-w-4xl w-full".to_string()),
        ("limit_width_drawer_size".to_string(), "max-w-md w-full".to_string()),
        ("limit_width_widget_size".to_string(), "max-w-sm w-full".to_string()),

        // Alert
        ("alert_base".to_string(), "mx-auto {{ limit_width_content_size }} {{ padding_4 }} {{ default_rounding }} {{ default_shadow }} flex items-center".to_string()),
        ("alert_success".to_string(), "{{ alert_base }} {{ success_background }} {{ text }} border-l-4 border-green-500".to_string()),
        ("alert_warning".to_string(), "{{ alert_base }} {{ warning_background }} {{ text }} border-l-4 border-yellow-500".to_string()),
        ("alert_error".to_string(), "{{ alert_base }} {{ error_background }} {{ text }} border-l-4 border-red-500".to_string()),
        ("alert_info".to_string(), "{{ alert_base }} {{ primary_background }} {{ text }} border-l-4 border-blue-500".to_string()),

        // Avatar
        ("rounded_full".to_string(), "rounded-full".to_string()),
        ("avatar_small".to_string(), "w-8 h-8".to_string()),
        ("avatar_medium".to_string(), "w-12 h-12".to_string()),
        ("avatar_large".to_string(), "w-16 h-16".to_string()),
        ("avatar_base".to_string(), "{{ rounded_full }} object-cover".to_string()),

        // Badge
        ("badge_base".to_string(), "{{ font_medium }} {{ text_container_small_padding }} inline-flex items-center {{ default_rounding_smaller }} text-xs".to_string()),
        ("badge_success".to_string(), "{{ badge_base }} {{ success_all }}".to_string()),
        ("badge_warning".to_string(), "{{ badge_base }} {{ warning_all }}".to_string()),
        ("badge_error".to_string(), "{{ badge_base }} {{ error_all }}".to_string()),
        ("badge_info".to_string(), "{{ badge_base }} {{ info_all }}".to_string()),
        ("badge_default".to_string(), "{{ badge_base }} {{ default_all }}".to_string()),

        // Breadcrumb
        ("breadcrumb_base".to_string(), "flex flex-wrap items-center gap-2 text-sm {{ text_secondary }}".to_string()),
        ("breadcrumb_nav".to_string(), "{{ breadcrumb_base }}".to_string()),
        ("breadcrumb_list".to_string(), "{{ breadcrumb_base }}".to_string()),
        ("breadcrumb_item".to_string(), "inline-flex items-center gap-2".to_string()),
        ("breadcrumb_separator".to_string(), "[&>svg]:size-4".to_string()),

        // Button
        ("button_base".to_string(), "{{ text_container_medium_padding }} {{ font_medium }} {{ default_rounding }} transition-colors duration-150 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2".to_string()),
        ("button_primary".to_string(), "{{ button_base }} {{ primary_all_hover }} focus:ring-blue-500 shadow-sm".to_string()),
        ("button_secondary".to_string(), "{{ button_base }} {{ secondary_all_hover }} focus:ring-gray-500 shadow-sm".to_string()),
        ("button_danger".to_string(), "{{ button_base }} {{ error_all_hover }} focus:ring-red-500 shadow-sm".to_string()),
        ("button_success".to_string(), "{{ button_base }} {{ success_all_hover }} focus:ring-green-500 shadow-sm".to_string()),
        ("button_warning".to_string(), "{{ button_base }} {{ warning_all_hover }} focus:ring-yellow-500 shadow-sm".to_string()),
        ("button_ghost".to_string(), "{{ button_base }} {{ text }} hover:bg-gray-100 dark:hover:bg-zinc-800 focus:ring-gray-500".to_string()),
        ("button_default".to_string(), "{{ button_base }} {{ default_all_hover }} focus:ring-gray-500 shadow-sm".to_string()),

        // Calendar
        ("calendar_base".to_string(), "p-3 {{ default_rounding }} {{ border }} {{ default_shadow }}".to_string()),
        ("calendar_container".to_string(), "{{ calendar_base }}".to_string()),
        ("calendar_header".to_string(), "flex justify-between items-center mb-4".to_string()),
        ("button_base".to_string(), "{{ text_container_medium_padding }} font-semibold {{ default_rounding }}  transition-colors".to_string()),
        ("button_primary".to_string(), "{{ button_base }} {{ primary_all_hover }}".to_string()),
        ("button_secondary".to_string(), "{{ button_base }} {{ secondary_all_hover }}".to_string()),
        ("button_danger".to_string(), "{{ button_base }} {{ error_all_hover }}".to_string()),
        ("button_success".to_string(), "{{ button_base }} {{ success_all_hover }}".to_string()),
        ("button_warning".to_string(), "{{ button_base }} {{ warning_all_hover }}".to_string()),
        ("button_ghost".to_string(), "{{ button_base }} {{ text }} {{ border }}".to_string()),
        ("button_default".to_string(), "{{ button_base }} {{ default_all_hover }}".to_string()),

        // Calendar
        ("calendar_base".to_string(), "p-3 {{ default_rounding }} {{ border }} {{ border_dark }}".to_string()),
        ("calendar_container".to_string(), "{{ calendar_base }}".to_string()),
        ("calendar_header".to_string(), "flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0".to_string()),
        ("calendar_grid".to_string(), "space-y-4 rdp-caption_start rdp-caption_end".to_string()),
        ("calendar_day".to_string(), "flex justify-center pt-1 relative items-center".to_string()),

        // Carousel
        ("carousel_container".to_string(), "overflow-hidden preview flex min-h-[350px] w-full justify-center p-10 items-center".to_string()),
        ("carousel_inner".to_string(), "relative w-full max-w-xs overflow-hidden".to_string()),
        ("carousel_item_base".to_string(), "min-w-0 shrink-0 grow-0 basis-full pl-4 p-1 {{ default_rounding_larger }} {{ border }} {{ default_shadow }} flex aspect-square items-center justify-center p-6 {{ background }}".to_string()),
        ("carousel_item".to_string(), "{{ carousel_item_base }}".to_string()),
        ("carousel_item_active".to_string(), "{{ carousel_item }}".to_string()),
        ("carousel_controls".to_string(), "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground absolute h-8 w-8 {{ rounded_full }} {{ border_dark }} dark:hover:bg-zinc-700 dark:hover:text-white".to_string()),

        // Card
        ("card_container".to_string(), "{{ default_rounding_larger }} {{ border }} {{ default_shadow }} {{ background }} {{ text }} overflow-hidden".to_string()),
        ("card_header".to_string(), "flex flex-row justify-between items-center p-6".to_string()),
        ("card_title".to_string(), "{{ text_xl }} {{ font_semibold }} leading-none tracking-tight".to_string()),
        ("card_body".to_string(), "p-6".to_string()),

        // Checkbox
        ("checkbox_base".to_string(), "h-4 w-4 shrink-0 {{ default_rounding_smaller }} border {{ border_dark }} ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-500".to_string()),
        ("checkbox_checked".to_string(), "{{ primary_background }} {{ text_dark }} border-transparent".to_string()),
        ("checkbox_unchecked".to_string(), "border-gray-300 dark:border-gray-600".to_string()),
        ("checkbox_disabled".to_string(), "opacity-50 cursor-not-allowed".to_string()),
        ("checkbox_label".to_string(), "ml-2 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70".to_string()),

        // Collapsible
        ("collapsible_base".to_string(), "{{ default_rounding }} {{ border }} {{ border_dark }} text-sm".to_string()),
        ("collapsible_container".to_string(), "w-[350px] space-y-2".to_string()),
        ("collapsible_header".to_string(), "flex items-center justify-between space-x-4 px-4".to_string()),
        ("collapsible_title".to_string(), "text-sm font-semibold".to_string()),
        ("collapsible_button".to_string(), "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 {{ default_rounding }} w-9 p-0 dark:ring-offset-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white".to_string()),
        ("collapsible_content".to_string(), "{{ collapsible_base }} {{ text_container_medium_padding }}".to_string()),
        ("collapsible_item".to_string(), "{{ collapsible_content }}".to_string()),

        // Combobox
        ("combobox_button".to_string(), "inline-flex items-center whitespace-nowrap {{ default_rounding }} text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2 w-[200px] justify-between {{ border_dark }} {{ background }} dark:hover:bg-zinc-700 dark:hover:text-white".to_string()),
        ("combobox_button_open".to_string(), "bg-accent text-accent-foreground dark:bg-zinc-700 dark:text-white".to_string()),
        ("combobox_button_disabled".to_string(), "disabled:pointer-events-none disabled:opacity-50".to_string()),
        ("combobox_list".to_string(), "absolute mt-1 w-[200px] bg-background border border-input {{ default_rounding }} {{ default_shadow }} {{ border_dark }} {{ background }}".to_string()),
        ("combobox_item".to_string(), "{{ text_container_medium_padding }} cursor-pointer hover:bg-accent hover:text-accent-foreground dark:hover:bg-zinc-700 dark:hover:text-white".to_string()),
        ("combobox_item_selected".to_string(), "{{ combobox_item }}".to_string()),

        // Command
        ("command_container".to_string(), "flex h-full w-full flex-col overflow-hidden {{ background }} {{ text }} {{ default_rounding_larger }} {{ border }} {{ default_shadow }}".to_string()),
        ("command_input_wrapper".to_string(), "flex items-center border-b px-3 {{ border_dark }}".to_string()),
        ("command_icon".to_string(), "mr-2 h-4 w-4 shrink-0 opacity-50".to_string()),
        ("command_input".to_string(), "flex h-11 w-full {{ default_rounding }} bg-transparent py-3 text-sm outline-none placeholder-gray-500 disabled:cursor-not-allowed disabled:opacity-50 dark:placeholder-zinc-400".to_string()),
        ("command_list".to_string(), "max-h-[300px] overflow-y-auto overflow-x-hidden".to_string()),
        ("command_item".to_string(), "relative flex cursor-default select-none items-center {{ default_rounding_smaller}} {{ text_container_small_padding }} text-sm outline-none hover:bg-gray-200 dark:hover:bg-zinc-700".to_string()),
        ("command_item_icon".to_string(), "mr-2 h-4 w-4".to_string()),

        // Popover
        ("popover_container".to_string(), "relative inline-block".to_string()),
        ("popover_trigger".to_string(), "cursor-pointer".to_string()),
        ("popover_content".to_string(), "absolute {{ text }} {{ background }} {{ border }} {{ default_rounding }} {{ padding_4 }} z-10".to_string()),

        // Dropdown
        ("dropdown_content".to_string(), "mt-2 min-w-[200px] {{ border }} {{ text }} {{ default_rounding }} {{ default_shadow }} overflow-hidden".to_string()),
        ("dropdown_item".to_string(), "{{ text_container_medium_padding }} cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-700 flex items-center".to_string()),
        ("dropdown_item_icon".to_string(), "mr-2 h-4 w-4".to_string()),
        ("dropdown_separator".to_string(), "my-1 h-px bg-gray-200 dark:bg-zinc-600".to_string()),

        // Drawer
        ("drawer_container".to_string(), "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75 dark:bg-zinc-900 dark:bg-opacity-75".to_string()),
        ("drawer_content".to_string(), "{{ background }} {{ default_rounding_larger }} {{ default_shadow }} {{ limit_width_drawer_size }}".to_string()),
        ("drawer_header".to_string(), "{{ padding_4 }} border-b {{ border }}".to_string()),
        ("drawer_title".to_string(), "text-lg font-semibold text-gray-900 {{ text_dark }}".to_string()),
        ("drawer_description".to_string(), "text-sm text-gray-600 dark:text-zinc-400".to_string()),
        ("drawer_footer".to_string(), "{{ padding_4 }} border-t {{ border }}".to_string()),

        // Dialog
        ("dialog_container".to_string(), "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75 dark:bg-zinc-900 dark:bg-opacity-75".to_string()),
        ("dialog_content".to_string(), "{{ background }} {{ text }} {{ default_rounding_larger }} {{ default_shadow }} {{ limit_width_drawer_size }}".to_string()),
        ("dialog_header".to_string(), "{{ padding_4 }} border-b {{ border }}".to_string()),
        ("dialog_title".to_string(), "text-lg font-semibold text-gray-900 {{ text_dark }}".to_string()),
        ("dialog_description".to_string(), "text-sm text-gray-600 dark:text-zinc-400".to_string()),
        ("dialog_footer".to_string(), "{{ padding_4 }} border-t {{ border }}".to_string()),

        // Notification
        ("notification_container".to_string(), "fixed bottom-4 right-4 z-50 {{ limit_width_widget_size }} {{ background }} {{ default_shadow }} {{ default_rounding_larger }} {{ padding_4 }}".to_string()),
        ("notification_title".to_string(), "text-lg font-semibold text-gray-900 {{ text_dark }}".to_string()),
        ("notification_description".to_string(), "text-sm text-gray-600 dark:text-zinc-400".to_string()),
        ("notification_button".to_string(), "{{ border }} rounded {{ text_container_medium_padding }} {{ border_dark }}".to_string()),
        ("notification_action".to_string(), "{{ primary_background }}".to_string()),

        // Toggle
        ("toggle_base".to_string(), "h-8 w-8 rounded-full border ring-offset-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 dark:ring-offset-zinc-800".to_string()),
        ("toggle_checked".to_string(), "{{ primary_background }} {{ text_dark }}".to_string()),
        ("toggle_unchecked".to_string(), "border-blue-500 dark:border-blue-700".to_string()),
        ("toggle_disabled".to_string(), "disabled:cursor-not-allowed disabled:opacity-50".to_string()),
        ("toggle_label".to_string(), "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70".to_string()),

        ("switch_base".to_string(), "flex-shrink-0 cursor-pointer rounded border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none dark:bg-gray-700 dark:border-gray-600".to_string()),
        ("switch_checked".to_string(), "bg-indigo-600 dark:bg-indigo-500".to_string()),
        ("switch_unchecked".to_string(), "bg-gray-200 dark:bg-gray-500".to_string()),
        ("switch_disabled".to_string(), "disabled:cursor-not-allowed disabled:opacity-50 dark:disabled:bg-gray-600".to_string()),
        ("switch_label".to_string(), "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 dark:text-gray-300".to_string()),
        ("switch_thumb".to_string(), "pointer-events-none inline-block h-5 w-5 transform rounded bg-white shadow ring-0 transition duration-200 ease-in-out dark:bg-gray-300".to_string()),

        // Select
        ("select_container".to_string(), "relative inline-block text-left".to_string()),
        ("select_trigger".to_string(), "flex h-10 items-center justify-between {{ default_rounding }} {{ border }} {{ border }} {{ background }} {{ text_container_medium_padding }} text-sm ring-offset-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-[180px] {{ border_dark }} dark:ring-offset-zinc-800".to_string()),
        ("select_trigger_placeholder".to_string(), "pointer-events-none".to_string()),
        ("select_trigger_icon".to_string(), "lucide lucide-chevron-down h-4 w-4 opacity-50".to_string()),
        ("select_content_container".to_string(), "absolute mt-1 w-full {{ default_rounding }} {{ background }} {{ default_shadow }} z-10".to_string()),
        ("select_content_list".to_string(), "max-h-60 {{ default_rounding }} py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm dark:ring-zinc-700".to_string()),
        ("select_group".to_string(), "text-gray-900 {{ text_dark }}".to_string()),
        ("select_label".to_string(), "{{ text_container_medium_padding }} text-sm text-gray-700 dark:text-zinc-400".to_string()),
        ("select_item".to_string(), "text-gray-900 cursor-pointer select-none relative py-2 pl-3 pr-9 hover:bg-gray-100 dark:text-white dark:hover:bg-zinc-700".to_string()),

        // Table
        ("table_container".to_string(), "overflow-x-auto".to_string()),
        ("table".to_string(), "min-w-full {{ text }} {{ background }} {{ border }} {{ default_rounding }}".to_string()),
        ("table_head".to_string(), "bg-gray-50 dark:bg-zinc-800".to_string()),
        ("table_row".to_string(), "hover:bg-gray-50 dark:hover:bg-zinc-700".to_string()),
        ("table_cell".to_string(), "py-3 px-4".to_string()),
        ("table_body".to_string(), "divide-y {{ border }}".to_string()),
        ("table_footer".to_string(), "bg-gray-50 dark:bg-zinc-800".to_string()),


        // Tabs
        ("tabs_container".to_string(), "mt-4 space-y-4".to_string()),
        ("tabs_list".to_string(), "flex p-1 space-x-1 bg-gray-100 {{ default_rounding }} dark:bg-zinc-800".to_string()),
        ("tabs_trigger".to_string(), "flex-1 inline-flex items-center justify-center whitespace-nowrap px-3 py-1.5 text-sm font-medium {{ default_rounding }} transition-all".to_string()),
        ("tabs_trigger_inactive".to_string(), "{{ text }} hover:bg-white dark:hover:bg-zinc-700".to_string()),
        ("tabs_trigger_active".to_string(), "{{ primary_background }} {{ text_dark }} shadow-sm".to_string()),
        ("tabs_trigger_disabled".to_string(), "opacity-50 cursor-not-allowed".to_string()),
        ("tabs_content".to_string(), "mt-2 p-4 {{ border }} {{ default_rounding }} {{ background }}".to_string()),

        // GroupButton
        ("group_button_list".to_string(), "inline-flex h-10 items-center justify-center rounded-md bg-gray-100 p-1 dark:bg-zinc-800".to_string()),
        ("group_button_trigger".to_string(), "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50".to_string()),

    ];

    let final_dict = render_vec_to_hashmap(elements)?;

    Ok(final_dict)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_separator: String,
    pub typography_h1: String,
    pub typography_h2: String,
    pub typography_h3: String,
    pub typography_h4: String,
    pub typography_h5: String,
    pub typography_h6: String,
    pub typography_p: String,
    pub input_base: String,
    pub label_base: String,
    pub textarea_base: String,
    pub alert_success: String,
    pub alert_warning: String,
    pub alert_error: String,
    pub alert_info: String,
    pub alert_base: String,
    pub avatar_small: String,
    pub avatar_medium: String,
    pub avatar_large: String,
    pub avatar_base: String,
    pub badge_success: String,
    pub badge_warning: String,
    pub badge_error: String,
    pub badge_info: String,
    pub badge_default: String,
    pub badge_base: String,
    pub breadcrumb_nav: String,
    pub breadcrumb_list: String,
    pub breadcrumb_item: String,
    pub breadcrumb_separator: String,
    pub button_primary: String,
    pub button_secondary: String,
    pub button_danger: String,
    pub button_default: String,
    pub button_success: String,
    pub button_warning: String,
    pub button_ghost: String,
    pub button_base: String,
    pub calendar_container: String,
    pub calendar_header: String,
    pub calendar_grid: String,
    pub calendar_day: String,
    pub carousel_container: String,
    pub carousel_inner: String,
    pub carousel_item: String,
    pub carousel_item_active: String,
    pub carousel_controls: String,
    pub card_container: String,
    pub card_header: String,
    pub card_title: String,
    pub card_body: String,
    pub checkbox_base: String,
    pub checkbox_checked: String,
    pub checkbox_unchecked: String,
    pub checkbox_disabled: String,
    pub checkbox_label: String,

    pub collapsible_container: String,
    pub collapsible_header: String,
    pub collapsible_title: String,
    pub collapsible_button: String,
    pub collapsible_content: String,
    pub collapsible_item: String,

    pub combobox_button: String,
    pub combobox_button_open: String,
    pub combobox_button_disabled: String,
    pub combobox_list: String,
    pub combobox_item: String,
    pub combobox_item_selected: String,

    pub command_container: String,
    pub command_input_wrapper: String,
    pub command_icon: String,
    pub command_input: String,
    pub command_list: String,
    pub command_item: String,
    pub command_item_icon: String,

    pub popover_container: String,
    pub popover_trigger: String,
    pub popover_content: String,

    pub dropdown_content: String,
    pub dropdown_item: String,
    pub dropdown_item_icon: String,
    pub dropdown_separator: String,

    pub drawer_container: String,
    pub drawer_content: String,
    pub drawer_header: String,
    pub drawer_title: String,
    pub drawer_description: String,
    pub drawer_footer: String,

    pub dialog_container: String,
    pub dialog_content: String,
    pub dialog_header: String,
    pub dialog_title: String,
    pub dialog_description: String,
    pub dialog_footer: String,

    pub notification_container: String,
    pub notification_title: String,
    pub notification_description: String,
    pub notification_button: String,
    pub notification_action: String,

    pub toggle_base: String,
    pub toggle_checked: String,
    pub toggle_unchecked: String,
    pub toggle_disabled: String,
    pub toggle_label: String,

    pub switch_base: String,
    pub switch_checked: String,
    pub switch_unchecked: String,
    pub switch_disabled: String,
    pub switch_label: String,
    pub switch_thumb: String,

    pub select_container: String,
    pub select_trigger: String,
    pub select_trigger_placeholder: String,
    pub select_trigger_icon: String,
    pub select_content_container: String,
    pub select_content_list: String,
    pub select_group: String,
    pub select_label: String,
    pub select_item: String,

    pub table_container: String,
    pub table: String,
    pub table_head: String,
    pub table_row: String,
    pub table_cell: String,
    pub table_body: String,
    pub table_footer: String,

    pub tabs_container: String,
    pub tabs_list: String,
    pub tabs_trigger: String,
    pub tabs_trigger_inactive: String,
    pub tabs_trigger_active: String,
    pub tabs_trigger_disabled: String,
    pub tabs_content: String,

    pub group_button_list: String,
    pub group_button_trigger: String,
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f = File::create(&dest_path).unwrap();

    create_baseclasses();

    // Default values if config file is not provided
    let default_config_hm =
        get_default_config().expect("Unable to generate config - error in template");
    let default_config = Config {
        default_separator: default_config_hm
            .get("default_separator")
            .expect("Template parameter missing")
            .clone(),
        typography_h1: default_config_hm
            .get("typography_h1")
            .expect("Template parameter missing")
            .clone(),
        typography_h2: default_config_hm
            .get("typography_h2")
            .expect("Template parameter missing")
            .clone(),
        typography_h3: default_config_hm
            .get("typography_h3")
            .expect("Template parameter missing")
            .clone(),
        typography_h4: default_config_hm
            .get("typography_h4")
            .expect("Template parameter missing")
            .clone(),
        typography_h5: default_config_hm
            .get("typography_h5")
            .expect("Template parameter missing")
            .clone(),
        typography_h6: default_config_hm
            .get("typography_h6")
            .expect("Template parameter missing")
            .clone(),
        typography_p: default_config_hm
            .get("typography_p")
            .expect("Template parameter missing")
            .clone(),
        input_base: default_config_hm
            .get("input_base")
            .expect("Template parameter missing")
            .clone(),
        label_base: default_config_hm
            .get("label_base")
            .expect("Template parameter missing")
            .clone(),
        textarea_base: default_config_hm
            .get("textarea_base")
            .expect("Template parameter missing")
            .clone(),
        alert_success: default_config_hm
            .get("alert_success")
            .expect("Template parameter missing")
            .clone(),
        alert_warning: default_config_hm
            .get("alert_warning")
            .expect("Template parameter missing")
            .clone(),
        alert_error: default_config_hm
            .get("alert_error")
            .expect("Template parameter missing")
            .clone(),
        alert_info: default_config_hm
            .get("alert_info")
            .expect("Template parameter missing")
            .clone(),
        alert_base: default_config_hm
            .get("alert_base")
            .expect("Template parameter missing")
            .clone(),
        avatar_small: default_config_hm
            .get("avatar_small")
            .expect("Template parameter missing")
            .clone(),
        avatar_medium: default_config_hm
            .get("avatar_medium")
            .expect("Template parameter missing")
            .clone(),
        avatar_large: default_config_hm
            .get("avatar_large")
            .expect("Template parameter missing")
            .clone(),
        avatar_base: default_config_hm
            .get("avatar_base")
            .expect("Template parameter missing")
            .clone(),
        badge_success: default_config_hm
            .get("badge_success")
            .expect("Template parameter missing")
            .clone(),
        badge_warning: default_config_hm
            .get("badge_warning")
            .expect("Template parameter missing")
            .clone(),
        badge_error: default_config_hm
            .get("badge_error")
            .expect("Template parameter missing")
            .clone(),
        badge_info: default_config_hm
            .get("badge_info")
            .expect("Template parameter missing")
            .clone(),
        badge_default: default_config_hm
            .get("badge_default")
            .expect("Template parameter missing")
            .clone(),
        badge_base: default_config_hm
            .get("badge_base")
            .expect("Template parameter missing")
            .clone(),
        breadcrumb_nav: default_config_hm
            .get("breadcrumb_nav")
            .expect("Template parameter missing")
            .clone(),
        breadcrumb_list: default_config_hm
            .get("breadcrumb_list")
            .expect("Template parameter missing")
            .clone(),
        breadcrumb_item: default_config_hm
            .get("breadcrumb_item")
            .expect("Template parameter missing")
            .clone(),
        breadcrumb_separator: default_config_hm
            .get("breadcrumb_separator")
            .expect("Template parameter missing")
            .clone(),
        button_primary: default_config_hm
            .get("button_primary")
            .expect("Template parameter missing")
            .clone(),
        button_secondary: default_config_hm
            .get("button_secondary")
            .expect("Template parameter missing")
            .clone(),
        button_danger: default_config_hm
            .get("button_danger")
            .expect("Template parameter missing")
            .clone(),
        button_success: default_config_hm
            .get("button_success")
            .expect("Template parameter missing")
            .clone(),
        button_warning: default_config_hm
            .get("button_warning")
            .expect("Template parameter missing")
            .clone(),
        button_ghost: default_config_hm
            .get("button_ghost")
            .expect("Template parameter missing")
            .clone(),
        button_default: default_config_hm
            .get("button_default")
            .expect("Template parameter missing")
            .clone(),
        button_base: default_config_hm
            .get("button_base")
            .expect("Template parameter missing")
            .clone(),
        calendar_container: default_config_hm
            .get("calendar_container")
            .expect("Template parameter missing")
            .clone(),
        calendar_header: default_config_hm
            .get("calendar_header")
            .expect("Template parameter missing")
            .clone(),
        calendar_grid: default_config_hm
            .get("calendar_grid")
            .expect("Template parameter missing")
            .clone(),
        calendar_day: default_config_hm
            .get("calendar_day")
            .expect("Template parameter missing")
            .clone(),
        carousel_container: default_config_hm
            .get("carousel_container")
            .expect("Template parameter missing")
            .clone(),
        carousel_inner: default_config_hm
            .get("carousel_inner")
            .expect("Template parameter missing")
            .clone(),
        carousel_item: default_config_hm
            .get("carousel_item")
            .expect("Template parameter missing")
            .clone(),
        carousel_item_active: default_config_hm
            .get("carousel_item_active")
            .expect("Template parameter missing")
            .clone(),
        carousel_controls: default_config_hm
            .get("carousel_controls")
            .expect("Template parameter missing")
            .clone(),
        card_container: default_config_hm
            .get("card_container")
            .expect("Template parameter missing")
            .clone(),
        card_header: default_config_hm
            .get("card_header")
            .expect("Template parameter missing")
            .clone(),
        card_title: default_config_hm
            .get("card_title")
            .expect("Template parameter missing")
            .clone(),
        card_body: default_config_hm
            .get("card_body")
            .expect("Template parameter missing")
            .clone(),
        checkbox_base: default_config_hm
            .get("checkbox_base")
            .expect("Template parameter missing")
            .clone(),
        checkbox_checked: default_config_hm
            .get("checkbox_checked")
            .expect("Template parameter missing")
            .clone(),
        checkbox_unchecked: default_config_hm
            .get("checkbox_unchecked")
            .expect("Template parameter missing")
            .clone(),
        checkbox_disabled: default_config_hm
            .get("checkbox_disabled")
            .expect("Template parameter missing")
            .clone(),
        checkbox_label: default_config_hm
            .get("checkbox_label")
            .expect("Template parameter missing")
            .clone(),
        collapsible_container: default_config_hm
            .get("collapsible_container")
            .expect("Template parameter missing")
            .clone(),
        collapsible_header: default_config_hm
            .get("collapsible_header")
            .expect("Template parameter missing")
            .clone(),
        collapsible_title: default_config_hm
            .get("collapsible_title")
            .expect("Template parameter missing")
            .clone(),
        collapsible_button: default_config_hm
            .get("collapsible_button")
            .expect("Template parameter missing")
            .clone(),
        collapsible_content: default_config_hm
            .get("collapsible_content")
            .expect("Template parameter missing")
            .clone(),
        collapsible_item: default_config_hm
            .get("collapsible_item")
            .expect("Template parameter missing")
            .clone(),
        combobox_button: default_config_hm
            .get("combobox_button")
            .expect("Template parameter missing")
            .clone(),
        combobox_button_open: default_config_hm
            .get("combobox_button_open")
            .expect("Template parameter missing")
            .clone(),
        combobox_button_disabled: default_config_hm
            .get("combobox_button_disabled")
            .expect("Template parameter missing")
            .clone(),
        combobox_list: default_config_hm
            .get("combobox_list")
            .expect("Template parameter missing")
            .clone(),
        combobox_item: default_config_hm
            .get("combobox_item")
            .expect("Template parameter missing")
            .clone(),
        combobox_item_selected: default_config_hm
            .get("combobox_item_selected")
            .expect("Template parameter missing")
            .clone(),
        command_container: default_config_hm
            .get("command_container")
            .expect("Template parameter missing")
            .clone(),
        command_input_wrapper: default_config_hm
            .get("command_input_wrapper")
            .expect("Template parameter missing")
            .clone(),
        command_icon: default_config_hm
            .get("command_icon")
            .expect("Template parameter missing")
            .clone(),
        command_input: default_config_hm
            .get("command_input")
            .expect("Template parameter missing")
            .clone(),
        command_list: default_config_hm
            .get("command_list")
            .expect("Template parameter missing")
            .clone(),
        command_item: default_config_hm
            .get("command_item")
            .expect("Template parameter missing")
            .clone(),
        command_item_icon: default_config_hm
            .get("command_item_icon")
            .expect("Template parameter missing")
            .clone(),
        popover_container: default_config_hm
            .get("popover_container")
            .expect("Template parameter missing")
            .clone(),
        popover_trigger: default_config_hm
            .get("popover_trigger")
            .expect("Template parameter missing")
            .clone(),
        popover_content: default_config_hm
            .get("popover_content")
            .expect("Template parameter missing")
            .clone(),
        dropdown_content: default_config_hm
            .get("dropdown_content")
            .expect("Template parameter missing")
            .clone(),
        dropdown_item: default_config_hm
            .get("dropdown_item")
            .expect("Template parameter missing")
            .clone(),
        dropdown_item_icon: default_config_hm
            .get("dropdown_item_icon")
            .expect("Template parameter missing")
            .clone(),
        dropdown_separator: default_config_hm
            .get("dropdown_separator")
            .expect("Template parameter missing")
            .clone(),
        drawer_container: default_config_hm
            .get("drawer_container")
            .expect("Template parameter missing")
            .clone(),
        drawer_content: default_config_hm
            .get("drawer_content")
            .expect("Template parameter missing")
            .clone(),
        drawer_header: default_config_hm
            .get("drawer_header")
            .expect("Template parameter missing")
            .clone(),
        drawer_title: default_config_hm
            .get("drawer_title")
            .expect("Template parameter missing")
            .clone(),
        drawer_description: default_config_hm
            .get("drawer_description")
            .expect("Template parameter missing")
            .clone(),
        drawer_footer: default_config_hm
            .get("drawer_footer")
            .expect("Template parameter missing")
            .clone(),
        dialog_container: default_config_hm
            .get("dialog_container")
            .expect("Template parameter missing")
            .clone(),
        dialog_content: default_config_hm
            .get("dialog_content")
            .expect("Template parameter missing")
            .clone(),
        dialog_header: default_config_hm
            .get("dialog_header")
            .expect("Template parameter missing")
            .clone(),
        dialog_title: default_config_hm
            .get("dialog_title")
            .expect("Template parameter missing")
            .clone(),
        dialog_description: default_config_hm
            .get("dialog_description")
            .expect("Template parameter missing")
            .clone(),
        dialog_footer: default_config_hm
            .get("dialog_footer")
            .expect("Template parameter missing")
            .clone(),
        notification_container: default_config_hm
            .get("notification_container")
            .expect("Template parameter missing")
            .clone(),
        notification_title: default_config_hm
            .get("notification_title")
            .expect("Template parameter missing")
            .clone(),
        notification_description: default_config_hm
            .get("notification_description")
            .expect("Template parameter missing")
            .clone(),
        notification_button: default_config_hm
            .get("notification_button")
            .expect("Template parameter missing")
            .clone(),
        notification_action: default_config_hm
            .get("notification_action")
            .expect("Template parameter missing")
            .clone(),
        toggle_base: default_config_hm
            .get("toggle_base")
            .expect("Template parameter missing")
            .clone(),
        toggle_checked: default_config_hm
            .get("toggle_checked")
            .expect("Template parameter missing")
            .clone(),
        toggle_unchecked: default_config_hm
            .get("toggle_unchecked")
            .expect("Template parameter missing")
            .clone(),
        toggle_disabled: default_config_hm
            .get("toggle_disabled")
            .expect("Template parameter missing")
            .clone(),
        toggle_label: default_config_hm
            .get("toggle_label")
            .expect("Template parameter missing")
            .clone(),
        switch_base: default_config_hm
            .get("switch_base")
            .expect("Template parameter missing")
            .clone(),
        switch_checked: default_config_hm
            .get("switch_checked")
            .expect("Template parameter missing")
            .clone(),
        switch_unchecked: default_config_hm
            .get("switch_unchecked")
            .expect("Template parameter missing")
            .clone(),
        switch_disabled: default_config_hm
            .get("switch_disabled")
            .expect("Template parameter missing")
            .clone(),
        switch_label: default_config_hm
            .get("switch_label")
            .expect("Template parameter missing")
            .clone(),
        switch_thumb: default_config_hm
            .get("switch_thumb")
            .expect("Template parameter missing")
            .clone(),
        select_container: default_config_hm
            .get("select_container")
            .expect("Template parameter missing")
            .clone(),
        select_trigger: default_config_hm
            .get("select_trigger")
            .expect("Template parameter missing")
            .clone(),
        select_trigger_placeholder: default_config_hm
            .get("select_trigger_placeholder")
            .expect("Template parameter missing")
            .clone(),
        select_trigger_icon: default_config_hm
            .get("select_trigger_icon")
            .expect("Template parameter missing")
            .clone(),
        select_content_container: default_config_hm
            .get("select_content_container")
            .expect("Template parameter missing")
            .clone(),
        select_content_list: default_config_hm
            .get("select_content_list")
            .expect("Template parameter missing")
            .clone(),
        select_group: default_config_hm
            .get("select_group")
            .expect("Template parameter missing")
            .clone(),
        select_label: default_config_hm
            .get("select_label")
            .expect("Template parameter missing")
            .clone(),
        select_item: default_config_hm
            .get("select_item")
            .expect("Template parameter missing")
            .clone(),
        table_container: default_config_hm
            .get("table_container")
            .expect("Template parameter missing")
            .clone(),
        table: default_config_hm
            .get("table")
            .expect("Template parameter missing")
            .clone(),
        table_head: default_config_hm
            .get("table_head")
            .expect("Template parameter missing")
            .clone(),
        table_row: default_config_hm
            .get("table_row")
            .expect("Template parameter missing")
            .clone(),
        table_cell: default_config_hm
            .get("table_cell")
            .expect("Template parameter missing")
            .clone(),
        table_body: default_config_hm
            .get("table_body")
            .expect("Template parameter missing")
            .clone(),
        table_footer: default_config_hm
            .get("table_footer")
            .expect("Template parameter missing")
            .clone(),
        tabs_container: default_config_hm
            .get("tabs_container")
            .expect("Template parameter missing")
            .clone(),
        tabs_list: default_config_hm
            .get("tabs_list")
            .expect("Template parameter missing")
            .clone(),
        tabs_trigger: default_config_hm
            .get("tabs_trigger")
            .expect("Template parameter missing")
            .clone(),
        tabs_trigger_inactive: default_config_hm
            .get("tabs_trigger_inactive")
            .expect("Template parameter missing")
            .clone(),
        tabs_trigger_active: default_config_hm
            .get("tabs_trigger_active")
            .expect("Template parameter missing")
            .clone(),
        tabs_trigger_disabled: default_config_hm
            .get("tabs_trigger_disabled")
            .expect("Template parameter missing")
            .clone(),
        tabs_content: default_config_hm
            .get("tabs_content")
            .expect("Template parameter missing")
            .clone(),
        group_button_list: default_config_hm
            .get("group_button_list")
            .expect("Template parameter missing")
            .clone(),
        group_button_trigger: default_config_hm
            .get("group_button_trigger")
            .expect("Template parameter missing")
            .clone(),
    };

    let base_dir = out_dir.split("/target").next().unwrap();
    let fallback_path = Path::new(&base_dir).join("wonopui.json");
    // Path to the user's configuration file
    let config_path = match env::var("WONOPUI_CONFIG_PATH") {
        Ok(path) => Path::new(&path).join("wonopui.json"),
        Err(_) => fallback_path,
    };

    // Read the configuration file
    let config: Config = if Path::new(&config_path).exists() {
        println!("cargo:rerun-if-changed={}", config_path.display());
        let mut config_file = File::open(config_path).unwrap();
        let mut config_content = String::new();
        config_file.read_to_string(&mut config_content).unwrap();
        serde_json::from_str(&config_content).expect("Failed to parse wonopui.json config file")
    } else {
        default_config
    };

    // Write the configuration to [base_dir]/target/wonopui.json
    let target_config_path = Path::new(&base_dir).join("target").join("wonopui.json");
    let mut target_config_file =
        File::create(&target_config_path).expect("Failed to create target wonopui.json file");
    let config_content =
        serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    target_config_file
        .write_all(config_content.as_bytes())
        .expect("Failed to write config to target wonopui.json file");

    // Write the configuration to config.rs
    writeln!(
        f,
        "pub const BRANDGUIDE: BrandGuide = BrandGuide {{
        default_separator: \"{default_separator}\",
        typography_h1: \"{typography_h1}\",
        typography_h2: \"{typography_h2}\",
        typography_h3: \"{typography_h3}\",
        typography_h4: \"{typography_h4}\",
        typography_h5: \"{typography_h5}\",
        typography_h6: \"{typography_h6}\",
        typography_p: \"{typography_p}\",
        input_base: \"{input_base}\",
        label_base: \"{label_base}\",
        textarea_base: \"{textarea_base}\",
        alert_success: \"{alert_success}\",
        alert_warning: \"{alert_warning}\",
        alert_error: \"{alert_error}\",
        alert_info: \"{alert_info}\",
        alert_base: \"{alert_base}\",
        avatar_small: \"{avatar_small}\",
        avatar_medium: \"{avatar_medium}\",
        avatar_large: \"{avatar_large}\",
        avatar_base: \"{avatar_base}\",
        badge_success: \"{badge_success}\",
        badge_warning: \"{badge_warning}\",
        badge_error: \"{badge_error}\",
        badge_info: \"{badge_info}\",
        badge_default: \"{badge_default}\",
        badge_base: \"{badge_base}\",
        breadcrumb_nav: \"{breadcrumb_nav}\",
        breadcrumb_list: \"{breadcrumb_list}\",
        breadcrumb_item: \"{breadcrumb_item}\",
        breadcrumb_separator: \"{breadcrumb_separator}\",
        button_primary: \"{button_primary}\",
        button_secondary: \"{button_secondary}\",
        button_danger: \"{button_danger}\",
        button_default: \"{button_default}\",
        button_success: \"{button_success}\",
        button_warning: \"{button_warning}\",
        button_ghost: \"{button_ghost}\",
        button_base: \"{button_base}\",
        calendar_container: \"{calendar_container}\",
        calendar_header: \"{calendar_header}\",
        calendar_grid: \"{calendar_grid}\",
        calendar_day: \"{calendar_day}\",
        carousel_container: \"{carousel_container}\",
        carousel_inner: \"{carousel_inner}\",
        carousel_item: \"{carousel_item}\",
        carousel_item_active: \"{carousel_item_active}\",
        carousel_controls: \"{carousel_controls}\",
        card_container: \"{card_container}\",
        card_header: \"{card_header}\",
        card_title: \"{card_title}\",
        card_body: \"{card_body}\",
        checkbox_base: \"{checkbox_base}\",
        checkbox_checked: \"{checkbox_checked}\",
        checkbox_unchecked: \"{checkbox_unchecked}\",
        checkbox_disabled: \"{checkbox_disabled}\",
        checkbox_label: \"{checkbox_label}\",
        collapsible_container: \"{collapsible_container}\",
        collapsible_header: \"{collapsible_header}\",
        collapsible_title: \"{collapsible_title}\",
        collapsible_button: \"{collapsible_button}\",
        collapsible_content: \"{collapsible_content}\",
        collapsible_item: \"{collapsible_item}\",
        combobox_button: \"{combobox_button}\",
        combobox_button_open: \"{combobox_button_open}\",
        combobox_button_disabled: \"{combobox_button_disabled}\",
        combobox_list: \"{combobox_list}\",
        combobox_item: \"{combobox_item}\",
        combobox_item_selected: \"{combobox_item_selected}\",
        command_container: \"{command_container}\",
        command_input_wrapper: \"{command_input_wrapper}\",
        command_icon: \"{command_icon}\",
        command_input: \"{command_input}\",
        command_list: \"{command_list}\",
        command_item: \"{command_item}\",
        command_item_icon: \"{command_item_icon}\",
        popover_container: \"{popover_container}\",
        popover_trigger: \"{popover_trigger}\",
        popover_content: \"{popover_content}\",
        dropdown_content: \"{dropdown_content}\",
        dropdown_item: \"{dropdown_item}\",
        dropdown_item_icon: \"{dropdown_item_icon}\",
        dropdown_separator: \"{dropdown_separator}\",
        drawer_container: \"{drawer_container}\",
        drawer_content: \"{drawer_content}\",
        drawer_header: \"{drawer_header}\",
        drawer_title: \"{drawer_title}\",
        drawer_description: \"{drawer_description}\",
        drawer_footer: \"{drawer_footer}\",
        dialog_container: \"{dialog_container}\",
        dialog_content: \"{dialog_content}\",
        dialog_header: \"{dialog_header}\",
        dialog_title: \"{dialog_title}\",
        dialog_description: \"{dialog_description}\",
        dialog_footer: \"{dialog_footer}\",
        notification_container: \"{notification_container}\",
        notification_title: \"{notification_title}\",
        notification_description: \"{notification_description}\",
        notification_button: \"{notification_button}\",
        notification_action: \"{notification_action}\",
        toggle_base: \"{toggle_base}\",
        toggle_checked: \"{toggle_checked}\",
        toggle_unchecked: \"{toggle_unchecked}\",
        toggle_disabled: \"{toggle_disabled}\",
        toggle_label: \"{toggle_label}\",
        switch_base: \"{switch_base}\",
        switch_checked: \"{switch_checked}\",
        switch_unchecked: \"{switch_unchecked}\",
        switch_disabled: \"{switch_disabled}\",
        switch_label: \"{switch_label}\",
        switch_thumb: \"{switch_thumb}\",
        select_container: \"{select_container}\",
        select_trigger: \"{select_trigger}\",
        select_trigger_placeholder: \"{select_trigger_placeholder}\",
        select_trigger_icon: \"{select_trigger_icon}\",
        select_content_container: \"{select_content_container}\",
        select_content_list: \"{select_content_list}\",
        select_group: \"{select_group}\",
        select_label: \"{select_label}\",
        select_item: \"{select_item}\",
        table_container: \"{table_container}\",
        table: \"{table}\",
        table_head: \"{table_head}\",
        table_row: \"{table_row}\",
        table_cell: \"{table_cell}\",
        table_body: \"{table_body}\",
        table_footer: \"{table_footer}\",
        tabs_container: \"{tabs_container}\",
        tabs_list: \"{tabs_list}\",
        tabs_trigger: \"{tabs_trigger}\",
        tabs_trigger_inactive: \"{tabs_trigger_inactive}\",
        tabs_trigger_active: \"{tabs_trigger_active}\",
        tabs_trigger_disabled: \"{tabs_trigger_disabled}\",
        tabs_content: \"{tabs_content}\",
        group_button_list: \"{group_button_list}\",
        group_button_trigger: \"{group_button_trigger}\",
    }};",
        default_separator = config.default_separator,
        typography_h1 = config.typography_h1,
        typography_h2 = config.typography_h2,
        typography_h3 = config.typography_h3,
        typography_h4 = config.typography_h4,
        typography_h5 = config.typography_h5,
        typography_h6 = config.typography_h6,
        typography_p = config.typography_p,
        input_base = config.input_base,
        label_base = config.label_base,
        textarea_base = config.textarea_base,
        alert_success = config.alert_success,
        alert_warning = config.alert_warning,
        alert_error = config.alert_error,
        alert_info = config.alert_info,
        alert_base = config.alert_base,
        avatar_small = config.avatar_small,
        avatar_medium = config.avatar_medium,
        avatar_large = config.avatar_large,
        avatar_base = config.avatar_base,
        badge_success = config.badge_success,
        badge_warning = config.badge_warning,
        badge_error = config.badge_error,
        badge_info = config.badge_info,
        badge_default = config.badge_default,
        badge_base = config.badge_base,
        breadcrumb_nav = config.breadcrumb_nav,
        breadcrumb_list = config.breadcrumb_list,
        breadcrumb_item = config.breadcrumb_item,
        breadcrumb_separator = config.breadcrumb_separator,
        button_primary = config.button_primary,
        button_secondary = config.button_secondary,
        button_danger = config.button_danger,
        button_default = config.button_default,
        button_success = config.button_success,
        button_warning = config.button_warning,
        button_ghost = config.button_ghost,
        button_base = config.button_base,
        calendar_container = config.calendar_container,
        calendar_header = config.calendar_header,
        calendar_grid = config.calendar_grid,
        calendar_day = config.calendar_day,
        carousel_container = config.carousel_container,
        carousel_inner = config.carousel_inner,
        carousel_item = config.carousel_item,
        carousel_item_active = config.carousel_item_active,
        carousel_controls = config.carousel_controls,
        card_container = config.card_container,
        card_header = config.card_header,
        card_title = config.card_title,
        card_body = config.card_body,
        checkbox_base = config.checkbox_base,
        checkbox_checked = config.checkbox_checked,
        checkbox_unchecked = config.checkbox_unchecked,
        checkbox_disabled = config.checkbox_disabled,
        checkbox_label = config.checkbox_label,
        collapsible_container = config.collapsible_container,
        collapsible_header = config.collapsible_header,
        collapsible_title = config.collapsible_title,
        collapsible_button = config.collapsible_button,
        collapsible_content = config.collapsible_content,
        collapsible_item = config.collapsible_item,
        combobox_button = config.combobox_button,
        combobox_button_open = config.combobox_button_open,
        combobox_button_disabled = config.combobox_button_disabled,
        combobox_list = config.combobox_list,
        combobox_item = config.combobox_item,
        combobox_item_selected = config.combobox_item_selected,
        command_container = config.command_container,
        command_input_wrapper = config.command_input_wrapper,
        command_icon = config.command_icon,
        command_input = config.command_input,
        command_list = config.command_list,
        command_item = config.command_item,
        command_item_icon = config.command_item_icon,
        popover_container = config.popover_container,
        popover_trigger = config.popover_trigger,
        popover_content = config.popover_content,
        dropdown_content = config.dropdown_content,
        dropdown_item = config.dropdown_item,
        dropdown_item_icon = config.dropdown_item_icon,
        dropdown_separator = config.dropdown_separator,
        drawer_container = config.drawer_container,
        drawer_content = config.drawer_content,
        drawer_header = config.drawer_header,
        drawer_title = config.drawer_title,
        drawer_description = config.drawer_description,
        drawer_footer = config.drawer_footer,
        dialog_container = config.dialog_container,
        dialog_content = config.dialog_content,
        dialog_header = config.dialog_header,
        dialog_title = config.dialog_title,
        dialog_description = config.dialog_description,
        dialog_footer = config.dialog_footer,
        notification_container = config.notification_container,
        notification_title = config.notification_title,
        notification_description = config.notification_description,
        notification_button = config.notification_button,
        notification_action = config.notification_action,
        toggle_base = config.toggle_base,
        toggle_checked = config.toggle_checked,
        toggle_unchecked = config.toggle_unchecked,
        toggle_disabled = config.toggle_disabled,
        toggle_label = config.toggle_label,
        switch_base = config.switch_base,
        switch_checked = config.switch_checked,
        switch_unchecked = config.switch_unchecked,
        switch_disabled = config.switch_disabled,
        switch_label = config.switch_label,
        switch_thumb = config.switch_thumb,
        select_container = config.select_container,
        select_trigger = config.select_trigger,
        select_trigger_placeholder = config.select_trigger_placeholder,
        select_trigger_icon = config.select_trigger_icon,
        select_content_container = config.select_content_container,
        select_content_list = config.select_content_list,
        select_group = config.select_group,
        select_label = config.select_label,
        select_item = config.select_item,
        table_container = config.table_container,
        table = config.table,
        table_head = config.table_head,
        table_row = config.table_row,
        table_cell = config.table_cell,
        table_body = config.table_body,
        table_footer = config.table_footer,
        tabs_container = config.tabs_container,
        tabs_list = config.tabs_list,
        tabs_trigger = config.tabs_trigger,
        tabs_trigger_inactive = config.tabs_trigger_inactive,
        tabs_trigger_active = config.tabs_trigger_active,
        tabs_trigger_disabled = config.tabs_trigger_disabled,
        tabs_content = config.tabs_content,
        group_button_list = config.group_button_list,
        group_button_trigger = config.group_button_trigger,
    )
    .unwrap();
}

/*

        default_separator: "border-b border-gray-200 dark:border-zinc-700".to_string(),
        typography_h1: "text-4xl font-bold".to_string(),
        typography_h2: "text-3xl font-semibold".to_string(),
        typography_h3: "text-2xl font-medium".to_string(),
        typography_h4: "text-xl font-normal".to_string(),
        typography_h5: "text-lg font-light".to_string(),
        typography_h6: "text-base font-thin".to_string(),
        typography_p: "text-base font-normal".to_string(),
        alert_success: "mx-auto max-w-4xl w-full bg-green-500 text-white dark:bg-green-700".to_string(),
        alert_warning: "mx-auto max-w-4xl w-full bg-yellow-500 text-white dark:bg-yellow-700".to_string(),
        alert_error: "mx-auto max-w-4xl w-full bg-red-500 text-white dark:bg-red-700".to_string(),
        alert_info: "mx-auto max-w-4xl w-full bg-blue-500 text-white dark:bg-blue-700".to_string(),
        alert_base: "{{ padding_4 }} rounded".to_string(),
        avatar_small: "w-8 h-8".to_string(),
        avatar_medium: "w-16 h-16".to_string(),
        avatar_large: "w-32 h-32".to_string(),
        avatar_base: "rounded-full".to_string(),
        badge_success: "bg-green-500 dark:bg-green-700".to_string(),
        badge_warning: "bg-yellow-500 dark:bg-yellow-700".to_string(),
        badge_error: "bg-red-500 dark:bg-red-700".to_string(),
        badge_info: "bg-blue-500 dark:bg-blue-700".to_string(),
        badge_default: "bg-gray-500 dark:bg-zinc-700".to_string(),
        badge_base: "font-medium {{ text_container_small_padding }} inline-flex items-center rounded text-xs text-white text-center items-center".to_string(),
        breadcrumb_nav: "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 dark:text-zinc-400".to_string(),
        breadcrumb_list: "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 dark:text-zinc-400".to_string(),
        breadcrumb_item: "inline-flex items-center gap-1.5".to_string(),
        breadcrumb_separator: "[&>svg]:size-3.5".to_string(),
        button_primary: "bg-blue-500 text-white dark:bg-blue-700".to_string(),
        button_secondary: "bg-gray-500 text-white dark:bg-zinc-700".to_string(),
        button_danger: "bg-red-500 text-white dark:bg-red-700".to_string(),
        button_success: "bg-green-500 text-white dark:bg-green-700".to_string(),
        button_warning: "bg-yellow-500 text-white dark:bg-yellow-700".to_string(),
        button_ghost: "border border-zinc-900 dark:bg-zinc-700".to_string(),
        button_default: "bg-white text-black dark:bg-zinc-800 dark:text-white".to_string(),
        button_base: "p-2 rounded".to_string(),
        calendar_container: "rdp p-3 rounded-md border dark:border-zinc-700".to_string(),
        calendar_header: "flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0".to_string(),
        calendar_grid: "space-y-4 rdp-caption_start rdp-caption_end".to_string(),
        calendar_day: "flex justify-center pt-1 relative items-center".to_string(),
        carousel_container: "overflow-hidden preview flex min-h-[350px] w-full justify-center p-10 items-center".to_string(),
        carousel_inner: "relative w-full max-w-xs overflow-hidden".to_string(),
        carousel_item: "min-w-0 shrink-0 grow-0 basis-full pl-4 p-1 rounded-lg border {{ default_shadow }} flex aspect-square items-center justify-center p-6 dark:bg-zinc-800 dark:text-white".to_string(),
        carousel_item_active: "min-w-0 shrink-0 grow-0 basis-full pl-4 p-1 rounded-lg border {{ default_shadow }} flex aspect-square items-center justify-center p-6 dark:bg-zinc-800 dark:text-white".to_string(),
        carousel_controls: "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground absolute h-8 w-8 rounded-full dark:border-zinc-700 dark:bg-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white".to_string(),
        card_container: "rounded-lg border {{ default_shadow }} dark:bg-zinc-800 dark:text-white".to_string(),
        card_header: "flex flex-col space-y-1.5 p-6".to_string(),
        card_title: "text-2xl font-semibold leading-none tracking-tight".to_string(),
        card_body: "p-6 pt-0".to_string(),
        checkbox_base: "h-4 w-4 shrink-0 {{ default_rounding_smaller}} border ring-offset-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 dark:ring-offset-zinc-800".to_string(),
        checkbox_checked: "bg-blue-500 text-white dark:bg-blue-700".to_string(),
        checkbox_unchecked: "border-blue-500 dark:border-blue-700".to_string(),
        checkbox_disabled: "disabled:cursor-not-allowed disabled:opacity-50".to_string(),
        checkbox_label: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70".to_string(),
        collapsible_container: "w-[350px] space-y-2".to_string(),
        collapsible_header: "flex items-center justify-between space-x-4 px-4".to_string(),
        collapsible_title: "text-sm font-semibold".to_string(),
        collapsible_button: "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md w-9 p-0 dark:ring-offset-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white".to_string(),
        collapsible_content: "rounded-md border px-4 py-3 text-sm dark:border-zinc-700".to_string(),
        collapsible_item: "rounded-md border px-4 py-3 text-sm dark:border-zinc-700".to_string(),
        combobox_button: "inline-flex items-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2 w-[200px] justify-between dark:border-zinc-700 dark:bg-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white".to_string(),
        combobox_button_open: "bg-accent text-accent-foreground dark:bg-zinc-700 dark:text-white".to_string(),
        combobox_button_disabled: "disabled:pointer-events-none disabled:opacity-50".to_string(),
        combobox_list: "absolute mt-1 w-[200px] bg-background border border-input rounded-md {{ default_shadow }} dark:border-zinc-700 dark:bg-zinc-800".to_string(),
        combobox_item: "px-4 py-2 cursor-pointer hover:bg-accent hover:text-accent-foreground dark:hover:bg-zinc-700 dark:hover:text-white".to_string(),
        combobox_item_selected: "bg-accent text-accent-foreground dark:bg-zinc-700 dark:text-white".to_string(),
        command_container: "flex h-full w-full flex-col overflow-hidden bg-white text-black rounded-lg border {{ default_shadoow dark:bg-zinc-800 dark:text-white".to_string(),
        command_input_wrapper: "flex items-center border-b px-3 dark:border-zinc-700".to_string(),
        command_icon: "mr-2 h-4 w-4 shrink-0 opacity-50".to_string(),
        command_input: "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder-gray-500 disabled:cursor-not-allowed disabled:opacity-50 dark:placeholder-zinc-400".to_string(),
        command_list: "max-h-[300px] overflow-y-auto overflow-x-hidden".to_string(),
        command_item: "relative flex cursor-default select-none items-center {{ default_rounding_smaller}} {{ text_container_small_padding }}.5 text-sm outline-none hover:bg-gray-200 dark:hover:bg-zinc-700".to_string(),
        command_item_icon: "mr-2 h-4 w-4".to_string(),
        popover_container: "relative inline-block".to_string(),
        popover_trigger: "cursor-pointer".to_string(),
        popover_content: "absolute bg-white border {{ border }} rounded-md {{ default_shadow }} p-4 z-10 dark:bg-zinc-800 dark:border-zinc-700".to_string(),
        dropdown_content: "mt-1 min-w-[200px] border rounded-md {{ default_shadow }}".to_string(),
        dropdown_item: "px-4 py-2 cursor-pointer hover:bg-accent hover:text-accent-foreground flex items-center".to_string(),
        dropdown_item_icon: "mr-2".to_string(),
        drawer_container: "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75 dark:bg-zinc-900 dark:bg-opacity-75".to_string(),
        drawer_content: "bg-white rounded-lg {{ default_shadow }} {{ limit_width_drawer_size }} w-full dark:bg-zinc-800".to_string(),
        drawer_header: "p-4 border-b border-gray-200 dark:border-zinc-700".to_string(),
        drawer_title: "text-lg font-semibold text-gray-900 dark:text-white".to_string(),
        drawer_description: "text-sm text-gray-600 dark:text-zinc-400".to_string(),
        drawer_footer: "p-4 border-t border-gray-200 dark:border-zinc-700".to_string(),
        dialog_container: "fixed inset-0 z-50 flex items-center justify-center bg-gray-800 bg-opacity-75 dark:bg-zinc-900 dark:bg-opacity-75".to_string(),
        dialog_content: "bg-white rounded-lg {{ default_shadow }} {{ limit_width_drawer_size }} w-full dark:bg-zinc-800".to_string(),
        dialog_header: "p-4 border-b border-gray-200 dark:border-zinc-700".to_string(),
        dialog_title: "text-lg font-semibold text-gray-900 dark:text-white".to_string(),
        dialog_description: "text-sm text-gray-600 dark:text-zinc-400".to_string(),
        dialog_footer: "p-4 border-t border-gray-200 dark:border-zinc-700".to_string(),
        notification_container: "fixed bottom-4 right-4 z-50 smalli}} w-full bg-white {{ default_shadow }} rounded-lg p-4 dark:bg-zinc-800".to_string(),
        notification_title: "text-lg font-semibold text-gray-900 dark:text-white".to_string(),
        notification_description: "text-sm text-gray-600 dark:text-zinc-400".to_string(),
        notification_button: "border {{ border }} rounded px-4 py-2 dark:border-zinc-700".to_string(),
        notification_action: "text-blue-500 dark:text-blue-700".to_string(),
        toggle_base: "h-8 w-8 rounded-full border ring-offset-gray-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 dark:ring-offset-zinc-800".to_string(),
        toggle_checked: "bg-blue-500 text-white dark:bg-blue-700".to_string(),
        toggle_unchecked: "border-blue-500 dark:border-blue-700".to_string(),
        toggle_disabled: "disabled:cursor-not-allowed disabled:opacity-50".to_string(),
        toggle_label: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70".to_string(),
        select_container: "relative inline-block text-left".to_string(),
        select_trigger: "flex h-10 items-center justify-between rounded-md border {{ border }} bg-white px-3 py-2 text-sm ring-offset-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-[180px] dark:border-zinc-700 dark:bg-zinc-800 dark:ring-offset-zinc-800".to_string(),
        select_trigger_placeholder: "pointer-events-none".to_string(),
        select_trigger_icon: "lucide lucide-chevron-down h-4 w-4 opacity-50".to_string(),
        select_content_container: "absolute mt-1 w-full rounded-md bg-white {{ default_shadow }} z-10 dark:bg-zinc-800".to_string(),
        select_content_list: "max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm dark:ring-zinc-700".to_string(),
        select_group: "text-gray-900 dark:text-white".to_string(),
        select_label: "px-4 py-2 text-sm text-gray-700 dark:text-zinc-400".to_string(),
        select_item: "text-gray-900 cursor-pointer select-none relative py-2 pl-3 pr-9 hover:bg-gray-100 dark:text-white dark:hover:bg-zinc-700".to_string(),
        tabs_list: "p-1 bg-gray-100 flex flex-col gap-2 rounded-md dark:bg-zinc-800".to_string(),
        tabs_trigger: "flex items-center justify-center rounded-md bg-white px-4 py-2 text-sm transition-colors hover:bg-gray-100 dark:border-zinc-700 dark:bg-zinc-800 dark:hover:bg-zinc-700".to_string(),
        tabs_content: "px-4 py-3 text-sm".to_string(),
*/
