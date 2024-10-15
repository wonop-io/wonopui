#[cfg(feature = "ThemeProvider")]
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct ClassesContainer<T> {
    pub value: T,
}

#[cfg(not(feature = "ThemeProvider"))]
impl ClassesContainer<&'static str> {
    pub fn empty() -> Self {
        ClassesContainer { value: "" }
    }
}

#[cfg(feature = "ThemeProvider")]
impl ClassesContainer<String> {
    pub fn empty() -> Self {
        ClassesContainer {
            value: String::new(),
        }
    }
}

impl Copy for ClassesContainer<&'static str> {}

impl<T: AsRef<str> + 'static> ClassesContainer<T> {
    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    pub fn to_owned(&self) -> ClassesContainer<String> {
        ClassesContainer {
            value: self.value.as_ref().to_owned(),
        }
    }
}

impl<T: AsRef<str> + 'static> From<&ClassesContainer<T>> for yew::Classes {
    fn from(container: &ClassesContainer<T>) -> Self {
        yew::Classes::from(container.value.as_ref().to_string())
    }
}

impl<T: AsRef<str>> From<ClassesContainer<T>> for yew::Classes {
    fn from(container: ClassesContainer<T>) -> Self {
        yew::Classes::from(container.value.as_ref().to_string())
    }
}

impl<T: AsRef<str>> std::fmt::Display for ClassesContainer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_ref())
    }
}

#[cfg(not(feature = "ThemeProvider"))]
pub type ClassesStr = ClassesContainer<&'static str>;

#[cfg(feature = "ThemeProvider")]
pub type ClassesStr = ClassesContainer<String>;

#[cfg_attr(feature = "ThemeProvider", derive(Clone))]
#[derive(PartialEq)]
pub struct BrandGuide<T> {
    pub default_separator: ClassesContainer<T>,

    pub typography_h1: ClassesContainer<T>,
    pub typography_h2: ClassesContainer<T>,
    pub typography_h3: ClassesContainer<T>,
    pub typography_h4: ClassesContainer<T>,
    pub typography_h5: ClassesContainer<T>,
    pub typography_h6: ClassesContainer<T>,
    pub typography_p: ClassesContainer<T>,

    pub input_base: ClassesContainer<T>,
    pub label_base: ClassesContainer<T>,
    pub textarea_base: ClassesContainer<T>,
    pub textarea_disabled: ClassesContainer<T>,

    pub alert_success: ClassesContainer<T>,
    pub alert_warning: ClassesContainer<T>,
    pub alert_error: ClassesContainer<T>,
    pub alert_info: ClassesContainer<T>,
    pub alert_base: ClassesContainer<T>,

    pub avatar_small: ClassesContainer<T>,
    pub avatar_medium: ClassesContainer<T>,
    pub avatar_large: ClassesContainer<T>,
    pub avatar_base: ClassesContainer<T>,

    pub badge_success: ClassesContainer<T>,
    pub badge_warning: ClassesContainer<T>,
    pub badge_error: ClassesContainer<T>,
    pub badge_info: ClassesContainer<T>,
    pub badge_default: ClassesContainer<T>,
    pub badge_base: ClassesContainer<T>,

    pub breadcrumb_nav: ClassesContainer<T>,
    pub breadcrumb_list: ClassesContainer<T>,
    pub breadcrumb_item: ClassesContainer<T>,
    pub breadcrumb_separator: ClassesContainer<T>,

    pub button_primary: ClassesContainer<T>,
    pub button_secondary: ClassesContainer<T>,
    pub button_danger: ClassesContainer<T>,
    pub button_default: ClassesContainer<T>,
    pub button_success: ClassesContainer<T>,
    pub button_warning: ClassesContainer<T>,
    pub button_ghost: ClassesContainer<T>,
    pub button_base: ClassesContainer<T>,
    pub button_small: ClassesContainer<T>,
    pub button_medium: ClassesContainer<T>,
    pub button_large: ClassesContainer<T>,

    pub calendar_container: ClassesContainer<T>,
    pub calendar_wrapper: ClassesContainer<T>,
    pub calendar_header: ClassesContainer<T>,
    pub calendar_title: ClassesContainer<T>,
    pub calendar_month_year: ClassesContainer<T>,
    pub calendar_nav: ClassesContainer<T>,
    pub calendar_nav_button: ClassesContainer<T>,
    pub calendar_grid: ClassesContainer<T>,
    pub calendar_thead: ClassesContainer<T>,
    pub calendar_weekdays: ClassesContainer<T>,
    pub calendar_weekday: ClassesContainer<T>,
    pub calendar_tbody: ClassesContainer<T>,
    pub calendar_week: ClassesContainer<T>,
    pub calendar_day: ClassesContainer<T>,
    pub calendar_day_button: ClassesContainer<T>,
    pub calendar_day_today: ClassesContainer<T>,
    pub calendar_day_selected: ClassesContainer<T>,
    pub calendar_day_outside: ClassesContainer<T>,

    pub carousel_container: ClassesContainer<T>,
    pub carousel_inner: ClassesContainer<T>,
    pub carousel_item: ClassesContainer<T>,
    pub carousel_item_active: ClassesContainer<T>,
    pub carousel_controls: ClassesContainer<T>,

    pub card_container: ClassesContainer<T>,
    pub card_header: ClassesContainer<T>,
    pub card_title: ClassesContainer<T>,
    pub card_body: ClassesContainer<T>,

    pub checkbox_base: ClassesContainer<T>,
    pub checkbox_checked: ClassesContainer<T>,
    pub checkbox_unchecked: ClassesContainer<T>,
    pub checkbox_disabled: ClassesContainer<T>,
    pub checkbox_label: ClassesContainer<T>,

    pub col_container: ClassesContainer<T>,

    pub collapsible_container: ClassesContainer<T>,
    pub collapsible_header: ClassesContainer<T>,
    pub collapsible_title: ClassesContainer<T>,
    pub collapsible_button: ClassesContainer<T>,
    pub collapsible_content: ClassesContainer<T>,
    pub collapsible_item: ClassesContainer<T>,

    pub container_padding_x: ClassesContainer<T>,
    pub container_padding_y: ClassesContainer<T>,
    pub container_expanding: ClassesContainer<T>,
    pub container_small: ClassesContainer<T>,
    pub container_narrow: ClassesContainer<T>,
    pub container_large: ClassesContainer<T>,
    pub container_responsive: ClassesContainer<T>,

    pub content_with_aside: ClassesContainer<T>,
    pub content_aside: ClassesContainer<T>,
    pub content_aside_container: ClassesContainer<T>,

    pub combobox_button: ClassesContainer<T>,
    pub combobox_button_open: ClassesContainer<T>,
    pub combobox_button_disabled: ClassesContainer<T>,
    pub combobox_list: ClassesContainer<T>,
    pub combobox_item: ClassesContainer<T>,
    pub combobox_item_selected: ClassesContainer<T>,

    pub command_container: ClassesContainer<T>,
    pub command_input_wrapper: ClassesContainer<T>,
    pub command_icon: ClassesContainer<T>,
    pub command_input: ClassesContainer<T>,
    pub command_list: ClassesContainer<T>,
    pub command_item: ClassesContainer<T>,
    pub command_selected_item: ClassesContainer<T>,
    pub command_item_icon: ClassesContainer<T>,

    pub dialog_container: ClassesContainer<T>,
    pub dialog_content: ClassesContainer<T>,
    pub dialog_header: ClassesContainer<T>,
    pub dialog_title: ClassesContainer<T>,
    pub dialog_description: ClassesContainer<T>,
    pub dialog_footer: ClassesContainer<T>,

    pub dropdown_content: ClassesContainer<T>,
    pub dropdown_item: ClassesContainer<T>,
    pub dropdown_item_icon: ClassesContainer<T>,
    pub dropdown_item_disabled: ClassesContainer<T>,
    pub dropdown_item_widget: ClassesContainer<T>,
    pub dropdown_separator: ClassesContainer<T>,

    pub drawer_provider: ClassesContainer<T>,
    pub drawer_container: ClassesContainer<T>,
    pub drawer_header: ClassesContainer<T>,
    pub drawer_title: ClassesContainer<T>,
    pub drawer_description: ClassesContainer<T>,
    pub drawer_footer: ClassesContainer<T>,
    pub drawer_right: ClassesContainer<T>,
    pub drawer_top: ClassesContainer<T>,
    pub drawer_bottom: ClassesContainer<T>,
    pub drawer_left: ClassesContainer<T>,

    pub group_button_container: ClassesContainer<T>,
    pub group_button_list: ClassesContainer<T>,
    pub group_button_trigger: ClassesContainer<T>,
    pub group_button_trigger_active: ClassesContainer<T>,
    pub group_button_trigger_inactive: ClassesContainer<T>,
    pub group_button_demo: ClassesContainer<T>,

    pub notification_container: ClassesContainer<T>,
    pub notification_content: ClassesContainer<T>,
    pub notification_title: ClassesContainer<T>,
    pub notification_description: ClassesContainer<T>,
    pub notification_timestamp: ClassesContainer<T>,
    pub notification_close_button: ClassesContainer<T>,
    pub notification_close_icon: ClassesContainer<T>,
    pub notification_action_container: ClassesContainer<T>,
    pub notification_list_container: ClassesContainer<T>,

    pub page_header_container: ClassesContainer<T>,
    pub page_header_title: ClassesContainer<T>,
    pub page_header_actions: ClassesContainer<T>,

    pub placeholder_container: ClassesContainer<T>,
    pub placeholder_svg: ClassesContainer<T>,
    pub placeholder_text: ClassesContainer<T>,

    pub pagination_container: ClassesContainer<T>,
    pub pagination_list: ClassesContainer<T>,
    pub pagination_item: ClassesContainer<T>,
    pub pagination_item_current: ClassesContainer<T>,

    pub popover_container: ClassesContainer<T>,
    pub popover_trigger: ClassesContainer<T>,
    pub popover_content: ClassesContainer<T>,
    pub popover_position_north_start: ClassesContainer<T>,
    pub popover_position_north_middle: ClassesContainer<T>,
    pub popover_position_north_end: ClassesContainer<T>,
    pub popover_position_south_start: ClassesContainer<T>,
    pub popover_position_south_middle: ClassesContainer<T>,
    pub popover_position_south_end: ClassesContainer<T>,
    pub popover_position_east_start: ClassesContainer<T>,
    pub popover_position_east_middle: ClassesContainer<T>,
    pub popover_position_east_end: ClassesContainer<T>,
    pub popover_position_west_start: ClassesContainer<T>,
    pub popover_position_west_middle: ClassesContainer<T>,
    pub popover_position_west_end: ClassesContainer<T>,

    pub resizable_container: ClassesContainer<T>,
    pub resizable_box: ClassesContainer<T>,
    pub resizable_handle_visible: ClassesContainer<T>,
    pub resizable_handle_hidden: ClassesContainer<T>,
    pub resizable_handle_nw: ClassesContainer<T>,
    pub resizable_handle_ne: ClassesContainer<T>,
    pub resizable_handle_sw: ClassesContainer<T>,
    pub resizable_handle_se: ClassesContainer<T>,
    pub resizable_handle_n: ClassesContainer<T>,
    pub resizable_handle_s: ClassesContainer<T>,
    pub resizable_handle_w: ClassesContainer<T>,
    pub resizable_handle_e: ClassesContainer<T>,

    pub selectable_indicator: ClassesContainer<T>,
    pub selectable_hover: ClassesContainer<T>,
    pub selectable_selected: ClassesContainer<T>,
    pub selectable_cursor: ClassesContainer<T>,

    pub toggle_container: ClassesContainer<T>,
    pub toggle_base: ClassesContainer<T>,
    pub toggle_checked: ClassesContainer<T>,
    pub toggle_unchecked: ClassesContainer<T>,
    pub toggle_disabled: ClassesContainer<T>,
    pub toggle_label: ClassesContainer<T>,
    pub toggle_icon: ClassesContainer<T>,

    pub switch_base: ClassesContainer<T>,
    pub switch_thumb: ClassesContainer<T>,
    pub switch_checked: ClassesContainer<T>,
    pub switch_unchecked: ClassesContainer<T>,
    pub switch_translate_checked: ClassesContainer<T>,
    pub switch_translate_unchecked: ClassesContainer<T>,
    pub switch_disabled: ClassesContainer<T>,
    pub switch_label: ClassesContainer<T>,

    pub select_container: ClassesContainer<T>,
    pub select_trigger: ClassesContainer<T>,
    pub select_trigger_placeholder: ClassesContainer<T>,
    pub select_trigger_icon: ClassesContainer<T>,
    pub select_content_container: ClassesContainer<T>,
    pub select_content_list: ClassesContainer<T>,
    pub select_group: ClassesContainer<T>,
    pub select_label: ClassesContainer<T>,
    pub select_item: ClassesContainer<T>,

    pub table_container: ClassesContainer<T>,
    pub table: ClassesContainer<T>,
    pub table_head: ClassesContainer<T>,
    pub table_row: ClassesContainer<T>,
    pub table_head_row: ClassesContainer<T>,
    pub table_cell: ClassesContainer<T>,
    pub table_body: ClassesContainer<T>,
    pub table_footer: ClassesContainer<T>,

    pub tabs_container: ClassesContainer<T>,
    pub tabs_list: ClassesContainer<T>,
    pub tabs_list_column: ClassesContainer<T>,
    pub tabs_list_row: ClassesContainer<T>,
    pub tabs_trigger: ClassesContainer<T>,
    pub tabs_trigger_inactive: ClassesContainer<T>,
    pub tabs_trigger_active: ClassesContainer<T>,
    pub tabs_trigger_disabled: ClassesContainer<T>,
    pub tabs_content: ClassesContainer<T>,

    pub tag_input_container: ClassesContainer<T>,
    pub tag_input_tags_container: ClassesContainer<T>,
    pub tag_input_tag: ClassesContainer<T>,
    pub tag_input_remove_button: ClassesContainer<T>,
    pub tag_input_input: ClassesContainer<T>,
    pub tag_input_candidates_container: ClassesContainer<T>,
    pub tag_input_candidate_button: ClassesContainer<T>,

    pub tailwind_color_picker_container: ClassesContainer<T>,
    pub tailwind_color_picker_button: ClassesContainer<T>,
    pub tailwind_color_picker_selected_color: ClassesContainer<T>,
    pub tailwind_color_picker_dropdown: ClassesContainer<T>,
    pub tailwind_color_picker_row: ClassesContainer<T>,
    pub tailwind_color_picker_cell: ClassesContainer<T>,

    pub accordion_container: ClassesContainer<T>,
    pub accordion_header: ClassesContainer<T>,
    pub accordion_title: ClassesContainer<T>,
    pub accordion_content: ClassesContainer<T>,
}

impl BrandGuide<&'static str> {
    pub fn to_owned(&self) -> BrandGuide<String> {
        BrandGuide {
            default_separator: self.default_separator.to_owned(),
            typography_h1: self.typography_h1.to_owned(),
            typography_h2: self.typography_h2.to_owned(),
            typography_h3: self.typography_h3.to_owned(),
            typography_h4: self.typography_h4.to_owned(),
            typography_h5: self.typography_h5.to_owned(),
            typography_h6: self.typography_h6.to_owned(),
            typography_p: self.typography_p.to_owned(),
            input_base: self.input_base.to_owned(),
            label_base: self.label_base.to_owned(),
            textarea_base: self.textarea_base.to_owned(),
            textarea_disabled: self.textarea_disabled.to_owned(),
            alert_success: self.alert_success.to_owned(),
            alert_warning: self.alert_warning.to_owned(),
            alert_error: self.alert_error.to_owned(),
            alert_info: self.alert_info.to_owned(),
            alert_base: self.alert_base.to_owned(),
            avatar_small: self.avatar_small.to_owned(),
            avatar_medium: self.avatar_medium.to_owned(),
            avatar_large: self.avatar_large.to_owned(),
            avatar_base: self.avatar_base.to_owned(),
            badge_success: self.badge_success.to_owned(),
            badge_warning: self.badge_warning.to_owned(),
            badge_error: self.badge_error.to_owned(),
            badge_info: self.badge_info.to_owned(),
            badge_default: self.badge_default.to_owned(),
            badge_base: self.badge_base.to_owned(),
            breadcrumb_nav: self.breadcrumb_nav.to_owned(),
            breadcrumb_list: self.breadcrumb_list.to_owned(),
            breadcrumb_item: self.breadcrumb_item.to_owned(),
            breadcrumb_separator: self.breadcrumb_separator.to_owned(),
            button_primary: self.button_primary.to_owned(),
            button_secondary: self.button_secondary.to_owned(),
            button_danger: self.button_danger.to_owned(),
            button_default: self.button_default.to_owned(),
            button_success: self.button_success.to_owned(),
            button_warning: self.button_warning.to_owned(),
            button_ghost: self.button_ghost.to_owned(),
            button_base: self.button_base.to_owned(),
            button_small: self.button_small.to_owned(),
            button_medium: self.button_medium.to_owned(),
            button_large: self.button_large.to_owned(),
            calendar_container: self.calendar_container.to_owned(),
            calendar_wrapper: self.calendar_wrapper.to_owned(),
            calendar_header: self.calendar_header.to_owned(),
            calendar_title: self.calendar_title.to_owned(),
            calendar_month_year: self.calendar_month_year.to_owned(),
            calendar_nav: self.calendar_nav.to_owned(),
            calendar_nav_button: self.calendar_nav_button.to_owned(),
            calendar_grid: self.calendar_grid.to_owned(),
            calendar_thead: self.calendar_thead.to_owned(),
            calendar_weekdays: self.calendar_weekdays.to_owned(),
            calendar_weekday: self.calendar_weekday.to_owned(),
            calendar_tbody: self.calendar_tbody.to_owned(),
            calendar_week: self.calendar_week.to_owned(),
            calendar_day: self.calendar_day.to_owned(),
            calendar_day_button: self.calendar_day_button.to_owned(),
            calendar_day_today: self.calendar_day_today.to_owned(),
            calendar_day_selected: self.calendar_day_selected.to_owned(),
            calendar_day_outside: self.calendar_day_outside.to_owned(),
            carousel_container: self.carousel_container.to_owned(),
            carousel_inner: self.carousel_inner.to_owned(),
            carousel_item: self.carousel_item.to_owned(),
            carousel_item_active: self.carousel_item_active.to_owned(),
            carousel_controls: self.carousel_controls.to_owned(),
            card_container: self.card_container.to_owned(),
            card_header: self.card_header.to_owned(),
            card_title: self.card_title.to_owned(),
            card_body: self.card_body.to_owned(),
            checkbox_base: self.checkbox_base.to_owned(),
            checkbox_checked: self.checkbox_checked.to_owned(),
            checkbox_unchecked: self.checkbox_unchecked.to_owned(),
            checkbox_disabled: self.checkbox_disabled.to_owned(),
            checkbox_label: self.checkbox_label.to_owned(),
            collapsible_container: self.collapsible_container.to_owned(),
            collapsible_header: self.collapsible_header.to_owned(),
            collapsible_title: self.collapsible_title.to_owned(),
            collapsible_button: self.collapsible_button.to_owned(),
            collapsible_content: self.collapsible_content.to_owned(),
            collapsible_item: self.collapsible_item.to_owned(),
            col_container: self.col_container.to_owned(),

            container_padding_x: self.container_padding_x.to_owned(),
            container_padding_y: self.container_padding_y.to_owned(),
            container_expanding: self.container_expanding.to_owned(),
            container_small: self.container_small.to_owned(),
            container_narrow: self.container_narrow.to_owned(),
            container_large: self.container_large.to_owned(),
            container_responsive: self.container_responsive.to_owned(),

            content_with_aside: self.content_with_aside.to_owned(),
            content_aside: self.content_aside.to_owned(),
            content_aside_container: self.content_aside_container.to_owned(),

            combobox_button: self.combobox_button.to_owned(),
            combobox_button_open: self.combobox_button_open.to_owned(),
            combobox_button_disabled: self.combobox_button_disabled.to_owned(),
            combobox_list: self.combobox_list.to_owned(),
            combobox_item: self.combobox_item.to_owned(),
            combobox_item_selected: self.combobox_item_selected.to_owned(),
            command_container: self.command_container.to_owned(),
            command_input_wrapper: self.command_input_wrapper.to_owned(),
            command_icon: self.command_icon.to_owned(),
            command_input: self.command_input.to_owned(),
            command_list: self.command_list.to_owned(),
            command_item: self.command_item.to_owned(),
            command_selected_item: self.command_selected_item.to_owned(),
            command_item_icon: self.command_item_icon.to_owned(),

            pagination_container: self.pagination_container.to_owned(),
            pagination_list: self.pagination_list.to_owned(),
            pagination_item: self.pagination_item.to_owned(),
            pagination_item_current: self.pagination_item_current.to_owned(),

            popover_container: self.popover_container.to_owned(),
            popover_trigger: self.popover_trigger.to_owned(),
            popover_content: self.popover_content.to_owned(),
            popover_position_north_start: self.popover_position_north_start.to_owned(),
            popover_position_north_middle: self.popover_position_north_middle.to_owned(),
            popover_position_north_end: self.popover_position_north_end.to_owned(),
            popover_position_south_start: self.popover_position_south_start.to_owned(),
            popover_position_south_middle: self.popover_position_south_middle.to_owned(),
            popover_position_south_end: self.popover_position_south_end.to_owned(),
            popover_position_east_start: self.popover_position_east_start.to_owned(),
            popover_position_east_middle: self.popover_position_east_middle.to_owned(),
            popover_position_east_end: self.popover_position_east_end.to_owned(),
            popover_position_west_start: self.popover_position_west_start.to_owned(),
            popover_position_west_middle: self.popover_position_west_middle.to_owned(),
            popover_position_west_end: self.popover_position_west_end.to_owned(),

            resizable_container: self.resizable_container.to_owned(),
            resizable_box: self.resizable_box.to_owned(),
            resizable_handle_visible: self.resizable_handle_visible.to_owned(),
            resizable_handle_hidden: self.resizable_handle_hidden.to_owned(),
            resizable_handle_nw: self.resizable_handle_nw.to_owned(),
            resizable_handle_ne: self.resizable_handle_ne.to_owned(),
            resizable_handle_sw: self.resizable_handle_sw.to_owned(),
            resizable_handle_se: self.resizable_handle_se.to_owned(),
            resizable_handle_n: self.resizable_handle_n.to_owned(),
            resizable_handle_s: self.resizable_handle_s.to_owned(),
            resizable_handle_w: self.resizable_handle_w.to_owned(),
            resizable_handle_e: self.resizable_handle_e.to_owned(),

            selectable_indicator: self.selectable_indicator.to_owned(),
            selectable_hover: self.selectable_hover.to_owned(),
            selectable_selected: self.selectable_selected.to_owned(),
            selectable_cursor: self.selectable_cursor.to_owned(),

            dialog_container: self.dialog_container.to_owned(),
            dialog_content: self.dialog_content.to_owned(),
            dialog_header: self.dialog_header.to_owned(),
            dialog_title: self.dialog_title.to_owned(),
            dialog_description: self.dialog_description.to_owned(),
            dialog_footer: self.dialog_footer.to_owned(),

            dropdown_content: self.dropdown_content.to_owned(),
            dropdown_item: self.dropdown_item.to_owned(),
            dropdown_item_icon: self.dropdown_item_icon.to_owned(),
            dropdown_item_widget: self.dropdown_item_widget.to_owned(),
            dropdown_item_disabled: self.dropdown_item_disabled.to_owned(),
            dropdown_separator: self.dropdown_separator.to_owned(),

            drawer_provider: self.drawer_provider.to_owned(),
            drawer_container: self.drawer_container.to_owned(),
            drawer_header: self.drawer_header.to_owned(),
            drawer_title: self.drawer_title.to_owned(),
            drawer_description: self.drawer_description.to_owned(),
            drawer_footer: self.drawer_footer.to_owned(),
            drawer_right: self.drawer_right.to_owned(),
            drawer_top: self.drawer_top.to_owned(),
            drawer_bottom: self.drawer_bottom.to_owned(),
            drawer_left: self.drawer_left.to_owned(),

            group_button_container: self.group_button_container.to_owned(),
            group_button_list: self.group_button_list.to_owned(),
            group_button_trigger: self.group_button_trigger.to_owned(),
            group_button_trigger_active: self.group_button_trigger_active.to_owned(),
            group_button_trigger_inactive: self.group_button_trigger_inactive.to_owned(),
            group_button_demo: self.group_button_demo.to_owned(),

            notification_container: self.notification_container.to_owned(),
            notification_content: self.notification_content.to_owned(),
            notification_title: self.notification_title.to_owned(),
            notification_description: self.notification_description.to_owned(),
            notification_timestamp: self.notification_timestamp.to_owned(),
            notification_close_button: self.notification_close_button.to_owned(),
            notification_close_icon: self.notification_close_icon.to_owned(),
            notification_action_container: self.notification_action_container.to_owned(),
            notification_list_container: self.notification_list_container.to_owned(),

            page_header_container: self.page_header_container.to_owned(),
            page_header_title: self.page_header_title.to_owned(),
            page_header_actions: self.page_header_actions.to_owned(),

            placeholder_container: self.placeholder_container.to_owned(),
            placeholder_svg: self.placeholder_svg.to_owned(),
            placeholder_text: self.placeholder_text.to_owned(),

            tag_input_container: self.tag_input_container.to_owned(),
            tag_input_tags_container: self.tag_input_tags_container.to_owned(),
            tag_input_tag: self.tag_input_tag.to_owned(),
            tag_input_remove_button: self.tag_input_remove_button.to_owned(),
            tag_input_input: self.tag_input_input.to_owned(),
            tag_input_candidates_container: self.tag_input_candidates_container.to_owned(),
            tag_input_candidate_button: self.tag_input_candidate_button.to_owned(),

            toggle_container: self.toggle_container.to_owned(),
            toggle_base: self.toggle_base.to_owned(),
            toggle_checked: self.toggle_checked.to_owned(),
            toggle_unchecked: self.toggle_unchecked.to_owned(),
            toggle_disabled: self.toggle_disabled.to_owned(),
            toggle_label: self.toggle_label.to_owned(),
            toggle_icon: self.toggle_icon.to_owned(),

            switch_base: self.switch_base.to_owned(),
            switch_thumb: self.switch_thumb.to_owned(),
            switch_checked: self.switch_checked.to_owned(),
            switch_unchecked: self.switch_unchecked.to_owned(),
            switch_translate_checked: self.switch_translate_checked.to_owned(),
            switch_translate_unchecked: self.switch_translate_unchecked.to_owned(),
            switch_disabled: self.switch_disabled.to_owned(),
            switch_label: self.switch_label.to_owned(),

            select_container: self.select_container.to_owned(),
            select_trigger: self.select_trigger.to_owned(),
            select_trigger_placeholder: self.select_trigger_placeholder.to_owned(),
            select_trigger_icon: self.select_trigger_icon.to_owned(),
            select_content_container: self.select_content_container.to_owned(),
            select_content_list: self.select_content_list.to_owned(),
            select_group: self.select_group.to_owned(),
            select_label: self.select_label.to_owned(),
            select_item: self.select_item.to_owned(),

            table_container: self.table_container.to_owned(),
            table: self.table.to_owned(),
            table_head: self.table_head.to_owned(),
            table_row: self.table_row.to_owned(),
            table_head_row: self.table_head_row.to_owned(),
            table_cell: self.table_cell.to_owned(),
            table_body: self.table_body.to_owned(),
            table_footer: self.table_footer.to_owned(),

            tabs_container: self.tabs_container.to_owned(),
            tabs_list: self.tabs_list.to_owned(),
            tabs_list_column: self.tabs_list_column.to_owned(),
            tabs_list_row: self.tabs_list_row.to_owned(),
            tabs_trigger: self.tabs_trigger.to_owned(),
            tabs_trigger_inactive: self.tabs_trigger_inactive.to_owned(),
            tabs_trigger_active: self.tabs_trigger_active.to_owned(),
            tabs_trigger_disabled: self.tabs_trigger_disabled.to_owned(),
            tabs_content: self.tabs_content.to_owned(),

            tailwind_color_picker_container: self.tailwind_color_picker_container.to_owned(),
            tailwind_color_picker_button: self.tailwind_color_picker_button.to_owned(),
            tailwind_color_picker_selected_color: self
                .tailwind_color_picker_selected_color
                .to_owned(),
            tailwind_color_picker_dropdown: self.tailwind_color_picker_dropdown.to_owned(),
            tailwind_color_picker_row: self.tailwind_color_picker_row.to_owned(),
            tailwind_color_picker_cell: self.tailwind_color_picker_cell.to_owned(),

            accordion_container: self.accordion_container.to_owned(),
            accordion_header: self.accordion_header.to_owned(),
            accordion_title: self.accordion_title.to_owned(),
            accordion_content: self.accordion_content.to_owned(),
        }
    }
}

// Include generated constants
include!(concat!(env!("OUT_DIR"), "/config.rs"));

#[cfg(not(feature = "ThemeProvider"))]
pub type BrandGuideType = &'static BrandGuide<&'static str>;

#[cfg(feature = "ThemeProvider")]
pub type BrandGuideType = BrandGuide<String>;

#[cfg(not(feature = "ThemeProvider"))]
pub fn get_brandguide() -> &'static BrandGuide<&'static str> {
    &BRANDGUIDE
}

#[cfg(feature = "ThemeProvider")]
#[derive(Clone, PartialEq)]
pub struct BrandGuideContext {
    pub brandguide: Rc<BrandGuide<String>>,
    pub set_brandguide: Callback<BrandGuide<String>>,
}

#[cfg(feature = "ThemeProvider")]
#[derive(Properties, PartialEq)]
pub struct BrandGuideProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[cfg(feature = "ThemeProvider")]
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &BrandGuideProviderProps) -> Html {
    let brandguide = use_state(|| Rc::new(BRANDGUIDE.to_owned()));

    let set_brandguide = {
        let brandguide = brandguide.clone();
        Callback::from(move |new_brandguide: BrandGuide<String>| {
            brandguide.set(Rc::new(new_brandguide));
        })
    };

    let context = BrandGuideContext {
        brandguide: (*brandguide).clone(),
        set_brandguide,
    };
    /*
    use_effect_with((), |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let head = document.head().unwrap();

        let tailwind_script = document.create_element("script").unwrap();
        tailwind_script
            .set_attribute("src", "https://cdn.tailwindcss.com")
            .unwrap();
        head.append_child(&tailwind_script).unwrap();

        let config_script = document.create_element("script").unwrap();
        config_script.set_text_content(Some(
            r#"
                tailwind.config = {
                  darkMode: "class",
                  mode: "jit",
                  content: ["./index.html", "./src/**/*.rs"],
                  theme: {
                    extend: {
                      colors: {
                        clifford: '#da373d',
                      }
                    }
                  }
                }
            "#,
        ));
        head.append_child(&config_script).unwrap();

        || ()
    });
    */

    html! {
        <ContextProvider<BrandGuideContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<BrandGuideContext>>
    }
}

#[cfg(feature = "ThemeProvider")]
#[hook]
pub fn use_brandguide() -> Rc<BrandGuide<String>> {
    let ctx = use_context::<BrandGuideContext>().expect("BrandGuideContext not found");
    ctx.brandguide.clone()
}

#[cfg(feature = "ThemeProvider")]
#[hook]
pub fn use_set_brandguide() -> Callback<BrandGuide<String>> {
    use_context::<BrandGuideContext>()
        .expect("BrandGuideContext not found")
        .set_brandguide
}
