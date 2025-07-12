use super::markdown_editor::utils::{document, window};
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FocusEvent, HtmlElement, InputEvent, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ContentEditableWithCommandsProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub content: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub is_active: bool,
    #[prop_or_default]
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub on_keydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or("div")]
    pub tag: &'static str,
    pub command_triggers: Vec<String>,
    pub command_options: Vec<(T, String, String, Option<Html>)>, // (value, keyword, display, icon)
    pub on_command_select: Callback<T>,
}

#[function_component(ContentEditableWithCommands)]
pub fn contenteditable_with_commands<T: Clone + PartialEq + 'static>(
    props: &ContentEditableWithCommandsProps<T>,
) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let node_ref = use_node_ref();
    let show_commands = use_state(|| false);
    let command_position = use_state(|| (0, 0));
    let command_filter = use_state(String::new);
    let selected_index = use_state(|| 0);
    let current_trigger = use_state(|| None::<String>);
    let markdown_command_menu_container = brandguide.markdown_command_menu_container.to_string();
    let command_list = brandguide.command_list.to_string();

    // Set initial content and ensure proper line height on first render
    {
        let node_ref = node_ref.clone();

        use_effect_with((), move |_| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                // Add a non-breaking space to ensure proper line height
                if element.inner_text().is_empty() {
                    // Add a <br> element to ensure proper height
                    element.set_inner_html("&nbsp;");
                    // Remove the content right after rendering to keep it actually empty
                    // but with proper height
                    let window = web_sys::window().expect("no global window exists");

                    // Create a proper closure for request_animation_frame
                    let closure = Closure::once_into_js(move |_: f64| {
                        element.set_inner_html("");
                    });

                    let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
                }
            }
            || {}
        });
    }

    // Update content when props change
    {
        let node_ref = node_ref.clone();
        let content = props.content.clone();

        use_effect_with(content, move |content| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                // Only update if different to avoid losing selection
                if element.inner_text() != *content {
                    if content.is_empty() {
                        // Add a <br> element to ensure proper height for empty content
                        element.set_inner_html("<br>");
                    } else {
                        element.set_inner_html(&content.replace("\n", "<br>"));
                    }
                }
            }
            || {}
        });
    }

    // Focus/blur based on is_active prop changes
    {
        let node_ref = node_ref.clone();
        let is_active = props.is_active;

        use_effect_with(is_active, move |is_active| {
            if let Some(element) = node_ref.cast::<HtmlElement>() {
                if *is_active {
                    let _ = element.focus();
                } else {
                    let _ = element.blur();
                }
            }
            || {}
        });
    }

    // Handle input events and detect command triggers
    let on_input = {
        let on_input = props.on_input.clone();
        let show_commands = show_commands.clone();
        let command_position = command_position.clone();
        let command_filter = command_filter.clone();
        let current_trigger = current_trigger.clone();
        let command_triggers = props.command_triggers.clone();
        let selected_index = selected_index.clone();
        let node_ref = node_ref.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlElement = e.target_unchecked_into();
            let content = input.inner_text();

            // Forward content update to parent
            on_input.emit(content.clone());

            // Debug logging
            log::info!("Input content: '{}'", content);
            log::info!("Triggers: {:?}", command_triggers);

            // Check for command triggers
            for trigger in &command_triggers {
                if content.contains(trigger) {
                    log::info!("Found trigger: {}", trigger);

                    // Find the last instance of the trigger
                    if let Some(trigger_pos) = content.rfind(trigger) {
                        // Only show commands if trigger is at beginning or after whitespace
                        let should_show_commands = trigger_pos == 0
                            || content
                                .chars()
                                .nth(trigger_pos - 1)
                                .map(|c| c.is_whitespace())
                                .unwrap_or(false);

                        log::info!(
                            "Trigger position: {}, should show: {}",
                            trigger_pos,
                            should_show_commands
                        );

                        if should_show_commands {
                            // Extract filter text after the trigger
                            let filter = &content[trigger_pos + trigger.len()..];
                            command_filter.set(filter.to_string());
                            selected_index.set(0); // Reset selection on new input

                            // Position the command menu at cursor
                            if let Some(selection) = window().get_selection().ok().flatten() {
                                if let Some(range) = selection.get_range_at(0).ok() {
                                    let rect = range.get_bounding_client_rect();
                                    command_position
                                        .set((rect.left() as i32, rect.bottom() as i32 + 5));
                                    log::info!(
                                        "Command position: {:?}",
                                        (rect.left(), rect.bottom())
                                    );
                                }
                            }

                            // Show command menu and store current trigger
                            show_commands.set(true);
                            current_trigger.set(Some(trigger.clone()));
                            return;
                        }
                    }
                }
            }

            // No trigger found, hide command menu
            show_commands.set(false);
            current_trigger.set(None);
        })
    };

    // Handle keyboard events
    let on_keydown = {
        let props_keydown = props.on_keydown.clone();
        let show_commands = show_commands.clone();
        let selected_index = selected_index.clone();
        let command_options = props.command_options.clone();
        let on_command_select = props.on_command_select.clone();
        let node_ref = node_ref.clone();
        let current_trigger = current_trigger.clone();
        let brandguide = brandguide.clone();
        let content = props.content.clone();
        Callback::from(move |e: KeyboardEvent| {
            // Handle command menu navigation and selection
            if *show_commands {
                match e.key().as_str() {
                    "Enter" => {
                        e.prevent_default();
                        e.stop_propagation();
                        let idx = *selected_index;
                        if idx < command_options.len() {
                            let (value, _, _, _) = &command_options[idx];
                            on_command_select.emit(value.clone());

                            // Clear the command trigger from content
                            if let Some(element) = node_ref.cast::<HtmlElement>() {
                                if let Some(trigger) = &*current_trigger {
                                    let content = element.inner_text();
                                    if let Some(pos) = content.rfind(trigger) {
                                        let new_content = content[..pos].to_string();
                                        element.set_inner_text(&new_content);
                                    }
                                }
                            }

                            show_commands.set(false);
                            current_trigger.set(None);
                        }
                    }
                    "Escape" => {
                        e.prevent_default();
                        e.stop_propagation();
                        show_commands.set(false);
                        current_trigger.set(None);
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        e.stop_propagation();
                        let new_index = if *selected_index > 0 {
                            *selected_index - 1
                        } else {
                            command_options.len() - 1
                        };
                        selected_index.set(new_index);

                        // Ensure the item is visible in the menu
                        let document = document();
                        if let Some(selected_element) = document
                            .query_selector(&format!(
                                ".{} > div:nth-child({})",
                                brandguide
                                    .command_list
                                    .value
                                    .split_whitespace()
                                    .next()
                                    .unwrap_or(""),
                                new_index + 1
                            ))
                            .ok()
                            .flatten()
                        {
                            selected_element.scroll_into_view();
                        }
                    }
                    "ArrowDown" => {
                        e.prevent_default();
                        e.stop_propagation();
                        let new_index = (*selected_index + 1) % command_options.len().max(1);
                        selected_index.set(new_index);

                        // Ensure the item is visible in the menu
                        let document = document();
                        if let Some(selected_element) = document
                            .query_selector(&format!(
                                ".{} > div:nth-child({})",
                                brandguide
                                    .command_list
                                    .value
                                    .split_whitespace()
                                    .next()
                                    .unwrap_or(""),
                                new_index + 1
                            ))
                            .ok()
                            .flatten()
                        {
                            selected_element.scroll_into_view();
                        }
                    }
                    "Tab" => {
                        // Prevent tab from moving focus away during command selection
                        e.prevent_default();
                        e.stop_propagation();
                    }
                    _ => {
                        // Forward to parent handler, but keep command menu open
                        props_keydown.emit(e.clone());
                    }
                }
            } else {
                let should_send = match e.key().as_str() {
                    "Enter" => !e.shift_key(),
                    "ArrowUp" => {
                        // Check if cursor is at the first line
                        if let Some(selection) = window().get_selection().ok().flatten() {
                            if let Some(range) = selection.get_range_at(0).ok() {
                                if let Some(node) = range.start_container().ok() {
                                    let offset = range.start_offset().ok().unwrap_or(0) as usize;

                                    // If we're at the beginning of text or in the first line
                                    if offset == 0 && node.node_type() == 3 {
                                        // 3 is text node
                                        let text = node.text_content().unwrap_or_default();
                                        !text.contains('\n') || !text[..offset].contains('\n')
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    "ArrowDown" => {
                        // Check if cursor is at the last line
                        if let Some(selection) = window().get_selection().ok().flatten() {
                            if let Some(range) = selection.get_range_at(0).ok() {
                                if let Some(node) = range.start_container().ok() {
                                    let offset = range.start_offset().ok().unwrap_or(0) as usize;

                                    // If we're at the end of text or in the last line
                                    if node.node_type() == 3 {
                                        // 3 is text node
                                        let text = node.text_content().unwrap_or_default();
                                        if offset == text.len() {
                                            true
                                        } else {
                                            !text[offset..].contains('\n')
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    "Backspace" => content.is_empty(),
                    _ => true,
                };
                // Normal mode - forward to parent handler
                if should_send {
                    props_keydown.emit(e);
                }
            }
        })
    };

    // Filter command options based on current input
    let filtered_options = {
        let filter = (*command_filter).clone();
        if filter.is_empty() {
            props.command_options.clone()
        } else {
            let filter_lower = filter.to_lowercase();
            props
                .command_options
                .iter()
                .filter(|(_, keywords, name, _)| {
                    keywords.to_lowercase().contains(&filter_lower)
                        || name.to_lowercase().contains(&filter_lower)
                })
                .cloned()
                .collect::<Vec<_>>()
        }
    };

    // Prepare command menu HTML
    let command_menu = if *show_commands {
        html! {
            <div
                class={classes!("absolute", "top-0", "left-0", markdown_command_menu_container)}
            >
                <div class={command_list}>
                    {
                        if filtered_options.is_empty() {
                            html! { <div class={classes!(&brandguide.command_item, "text-zinc-500")}>{"No results found"}</div> }
                        } else {
                            filtered_options.iter().enumerate().map(|(idx, (value, _, display, icon))| {
                                let is_selected = idx == *selected_index;
                                let item_class = if is_selected {
                                    classes!(&brandguide.command_selected_item)
                                } else {
                                    classes!(&brandguide.command_item)
                                };

                                let on_command_select = props.on_command_select.clone();
                                let value_clone = value.clone();
                                let show_commands = show_commands.clone();
                                let current_trigger = current_trigger.clone();
                                let node_ref = node_ref.clone();

                                html! {
                                    <div
                                        class={item_class}
                                        onclick={Callback::from(move |_| {
                                            on_command_select.emit(value_clone.clone());

                                            // Clear the command trigger from content
                                            if let Some(element) = node_ref.cast::<HtmlElement>() {
                                                if let Some(trigger) = &*current_trigger {
                                                    let content = element.inner_text();
                                                    if let Some(pos) = content.rfind(trigger) {
                                                        let new_content = content[..pos].to_string();
                                                        element.set_inner_text(&new_content);
                                                    }
                                                }
                                            }

                                            show_commands.set(false);
                                            current_trigger.set(None);
                                        })}
                                    >
                                        if let Some(icon_html) = icon {
                                            <span class={classes!(&brandguide.command_item_icon)}>
                                                { icon_html.clone() }
                                            </span>
                                        }
                                        <span>{ display }</span>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    }
                </div>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <>
            <@{props.tag}
                ref={node_ref}
                class={classes!("w-full","outline-none","min-h-[1.5rem]","p-1", "leading-normal", "empty:before:content-['\\00a0']", "empty:before:inline-block", "empty:before:h-0", props.class.clone())}
                contenteditable="true"
                oninput={on_input}
                onkeydown={on_keydown}
                onfocus={props.on_focus.clone()}
                onblur={props.on_blur.clone()}
            >
                {
                    if props.content.is_empty() {
                        html! {
                            <div class="text-zinc-400 pointer-events-none">{&props.placeholder}</div>
                        }
                    } else {
                        html! {}
                    }
                }
            </@>
            <div class="relative w-full">
                { command_menu }
            </div>
        </>
    }
}
