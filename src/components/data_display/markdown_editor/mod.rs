mod block;
mod command_menu;
mod editor_block;
mod utils;

#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

pub use block::{Block, BlockTrait};
use command_menu::{CommandMenu, CommandMenuProps};
use editor_block::{EditorBlock, EditorBlockProps};
use utils::{document, window};

// Editor properties
#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps<T: BlockTrait> {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub initial_content: Vec<Block<T>>,
    #[prop_or_default]
    pub on_change: Callback<Vec<Block<T>>>,
    #[prop_or_default]
    pub auto_focus: bool,
    #[prop_or_default]
    pub command_triggers: Option<Vec<String>>,
    #[prop_or_default]
    pub update_block: Option<Callback<(usize, T)>>,
    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent>>,
}

// Main editor component
#[function_component(MarkdownEditor)]
pub fn markdown_editor<T: BlockTrait>(props: &MarkdownEditorProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    // State for blocks
    let blocks = use_state(|| {
        if props.initial_content.is_empty() {
            // We need a default block type, but since we removed BlockType enum,
            // this will need to be provided by the consumer
            Vec::<Block<T>>::new()
        } else {
            props.initial_content.clone()
        }
    });

    // Get command triggers - either from props or from BlockTrait
    let command_triggers = props
        .command_triggers
        .clone()
        .unwrap_or_else(|| T::command_triggers());

    // State for active block index
    let active_block_index = use_state(|| 0);

    // State for command menu visibility
    let show_commands = use_state(|| false);
    let command_input = use_state(String::new);
    let command_position = use_state(|| (0, 0));

    // Store which trigger was used
    let current_trigger = use_state(|| None::<String>);

    // State for selected command option
    let selected_option_index = use_state(|| 0);

    // State for focused block element
    let focused_element = use_state(|| None::<NodeRef>);

    // State to track if we're currently processing a command
    let processing_command = use_state(|| false);

    // Notify parent about changes
    let on_change = props.on_change.clone();
    let blocks_clone = blocks.clone();
    use_effect_with(blocks.clone(), move |_| {
        on_change.emit((*blocks_clone).clone());
        || {}
    });

    // Auto-focus first block when mounted
    let auto_focus = props.auto_focus;
    use_effect_with((), move |_| {
        if auto_focus {
            // Focus the first block
            if let Some(editor) = document().get_element_by_id("block-0") {
                if let Some(editable) = editor
                    .query_selector("[contenteditable='true']")
                    .ok()
                    .flatten()
                {
                    if let Ok(html_element) = editable.dyn_into::<web_sys::HtmlElement>() {
                        let _ = html_element.focus();
                    }
                }
            }
        }
        || {}
    });

    // Get search results using the BlockTrait::search function
    let command_options = {
        let search_query = (*command_input).clone();
        log::info!("Search query: {}", search_query);
        let query = if search_query.is_empty() {
            None
        } else {
            Some(search_query)
        };

        // Use the search method to get block types matching the query
        let block_types = T::search(query);

        // Convert the returned block types to command options
        block_types
            .into_iter()
            .map(|block_type| {
                let name = block_type.name();
                let keywords = name.to_lowercase();
                let icon = Some(block_type.icon());
                (block_type, keywords, name, icon)
            })
            .collect::<Vec<_>>()
    };

    // Filtered options based on search input
    let filtered_options = command_options;

    // Handle selecting a block type from the command menu
    let on_select_block_type = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        let show_commands = show_commands.clone();
        let command_input = command_input.clone();
        let processing_command = processing_command.clone();
        let current_trigger = current_trigger.clone();
        let update_block_callback = props.update_block.clone();

        Callback::from(move |block_type: T| {
            let mut new_blocks = (*blocks).clone();
            let index = *active_block_index;
            if index < new_blocks.len() {
                // Update the block type with the selected one
                new_blocks[index].block_type = block_type.clone();

                // Clear the command trigger from content
                if let Some(trigger) = &*current_trigger {
                    if let Some(pos) = new_blocks[index].content.rfind(trigger) {
                        new_blocks[index].content = new_blocks[index].content[..pos].to_string();
                    }
                }

                blocks.set(new_blocks.clone());

                // Notify parent about block type change
                if let Some(callback) = &update_block_callback {
                    callback.emit((index, block_type));
                }
            }

            show_commands.set(false);
            command_input.set(String::new());
            processing_command.set(false);
            current_trigger.set(None);
        })
    };

    // Callback to update block type
    let on_update_block_type = {
        let blocks = blocks.clone();
        let update_block_callback = props.update_block.clone();

        Callback::from(move |(index, new_block_type): (usize, T)| {
            let mut new_blocks = (*blocks).clone();

            if index < new_blocks.len() {
                // Update the block type with the new one
                new_blocks[index].block_type = new_block_type.clone();
                blocks.set(new_blocks);

                // Notify parent about block type change
                if let Some(callback) = &update_block_callback {
                    callback.emit((index, new_block_type));
                }
            }
        })
    };

    // Handle focusing a block
    let on_focus_block = {
        let active_block_index = active_block_index.clone();
        let show_commands = show_commands.clone();
        let processing_command = processing_command.clone();

        Callback::from(move |index: usize| {
            active_block_index.set(index);

            // If the user clicks somewhere else while command menu is open, close it
            if *show_commands && *processing_command {
                // Don't close immediately to allow clicking on command options
                // This would be handled by the command option's onclick handler
            }
        })
    };

    // Handle blur events
    let on_blur_block = {
        let on_blur = props.on_blur.clone();

        Callback::from(move |event: FocusEvent| {
            if let Some(callback) = &on_blur {
                callback.emit(event);
            }
        })
    };

    // Handle input changes in a block
    let on_input_block = {
        let blocks = blocks.clone();
        let show_commands = show_commands.clone();
        let command_input = command_input.clone();
        let command_position = command_position.clone();
        let selected_option_index = selected_option_index.clone();
        let processing_command = processing_command.clone();
        let current_trigger = current_trigger.clone();
        let command_triggers = command_triggers.clone();

        Callback::from(move |(index, content): (usize, String)| {
            let mut new_blocks = (*blocks).clone();
            if index < new_blocks.len() {
                // Update the content
                new_blocks[index].content = content.clone();

                // Check if any trigger is present in the content
                let mut trigger_found = false;
                let mut found_trigger = String::new();

                for trigger in &command_triggers {
                    if content.contains(trigger) {
                        trigger_found = true;
                        found_trigger = trigger.clone();
                        break;
                    }
                }

                if trigger_found && !*processing_command {
                    // A trigger was found and we're not already processing a command
                    show_commands.set(true);
                    processing_command.set(true);
                    current_trigger.set(Some(found_trigger.clone()));

                    // Extract text after the trigger as search term
                    if let Some(trigger_pos) = content.rfind(&found_trigger) {
                        let search_term = &content[trigger_pos + found_trigger.len()..];
                        command_input.set(search_term.to_string());
                        selected_option_index.set(0); // Reset selected index on new input
                    } else {
                        command_input.set(String::new());
                    }

                    // Set command position to current caret position
                    if let Some(selection) = window().get_selection().ok().flatten() {
                        if let Some(range) = selection.get_range_at(0).ok() {
                            let rect = range.get_bounding_client_rect();
                            command_position.set((rect.left() as i32, rect.bottom() as i32 + 5));
                        }
                    }
                } else if *processing_command {
                    // We're already processing a command
                    if let Some(trigger) = &*current_trigger {
                        if content.contains(trigger) {
                            // Command is still being processed, update the search term
                            if let Some(trigger_pos) = content.rfind(trigger) {
                                let search_term = &content[trigger_pos + trigger.len()..];
                                command_input.set(search_term.to_string());
                                selected_option_index.set(0); // Reset selected index when search term changes
                            }
                        } else {
                            // Trigger was removed, close the command menu
                            show_commands.set(false);
                            processing_command.set(false);
                            current_trigger.set(None);
                        }
                    }
                } else if !trigger_found {
                    // No trigger found, make sure command menu is closed
                    show_commands.set(false);
                    processing_command.set(false);
                    current_trigger.set(None);
                }

                blocks.set(new_blocks);
            }
        })
    };

    // Handle keydown events in a block
    let on_keydown_block = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        let show_commands = show_commands.clone();
        let selected_option_index = selected_option_index.clone();
        let filtered_options = filtered_options.clone();
        let on_select_block_type = on_select_block_type.clone();
        let processing_command = processing_command.clone();
        let current_trigger = current_trigger.clone();
        let command_input = command_input.clone();
        let command_position = command_position.clone();
        let command_triggers = command_triggers.clone();

        Callback::from(move |e: KeyboardEvent| {
            let index = *active_block_index;
            let mut new_blocks = (*blocks).clone();
            let key = e.key();

            // Check if this keypress is a command trigger
            if !*processing_command && command_triggers.contains(&key) {
                // We should not prevent default as the user may press esc to keep the trigger key in the input
                // e.prevent_default();

                show_commands.set(true);
                processing_command.set(true);
                current_trigger.set(Some(key.clone()));
                command_input.set(String::new());
                selected_option_index.set(0);

                // Set command position to current caret position
                if let Some(selection) = window().get_selection().ok().flatten() {
                    if let Some(range) = selection.get_range_at(0).ok() {
                        let rect = range.get_bounding_client_rect();
                        command_position.set((rect.left() as i32, rect.bottom() as i32 + 5));
                    }
                }

                // Update the block content to include the trigger
                if index < new_blocks.len() {
                    new_blocks[index].content.push_str(&key);
                    blocks.set(new_blocks);
                }

                return;
            }

            // If in command mode, handle keydown events specially
            if *processing_command {
                match key.as_str() {
                    "Enter" => {
                        e.prevent_default();
                        // Select the current option from command menu
                        let selected_idx = *selected_option_index;
                        if selected_idx < filtered_options.len() {
                            let (block_type, _, _, _) = &filtered_options[selected_idx];
                            on_select_block_type.emit(block_type.clone());
                        }
                        return;
                    }
                    "Escape" => {
                        e.prevent_default();
                        show_commands.set(false);
                        processing_command.set(false);
                        current_trigger.set(None);

                        // Optionally remove the trigger from content
                        if index < new_blocks.len() {
                            if let Some(trigger) = &*current_trigger {
                                if let Some(pos) = new_blocks[index].content.rfind(trigger) {
                                    new_blocks[index].content =
                                        new_blocks[index].content[..pos].to_string();
                                    blocks.set(new_blocks);
                                }
                            }
                        }
                        return;
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        let current_idx = *selected_option_index;
                        if !filtered_options.is_empty() {
                            let new_idx = if current_idx == 0 {
                                filtered_options.len() - 1
                            } else {
                                current_idx - 1
                            };
                            selected_option_index.set(new_idx);
                        }
                        return;
                    }
                    "ArrowDown" => {
                        e.prevent_default();
                        let current_idx = *selected_option_index;
                        if !filtered_options.is_empty() {
                            let new_idx = (current_idx + 1) % filtered_options.len();
                            selected_option_index.set(new_idx);
                        }
                        return;
                    }
                    "Tab" => {
                        e.prevent_default();
                        return;
                    }
                    "Backspace" => {
                        // Check if we're deleting the trigger character
                        if index < new_blocks.len() {
                            if let Some(trigger) = &*current_trigger {
                                let content = &new_blocks[index].content;
                                if let Some(pos) = content.rfind(trigger) {
                                    // If the cursor is right after the trigger, we're deleting it
                                    if pos + trigger.len() == content.len() {
                                        show_commands.set(false);
                                        processing_command.set(false);
                                        current_trigger.set(None);
                                    }
                                }
                            }
                        }
                        // Don't prevent default to allow deleting characters
                    }
                    _ => {
                        // Update search query based on current input
                        if index < new_blocks.len() {
                            if let Some(trigger) = &*current_trigger {
                                if let Some(pos) = new_blocks[index].content.rfind(trigger) {
                                    let new_content = &new_blocks[index].content;
                                    let search_term = &new_content[pos + trigger.len()..];
                                    command_input.set(search_term.to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Normal editor operations when not processing a command
            if !*processing_command {
                match key.as_str() {
                    "Enter" => {
                        if e.meta_key() || e.ctrl_key() {
                            // Cmd+Enter or Ctrl+Enter: Insert a block above
                            e.prevent_default();

                            // Use new_block to create a new block
                            let new_block = Block::new(T::new_block());
                            new_blocks.insert(index, new_block);
                            blocks.set(new_blocks);

                            // Focus the newly created block
                            active_block_index.set(index);
                        } else if !e.shift_key() {
                            e.prevent_default();

                            // Split content at cursor position
                            if index < new_blocks.len() {
                                if let Some(selection) = window().get_selection().ok().flatten() {
                                    if selection.range_count() > 0 {
                                        if let Ok(range) = selection.get_range_at(0) {
                                            if let Ok(offset) = range.start_offset() {
                                                let content = &new_blocks[index].content;
                                                let content_len = content.len() as u32;

                                                // Get content before and after cursor
                                                let content_before =
                                                    if offset > 0 && offset <= content_len {
                                                        content[..offset as usize].to_string()
                                                    } else {
                                                        content.clone()
                                                    };

                                                let content_after = if offset < content_len {
                                                    content[offset as usize..].to_string()
                                                } else {
                                                    String::new()
                                                };

                                                // Update current block with content before cursor
                                                new_blocks[index].content = content_before;

                                                // Create new block with content after cursor
                                                let mut new_block = Block::new(T::new_block());
                                                new_block.content = content_after;

                                                // Insert new block
                                                new_blocks.insert(index + 1, new_block);
                                                blocks.set(new_blocks);

                                                // Focus the newly created block
                                                active_block_index.set(index + 1);
                                                return;
                                            }
                                        }
                                    }
                                }

                                // Fallback if we couldn't get selection: create empty block
                                let new_block = Block::new(T::new_block());
                                new_blocks.insert(index + 1, new_block);
                                blocks.set(new_blocks);
                                active_block_index.set(index + 1);
                            }
                        }
                    }
                    "Backspace" => {
                        // Check if we can delete this block
                        let can_delete = if index > 0 {
                            if let Some(block) = new_blocks.get(index) {
                                if block.content.is_empty() {
                                    block.block_type.can_delete()
                                } else {
                                    false // Not empty, can't delete
                                }
                            } else {
                                false // Block doesn't exist, can't delete
                            }
                        } else {
                            false // Can't delete the first block
                        };

                        if can_delete {
                            e.prevent_default();

                            // Remove current empty block and focus the previous one
                            new_blocks.remove(index);
                            blocks.set(new_blocks.clone());
                            active_block_index.set(index - 1);
                        }
                    }
                    "ArrowUp" => {
                        if index > 0 {
                            e.prevent_default();
                            active_block_index.set(index - 1);
                        }
                    }
                    "ArrowDown" => {
                        if index < new_blocks.len() - 1 {
                            e.prevent_default();
                            active_block_index.set(index + 1);
                        }
                    }
                    _ => {}
                }
            }
        })
    };

    // Prepare command menu props if needed
    let command_menu_props = if *show_commands {
        Some(CommandMenuProps {
            position: (0, 0), // Position will be relative to the block
            search_query: (*command_input).clone(),
            options: filtered_options.clone(),
            selected_index: *selected_option_index,
            on_select: on_select_block_type.clone(),
            on_close: {
                let show_commands = show_commands.clone();
                Callback::from(move |_| {
                    show_commands.set(false);
                    processing_command.set(false);
                    current_trigger.set(None);
                })
            },
        })
    } else {
        None
    };

    html! {
        <div class={classes!(&brandguide.markdown_editor_container, props.class.clone())}>
            <div class={classes!(&brandguide.markdown_editor_blocks_container)}>
                {
                    blocks.iter().enumerate().map(|(index, block)| {
                        let block_props = EditorBlockProps {
                            id: format!("block-{}", index),
                            index,
                            block: block.clone(),
                            is_active: index == *active_block_index,
                            on_focus: on_focus_block.clone(),
                            on_input: on_input_block.clone(),
                            on_keydown: on_keydown_block.clone(),
                            on_update_block_type: on_update_block_type.clone(),
                            on_blur: on_blur_block.clone(),
                        };

                        let is_active = index == *active_block_index;
                        let show_command_here = is_active && *show_commands;

                        html! {
                            <div key={block.id.clone()}>
                                <EditorBlock<T> ..block_props />

                                // Render command menu inside the active block
                                {
                                    if show_command_here {
                                        if let Some(menu_props) = command_menu_props.clone() {
                                            html! {
                                                <span class="relative z-10">
                                                    <CommandMenu<T> ..menu_props />
                                                </span>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
