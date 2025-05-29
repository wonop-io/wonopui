use crate::components::forms::{Input, Label, TagInput};
use crate::config::{use_brandguide, use_set_brandguide};
use crate::config::{BrandGuideType, ClassesStr};
use crate::Select;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ComponentEditorProps {
    pub fields: Vec<(String, String)>,
    pub preview: Html,
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SelectOption(String, String);

impl ToString for SelectOption {
    fn to_string(&self) -> String {
        self.1.clone()
    }
}
#[function_component(ComponentEditor)]
pub fn component_editor(props: &ComponentEditorProps) -> Html {
    let brandguide = use_brandguide();
    let set_brandguide = use_set_brandguide();
    let selected_field = use_state(|| {
        props
            .fields
            .first()
            .map(|(id, label)| SelectOption(id.clone(), label.clone()))
            .unwrap_or_default()
    });

    let on_select = {
        let selected_field = selected_field.clone();
        Callback::from(move |value: SelectOption| {
            gloo_console::log!(format!("Selected field: {:?}", value));
            selected_field.set(value);
        })
    };

    let selected_field_value = brandguide.get_value(&selected_field.0);

    html! {
        <div class="flex flex-row gap-4">
            <div class="flex flex-col gap-2 flex-shrink min-w-32 max-w-64 dark:text-white bg-zinc-100 dark:bg-zinc-900 p-4 rounded-md">
                <Select<SelectOption>
                    options={props.fields.iter().map(|(id, label)| SelectOption(id.clone(), label.clone())).collect::<Vec<SelectOption>>()}
                    selected={Some((*selected_field).clone())}
                    onchange={on_select}
                />
                <div>
                    <Label for_id={selected_field.0.clone()}>{selected_field.1.clone()}</Label>
                    <TagInput
                        key={selected_field.0.clone()}
                        id={selected_field.0.clone()}
                        default_value={selected_field_value.split_whitespace().map(String::from).collect::<Vec<String>>()}
                        onupdate={
                            let selected_field = selected_field.clone();
                            Callback::from(move |tags: Vec<String>| {
                                let mut new_brandguide = (*brandguide).clone();
                                new_brandguide.set_value(&selected_field.0, tags.join(" "));
                                set_brandguide.emit(new_brandguide);
                            })
                        }
                    />
                </div>
            </div>
            <div class="flex-grow">
                { props.preview.clone() }
            </div>
        </div>
    }
}

// Helper trait to get and set values from BrandGuide
trait BrandGuideValueAccessor {
    fn get_value(&self, field_id: &str) -> String;
    fn set_value(&mut self, field_id: &str, value: String);
}

impl BrandGuideValueAccessor for BrandGuideType {
    fn get_value(&self, field_id: &str) -> String {
        match field_id {
            "default_separator" => self.default_separator.value.clone(),

            "typography_h1" => self.typography_h1.value.clone(),
            "typography_h2" => self.typography_h2.value.clone(),
            "typography_h3" => self.typography_h3.value.clone(),
            "typography_h4" => self.typography_h4.value.clone(),
            "typography_h5" => self.typography_h5.value.clone(),
            "typography_h6" => self.typography_h6.value.clone(),
            "typography_p" => self.typography_p.value.clone(),

            "input_base" => self.input_base.value.clone(),
            "label_base" => self.label_base.value.clone(),
            "textarea_base" => self.textarea_base.value.clone(),
            "textarea_disabled" => self.textarea_disabled.value.clone(),

            "alert_success" => self.alert_success.value.clone(),
            "alert_warning" => self.alert_warning.value.clone(),
            "alert_error" => self.alert_error.value.clone(),
            "alert_info" => self.alert_info.value.clone(),
            "alert_base" => self.alert_base.value.clone(),

            "avatar_small" => self.avatar_small.value.clone(),
            "avatar_medium" => self.avatar_medium.value.clone(),
            "avatar_large" => self.avatar_large.value.clone(),
            "avatar_base" => self.avatar_base.value.clone(),

            "badge_success" => self.badge_success.value.clone(),
            "badge_warning" => self.badge_warning.value.clone(),
            "badge_error" => self.badge_error.value.clone(),
            "badge_info" => self.badge_info.value.clone(),
            "badge_default" => self.badge_default.value.clone(),
            "badge_base" => self.badge_base.value.clone(),

            "breadcrumb_nav" => self.breadcrumb_nav.value.clone(),
            "breadcrumb_list" => self.breadcrumb_list.value.clone(),
            "breadcrumb_item" => self.breadcrumb_item.value.clone(),
            "breadcrumb_separator" => self.breadcrumb_separator.value.clone(),

            "button_primary" => self.button_primary.value.clone(),
            "button_secondary" => self.button_secondary.value.clone(),
            "button_danger" => self.button_danger.value.clone(),
            "button_default" => self.button_default.value.clone(),
            "button_success" => self.button_success.value.clone(),
            "button_warning" => self.button_warning.value.clone(),
            "button_ghost" => self.button_ghost.value.clone(),
            "button_base" => self.button_base.value.clone(),
            "button_small" => self.button_small.value.clone(),
            "button_medium" => self.button_medium.value.clone(),
            "button_large" => self.button_large.value.clone(),

            "calendar_container" => self.calendar_container.value.clone(),
            "calendar_wrapper" => self.calendar_wrapper.value.clone(),
            "calendar_header" => self.calendar_header.value.clone(),
            "calendar_title" => self.calendar_title.value.clone(),
            "calendar_month_year" => self.calendar_month_year.value.clone(),
            "calendar_nav" => self.calendar_nav.value.clone(),
            "calendar_nav_button" => self.calendar_nav_button.value.clone(),
            "calendar_grid" => self.calendar_grid.value.clone(),
            "calendar_thead" => self.calendar_thead.value.clone(),
            "calendar_weekdays" => self.calendar_weekdays.value.clone(),
            "calendar_weekday" => self.calendar_weekday.value.clone(),
            "calendar_tbody" => self.calendar_tbody.value.clone(),
            "calendar_week" => self.calendar_week.value.clone(),
            "calendar_day" => self.calendar_day.value.clone(),
            "calendar_day_button" => self.calendar_day_button.value.clone(),
            "calendar_day_today" => self.calendar_day_today.value.clone(),
            "calendar_day_selected" => self.calendar_day_selected.value.clone(),
            "calendar_day_outside" => self.calendar_day_outside.value.clone(),

            "carousel_container" => self.carousel_container.value.clone(),
            "carousel_inner" => self.carousel_inner.value.clone(),
            "carousel_item" => self.carousel_item.value.clone(),
            "carousel_item_active" => self.carousel_item_active.value.clone(),
            "carousel_controls" => self.carousel_controls.value.clone(),

            "card_container" => self.card_container.value.clone(),
            "card_header" => self.card_header.value.clone(),
            "card_title" => self.card_title.value.clone(),
            "card_body" => self.card_body.value.clone(),

            "checkbox_base" => self.checkbox_base.value.clone(),
            "checkbox_checked" => self.checkbox_checked.value.clone(),
            "checkbox_unchecked" => self.checkbox_unchecked.value.clone(),
            "checkbox_disabled" => self.checkbox_disabled.value.clone(),
            "checkbox_label" => self.checkbox_label.value.clone(),

            "col_container" => self.col_container.value.clone(),

            "collapsible_container" => self.collapsible_container.value.clone(),
            "collapsible_header" => self.collapsible_header.value.clone(),
            "collapsible_title" => self.collapsible_title.value.clone(),
            "collapsible_button" => self.collapsible_button.value.clone(),
            "collapsible_content" => self.collapsible_content.value.clone(),
            "collapsible_item" => self.collapsible_item.value.clone(),

            "container_padding_x" => self.container_padding_x.value.clone(),
            "container_padding_y" => self.container_padding_y.value.clone(),
            "container_expanding" => self.container_expanding.value.clone(),
            "container_small" => self.container_small.value.clone(),
            "container_narrow" => self.container_narrow.value.clone(),
            "container_large" => self.container_large.value.clone(),
            "container_responsive" => self.container_responsive.value.clone(),

            "content_with_aside" => self.content_with_aside.value.clone(),
            "content_aside" => self.content_aside.value.clone(),
            "content_aside_container" => self.content_aside_container.value.clone(),

            "combobox_button" => self.combobox_button.value.clone(),
            "combobox_button_open" => self.combobox_button_open.value.clone(),
            "combobox_button_disabled" => self.combobox_button_disabled.value.clone(),
            "combobox_list" => self.combobox_list.value.clone(),
            "combobox_item" => self.combobox_item.value.clone(),
            "combobox_item_selected" => self.combobox_item_selected.value.clone(),

            "command_container" => self.command_container.value.clone(),
            "command_input_wrapper" => self.command_input_wrapper.value.clone(),
            "command_icon" => self.command_icon.value.clone(),
            "command_input" => self.command_input.value.clone(),
            "command_list" => self.command_list.value.clone(),
            "command_item" => self.command_item.value.clone(),
            "command_selected_item" => self.command_selected_item.value.clone(),
            "command_item_icon" => self.command_item_icon.value.clone(),

            "dialog_container" => self.dialog_container.value.clone(),
            "dialog_content" => self.dialog_content.value.clone(),
            "dialog_header" => self.dialog_header.value.clone(),
            "dialog_title" => self.dialog_title.value.clone(),
            "dialog_description" => self.dialog_description.value.clone(),
            "dialog_footer" => self.dialog_footer.value.clone(),

            "dropdown_content" => self.dropdown_content.value.clone(),
            "dropdown_item" => self.dropdown_item.value.clone(),
            "dropdown_item_icon" => self.dropdown_item_icon.value.clone(),
            "dropdown_item_disabled" => self.dropdown_item_disabled.value.clone(),
            "dropdown_item_widget" => self.dropdown_item_widget.value.clone(),
            "dropdown_separator" => self.dropdown_separator.value.clone(),

            "drawer_provider" => self.drawer_provider.value.clone(),
            "drawer_container" => self.drawer_container.value.clone(),
            "drawer_header" => self.drawer_header.value.clone(),
            "drawer_title" => self.drawer_title.value.clone(),
            "drawer_description" => self.drawer_description.value.clone(),
            "drawer_footer" => self.drawer_footer.value.clone(),
            "drawer_right" => self.drawer_right.value.clone(),
            "drawer_top" => self.drawer_top.value.clone(),
            "drawer_bottom" => self.drawer_bottom.value.clone(),
            "drawer_left" => self.drawer_left.value.clone(),

            "group_button_container" => self.group_button_container.value.clone(),
            "group_button_list" => self.group_button_list.value.clone(),
            "group_button_trigger" => self.group_button_trigger.value.clone(),
            "group_button_trigger_active" => self.group_button_trigger_active.value.clone(),
            "group_button_trigger_inactive" => self.group_button_trigger_inactive.value.clone(),
            "group_button_demo" => self.group_button_demo.value.clone(),

            "notification_container" => self.notification_container.value.clone(),
            "notification_content" => self.notification_content.value.clone(),
            "notification_title" => self.notification_title.value.clone(),
            "notification_description" => self.notification_description.value.clone(),
            "notification_timestamp" => self.notification_timestamp.value.clone(),
            "notification_close_button" => self.notification_close_button.value.clone(),
            "notification_close_icon" => self.notification_close_icon.value.clone(),
            "notification_action_container" => self.notification_action_container.value.clone(),
            "notification_list_container" => self.notification_list_container.value.clone(),

            "page_header_container" => self.page_header_container.value.clone(),
            "page_header_title" => self.page_header_title.value.clone(),
            "page_header_actions" => self.page_header_actions.value.clone(),

            "placeholder_container" => self.placeholder_container.value.clone(),
            "placeholder_svg" => self.placeholder_svg.value.clone(),
            "placeholder_text" => self.placeholder_text.value.clone(),

            "pagination_container" => self.pagination_container.value.clone(),
            "pagination_list" => self.pagination_list.value.clone(),
            "pagination_item" => self.pagination_item.value.clone(),
            "pagination_item_current" => self.pagination_item_current.value.clone(),

            "popover_container" => self.popover_container.value.clone(),
            "popover_trigger" => self.popover_trigger.value.clone(),
            "popover_content" => self.popover_content.value.clone(),
            "popover_position_north_start" => self.popover_position_north_start.value.clone(),
            "popover_position_north_middle" => self.popover_position_north_middle.value.clone(),
            "popover_position_north_end" => self.popover_position_north_end.value.clone(),
            "popover_position_south_start" => self.popover_position_south_start.value.clone(),
            "popover_position_south_middle" => self.popover_position_south_middle.value.clone(),
            "popover_position_south_end" => self.popover_position_south_end.value.clone(),
            "popover_position_east_start" => self.popover_position_east_start.value.clone(),
            "popover_position_east_middle" => self.popover_position_east_middle.value.clone(),
            "popover_position_east_end" => self.popover_position_east_end.value.clone(),
            "popover_position_west_start" => self.popover_position_west_start.value.clone(),
            "popover_position_west_middle" => self.popover_position_west_middle.value.clone(),
            "popover_position_west_end" => self.popover_position_west_end.value.clone(),

            "resizable_container" => self.resizable_container.value.clone(),
            "resizable_box" => self.resizable_box.value.clone(),
            "resizable_handle_visible" => self.resizable_handle_visible.value.clone(),
            "resizable_handle_hidden" => self.resizable_handle_hidden.value.clone(),
            "resizable_handle_nw" => self.resizable_handle_nw.value.clone(),
            "resizable_handle_ne" => self.resizable_handle_ne.value.clone(),
            "resizable_handle_sw" => self.resizable_handle_sw.value.clone(),
            "resizable_handle_se" => self.resizable_handle_se.value.clone(),
            "resizable_handle_n" => self.resizable_handle_n.value.clone(),
            "resizable_handle_s" => self.resizable_handle_s.value.clone(),
            "resizable_handle_w" => self.resizable_handle_w.value.clone(),
            "resizable_handle_e" => self.resizable_handle_e.value.clone(),

            "selectable_indicator" => self.selectable_indicator.value.clone(),
            "selectable_hover" => self.selectable_hover.value.clone(),
            "selectable_selected" => self.selectable_selected.value.clone(),
            "selectable_cursor" => self.selectable_cursor.value.clone(),

            "toggle_container" => self.toggle_container.value.clone(),
            "toggle_base" => self.toggle_base.value.clone(),
            "toggle_checked" => self.toggle_checked.value.clone(),
            "toggle_unchecked" => self.toggle_unchecked.value.clone(),
            "toggle_disabled" => self.toggle_disabled.value.clone(),
            "toggle_label" => self.toggle_label.value.clone(),
            "toggle_icon" => self.toggle_icon.value.clone(),

            "switch_base" => self.switch_base.value.clone(),
            "switch_thumb" => self.switch_thumb.value.clone(),
            "switch_checked" => self.switch_checked.value.clone(),
            "switch_unchecked" => self.switch_unchecked.value.clone(),
            "switch_translate_checked" => self.switch_translate_checked.value.clone(),
            "switch_translate_unchecked" => self.switch_translate_unchecked.value.clone(),
            "switch_disabled" => self.switch_disabled.value.clone(),
            "switch_label" => self.switch_label.value.clone(),

            "select_container" => self.select_container.value.clone(),
            "select_trigger" => self.select_trigger.value.clone(),
            "select_trigger_placeholder" => self.select_trigger_placeholder.value.clone(),
            "select_trigger_icon" => self.select_trigger_icon.value.clone(),
            "select_content_container" => self.select_content_container.value.clone(),
            "select_content_list" => self.select_content_list.value.clone(),
            "select_group" => self.select_group.value.clone(),
            "select_label" => self.select_label.value.clone(),
            "select_item" => self.select_item.value.clone(),

            "table_container" => self.table_container.value.clone(),
            "table" => self.table.value.clone(),
            "table_head" => self.table_head.value.clone(),
            "table_row" => self.table_row.value.clone(),
            "table_head_row" => self.table_head_row.value.clone(),
            "table_cell" => self.table_cell.value.clone(),
            "table_body" => self.table_body.value.clone(),
            "table_footer" => self.table_footer.value.clone(),

            "tabs_container" => self.tabs_container.value.clone(),
            "tabs_list" => self.tabs_list.value.clone(),
            "tabs_list_column" => self.tabs_list_column.value.clone(),
            "tabs_list_row" => self.tabs_list_row.value.clone(),
            "tabs_trigger" => self.tabs_trigger.value.clone(),
            "tabs_trigger_inactive" => self.tabs_trigger_inactive.value.clone(),
            "tabs_trigger_active" => self.tabs_trigger_active.value.clone(),
            "tabs_trigger_disabled" => self.tabs_trigger_disabled.value.clone(),
            "tabs_content" => self.tabs_content.value.clone(),

            "tag_input_container" => self.tag_input_container.value.clone(),
            "tag_input_tags_container" => self.tag_input_tags_container.value.clone(),
            "tag_input_tag" => self.tag_input_tag.value.clone(),
            "tag_input_remove_button" => self.tag_input_remove_button.value.clone(),
            "tag_input_input" => self.tag_input_input.value.clone(),
            "tag_input_candidates_container" => self.tag_input_candidates_container.value.clone(),
            "tag_input_candidate_button" => self.tag_input_candidate_button.value.clone(),

            "tailwind_color_picker_container" => self.tailwind_color_picker_container.value.clone(),
            "tailwind_color_picker_button" => self.tailwind_color_picker_button.value.clone(),
            "tailwind_color_picker_selected_color" => {
                self.tailwind_color_picker_selected_color.value.clone()
            }
            "tailwind_color_picker_dropdown" => self.tailwind_color_picker_dropdown.value.clone(),
            "tailwind_color_picker_row" => self.tailwind_color_picker_row.value.clone(),
            "tailwind_color_picker_cell" => self.tailwind_color_picker_cell.value.clone(),

            "accordion_container" => self.accordion_container.value.clone(),
            "accordion_header" => self.accordion_header.value.clone(),
            "accordion_title" => self.accordion_title.value.clone(),
            "accordion_content" => self.accordion_content.value.clone(),

            _ => String::new(),
        }
    }

    fn set_value(&mut self, field_id: &str, value: String) {
        match field_id {
            "default_separator" => self.default_separator.value = value,

            "typography_h1" => self.typography_h1.value = value,
            "typography_h2" => self.typography_h2.value = value,
            "typography_h3" => self.typography_h3.value = value,
            "typography_h4" => self.typography_h4.value = value,
            "typography_h5" => self.typography_h5.value = value,
            "typography_h6" => self.typography_h6.value = value,
            "typography_p" => self.typography_p.value = value,

            "input_base" => self.input_base.value = value,
            "label_base" => self.label_base.value = value,
            "textarea_base" => self.textarea_base.value = value,
            "textarea_disabled" => self.textarea_disabled.value = value,

            "alert_success" => self.alert_success.value = value,
            "alert_warning" => self.alert_warning.value = value,
            "alert_error" => self.alert_error.value = value,
            "alert_info" => self.alert_info.value = value,
            "alert_base" => self.alert_base.value = value,

            "avatar_small" => self.avatar_small.value = value,
            "avatar_medium" => self.avatar_medium.value = value,
            "avatar_large" => self.avatar_large.value = value,
            "avatar_base" => self.avatar_base.value = value,

            "badge_success" => self.badge_success.value = value,
            "badge_warning" => self.badge_warning.value = value,
            "badge_error" => self.badge_error.value = value,
            "badge_info" => self.badge_info.value = value,
            "badge_default" => self.badge_default.value = value,
            "badge_base" => self.badge_base.value = value,

            "breadcrumb_nav" => self.breadcrumb_nav.value = value,
            "breadcrumb_list" => self.breadcrumb_list.value = value,
            "breadcrumb_item" => self.breadcrumb_item.value = value,
            "breadcrumb_separator" => self.breadcrumb_separator.value = value,

            "button_primary" => self.button_primary.value = value,
            "button_secondary" => self.button_secondary.value = value,
            "button_danger" => self.button_danger.value = value,
            "button_default" => self.button_default.value = value,
            "button_success" => self.button_success.value = value,
            "button_warning" => self.button_warning.value = value,
            "button_ghost" => self.button_ghost.value = value,
            "button_base" => self.button_base.value = value,
            "button_small" => self.button_small.value = value,
            "button_medium" => self.button_medium.value = value,
            "button_large" => self.button_large.value = value,

            "calendar_container" => self.calendar_container.value = value,
            "calendar_wrapper" => self.calendar_wrapper.value = value,
            "calendar_header" => self.calendar_header.value = value,
            "calendar_title" => self.calendar_title.value = value,
            "calendar_month_year" => self.calendar_month_year.value = value,
            "calendar_nav" => self.calendar_nav.value = value,
            "calendar_nav_button" => self.calendar_nav_button.value = value,
            "calendar_grid" => self.calendar_grid.value = value,
            "calendar_thead" => self.calendar_thead.value = value,
            "calendar_weekdays" => self.calendar_weekdays.value = value,
            "calendar_weekday" => self.calendar_weekday.value = value,
            "calendar_tbody" => self.calendar_tbody.value = value,
            "calendar_week" => self.calendar_week.value = value,
            "calendar_day" => self.calendar_day.value = value,
            "calendar_day_button" => self.calendar_day_button.value = value,
            "calendar_day_today" => self.calendar_day_today.value = value,
            "calendar_day_selected" => self.calendar_day_selected.value = value,
            "calendar_day_outside" => self.calendar_day_outside.value = value,

            "carousel_container" => self.carousel_container.value = value,
            "carousel_inner" => self.carousel_inner.value = value,
            "carousel_item" => self.carousel_item.value = value,
            "carousel_item_active" => self.carousel_item_active.value = value,
            "carousel_controls" => self.carousel_controls.value = value,

            "card_container" => self.card_container.value = value,
            "card_header" => self.card_header.value = value,
            "card_title" => self.card_title.value = value,
            "card_body" => self.card_body.value = value,

            "checkbox_base" => self.checkbox_base.value = value,
            "checkbox_checked" => self.checkbox_checked.value = value,
            "checkbox_unchecked" => self.checkbox_unchecked.value = value,
            "checkbox_disabled" => self.checkbox_disabled.value = value,
            "checkbox_label" => self.checkbox_label.value = value,

            "col_container" => self.col_container.value = value,

            "collapsible_container" => self.collapsible_container.value = value,
            "collapsible_header" => self.collapsible_header.value = value,
            "collapsible_title" => self.collapsible_title.value = value,
            "collapsible_button" => self.collapsible_button.value = value,
            "collapsible_content" => self.collapsible_content.value = value,
            "collapsible_item" => self.collapsible_item.value = value,

            "container_padding_x" => self.container_padding_x.value = value,
            "container_padding_y" => self.container_padding_y.value = value,
            "container_expanding" => self.container_expanding.value = value,
            "container_small" => self.container_small.value = value,
            "container_narrow" => self.container_narrow.value = value,
            "container_large" => self.container_large.value = value,
            "container_responsive" => self.container_responsive.value = value,

            "content_with_aside" => self.content_with_aside.value = value,
            "content_aside" => self.content_aside.value = value,
            "content_aside_container" => self.content_aside_container.value = value,

            "combobox_button" => self.combobox_button.value = value,
            "combobox_button_open" => self.combobox_button_open.value = value,
            "combobox_button_disabled" => self.combobox_button_disabled.value = value,
            "combobox_list" => self.combobox_list.value = value,
            "combobox_item" => self.combobox_item.value = value,
            "combobox_item_selected" => self.combobox_item_selected.value = value,

            "command_container" => self.command_container.value = value,
            "command_input_wrapper" => self.command_input_wrapper.value = value,
            "command_icon" => self.command_icon.value = value,
            "command_input" => self.command_input.value = value,
            "command_list" => self.command_list.value = value,
            "command_item" => self.command_item.value = value,
            "command_selected_item" => self.command_selected_item.value = value,
            "command_item_icon" => self.command_item_icon.value = value,

            "dialog_container" => self.dialog_container.value = value,
            "dialog_content" => self.dialog_content.value = value,
            "dialog_header" => self.dialog_header.value = value,
            "dialog_title" => self.dialog_title.value = value,
            "dialog_description" => self.dialog_description.value = value,
            "dialog_footer" => self.dialog_footer.value = value,

            "dropdown_content" => self.dropdown_content.value = value,
            "dropdown_item" => self.dropdown_item.value = value,
            "dropdown_item_icon" => self.dropdown_item_icon.value = value,
            "dropdown_item_disabled" => self.dropdown_item_disabled.value = value,
            "dropdown_item_widget" => self.dropdown_item_widget.value = value,
            "dropdown_separator" => self.dropdown_separator.value = value,

            "drawer_provider" => self.drawer_provider.value = value,
            "drawer_container" => self.drawer_container.value = value,
            "drawer_header" => self.drawer_header.value = value,
            "drawer_title" => self.drawer_title.value = value,
            "drawer_description" => self.drawer_description.value = value,
            "drawer_footer" => self.drawer_footer.value = value,
            "drawer_right" => self.drawer_right.value = value,
            "drawer_top" => self.drawer_top.value = value,
            "drawer_bottom" => self.drawer_bottom.value = value,
            "drawer_left" => self.drawer_left.value = value,

            "group_button_container" => self.group_button_container.value = value,
            "group_button_list" => self.group_button_list.value = value,
            "group_button_trigger" => self.group_button_trigger.value = value,
            "group_button_trigger_active" => self.group_button_trigger_active.value = value,
            "group_button_trigger_inactive" => self.group_button_trigger_inactive.value = value,
            "group_button_demo" => self.group_button_demo.value = value,

            "notification_container" => self.notification_container.value = value,
            "notification_content" => self.notification_content.value = value,
            "notification_title" => self.notification_title.value = value,
            "notification_description" => self.notification_description.value = value,
            "notification_timestamp" => self.notification_timestamp.value = value,
            "notification_close_button" => self.notification_close_button.value = value,
            "notification_close_icon" => self.notification_close_icon.value = value,
            "notification_action_container" => self.notification_action_container.value = value,
            "notification_list_container" => self.notification_list_container.value = value,

            "page_header_container" => self.page_header_container.value = value,
            "page_header_title" => self.page_header_title.value = value,
            "page_header_actions" => self.page_header_actions.value = value,

            "placeholder_container" => self.placeholder_container.value = value,
            "placeholder_svg" => self.placeholder_svg.value = value,
            "placeholder_text" => self.placeholder_text.value = value,

            "pagination_container" => self.pagination_container.value = value,
            "pagination_list" => self.pagination_list.value = value,
            "pagination_item" => self.pagination_item.value = value,
            "pagination_item_current" => self.pagination_item_current.value = value,

            "popover_container" => self.popover_container.value = value,
            "popover_trigger" => self.popover_trigger.value = value,
            "popover_content" => self.popover_content.value = value,
            "popover_position_north_start" => self.popover_position_north_start.value = value,
            "popover_position_north_middle" => self.popover_position_north_middle.value = value,
            "popover_position_north_end" => self.popover_position_north_end.value = value,
            "popover_position_south_start" => self.popover_position_south_start.value = value,
            "popover_position_south_middle" => self.popover_position_south_middle.value = value,
            "popover_position_south_end" => self.popover_position_south_end.value = value,
            "popover_position_east_start" => self.popover_position_east_start.value = value,
            "popover_position_east_middle" => self.popover_position_east_middle.value = value,
            "popover_position_east_end" => self.popover_position_east_end.value = value,
            "popover_position_west_start" => self.popover_position_west_start.value = value,
            "popover_position_west_middle" => self.popover_position_west_middle.value = value,
            "popover_position_west_end" => self.popover_position_west_end.value = value,

            "resizable_container" => self.resizable_container.value = value,
            "resizable_box" => self.resizable_box.value = value,
            "resizable_handle_visible" => self.resizable_handle_visible.value = value,
            "resizable_handle_hidden" => self.resizable_handle_hidden.value = value,
            "resizable_handle_nw" => self.resizable_handle_nw.value = value,
            "resizable_handle_ne" => self.resizable_handle_ne.value = value,
            "resizable_handle_sw" => self.resizable_handle_sw.value = value,
            "resizable_handle_se" => self.resizable_handle_se.value = value,
            "resizable_handle_n" => self.resizable_handle_n.value = value,
            "resizable_handle_s" => self.resizable_handle_s.value = value,
            "resizable_handle_w" => self.resizable_handle_w.value = value,
            "resizable_handle_e" => self.resizable_handle_e.value = value,

            "selectable_indicator" => self.selectable_indicator.value = value,
            "selectable_hover" => self.selectable_hover.value = value,
            "selectable_selected" => self.selectable_selected.value = value,
            "selectable_cursor" => self.selectable_cursor.value = value,

            "toggle_container" => self.toggle_container.value = value,
            "toggle_base" => self.toggle_base.value = value,
            "toggle_checked" => self.toggle_checked.value = value,
            "toggle_unchecked" => self.toggle_unchecked.value = value,
            "toggle_disabled" => self.toggle_disabled.value = value,
            "toggle_label" => self.toggle_label.value = value,
            "toggle_icon" => self.toggle_icon.value = value,

            "switch_base" => self.switch_base.value = value,
            "switch_thumb" => self.switch_thumb.value = value,
            "switch_checked" => self.switch_checked.value = value,
            "switch_unchecked" => self.switch_unchecked.value = value,
            "switch_translate_checked" => self.switch_translate_checked.value = value,
            "switch_translate_unchecked" => self.switch_translate_unchecked.value = value,
            "switch_disabled" => self.switch_disabled.value = value,
            "switch_label" => self.switch_label.value = value,

            "select_container" => self.select_container.value = value,
            "select_trigger" => self.select_trigger.value = value,
            "select_trigger_placeholder" => self.select_trigger_placeholder.value = value,
            "select_trigger_icon" => self.select_trigger_icon.value = value,
            "select_content_container" => self.select_content_container.value = value,
            "select_content_list" => self.select_content_list.value = value,
            "select_group" => self.select_group.value = value,
            "select_label" => self.select_label.value = value,
            "select_item" => self.select_item.value = value,

            "table_container" => self.table_container.value = value,
            "table" => self.table.value = value,
            "table_head" => self.table_head.value = value,
            "table_row" => self.table_row.value = value,
            "table_head_row" => self.table_head_row.value = value,
            "table_cell" => self.table_cell.value = value,
            "table_body" => self.table_body.value = value,
            "table_footer" => self.table_footer.value = value,

            "tabs_container" => self.tabs_container.value = value,
            "tabs_list" => self.tabs_list.value = value,
            "tabs_trigger" => self.tabs_trigger.value = value,
            "tabs_trigger_inactive" => self.tabs_trigger_inactive.value = value,
            "tabs_trigger_active" => self.tabs_trigger_active.value = value,
            "tabs_trigger_disabled" => self.tabs_trigger_disabled.value = value,
            "tabs_content" => self.tabs_content.value = value,

            "tag_input_container" => self.tag_input_container.value = value,
            "tag_input_tags_container" => self.tag_input_tags_container.value = value,
            "tag_input_tag" => self.tag_input_tag.value = value,
            "tag_input_remove_button" => self.tag_input_remove_button.value = value,
            "tag_input_input" => self.tag_input_input.value = value,
            "tag_input_candidates_container" => self.tag_input_candidates_container.value = value,
            "tag_input_candidate_button" => self.tag_input_candidate_button.value = value,

            "tailwind_color_picker_container" => self.tailwind_color_picker_container.value = value,
            "tailwind_color_picker_button" => self.tailwind_color_picker_button.value = value,
            "tailwind_color_picker_selected_color" => {
                self.tailwind_color_picker_selected_color.value = value
            }
            "tailwind_color_picker_dropdown" => self.tailwind_color_picker_dropdown.value = value,
            "tailwind_color_picker_row" => self.tailwind_color_picker_row.value = value,
            "tailwind_color_picker_cell" => self.tailwind_color_picker_cell.value = value,

            "accordion_container" => self.accordion_container.value = value,
            "accordion_header" => self.accordion_header.value = value,
            "accordion_title" => self.accordion_title.value = value,
            "accordion_content" => self.accordion_content.value = value,

            _ => {}
        }
    }
}
