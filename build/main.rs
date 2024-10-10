mod brand_config;
mod default_config;
mod valid_classes;
use brand_config::BrandConfig as Config;
use default_config::get_default_config;
use valid_classes::is_valid_tailwind_class;

// build.rs
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

fn get_base_dir() -> PathBuf {
    let out_dir = env::var("OUT_DIR")
        .expect("Failed to get OUT_DIR environment variable in create_baseclasses()");
    let target_dir = if out_dir.contains("/target/") {
        println!("Found '/target/' in OUT_DIR: {}", out_dir);
        let parts: Vec<&str> = out_dir.split("/target/").collect();
        Path::new(parts[0]).join("target")
    } else if out_dir.contains("/release/") || out_dir.contains("/debug/") {
        let parts: Vec<&str> = out_dir.split("/release/").collect();
        let parts = if parts.len() == 1 {
            println!("Found '/debug/' in OUT_DIR: {}", out_dir);
            out_dir.split("/debug/").collect()
        } else {
            println!("Found '/release/' in OUT_DIR: {}", out_dir);
            parts
        };
        Path::new(parts[0]).to_path_buf()
    } else {
        println!("Found 'target' in OUT_DIR: {}", out_dir);

        let target_pos = out_dir
            .find("target")
            .expect("Failed to find 'target' in OUT_DIR");
        let remaining = &out_dir[target_pos..];
        let next_slash = remaining.find('/').unwrap_or(remaining.len());
        Path::new(&out_dir[..target_pos + next_slash]).to_path_buf()
    };
    fs::canonicalize(&target_dir).unwrap_or_else(|_| target_dir.to_path_buf())
}

fn create_baseclasses() {
    let target_dir = get_base_dir();
    println!("Target dir: {:#?}", target_dir);

    let baseclasses_path = target_dir.join("tailwindcss.txt");

    let mut classes = HashSet::new();

    // Recursively find all .rs files and extract Tailwind CSS classes

    visit_dirs(Path::new("."), &mut |file_path| {
        let absolute_path = fs::canonicalize(file_path).unwrap_or_else(|_| file_path.to_path_buf());

        // Ignore the actual target folder
        if absolute_path.starts_with(&target_dir)
            || file_path.starts_with("./target/")
            || file_path.starts_with("./target/")
        {
            return;
        }
        if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let start = Instant::now();

            let content = fs::read_to_string(file_path)
                .expect(&format!("Unable to read file: {:?}", file_path));
            let mut word = String::new();
            for c in content.chars() {
                if c.is_whitespace() || " ,._;(){}\"'`".contains(c) {
                    if !classes.contains(&word)
                        && !word.is_empty()
                        && is_valid_tailwind_class(&word)
                    {
                        // println!("Adding {}", word);
                        classes.insert(word.clone());
                    }
                    word.clear();
                } else {
                    word.push(c);
                }
            }
            // Check the last word if the file doesn't end with a delimiter
            if !word.is_empty() && is_valid_tailwind_class(&word) {
                classes.insert(word);
            }
            let _duration = start.elapsed();
        }
    })
    .expect("Error visiting directories in create_baseclasses()");

    let mut classes: Vec<_> = classes.into_iter().collect();
    classes.sort();
    let mut file = fs::File::create(&baseclasses_path)
        .expect(&format!("Unable to create file: {:?}", baseclasses_path));
    for class in &classes {
        writeln!(file, "{}", class)
            .expect(&format!("Unable to write to file: {:?}", baseclasses_path));
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

fn get_default_value(key: &str, map: &HashMap<String, String>) -> String {
    map.get(key)
        .expect(&format!(
            "Template parameter missing: {} in get_default_value()",
            key
        ))
        .clone()
}

fn main() {
    let out_dir =
        env::var("OUT_DIR").expect("Failed to get OUT_DIR environment variable in main()");
    let dest_path = Path::new(&out_dir).join("config.rs");

    create_baseclasses();

    // Default values if config file is not provided
    let default_config_hm =
        get_default_config().expect("Unable to generate config - error in template in main()");
    let default_config = Config {
        default_separator: get_default_value("default_separator", &default_config_hm),
        typography_h1: get_default_value("typography_h1", &default_config_hm),
        typography_h2: get_default_value("typography_h2", &default_config_hm),
        typography_h3: get_default_value("typography_h3", &default_config_hm),
        typography_h4: get_default_value("typography_h4", &default_config_hm),
        typography_h5: get_default_value("typography_h5", &default_config_hm),
        typography_h6: get_default_value("typography_h6", &default_config_hm),
        typography_p: get_default_value("typography_p", &default_config_hm),
        input_base: get_default_value("input_base", &default_config_hm),
        label_base: get_default_value("label_base", &default_config_hm),
        textarea_base: get_default_value("textarea_base", &default_config_hm),
        textarea_disabled: get_default_value("textarea_disabled", &default_config_hm),
        alert_success: get_default_value("alert_success", &default_config_hm),
        alert_warning: get_default_value("alert_warning", &default_config_hm),
        alert_error: get_default_value("alert_error", &default_config_hm),
        alert_info: get_default_value("alert_info", &default_config_hm),
        alert_base: get_default_value("alert_base", &default_config_hm),
        avatar_small: get_default_value("avatar_small", &default_config_hm),
        avatar_medium: get_default_value("avatar_medium", &default_config_hm),
        avatar_large: get_default_value("avatar_large", &default_config_hm),
        avatar_base: get_default_value("avatar_base", &default_config_hm),
        badge_success: get_default_value("badge_success", &default_config_hm),
        badge_warning: get_default_value("badge_warning", &default_config_hm),
        badge_error: get_default_value("badge_error", &default_config_hm),
        badge_info: get_default_value("badge_info", &default_config_hm),
        badge_default: get_default_value("badge_default", &default_config_hm),
        badge_base: get_default_value("badge_base", &default_config_hm),
        breadcrumb_nav: get_default_value("breadcrumb_nav", &default_config_hm),
        breadcrumb_list: get_default_value("breadcrumb_list", &default_config_hm),
        breadcrumb_item: get_default_value("breadcrumb_item", &default_config_hm),
        breadcrumb_separator: get_default_value("breadcrumb_separator", &default_config_hm),
        button_primary: get_default_value("button_primary", &default_config_hm),
        button_secondary: get_default_value("button_secondary", &default_config_hm),
        button_danger: get_default_value("button_danger", &default_config_hm),
        button_success: get_default_value("button_success", &default_config_hm),
        button_warning: get_default_value("button_warning", &default_config_hm),
        button_ghost: get_default_value("button_ghost", &default_config_hm),
        button_default: get_default_value("button_default", &default_config_hm),
        button_base: get_default_value("button_base", &default_config_hm),
        button_small: get_default_value("button_small", &default_config_hm),
        button_medium: get_default_value("button_medium", &default_config_hm),
        button_large: get_default_value("button_large", &default_config_hm),
        calendar_container: get_default_value("calendar_container", &default_config_hm),
        calendar_wrapper: get_default_value("calendar_wrapper", &default_config_hm),
        calendar_header: get_default_value("calendar_header", &default_config_hm),
        calendar_title: get_default_value("calendar_title", &default_config_hm),
        calendar_month_year: get_default_value("calendar_month_year", &default_config_hm),
        calendar_nav: get_default_value("calendar_nav", &default_config_hm),
        calendar_nav_button: get_default_value("calendar_nav_button", &default_config_hm),
        calendar_grid: get_default_value("calendar_grid", &default_config_hm),
        calendar_thead: get_default_value("calendar_thead", &default_config_hm),
        calendar_weekdays: get_default_value("calendar_weekdays", &default_config_hm),
        calendar_weekday: get_default_value("calendar_weekday", &default_config_hm),
        calendar_tbody: get_default_value("calendar_tbody", &default_config_hm),
        calendar_week: get_default_value("calendar_week", &default_config_hm),
        calendar_day: get_default_value("calendar_day", &default_config_hm),
        calendar_day_button: get_default_value("calendar_day_button", &default_config_hm),
        calendar_day_today: get_default_value("calendar_day_today", &default_config_hm),
        calendar_day_selected: get_default_value("calendar_day_selected", &default_config_hm),
        calendar_day_outside: get_default_value("calendar_day_outside", &default_config_hm),
        carousel_container: get_default_value("carousel_container", &default_config_hm),
        carousel_inner: get_default_value("carousel_inner", &default_config_hm),
        carousel_item: get_default_value("carousel_item", &default_config_hm),
        carousel_item_active: get_default_value("carousel_item_active", &default_config_hm),
        carousel_controls: get_default_value("carousel_controls", &default_config_hm),
        card_container: get_default_value("card_container", &default_config_hm),
        card_header: get_default_value("card_header", &default_config_hm),
        card_title: get_default_value("card_title", &default_config_hm),
        card_body: get_default_value("card_body", &default_config_hm),
        checkbox_base: get_default_value("checkbox_base", &default_config_hm),
        checkbox_checked: get_default_value("checkbox_checked", &default_config_hm),
        checkbox_unchecked: get_default_value("checkbox_unchecked", &default_config_hm),
        checkbox_disabled: get_default_value("checkbox_disabled", &default_config_hm),
        checkbox_label: get_default_value("checkbox_label", &default_config_hm),
        col_container: get_default_value("col_container", &default_config_hm),
        collapsible_container: get_default_value("collapsible_container", &default_config_hm),
        collapsible_header: get_default_value("collapsible_header", &default_config_hm),
        collapsible_title: get_default_value("collapsible_title", &default_config_hm),
        collapsible_button: get_default_value("collapsible_button", &default_config_hm),
        collapsible_content: get_default_value("collapsible_content", &default_config_hm),
        collapsible_item: get_default_value("collapsible_item", &default_config_hm),
        container_padding_x: get_default_value("container_padding_x", &default_config_hm),
        container_padding_y: get_default_value("container_padding_y", &default_config_hm),
        container_expanding: get_default_value("container_expanding", &default_config_hm),
        container_small: get_default_value("container_small", &default_config_hm),
        container_narrow: get_default_value("container_narrow", &default_config_hm),
        container_large: get_default_value("container_large", &default_config_hm),
        container_responsive: get_default_value("container_responsive", &default_config_hm),
        content_with_aside: get_default_value("content_with_aside", &default_config_hm),
        content_aside: get_default_value("content_aside", &default_config_hm),
        content_aside_container: get_default_value("content_aside_container", &default_config_hm),
        combobox_button: get_default_value("combobox_button", &default_config_hm),
        combobox_button_open: get_default_value("combobox_button_open", &default_config_hm),
        combobox_button_disabled: get_default_value("combobox_button_disabled", &default_config_hm),
        combobox_list: get_default_value("combobox_list", &default_config_hm),
        combobox_item: get_default_value("combobox_item", &default_config_hm),
        combobox_item_selected: get_default_value("combobox_item_selected", &default_config_hm),
        command_container: get_default_value("command_container", &default_config_hm),
        command_input_wrapper: get_default_value("command_input_wrapper", &default_config_hm),
        command_icon: get_default_value("command_icon", &default_config_hm),
        command_input: get_default_value("command_input", &default_config_hm),
        command_list: get_default_value("command_list", &default_config_hm),
        command_item: get_default_value("command_item", &default_config_hm),
        command_selected_item: get_default_value("command_selected_item", &default_config_hm),
        command_item_icon: get_default_value("command_item_icon", &default_config_hm),
        dropdown_content: get_default_value("dropdown_content", &default_config_hm),
        dropdown_item: get_default_value("dropdown_item", &default_config_hm),
        dropdown_item_icon: get_default_value("dropdown_item_icon", &default_config_hm),
        dropdown_item_disabled: get_default_value("dropdown_item_disabled", &default_config_hm),
        dropdown_item_widget: get_default_value("dropdown_item_widget", &default_config_hm),
        dropdown_separator: get_default_value("dropdown_separator", &default_config_hm),
        drawer_provider: get_default_value("drawer_provider", &default_config_hm),
        drawer_container: get_default_value("drawer_container", &default_config_hm),
        drawer_header: get_default_value("drawer_header", &default_config_hm),
        drawer_title: get_default_value("drawer_title", &default_config_hm),
        drawer_description: get_default_value("drawer_description", &default_config_hm),
        drawer_footer: get_default_value("drawer_footer", &default_config_hm),
        drawer_right: get_default_value("drawer_right", &default_config_hm),
        drawer_top: get_default_value("drawer_top", &default_config_hm),
        drawer_bottom: get_default_value("drawer_bottom", &default_config_hm),
        drawer_left: get_default_value("drawer_left", &default_config_hm),
        dialog_container: get_default_value("dialog_container", &default_config_hm),
        dialog_content: get_default_value("dialog_content", &default_config_hm),
        dialog_header: get_default_value("dialog_header", &default_config_hm),
        dialog_title: get_default_value("dialog_title", &default_config_hm),
        dialog_description: get_default_value("dialog_description", &default_config_hm),
        dialog_footer: get_default_value("dialog_footer", &default_config_hm),
        group_button_container: get_default_value("group_button_container", &default_config_hm),
        group_button_list: get_default_value("group_button_list", &default_config_hm),
        group_button_trigger: get_default_value("group_button_trigger", &default_config_hm),
        group_button_trigger_active: get_default_value(
            "group_button_trigger_active",
            &default_config_hm,
        ),
        group_button_trigger_inactive: get_default_value(
            "group_button_trigger_inactive",
            &default_config_hm,
        ),
        group_button_demo: get_default_value("group_button_demo", &default_config_hm),
        notification_container: get_default_value("notification_container", &default_config_hm),
        notification_content: get_default_value("notification_content", &default_config_hm),
        notification_title: get_default_value("notification_title", &default_config_hm),
        notification_description: get_default_value("notification_description", &default_config_hm),
        notification_timestamp: get_default_value("notification_timestamp", &default_config_hm),
        notification_close_button: get_default_value(
            "notification_close_button",
            &default_config_hm,
        ),
        notification_close_icon: get_default_value("notification_close_icon", &default_config_hm),
        notification_action_container: get_default_value(
            "notification_action_container",
            &default_config_hm,
        ),
        notification_list_container: get_default_value(
            "notification_list_container",
            &default_config_hm,
        ),
        page_header_container: get_default_value("page_header_container", &default_config_hm),
        page_header_title: get_default_value("page_header_title", &default_config_hm),
        page_header_actions: get_default_value("page_header_actions", &default_config_hm),

        pagination_container: get_default_value("pagination_container", &default_config_hm),
        pagination_list: get_default_value("pagination_list", &default_config_hm),
        pagination_item: get_default_value("pagination_item", &default_config_hm),
        pagination_item_current: get_default_value("pagination_item_current", &default_config_hm),

        placeholder_container: get_default_value("placeholder_container", &default_config_hm),
        placeholder_svg: get_default_value("placeholder_svg", &default_config_hm),
        placeholder_text: get_default_value("placeholder_text", &default_config_hm),
        popover_container: get_default_value("popover_container", &default_config_hm),
        popover_trigger: get_default_value("popover_trigger", &default_config_hm),
        popover_content: get_default_value("popover_content", &default_config_hm),
        popover_position_north_start: get_default_value(
            "popover_position_north_start",
            &default_config_hm,
        ),
        popover_position_north_middle: get_default_value(
            "popover_position_north_middle",
            &default_config_hm,
        ),
        popover_position_north_end: get_default_value(
            "popover_position_north_end",
            &default_config_hm,
        ),
        popover_position_south_start: get_default_value(
            "popover_position_south_start",
            &default_config_hm,
        ),
        popover_position_south_middle: get_default_value(
            "popover_position_south_middle",
            &default_config_hm,
        ),
        popover_position_south_end: get_default_value(
            "popover_position_south_end",
            &default_config_hm,
        ),
        popover_position_east_start: get_default_value(
            "popover_position_east_start",
            &default_config_hm,
        ),
        popover_position_east_middle: get_default_value(
            "popover_position_east_middle",
            &default_config_hm,
        ),
        popover_position_east_end: get_default_value(
            "popover_position_east_end",
            &default_config_hm,
        ),
        popover_position_west_start: get_default_value(
            "popover_position_west_start",
            &default_config_hm,
        ),
        popover_position_west_middle: get_default_value(
            "popover_position_west_middle",
            &default_config_hm,
        ),
        popover_position_west_end: get_default_value(
            "popover_position_west_end",
            &default_config_hm,
        ),
        resizable_container: get_default_value("resizable_container", &default_config_hm),
        resizable_box: get_default_value("resizable_box", &default_config_hm),
        resizable_handle_visible: get_default_value("resizable_handle_visible", &default_config_hm),
        resizable_handle_hidden: get_default_value("resizable_handle_hidden", &default_config_hm),
        resizable_handle_nw: get_default_value("resizable_handle_nw", &default_config_hm),
        resizable_handle_ne: get_default_value("resizable_handle_ne", &default_config_hm),
        resizable_handle_sw: get_default_value("resizable_handle_sw", &default_config_hm),
        resizable_handle_se: get_default_value("resizable_handle_se", &default_config_hm),
        resizable_handle_n: get_default_value("resizable_handle_n", &default_config_hm),
        resizable_handle_s: get_default_value("resizable_handle_s", &default_config_hm),
        resizable_handle_w: get_default_value("resizable_handle_w", &default_config_hm),
        resizable_handle_e: get_default_value("resizable_handle_e", &default_config_hm),
        selectable_indicator: get_default_value("selectable_indicator", &default_config_hm),
        selectable_hover: get_default_value("selectable_hover", &default_config_hm),
        selectable_selected: get_default_value("selectable_selected", &default_config_hm),
        selectable_cursor: get_default_value("selectable_cursor", &default_config_hm),
        toggle_container: get_default_value("toggle_container", &default_config_hm),
        toggle_base: get_default_value("toggle_base", &default_config_hm),
        toggle_checked: get_default_value("toggle_checked", &default_config_hm),
        toggle_unchecked: get_default_value("toggle_unchecked", &default_config_hm),
        toggle_disabled: get_default_value("toggle_disabled", &default_config_hm),
        toggle_label: get_default_value("toggle_label", &default_config_hm),
        toggle_icon: get_default_value("toggle_icon", &default_config_hm),
        switch_base: get_default_value("switch_base", &default_config_hm),
        switch_thumb: get_default_value("switch_thumb", &default_config_hm),
        switch_checked: get_default_value("switch_checked", &default_config_hm),
        switch_unchecked: get_default_value("switch_unchecked", &default_config_hm),
        switch_translate_checked: get_default_value("switch_translate_checked", &default_config_hm),
        switch_translate_unchecked: get_default_value(
            "switch_translate_unchecked",
            &default_config_hm,
        ),
        switch_disabled: get_default_value("switch_disabled", &default_config_hm),
        switch_label: get_default_value("switch_label", &default_config_hm),
        select_container: get_default_value("select_container", &default_config_hm),
        select_trigger: get_default_value("select_trigger", &default_config_hm),
        select_trigger_placeholder: get_default_value(
            "select_trigger_placeholder",
            &default_config_hm,
        ),
        select_trigger_icon: get_default_value("select_trigger_icon", &default_config_hm),
        select_content_container: get_default_value("select_content_container", &default_config_hm),
        select_content_list: get_default_value("select_content_list", &default_config_hm),
        select_group: get_default_value("select_group", &default_config_hm),
        select_label: get_default_value("select_label", &default_config_hm),
        select_item: get_default_value("select_item", &default_config_hm),
        table_container: get_default_value("table_container", &default_config_hm),
        table: get_default_value("table", &default_config_hm),
        table_head: get_default_value("table_head", &default_config_hm),
        table_row: get_default_value("table_row", &default_config_hm),
        table_head_row: get_default_value("table_head_row", &default_config_hm),
        table_cell: get_default_value("table_cell", &default_config_hm),
        table_body: get_default_value("table_body", &default_config_hm),
        table_footer: get_default_value("table_footer", &default_config_hm),
        tabs_container: get_default_value("tabs_container", &default_config_hm),
        tabs_list: get_default_value("tabs_list", &default_config_hm),
        tabs_list_column: get_default_value("tabs_list_column", &default_config_hm),
        tabs_list_row: get_default_value("tabs_list_row", &default_config_hm),
        tabs_trigger: get_default_value("tabs_trigger", &default_config_hm),
        tabs_trigger_inactive: get_default_value("tabs_trigger_inactive", &default_config_hm),
        tabs_trigger_active: get_default_value("tabs_trigger_active", &default_config_hm),
        tabs_trigger_disabled: get_default_value("tabs_trigger_disabled", &default_config_hm),
        tabs_content: get_default_value("tabs_content", &default_config_hm),
        tag_input_container: get_default_value("tag_input_container", &default_config_hm),
        tag_input_tags_container: get_default_value("tag_input_tags_container", &default_config_hm),
        tag_input_tag: get_default_value("tag_input_tag", &default_config_hm),
        tag_input_remove_button: get_default_value("tag_input_remove_button", &default_config_hm),
        tag_input_input: get_default_value("tag_input_input", &default_config_hm),
        tag_input_candidates_container: get_default_value(
            "tag_input_candidates_container",
            &default_config_hm,
        ),
        tag_input_candidate_button: get_default_value(
            "tag_input_candidate_button",
            &default_config_hm,
        ),
        tailwind_color_picker_container: get_default_value(
            "tailwind_color_picker_container",
            &default_config_hm,
        ),
        tailwind_color_picker_button: get_default_value(
            "tailwind_color_picker_button",
            &default_config_hm,
        ),
        tailwind_color_picker_selected_color: get_default_value(
            "tailwind_color_picker_selected_color",
            &default_config_hm,
        ),
        tailwind_color_picker_dropdown: get_default_value(
            "tailwind_color_picker_dropdown",
            &default_config_hm,
        ),
        tailwind_color_picker_row: get_default_value(
            "tailwind_color_picker_row",
            &default_config_hm,
        ),
        tailwind_color_picker_cell: get_default_value(
            "tailwind_color_picker_cell",
            &default_config_hm,
        ),
        accordion_container: get_default_value("accordion_container", &default_config_hm),
        accordion_header: get_default_value("accordion_header", &default_config_hm),
        accordion_title: get_default_value("accordion_title", &default_config_hm),
        accordion_content: get_default_value("accordion_content", &default_config_hm),
    };

    let base_dir = get_base_dir();
    let fallback_path = base_dir.join("wonopui.json");
    // Path to the user's configuration file
    let (using_custom_config, config_path) = match env::var("WONOPUI_CONFIG_PATH") {
        Ok(path) => (true, Path::new(&path).join("wonopui.json")),
        Err(_) => (false, fallback_path),
    };

    // Read the configuration file
    println!("cargo:rerun-if-changed={}", config_path.display());
    let config: Config = if !using_custom_config {
        default_config
    } else {
        let mut config_file = File::open(&config_path)
            .expect(&format!("Failed to open config file: {:?}", config_path));
        let mut config_content = String::new();
        config_file
            .read_to_string(&mut config_content)
            .expect(&format!("Failed to read config file: {:?}", config_path));
        serde_json::from_str(&config_content).expect(&format!(
            "Failed to parse wonopui.json config file: {:?}",
            config_path
        ))
    };

    // Write the configuration to [base_dir]/target/wonopui.json
    let target_config_path = Path::new(&base_dir).join("wonopui.json");
    let mut target_config_file = File::create(&target_config_path).expect(&format!(
        "Failed to create target wonopui.json file: {:?}",
        target_config_path
    ));
    let config_content =
        serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    target_config_file
        .write_all(config_content.as_bytes())
        .expect("Failed to write config to target wonopui.json file");

    // Write the configuration to config.rs
    config
        .write_config_to_file(&dest_path)
        .expect("Failed to write config to file");
}
