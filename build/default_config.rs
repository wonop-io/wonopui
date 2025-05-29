use std::collections::HashMap;
use tera::{Context, Result, Tera};

pub fn get_default_config() -> Result<HashMap<String, String>> {
    let elements = vec![
        // Base styles
        ("border_light".to_string(), "border-zinc-200".to_string()),
        ("border_dark".to_string(), "border-zinc-700".to_string()),
        ("border".to_string(), "border {{ border_light }} dark:{{ border_dark }}".to_string()),

        ("accordion_container".to_string(), "[&:not(:last-child)]:border-b border-zinc-200 dark:border-zinc-700 dark:text-zinc-100".to_string()),
        ("accordion_header".to_string(), "flex justify-between items-center py-4 cursor-pointer".to_string()),
        ("accordion_title".to_string(), "text-lg font-medium text-zinc-800 dark:text-zinc-100".to_string()),
        ("accordion_content".to_string(), "py-2 text-zinc-600 dark:text-zinc-300 mb-8".to_string()),
        // Candidate upgrade:
        // ("accordion_content".to_string(), "py-2 text-zinc-600 dark:text-zinc-300 mb-10 overflow-hidden transition-[max-height] duration-300 ease-in-out max-h-0".to_string()),
        // ("accordion_content_open".to_string(), "max-h-[1000px]".to_string()),



        ("bg_light".to_string(), "bg-white".to_string()),
        ("bg_dark".to_string(), "bg-zinc-900".to_string()),
        ("background".to_string(), "{{ bg_light }} dark:{{ bg_dark }}".to_string()),

        ("text_light".to_string(), "text-zinc-800".to_string()),
        ("text_dark".to_string(), "text-zinc-100".to_string()),
        ("text".to_string(), "{{ text_light }} dark:{{ text_dark }}".to_string()),
        ("text_inverted".to_string(), "dark:{{ text_light }} {{ text_dark }}".to_string()),
        ("text_muted".to_string(), "text-zinc-500 dark:text-zinc-400".to_string()),

        ("text_container_small_padding".to_string(), "px-2.5 py-1.5".to_string()),
        ("text_container_medium_padding".to_string(), "px-3.5 py-2.5".to_string()),
        ("text_container_large_padding".to_string(), "px-4.5 py-3.5".to_string()),
        ("text_container_content_padding".to_string(), "px-6 py-4".to_string()),

        ("default_opacity_addon".to_string(), "/90".to_string()),
        ("default_opacity_addon_hover".to_string(), "/100".to_string()),
        ("default_shade".to_string(), "500".to_string()),
        ("default_shade_lighter".to_string(), "400".to_string()),
        ("default_shade_darker".to_string(), "600".to_string()),

        ("primary_light".to_string(), "indigo-{{default_shade}}".to_string()),
        ("primary_dark".to_string(), "indigo-{{default_shade_darker}}".to_string()),
        ("primary_darker".to_string(), "indigo-700".to_string()),
        ("primary_background".to_string(), "bg-{{ primary_light }}{{ default_opacity_addon }} dark:bg-{{ primary_dark }}".to_string()),
        ("primary_background_hover".to_string(), "hover:bg-{{ primary_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ primary_darker }}".to_string()),
        ("text_primary".to_string(), "text-white".to_string()),
        ("border_primary".to_string(), "border border-{{ primary_light }} dark:border-{{ primary_dark }}".to_string()),

        ("secondary_light".to_string(), "zinc-{{default_shade}}".to_string()),
        ("secondary_dark".to_string(), "zinc-{{default_shade_darker}}".to_string()),
        ("secondary_darker".to_string(), "zinc-700".to_string()),
        ("secondary_background".to_string(), "bg-{{ secondary_light }}{{ default_opacity_addon }} dark:bg-{{ secondary_dark }}".to_string()),
        ("secondary_background_hover".to_string(), "hover:bg-{{ secondary_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ secondary_darker }}".to_string()),
        ("text_secondary".to_string(), "text-white".to_string()),
        ("border_secondary".to_string(), "border border-{{ secondary_light }} dark:border-{{ secondary_dark }}".to_string()),

        ("default_lighter".to_string(), "zinc-200".to_string()),
        ("default_light".to_string(), "zinc-300".to_string()),
        ("default_dark".to_string(), "zinc-700".to_string()),
        ("default_darker".to_string(), "zinc-800".to_string()),
        ("default_background".to_string(), "bg-{{ default_light }} dark:bg-{{ default_dark }}".to_string()),
        ("default_background_hover".to_string(), "hover:bg-{{ default_lighter }} dark:hover:bg-{{ default_darker }}".to_string()),
        ("text_default".to_string(), "text-zinc-700 dark:text-zinc-300".to_string()),
        ("border_default".to_string(), "border border-zinc-300 dark:border-zinc-600".to_string()),

        ("error_light".to_string(), "red-{{default_shade}}".to_string()),
        ("error_dark".to_string(), "red-{{default_shade_darker}}".to_string()),
        ("error_darker".to_string(), "red-700".to_string()),
        ("error_background".to_string(), "bg-{{ error_light }}{{ default_opacity_addon }} dark:bg-{{ error_dark }}".to_string()),
        ("error_background_hover".to_string(), "hover:bg-{{ error_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ error_darker }}".to_string()),
        ("text_danger".to_string(), "text-white".to_string()),
        ("border_danger".to_string(), "border border-{{ error_light }} dark:border-{{ error_dark }}".to_string()),

        ("success_light".to_string(), "emerald-{{default_shade}}".to_string()),
        ("success_dark".to_string(), "emerald-{{default_shade_darker}}".to_string()),
        ("success_darker".to_string(), "emerald-700".to_string()),
        ("success_background".to_string(), "bg-{{ success_light }}{{ default_opacity_addon }} dark:bg-{{ success_dark }}".to_string()),
        ("success_background_hover".to_string(), "hover:bg-{{ success_darker }}{{ default_opacity_addon_hover }} dark:hover:bg-{{ success_darker }}".to_string()),
        ("text_success".to_string(), "text-white".to_string()),
        ("border_success".to_string(), "border border-{{ success_light }} dark:border-{{ success_dark }}".to_string()),

        ("warning_light".to_string(), "amber-{{default_shade}}".to_string()),
        ("warning_dark".to_string(), "amber-{{default_shade_darker}}".to_string()),
        ("warning_darker".to_string(), "amber-700".to_string()),
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

        ("typography_h1".to_string(), "mt-6 mb-10 {{ text }} {{ text_4xl }} {{ font_bold }} tracking-tight".to_string()),
        ("typography_h2".to_string(), "mt-5 mb-8 {{ text }} {{ text_3xl }} {{ font_semibold }} tracking-tight".to_string()),
        ("typography_h3".to_string(), "mt-4 mb-6 {{ text }} {{ text_2xl }} {{ font_semibold }} tracking-tight".to_string()),
        ("typography_h4".to_string(), "mt-3 mb-4 {{ text }} {{ text_xl }} {{ font_semibold }}".to_string()),
        ("typography_h5".to_string(), "mt-2 mb-3 {{ text }} {{ text_lg }} {{ font_medium }}".to_string()),
        ("typography_h6".to_string(), "mt-2 mb-2 {{ text }} {{ text_base }} {{ font_medium }}".to_string()),
        ("typography_p".to_string(), "my-2 {{ text }} {{ text_base }} {{ font_normal }} mb-4 leading-relaxed".to_string()),

        ("input_base".to_string(), "{{ default_rounding }} {{ border }} {{ background }} w-full px-3.5 py-2.5 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out".to_string()),
        ("label_base".to_string(), "block text-sm font-medium {{ text }} mb-1.5".to_string()),
        ("textarea_base".to_string(), "{{ default_rounding }} {{ border }} {{ background }} w-full px-3.5 py-2.5 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out".to_string()),
        ("textarea_disabled".to_string(), "opacity-50 cursor-not-allowed".to_string()),

        ("default_separator".to_string(), "{{ border }}".to_string()),
        ("limit_width_content_size".to_string(), "max-w-4xl w-full".to_string()),
        ("limit_width_drawer_size".to_string(), "max-w-md w-full".to_string()),
        ("limit_width_widget_size".to_string(), "max-w-sm w-full".to_string()),

        // Alert
        ("alert_base".to_string(), "mx-auto {{ limit_width_content_size }} {{ padding_4 }} {{ default_rounding }} bg-zinc-50 dark:bg-zinc-800 border-l-8 border border-zinc-200 dark:border-zinc-700".to_string()),
        ("alert_success".to_string(), "{{ alert_base }} {{ text }} border-l-emerald-500 dark:border-l-emerald-500".to_string()),
        ("alert_warning".to_string(), "{{ alert_base }} {{ text }} border-l-amber-500 dark:border-l-amber-500".to_string()),
        ("alert_error".to_string(), "{{ alert_base }} {{ text }} border-l-red-500 dark:border-l-red-500".to_string()),
        ("alert_info".to_string(), "{{ alert_base }} {{ text }} border-l-indigo-500 dark:border-l-indigo-500".to_string()),
        ("alert_title".to_string(), "font-semibold text-lg mb-2".to_string()),
        ("alert_description".to_string(), "text-sm".to_string()),

        // Avatar
        ("rounded_full".to_string(), "rounded-full".to_string()),
        ("avatar_small".to_string(), "w-8 h-8".to_string()),
        ("avatar_medium".to_string(), "w-12 h-12".to_string()),
        ("avatar_large".to_string(), "w-16 h-16".to_string()),
        ("avatar_base".to_string(), "{{ rounded_full }} object-cover border-2 border-white dark:border-zinc-800 shadow-sm".to_string()),

        // Badge
        ("badge_base".to_string(), "{{ font_medium }} {{ text_container_small_padding }} inline-flex items-center {{ default_rounding_smaller }} text-xs".to_string()),
        ("badge_success".to_string(), "{{ badge_base }} {{ success_all }}".to_string()),
        ("badge_warning".to_string(), "{{ badge_base }} {{ warning_all }}".to_string()),
        ("badge_error".to_string(), "{{ badge_base }} {{ error_all }}".to_string()),
        ("badge_info".to_string(), "{{ badge_base }} {{ info_all }}".to_string()),
        ("badge_default".to_string(), "{{ badge_base }} {{ default_all }}".to_string()),

        // Breadcrumb
        ("breadcrumb_base".to_string(), "flex flex-wrap items-center gap-2 text-sm {{ text_default }}".to_string()),
        ("breadcrumb_nav".to_string(), "{{ breadcrumb_base }}".to_string()),
        ("breadcrumb_list".to_string(), "{{ breadcrumb_base }}".to_string()),
        ("breadcrumb_item".to_string(), "inline-flex items-center gap-2 hover:text-indigo-500 transition-colors duration-150".to_string()),
        ("breadcrumb_separator".to_string(), "[&>svg]:size-4 text-zinc-400".to_string()),

        // Button
        ("button_base".to_string(), "{{ text_container_medium_padding }} font-semibold {{ default_rounding }} transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800".to_string()),
        ("button_primary".to_string(), "{{ button_base }} {{ primary_all_hover }} focus:ring-indigo-600".to_string()),
        ("button_secondary".to_string(), "{{ button_base }} {{ secondary_all_hover }} focus:ring-zinc-500".to_string()),
        ("button_danger".to_string(), "{{ button_base }} {{ error_all_hover }} focus:ring-red-500".to_string()),
        ("button_success".to_string(), "{{ button_base }} {{ success_all_hover }} focus:ring-emerald-500".to_string()),
        ("button_warning".to_string(), "{{ button_base }} {{ warning_all_hover }} focus:ring-amber-500".to_string()),
        ("button_ghost".to_string(), "{{ button_base }} {{ text }} {{ border }} hover:bg-zinc-100 dark:hover:bg-zinc-800 focus:ring-zinc-400".to_string()),
        ("button_default".to_string(), "{{ button_base }} {{ default_all_hover }} focus:ring-zinc-400".to_string()),
        ("button_small".to_string(), "h-8 px-3 text-sm".to_string()),
        ("button_medium".to_string(), "h-10 py-2 px-4".to_string()),
        ("button_large".to_string(), "h-12 px-6 text-lg".to_string()),

        ("calendar_container".to_string(), "p-3 dark:text-zinc-100".to_string()),
        ("calendar_wrapper".to_string(), "flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0".to_string()),
        ("calendar_header".to_string(), "space-y-4".to_string()),
        ("calendar_title".to_string(), "flex justify-center pt-1 relative items-center".to_string()),
        ("calendar_month_year".to_string(), "text-sm font-medium".to_string()),
        ("calendar_nav".to_string(), "space-x-1 flex items-center".to_string()),
        ("calendar_nav_button".to_string(), "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input hover:bg-accent hover:text-accent-foreground h-7 w-7 bg-transparent p-0 opacity-50 hover:opacity-100".to_string()),
        ("calendar_grid".to_string(), "w-full border-collapse space-y-1".to_string()),
        ("calendar_thead".to_string(), "".to_string()),
        ("calendar_weekdays".to_string(), "flex".to_string()),
        ("calendar_weekday".to_string(), "text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]".to_string()),
        ("calendar_tbody".to_string(), "".to_string()),
        ("calendar_week".to_string(), "flex w-full mt-2".to_string()),
        ("calendar_day".to_string(), "h-9 w-9 text-center text-sm p-0 relative [&:has([aria-selected].day-range-end)]:rounded-r-md [&:has([aria-selected].day-outside)]:bg-accent/50 [&:has([aria-selected])]:bg-accent first:[&:has([aria-selected])]:rounded-l-md last:[&:has([aria-selected])]:rounded-r-md focus-within:relative focus-within:z-20".to_string()),
        ("calendar_day_button".to_string(), "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-zinc-100 dark:hover:bg-zinc-800 h-9 w-9 p-0 font-normal aria-selected:opacity-100".to_string()),
        ("calendar_day_today".to_string(), "bg-accent text-accent-foreground".to_string()),
        ("calendar_day_selected".to_string(), "bg-indigo-500 text-white hover:bg-indigo-600 hover:text-white focus:bg-indigo-600 focus:text-white".to_string()),
        ("calendar_day_outside".to_string(), "text-muted-foreground opacity-50 aria-selected:bg-accent/50 aria-selected:text-muted-foreground aria-selected:opacity-30".to_string()),

        // Carousel
        ("carousel_container".to_string(), "relative overflow-hidden min-w-full min-h-full max-w-xl mx-auto".to_string()),
        ("carousel_inner".to_string(), "relative w-full h-full".to_string()),
        ("carousel_item_base".to_string(), "absolute top-0 left-0 w-full h-full transition-opacity duration-300 ease-in-out".to_string()),
        ("carousel_item".to_string(), "{{ carousel_item_base }} opacity-0 pointer-events-none".to_string()),
        ("carousel_item_active".to_string(), "{{ carousel_item_base }} opacity-100 pointer-events-auto".to_string()),
        ("carousel_controls".to_string(), "absolute top-1/2 transform -translate-y-1/2 flex justify-between w-full".to_string()),
        ("carousel_control_prev".to_string(), "{{ button_ghost }} left-2".to_string()),
        ("carousel_control_next".to_string(), "{{ button_ghost }} right-2".to_string()),

        // Card
        ("card_container".to_string(), "{{ default_rounding_larger }} {{ border }} {{ default_shadow_larger }} {{ background }} {{ text }}".to_string()),
        ("card_header".to_string(), "p-6 border-b border-zinc-200 dark:border-zinc-700".to_string()),
        ("card_title".to_string(), "{{ text_xl }} {{ font_semibold }} leading-none tracking-tight".to_string()),
        ("card_body".to_string(), "p-6".to_string()),

        // Checkbox
        ("checkbox_base".to_string(), "h-4 w-4 shrink-0 {{ default_rounding_smaller }} border {{ border_dark }} ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500 text-indigo-600 dark:text-indigo-400".to_string()),
        ("checkbox_checked".to_string(), "{{ primary_background }} {{ text_dark }} border-transparent".to_string()),
        ("checkbox_unchecked".to_string(), "border-zinc-300 dark:border-zinc-600".to_string()),
        ("checkbox_disabled".to_string(), "opacity-50 cursor-not-allowed".to_string()),
        ("checkbox_label".to_string(), "ml-2 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70".to_string()),

        // Col
        ("col_container".to_string(), "flex flex-col".to_string()),

        // Collapsible
        ("collapsible_base".to_string(), "{{ default_rounding }} {{ border }} {{ border_dark }} text-sm".to_string()),
        ("collapsible_container".to_string(), "w-[350px] space-y-2".to_string()),
        ("collapsible_header".to_string(), "flex items-center justify-between space-x-4 px-4".to_string()),
        ("collapsible_title".to_string(), "text-sm font-semibold".to_string()),
        ("collapsible_button".to_string(), "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-zinc-100 dark:hover:bg-zinc-800 h-9 {{ default_rounding }} w-9 p-0 dark:ring-offset-zinc-800".to_string()),
        ("collapsible_content".to_string(), "{{ collapsible_base }} {{ text_container_medium_padding }}".to_string()),
        ("collapsible_item".to_string(), "{{ collapsible_content }}".to_string()),

        // Container
        ("container_padding_x".to_string(), "px-4 sm:px-6 lg:px-8".to_string()),
        ("container_padding_y".to_string(), "py-4 sm:py-6 lg:py-8".to_string()),

        ("container_expanding".to_string(), "grow-1".to_string()),
        ("container_small".to_string(), "mx-auto w-full max-w-96".to_string()),
        ("container_narrow".to_string(), "mx-auto w-full max-w-3xl".to_string()),
        ("container_large".to_string(), "mx-auto w-full max-w-7xl".to_string()),
        ("container_responsive".to_string(), "mx-auto container".to_string()),

        ("content_with_aside".to_string(), "md:mr-96".to_string()),
        ("content_aside".to_string(), "fixed top-0 right-0 h-full w-96 bg-white dark:bg-zinc-800 border-l border-zinc-200 dark:border-zinc-700 overflow-y-auto hidden md:block".to_string()),
        ("content_aside_container".to_string(), "h-full".to_string()),


        // Combobox
        ("combobox_button".to_string(), "inline-flex items-center whitespace-nowrap {{ default_rounding }} text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-zinc-100 dark:hover:bg-zinc-800 h-10 px-4 py-2 w-[200px] justify-between {{ border_dark }} {{ background }}".to_string()),
        ("combobox_button_open".to_string(), "bg-zinc-100 text-zinc-900 dark:bg-zinc-800 dark:text-white".to_string()),
        ("combobox_button_disabled".to_string(), "disabled:pointer-events-none disabled:opacity-50".to_string()),
        ("combobox_list".to_string(), "absolute mt-1 w-[200px] bg-background border border-input {{ default_rounding }} {{ default_shadow }} {{ border_dark }} {{ background }}".to_string()),
        ("combobox_item".to_string(), "{{ text_container_medium_padding }} cursor-pointer hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-white".to_string()),
        ("combobox_item_selected".to_string(), "{{ combobox_item }} bg-indigo-100 text-indigo-900 dark:bg-indigo-800 dark:text-indigo-100".to_string()),

        // Command
        ("command_container".to_string(), "flex h-auto w-full flex-col overflow-hidden {{ background }} {{ text }} {{ default_rounding_larger }} {{ border }} {{ default_shadow }}".to_string()),
        ("command_input_wrapper".to_string(), "flex items-center [&:not(:last-child)]:border-b px-3 dark:border-zinc-700".to_string()),
        ("command_icon".to_string(), "mr-2 h-4 w-4 shrink-0 opacity-50".to_string()),
        ("command_input".to_string(), "flex h-11 w-full {{ default_rounding }} bg-transparent py-3 text-sm outline-none placeholder-zinc-500 disabled:cursor-not-allowed disabled:opacity-50 dark:placeholder-zinc-400".to_string()),
        ("command_list".to_string(), "max-h-[300px] overflow-y-auto overflow-x-hidden".to_string()),
        ("command_item".to_string(), "relative flex cursor-default select-none items-center {{ default_rounding_smaller}} {{ text_container_medium_padding }} text-sm outline-none hover:bg-zinc-100 dark:hover:bg-zinc-800 space-x-2".to_string()),
        ("command_selected_item".to_string(), "{{ command_item }} bg-zinc-100 dark:bg-zinc-800".to_string()),
        ("command_item_icon".to_string(), "mr-2 h-4 w-4".to_string()),

        // Dropdown
        ("dropdown_content".to_string(), "mt-2 min-w-[200px] {{ border }} {{ text }} {{ default_rounding }} {{ default_shadow }} overflow-hidden bg-white dark:bg-zinc-800".to_string()),
        ("dropdown_item".to_string(), "{{ text_container_medium_padding }} cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-700 flex items-center transition-colors duration-150".to_string()),
        ("dropdown_item_icon".to_string(), "mr-2 h-4 w-4 text-zinc-500 dark:text-zinc-400".to_string()),
        ("dropdown_item_disabled".to_string(), "opacity-50 cursor-not-allowed".to_string()),
        ("dropdown_item_widget".to_string(), "{{ text_container_medium_padding }} flex items-center".to_string()),
        ("dropdown_separator".to_string(), "my-1 h-px bg-zinc-200 dark:bg-zinc-700 border-zinc-200 dark:border-zinc-700".to_string()),

        // Drawer

        ("drawer_provider".to_string(), "fixed inset-0 z-50 flex items-center justify-center bg-zinc-900/80 dark:bg-zinc-950/90 backdrop-blur-sm".to_string()),
        ("drawer_container".to_string(), "{{ background }} {{ default_rounding_larger }} {{ default_shadow }} {{ limit_width_drawer_size }} border border-zinc-200 dark:border-zinc-700 overflow-hidden".to_string()),
        ("drawer_header".to_string(), "{{ padding_4 }} border-b {{ border }} sticky top-0 z-10 flex items-center justify-between bg-white dark:bg-zinc-800".to_string()),
        ("drawer_title".to_string(), "text-lg font-semibold text-zinc-900 dark:text-zinc-100".to_string()),
        ("drawer_description".to_string(), "text-sm text-zinc-600 dark:text-zinc-400 mt-1".to_string()),
        ("drawer_footer".to_string(), "{{ padding_4 }} border-t {{ border }} sticky bottom-0 z-10 flex justify-end space-x-2 bg-white dark:bg-zinc-800".to_string()),

        /*
        ("drawer_right".to_string(), "fixed inset-y-0 right-0 w-full sm:max-w-sm md:max-w-md lg:max-w-lg xl:max-w-xl transition-transform duration-300 ease-in-out transform translate-x-full".to_string()),
        ("drawer_top".to_string(), "fixed inset-x-0 top-0 h-1/2 max-h-96 transition-transform duration-300 ease-in-out transform -translate-y-full".to_string()),
        ("drawer_bottom".to_string(), "fixed inset-x-0 bottom-0 h-1/2 max-h-96 transition-transform duration-300 ease-in-out transform translate-y-full".to_string()),
        ("drawer_left".to_string(), "fixed inset-y-0 left-0 w-full sm:max-w-sm md:max-w-md lg:max-w-lg xl:max-w-xl transition-transform duration-300 ease-in-out transform -translate-x-full".to_string()),
        */
        ("drawer_right".to_string(), "fixed inset-y-0 right-0".to_string()),
        ("drawer_top".to_string(), "fixed inset-x-0 top-0".to_string()),
        ("drawer_bottom".to_string(), "fixed inset-x-0 bottom-0".to_string()),
        ("drawer_left".to_string(), "fixed inset-y-0 left-0".to_string()),

        // Dialog
        ("dialog_container".to_string(), "fixed inset-0 z-50 flex items-center justify-center bg-zinc-900/80 dark:bg-zinc-950/90 backdrop-blur-sm overflow-auto pointer-events-auto".to_string()),
        ("dialog_content".to_string(), "bg-white dark:bg-zinc-800 rounded-lg shadow-xl max-w-md w-full border border-zinc-200 dark:border-zinc-700 transition-all duration-300 ease-out transform".to_string()),
        // TODO: ("dialog_content_active".to_string(), "scale-95 opacity-0".to_string()),
        // ("dialog_content_inactive".to_string(), "scale-95 opacity-0".to_string()),
        ("dialog_header".to_string(), "p-4 border-b border-zinc-200 dark:border-zinc-700 flex items-center justify-between".to_string()),
        ("dialog_title".to_string(), "text-lg font-semibold text-zinc-900 dark:text-zinc-100".to_string()),
        ("dialog_description".to_string(), "text-sm text-zinc-600 dark:text-zinc-400 p-4".to_string()),
        ("dialog_footer".to_string(), "p-4 border-t border-zinc-200 dark:border-zinc-700 flex justify-end space-x-2".to_string()),

        // Group Button
        ("group_button_container".to_string(), "flex w-full".to_string()),
        ("group_button_list".to_string(), "h-10 items-center justify-center rounded-md bg-zinc-100 dark:bg-zinc-800 p-1 text-zinc-600 dark:text-zinc-300".to_string()),
        ("group_button_trigger".to_string(), "inline-flex items-center justify-center whitespace-nowrap px-3 py-1.5 text-sm font-medium transition-all ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-400 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50".to_string()),
        ("group_button_trigger_active".to_string(), "bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 shadow-sm".to_string()),
        ("group_button_trigger_inactive".to_string(), "bg-zinc-200 dark:bg-zinc-600 text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 dark:hover:bg-zinc-500".to_string()),
        ("group_button_demo".to_string(), "w-[400px]".to_string()),

        // Kanban
        ("kanban_container".to_string(), "flex overflow-x-auto gap-6 p-6 {{ text }}".to_string()),
        ("kanban_column".to_string(), "flex flex-col min-w-[300px] {{ border }} {{ default_rounding }} {{ background }} {{ default_shadow }}".to_string()),
        ("kanban_column_header".to_string(), "p-4 {{ font_semibold }} border-b border-zinc-200 dark:border-zinc-700".to_string()),
        ("kanban_column_body".to_string(), "p-3 flex-1 flex flex-col gap-3 min-h-[100px] overflow-y-auto".to_string()),
        ("kanban_column_over".to_string(), "border-2 border-dashed border-indigo-400 dark:border-indigo-500".to_string()),
        ("kanban_card".to_string(), "{{ border }} {{ default_rounding }} {{ background }} {{ default_shadow_larger }} p-4 cursor-grab active:cursor-grabbing".to_string()),
        ("kanban_card_title".to_string(), "{{ font_medium }} pb-2".to_string()),
        ("kanban_card_content".to_string(), "text-sm text-zinc-600 dark:text-zinc-400".to_string()),
        ("kanban_card_dragging".to_string(), "opacity-50 {{ default_shadow }}".to_string()),

        // Notification
        ("notification_container".to_string(), "z-50 relative {{ limit_width_widget_size }} bg-white dark:bg-zinc-800 {{ default_shadow }} {{ default_rounding_larger }} {{ padding_4 }} border border-zinc-200 dark:border-zinc-700".to_string()),
        ("notification_title".to_string(), "text-lg font-semibold text-zinc-900 dark:text-zinc-100".to_string()),
        ("notification_description".to_string(), "text-sm text-zinc-600 dark:text-zinc-400 mt-1".to_string()),
        ("notification_content".to_string(), "flex justify-between items-start".to_string()),
        ("notification_timestamp".to_string(), "text-xs text-zinc-400 dark:text-zinc-500 mt-2".to_string()),
        ("notification_close_button".to_string(), "absolute top-4 right-4 text-zinc-400 hover:text-zinc-600 dark:text-zinc-500 dark:hover:text-zinc-300 transition-colors".to_string()),
        ("notification_close_icon".to_string(), "h-5 w-5".to_string()),
        ("notification_action_container".to_string(), "mt-4 flex space-x-2".to_string()),
        ("notification_list_container".to_string(), "fixed bottom-4 right-4 z-50 space-y-4 flex flex-col".to_string()),

        // Page Header
        ("page_header_container".to_string(), "flex justify-between items-center mb-8 border-b border-zinc-200 dark:border-zinc-700 pb-4".to_string()),
        ("page_header_title".to_string(), "text-3xl font-bold text-zinc-900 dark:text-zinc-100".to_string()),
        ("page_header_actions".to_string(), "flex space-x-2".to_string()),

        ("pagination_container".to_string(), "flex justify-center mt-8".to_string()),
        ("pagination_list".to_string(), "inline-flex items-center -space-x-px".to_string()),
        ("pagination_item".to_string(), "px-3 py-2 leading-tight {{ text }} {{ border }} {{ background }} hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors duration-150".to_string()),
        ("pagination_item_current".to_string(), "z-10 px-3 py-2 leading-tight {{ primary_background }} {{ text_primary }} {{ border_primary }} hover:bg-indigo-600 dark:hover:bg-indigo-700".to_string()),

        // Placeholder
        ("placeholder_container".to_string(), "relative overflow-hidden rounded-md border border-zinc-200 dark:border-zinc-700 w-full h-[100%] text-zinc-700 dark:text-zinc-300 flex justify-center items-center bg-zinc-50 dark:bg-zinc-800".to_string()),
        ("placeholder_svg".to_string(), "absolute inset-0 h-full w-full stroke-zinc-300 dark:stroke-zinc-600".to_string()),
        ("placeholder_text".to_string(), "p-2 z-10 bg-white dark:bg-zinc-800 rounded-md text-zinc-600 dark:text-zinc-400".to_string()),

        // Popover
        ("popover_container".to_string(), "relative inline-block".to_string()),
        ("popover_trigger".to_string(), "cursor-pointer".to_string()),
        ("popover_content".to_string(), "absolute {{ text }} {{ background }} {{ border }} {{ default_rounding }} {{ padding_4 }} z-10 shadow-lg".to_string()),
        ("popover_position_north_start".to_string(), "bottom-full left-0 transform translate-x-0 mb-2".to_string()),
        ("popover_position_north_middle".to_string(), "bottom-full left-1/2 transform -translate-x-1/2 mb-2".to_string()),
        ("popover_position_north_end".to_string(), "bottom-full right-0 transform translate-x-0 mb-2".to_string()),
        ("popover_position_south_start".to_string(), "top-full left-0 transform translate-x-0 mt-2".to_string()),
        ("popover_position_south_middle".to_string(), "top-full left-1/2 transform -translate-x-1/2 mt-2".to_string()),
        ("popover_position_south_end".to_string(), "top-full right-0 transform mt-2".to_string()),
        ("popover_position_east_start".to_string(), "top-0 left-full transform translate-y-0 ml-2".to_string()),
        ("popover_position_east_middle".to_string(), "top-1/2 left-full transform -translate-y-1/2 ml-2".to_string()),
        ("popover_position_east_end".to_string(), "bottom-0 left-full transform ml-2".to_string()),
        ("popover_position_west_start".to_string(), "top-0 right-full transform translate-y-0 mr-2".to_string()),
        ("popover_position_west_middle".to_string(), "top-1/2 right-full transform -translate-y-1/2 mr-2".to_string()),
        ("popover_position_west_end".to_string(), "bottom-0 right-full transform mr-2".to_string()),

        // Resizable
        ("resizable_container".to_string(), "container relative".to_string()),
        ("resizable_box".to_string(), "border-2 border-indigo-500 border-dashed absolute bg-zinc-100 dark:bg-zinc-800".to_string()),
        ("resizable_handle_visible".to_string(), "block".to_string()),
        ("resizable_handle_hidden".to_string(), "hidden".to_string()),
        ("resizable_handle_nw".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform top-0 left-0 -translate-x-2 -translate-y-2 cursor-nw-resize".to_string()),
        ("resizable_handle_ne".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform top-0 right-0 translate-x-2 -translate-y-2 cursor-ne-resize".to_string()),
        ("resizable_handle_sw".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform bottom-0 left-0 -translate-x-2 translate-y-2 cursor-sw-resize".to_string()),
        ("resizable_handle_se".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform bottom-0 right-0 translate-x-2 translate-y-2 cursor-se-resize".to_string()),
        ("resizable_handle_n".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform top-0 left-1/2 -translate-x-1/2 -translate-y-2 cursor-n-resize".to_string()),
        ("resizable_handle_s".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform bottom-0 left-1/2 -translate-x-1/2 translate-y-2 cursor-s-resize".to_string()),
        ("resizable_handle_w".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform left-0 top-1/2 -translate-y-1/2 -translate-x-2 cursor-w-resize".to_string()),
        ("resizable_handle_e".to_string(), "h-4 w-4 absolute rounded-full bg-indigo-500 transform right-0 top-1/2 -translate-y-1/2 translate-x-2 cursor-e-resize".to_string()),

        // Selectable
        ("selectable_indicator".to_string(), "outline outline-2 outline-zinc-400 outline-dashed".to_string()),
        ("selectable_hover".to_string(), "hover:outline hover:outline-dashed hover:outline-2 hover:outline-indigo-500 transition-all duration-200".to_string()),
        ("selectable_selected".to_string(), "outline outline-2 outline-indigo-500".to_string()),
        ("selectable_cursor".to_string(), "cursor-pointer".to_string()),

        // Toggle
        ("toggle_base".to_string(), "px-3.5 py-2.5 font-semibold rounded-md transition-all duration-200 ease-in-out flex space-x-2 justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-zinc-800".to_string()),
        ("toggle_checked".to_string(), "bg-indigo-500/90 dark:bg-indigo-600 text-white border border-indigo-500 dark:border-indigo-600 hover:bg-indigo-700/100 dark:hover:bg-indigo-700 focus:ring-indigo-600".to_string()),
        ("toggle_unchecked".to_string(), "text-zinc-800 dark:text-zinc-100 border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-100 dark:hover:bg-zinc-800 focus:ring-zinc-400".to_string()),
        ("toggle_disabled".to_string(), "disabled:cursor-not-allowed disabled:opacity-50".to_string()),
        ("toggle_label".to_string(), "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-zinc-700 dark:text-zinc-300".to_string()),
        ("toggle_container".to_string(), "inline-flex items-center justify-center".to_string()),
        ("toggle_icon".to_string(), "h-4 w-4".to_string()),

        ("switch_base".to_string(), "flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:focus:ring-offset-zinc-800 flex w-12 h-6".to_string()),
        ("switch_checked".to_string(), "bg-indigo-600 dark:bg-indigo-500".to_string()),
        ("switch_unchecked".to_string(), "bg-zinc-200 dark:bg-zinc-700".to_string()),
        ("switch_disabled".to_string(), "disabled:cursor-not-allowed disabled:opacity-50 dark:disabled:bg-zinc-600".to_string()),
        ("switch_label".to_string(), "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-zinc-700".to_string()),
        ("switch_thumb".to_string(), "pointer-events-none inline-block h-5 w-5 text-zinc-700 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out".to_string()),
        ("switch_translate_checked".to_string(), "translate-x-6".to_string()),
        ("switch_translate_unchecked".to_string(), "translate-x-0".to_string()),

        // Select
        ("select_container".to_string(), "relative inline-block text-left dark:text-white".to_string()),
        ("select_trigger".to_string(), "flex h-10 items-center justify-between {{ default_rounding }} {{ border }} {{ background }} {{ text_container_medium_padding }} text-sm ring-offset-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-[180px] {{ border_dark }} dark:ring-offset-zinc-800 transition-colors duration-200".to_string()),
        ("select_trigger_placeholder".to_string(), "pointer-events-none text-zinc-500 dark:text-zinc-400".to_string()),
        ("select_trigger_icon".to_string(), "lucide lucide-chevron-down h-4 w-4 opacity-50 dark:text-white".to_string()),
        ("select_content_container".to_string(), "absolute mt-1 w-full {{ default_rounding }} {{ background }} {{ default_shadow }} z-10 border border-zinc-200 dark:border-zinc-700".to_string()),
        ("select_content_list".to_string(), "max-h-60 {{ default_rounding }} py-1 text-base ring-1 ring-zinc-200 dark:ring-zinc-700 overflow-auto focus:outline-none sm:text-sm".to_string()),
        ("select_group".to_string(), "text-zinc-900 {{ text_dark }}".to_string()),
        ("select_label".to_string(), "{{ text_container_medium_padding }} text-sm font-semibold text-zinc-700 dark:text-zinc-300".to_string()),
        ("select_item".to_string(), "text-zinc-900 cursor-pointer select-none relative py-2 pl-3 pr-9 hover:bg-zinc-100 dark:text-zinc-100 dark:hover:bg-zinc-700 transition-colors duration-150".to_string()),

        // Table
        ("table_container".to_string(), "overflow-x-auto {{ border }} {{ default_rounding }}".to_string()),
        ("table".to_string(), "min-w-full {{ text }} {{ background }} border-zinc-200 dark:border-zinc-700 divide-zinc-200 dark:divide-zinc-700".to_string()),
        ("table_head".to_string(), "bg-zinc-50 dark:bg-zinc-800".to_string()),
        ("table_row".to_string(), "hover:bg-zinc-50 dark:hover:bg-zinc-700 transition-colors duration-150".to_string()),
        ("table_head_row".to_string(), "bg-zinc-50 dark:bg-zinc-800".to_string()),
        ("table_cell".to_string(), "py-3 px-4".to_string()),
        ("table_body".to_string(), "divide-y divide-zinc-200 dark:divide-zinc-700 hidden [&:not(:empty)]:table-row-group border-t border-zinc-200 dark:border-zinc-700".to_string()),
        ("table_footer".to_string(), "bg-zinc-50 dark:bg-zinc-800 border-t border-zinc-200 dark:border-zinc-700".to_string()),

        // Tailwind Color Picker
        ("tailwind_color_picker_container".to_string(), "relative".to_string()),
        ("tailwind_color_picker_button".to_string(), "h-10 w-10 mb-4 p-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-full flex justify-center items-center shadow-md hover:shadow-lg transition-shadow duration-200".to_string()),
        ("tailwind_color_picker_selected_color".to_string(), "h-6 w-6 rounded-full".to_string()),
        ("tailwind_color_picker_dropdown".to_string(), "z-20 bg-white dark:bg-zinc-800 absolute top-12 left-0 flex-grow palettes overflow-auto flex flex-col p-2 rounded-lg shadow-xl border border-zinc-200 dark:border-zinc-700".to_string()),
        ("tailwind_color_picker_row".to_string(), "flex flex-row space-x-1 mb-1".to_string()),
        ("tailwind_color_picker_cell".to_string(), "cursor-pointer border h-6 w-6 rounded-full transition-transform duration-150 hover:scale-110".to_string()),

        // Tabs
        ("tabs_container".to_string(), "mt-4 space-y-4 flex flex-col justify-start items-start".to_string()),
        ("tabs_list".to_string(), "space-x-1 bg-zinc-100 {{ default_rounding }} dark:bg-zinc-800 inline-flex".to_string()),
        ("tabs_list_column".to_string(), "flex-col p-4 items-stretch space-y-1".to_string()),
        ("tabs_list_row".to_string(), "flex-row p-1".to_string()),
        // TODO: ("tabs_list_auto".to_string(), "flex-row p-1".to_string()),
        // tabs_container_auto: flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0
        ("tabs_trigger".to_string(), "flex-1 inline-flex items-center justify-center whitespace-nowrap px-3 py-1.5 text-sm font-medium {{ default_rounding }} transition-all duration-200".to_string()),
        ("tabs_trigger_inactive".to_string(), "{{ text }} hover:bg-white dark:hover:bg-zinc-700".to_string()),
        ("tabs_trigger_active".to_string(), "{{ primary_background }} {{ text_dark }} shadow-sm".to_string()),
        ("tabs_trigger_disabled".to_string(), "opacity-50 cursor-not-allowed".to_string()),
        ("tabs_content".to_string(), "".to_string()),

        ("tag_input_container".to_string(), "cursor-text flex flex-col space-y-1 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-600 text-zinc-900 dark:text-zinc-100 text-sm rounded-lg focus-within:ring-2 focus-within:ring-indigo-500 focus-within:border-indigo-500 block w-full p-2.5 transition-all duration-200".to_string()),
        ("tag_input_tags_container".to_string(), "flex flex-wrap gap-2".to_string()),
        ("tag_input_tag".to_string(), "bg-indigo-100 text-indigo-800 text-sm font-medium px-2.5 py-0.5 rounded dark:bg-indigo-900 dark:text-indigo-300 flex items-center".to_string()),
        ("tag_input_remove_button".to_string(), "ml-1 text-indigo-600 rounded-full hover:text-indigo-800 dark:text-indigo-300 dark:hover:text-indigo-100 cursor-pointer transition-colors duration-150".to_string()),
        ("tag_input_input".to_string(), "bg-transparent outline-none focus:outline-none flex-grow text-zinc-700 dark:text-zinc-300".to_string()),
        ("tag_input_candidates_container".to_string(), "flex flex-wrap gap-2 mt-2".to_string()),
        ("tag_input_candidate_button".to_string(), "text-indigo-600 bg-indigo-100 hover:bg-indigo-200 focus:ring-4 focus:outline-none focus:ring-indigo-300 font-medium rounded-lg text-sm px-4 py-2 text-center dark:text-indigo-300 dark:bg-indigo-900 dark:hover:bg-indigo-800 dark:focus:ring-indigo-800 transition-colors duration-150".to_string()),

    ];

    let final_dict = render_vec_to_hashmap(elements)?;

    Ok(final_dict)
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
