#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use chrono::Utc;
use gloo::timers::callback::Timeout;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlDivElement, HtmlElement, KeyboardEvent};
use yew::prelude::*;

// Trait that an enum should implement to be used as a block type
pub trait BlockTrait: Clone + PartialEq + 'static {
    fn is_tag(&self) -> bool;
    fn tag(&self) -> Option<String>;
    fn classes(&self) -> Classes;
    fn icon(&self) -> Html;
    fn name(&self) -> String;
    fn render(&self, arguments: String, update_block: Callback<Self>) -> Option<Html>;
    fn can_delete(&self) -> bool {
        true // Default implementation returns true
    }

    // Define command triggers for this block type (default is "/")
    fn command_triggers() -> Vec<String> {
        vec!["/".to_string()]
    }

    // Only search is a static method, as it needs to return multiple instances
    fn search(query: Option<String>) -> Vec<Self>;
}

// Block model representing a single block in the editor
#[derive(Clone, PartialEq)]
pub struct Block<T: BlockTrait> {
    pub id: String,
    pub content: String,
    pub block_type: T,
}

impl<T: BlockTrait> Block<T> {
    pub fn new(block_type: T) -> Self {
        Self {
            id: format!("block-{}", Utc::now().timestamp_millis()),
            content: String::new(),
            block_type,
        }
    }

    pub fn get_classes(&self, brandguide: &BrandGuideType) -> Classes {
        self.block_type.classes()
    }
}

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
                    let _ = editable.dyn_into::<HtmlElement>().unwrap().focus();
                }
            }
        }
        || {}
    });

    // Get search results using the BlockTrait::search function
    let command_options = {
        let search_query = (*command_input).clone();
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

    // Filtered options based on search input (though the search is now done in BlockTrait::search)
    let filtered_options = command_options;

    // Handle selecting a block type from the command menu
    let on_select_block_type = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        let show_commands = show_commands.clone();
        let command_input = command_input.clone();
        let processing_command = processing_command.clone();
        let current_trigger = current_trigger.clone();

        Callback::from(move |block_type: T| {
            let mut new_blocks = (*blocks).clone();
            let index = *active_block_index;

            if index < new_blocks.len() {
                if !block_type.is_tag() {
                    // This is where we would handle special block types like dividers
                    // For simplicity, we'll just assume all block types are tags for now
                    new_blocks[index].block_type = block_type.clone();
                } else {
                    // For tag-based block types, we change the type of the current block
                    new_blocks[index].block_type = block_type;
                }

                // Clear the command trigger from content
                if let Some(trigger) = &*current_trigger {
                    if let Some(pos) = new_blocks[index].content.rfind(trigger) {
                        new_blocks[index].content = new_blocks[index].content[..pos].to_string();
                    }
                }

                blocks.set(new_blocks.clone());
            }

            show_commands.set(false);
            command_input.set(String::new());
            processing_command.set(false);
            current_trigger.set(None);

            // Update the DOM after a short delay to ensure contenteditable content is correct
            let index_clone = index;
            Timeout::new(10, move || {
                if let Some(element) =
                    document().get_element_by_id(&format!("block-{}", index_clone))
                {
                    if let Some(editable) = element
                        .query_selector("[contenteditable='true']")
                        .ok()
                        .flatten()
                    {
                        // Ensure the DOM element's content matches our state
                        if let Some(div) = editable.dyn_into::<HtmlDivElement>().ok() {
                            // Get current state content and set it back to the DOM
                            if let Some(block) = new_blocks.get(index_clone) {
                                div.set_inner_text(&block.content);
                            }
                        }
                    }
                }
            })
            .forget();
        })
    };

    // Callback to update block type
    let on_update_block_type = {
        let blocks = blocks.clone();

        Callback::from(move |(index, new_block_type): (usize, T)| {
            let mut new_blocks = (*blocks).clone();

            if index < new_blocks.len() {
                // Update the block type with the new one
                new_blocks[index].block_type = new_block_type;
                blocks.set(new_blocks);
            }
        })
    };

    // Handle focusing a block
    let on_focus_block = {
        let active_block_index = active_block_index.clone();
        Callback::from(move |index: usize| {
            active_block_index.set(index);
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

        Callback::from(move |e: KeyboardEvent| {
            let index = *active_block_index;
            let mut new_blocks = (*blocks).clone();

            match e.key().as_str() {
                "Enter" => {
                    if *show_commands {
                        e.prevent_default();
                        // Select the current option from command menu
                        let selected_idx = *selected_option_index;
                        if selected_idx < filtered_options.len() {
                            let (block_type, _, _, _) = &filtered_options[selected_idx];
                            on_select_block_type.emit(block_type.clone());
                        }
                        return;
                    }

                    if e.meta_key() || e.ctrl_key() {
                        // Cmd+Enter or Ctrl+Enter: Insert a block above
                        e.prevent_default();

                        if let Some(block_type) =
                            new_blocks.get(index).map(|b| b.block_type.clone())
                        {
                            let new_block = Block::new(block_type);
                            new_blocks.insert(index, new_block);
                            blocks.set(new_blocks);

                            // Keep focus on the newly inserted block
                            Timeout::new(10, move || {
                                if let Some(element) =
                                    document().get_element_by_id(&format!("block-{}", index))
                                {
                                    if let Some(editable) = element
                                        .query_selector("[contenteditable='true']")
                                        .ok()
                                        .flatten()
                                    {
                                        let _ = editable.dyn_into::<HtmlElement>().unwrap().focus();
                                    }
                                }
                            })
                            .forget();
                        }
                    } else if !e.shift_key() {
                        e.prevent_default();

                        // Normal Enter: Create a new block below
                        if let Some(block_type) =
                            new_blocks.get(index).map(|b| b.block_type.clone())
                        {
                            let new_block = Block::new(block_type);
                            new_blocks.insert(index + 1, new_block);

                            blocks.set(new_blocks.clone());
                            active_block_index.set(index + 1);

                            // Focus the new block after render
                            let new_index = index + 1;
                            Timeout::new(10, move || {
                                if let Some(element) =
                                    document().get_element_by_id(&format!("block-{}", new_index))
                                {
                                    if let Some(editable) = element
                                        .query_selector("[contenteditable='true']")
                                        .ok()
                                        .flatten()
                                    {
                                        let _ = editable.dyn_into::<HtmlElement>().unwrap().focus();
                                    }
                                }
                            })
                            .forget();
                        }
                    }
                }
                "Backspace" => {
                    // Check if we can delete this block
                    let can_delete = if index > 0 {
                        if let Some(block) = new_blocks.get(index) {
                            if block.content.is_empty() {
                                if block.block_type.is_tag() {
                                    // Tags can be deleted if empty
                                    true
                                } else {
                                    // Non-tags use their can_delete method
                                    block.block_type.can_delete()
                                }
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

                        // Focus the previous block after render
                        let prev_index = index - 1;
                        Timeout::new(10, move || {
                            if let Some(element) =
                                document().get_element_by_id(&format!("block-{}", prev_index))
                            {
                                if let Some(editable) = element
                                    .query_selector("[contenteditable='true']")
                                    .ok()
                                    .flatten()
                                {
                                    let editable_element =
                                        editable.clone().dyn_into::<HtmlElement>().unwrap();
                                    let _ = editable_element.focus();

                                    // Place cursor at the end of the text
                                    let range = document().create_range().unwrap();
                                    range.select_node_contents(&editable).unwrap();
                                    range.collapse_with_to_start(false); // Use collapse_with_to_start with false instead of collapse_with_to_end

                                    let selection = window().get_selection().unwrap().unwrap();
                                    selection.remove_all_ranges().unwrap();
                                    selection.add_range(&range).unwrap();
                                }
                            }
                        })
                        .forget();
                    }
                }
                "ArrowUp" => {
                    if *show_commands {
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
                    } else if index > 0 {
                        e.prevent_default();

                        // Focus the block above
                        let new_index = index - 1;
                        active_block_index.set(new_index);

                        Timeout::new(10, move || {
                            if let Some(element) =
                                document().get_element_by_id(&format!("block-{}", new_index))
                            {
                                if let Some(editable) = element
                                    .query_selector("[contenteditable='true']")
                                    .ok()
                                    .flatten()
                                {
                                    let _ = editable.dyn_into::<HtmlElement>().unwrap().focus();
                                }
                            }
                        })
                        .forget();
                    }
                }
                "ArrowDown" => {
                    if *show_commands {
                        e.prevent_default();
                        let current_idx = *selected_option_index;
                        if !filtered_options.is_empty() {
                            let new_idx = (current_idx + 1) % filtered_options.len();
                            selected_option_index.set(new_idx);
                        }
                    } else if index < new_blocks.len() - 1 {
                        e.prevent_default();

                        // Focus the block below
                        let new_index = index + 1;
                        active_block_index.set(new_index);

                        Timeout::new(10, move || {
                            if let Some(element) =
                                document().get_element_by_id(&format!("block-{}", new_index))
                            {
                                if let Some(editable) = element
                                    .query_selector("[contenteditable='true']")
                                    .ok()
                                    .flatten()
                                {
                                    let _ = editable.dyn_into::<HtmlElement>().unwrap().focus();
                                }
                            }
                        })
                        .forget();
                    }
                }
                "Escape" => {
                    if *show_commands {
                        show_commands.set(false);
                        processing_command.set(false);
                        current_trigger.set(None);
                        e.prevent_default();

                        // Ensure the DOM content is updated to remove the command trigger
                        let index_clone = index;
                        let blocks_clone = new_blocks.clone();

                        Timeout::new(10, move || {
                            if let Some(element) =
                                document().get_element_by_id(&format!("block-{}", index_clone))
                            {
                                if let Some(editable) = element
                                    .query_selector("[contenteditable='true']")
                                    .ok()
                                    .flatten()
                                {
                                    // Set the actual content from our state
                                    if let Some(div) = editable.dyn_into::<HtmlDivElement>().ok() {
                                        if let Some(block) = blocks_clone.get(index_clone) {
                                            div.set_inner_text(&block.content);
                                        }
                                    }
                                }
                            }
                        })
                        .forget();
                    }
                }
                "Tab" => {
                    if *show_commands {
                        // Prevent tab navigation when command menu is open
                        e.prevent_default();
                    }
                }
                _ => {}
            }
        })
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
                        };
                        html! {
                            <div key={block.id.clone()}>
                                <EditorBlock<T> ..block_props />
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            // Command menu - Custom implementation instead of using Command component
            {
                if *show_commands {
                    let pos_style = format!(
                        "position: absolute; left: {}px; top: {}px;",
                        command_position.0, command_position.1
                    );

                    html! {
                        <div class={classes!(&brandguide.markdown_command_menu_container)} style={pos_style}>
                            <div class={classes!(&brandguide.command_container)}>
                                <div class={classes!(&brandguide.command_list)} role="listbox">
                                    {
                                        if filtered_options.is_empty() {
                                            html! {
                                                <div class={classes!(&brandguide.command_item)}>
                                                    { "No results available" }
                                                </div>
                                            }
                                        } else {
                                            filtered_options.iter().enumerate().map(|(index, (block_type, _, label, icon))| {
                                                let on_select = on_select_block_type.clone();
                                                let block_type = block_type.clone();
                                                let is_selected = index == *selected_option_index;

                                                html! {
                                                    <div
                                                        class={classes!(if is_selected { &brandguide.command_selected_item } else { &brandguide.command_item })}
                                                        onclick={Callback::from(move |_| on_select.emit(block_type.clone()))}
                                                        role="option"
                                                        aria-selected={is_selected.to_string()}
                                                    >
                                                        if let Some(icon) = icon {
                                                            <span class={classes!(&brandguide.command_item_icon)}>
                                                                {icon.clone()}
                                                            </span>
                                                        }
                                                        <span>{ label }</span>
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

// Helper function to access window
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

// Helper function to access document
fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

// Properties for a single editor block
#[derive(Properties, PartialEq)]
struct EditorBlockProps<T: BlockTrait> {
    id: String,
    index: usize,
    block: Block<T>,
    is_active: bool,
    on_focus: Callback<usize>,
    on_input: Callback<(usize, String)>,
    on_keydown: Callback<KeyboardEvent>,
    on_update_block_type: Callback<(usize, T)>,
}

// Component for a single editor block
#[function_component(EditorBlock)]
fn editor_block<T: BlockTrait>(props: &EditorBlockProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let node_ref = use_node_ref();
    let content_updated = use_state(|| false);

    // Use effect to set the inner text when content changes from outside
    {
        let node_ref = node_ref.clone();
        let content = props.block.content.clone();
        let content_updated = content_updated.clone();

        use_effect_with(
            (props.block.id.clone(), content.clone()),
            move |(_, content)| {
                let node_ref = node_ref.clone();
                let content = content.clone();
                let content_updated = content_updated.clone();
                move || {
                    if !*content_updated {
                        if let Some(element) = node_ref.cast::<HtmlDivElement>() {
                            let current_text = element.inner_text();
                            if current_text != *content {
                                element.set_inner_text(&content);
                            }
                        }
                    }
                }
            },
        );
    }

    // Handle content editable focus
    let on_focus = {
        let index = props.index;
        let on_focus = props.on_focus.clone();
        Callback::from(move |_| {
            on_focus.emit(index);
        })
    };

    // Handle content editable input
    let on_input = {
        let index = props.index;
        let on_input = props.on_input.clone();
        let node_ref = node_ref.clone();
        let content_updated = content_updated.clone();

        Callback::from(move |_| {
            if let Some(element) = node_ref.cast::<HtmlDivElement>() {
                let content = element.inner_text();
                content_updated.set(true);
                on_input.emit((index, content));
            }
        })
    };

    // Handle content editable keydown
    let on_keydown = {
        let on_keydown = props.on_keydown.clone();
        Callback::from(move |e: KeyboardEvent| {
            on_keydown.emit(e);
        })
    };

    // Create callback for updating block type
    let update_block_callback = {
        let on_update_block_type = props.on_update_block_type.clone();
        let index = props.index;
        Callback::from(move |new_block_type: T| {
            on_update_block_type.emit((index, new_block_type));
        })
    };

    // Render the block based on its type
    if !props.block.block_type.is_tag() {
        // This is for non-tag blocks like dividers
        if let Some(rendered) = props
            .block
            .block_type
            .render(props.block.content.clone(), update_block_callback.clone())
        {
            html! {
                <div id={props.id.clone()} class={classes!(
                        &brandguide.markdown_editor_block,
                        if props.is_active { &brandguide.markdown_editor_block_active } else { &brandguide.markdown_editor_block }
                    )}
                    onfocus={on_focus}
                    oninput={on_input}
                    onkeydown={on_keydown}>
                    { rendered }
                </div>
            }
        } else {
            html! {
                <div id={props.id.clone()} class={classes!(
                        &brandguide.markdown_editor_block,
                        if props.is_active { &brandguide.markdown_editor_block_active } else { &brandguide.markdown_editor_block }
                    )}>
                    // Fallback rendering if render() returns None
                    <div
                        ref={node_ref}
                        class={props.block.block_type.classes()}
                        contenteditable="true"
                        onfocus={on_focus}
                        oninput={on_input}
                        onkeydown={on_keydown}
                    >
                        {props.block.content.clone()}
                    </div>
                </div>
            }
        }
    } else {
        // For tag-based blocks
        html! {
            <div id={props.id.clone()}
                class={classes!(
                    &brandguide.markdown_editor_block,
                    if props.is_active { &brandguide.markdown_editor_block_active } else { &brandguide.markdown_editor_block }
                )}
            >
                <div
                    ref={node_ref}
                    class={props.block.block_type.classes()}
                    contenteditable="true"
                    onfocus={on_focus}
                    oninput={on_input}
                    onkeydown={on_keydown}
                >
                    {props.block.content.clone()}
                </div>
            </div>
        }
    }
}
