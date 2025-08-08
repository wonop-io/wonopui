mod block;
mod editor_block;
pub mod utils;

#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::KeyboardEvent;
use yew::prelude::*;

pub use block::BlockTrait;
use editor_block::{EditorBlock, EditorBlockProps};
use utils::{document, window};

// Add the ContentEditableWithCommands component
use crate::components::data_display::contenteditable_commands::{
    ContentEditableWithCommands, ContentEditableWithCommandsProps,
};

// Simplified editor properties
#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps<T: BlockTrait> {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub initial_content: Vec<T>,
    #[prop_or_default]
    pub on_change: Callback<Vec<T>>,
    #[prop_or_default]
    pub auto_focus: bool,
    #[prop_or_default]
    pub update_block: Option<Callback<(usize, T)>>,
    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent>>,
    #[prop_or(false)]
    pub show_block_actions: bool,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor<T: BlockTrait>(props: &MarkdownEditorProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    // Core state
    let blocks = use_state(|| {
        if props.initial_content.is_empty() {
            vec![T::new_block()]
        } else {
            props.initial_content.clone()
        }
    });

    let active_block_index = use_state(|| 0);

    // State for selection tracking
    let selection_info = use_state(|| None::<(usize, usize, usize)>);

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

    // Core callback to update a block
    let on_update_block_type = {
        let blocks = blocks.clone();
        let update_block_callback = props.update_block.clone();
        let selection_info = selection_info.clone();
        let active_block_index = active_block_index.clone();

        Callback::from(move |(index, new_block_type): (usize, T)| {
            // Store selection before update
            if let Some(selection) = window().get_selection().ok().flatten() {
                if selection.range_count() > 0 {
                    if let Ok(range) = selection.get_range_at(0) {
                        if let (Ok(start), Ok(end)) = (range.start_offset(), range.end_offset()) {
                            selection_info.set(Some((
                                *active_block_index,
                                start as usize,
                                end as usize,
                            )));
                        }
                    }
                }
            }

            let mut new_blocks = (*blocks).clone();
            if index < new_blocks.len() {
                new_blocks[index] = new_block_type.clone();
                blocks.set(new_blocks);

                if let Some(callback) = &update_block_callback {
                    callback.emit((index, new_block_type));
                }
            }
        })
    };

    // Handle block actions (duplicate, delete, move, etc.)
    let on_block_action = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        Callback::from(move |(index, action): (usize, String)| {
            let mut new_blocks = (*blocks).clone();
            
            match action.as_str() {
                "duplicate" => {
                    if let Some(block) = new_blocks.get(index) {
                        let duplicated = block.clone();
                        new_blocks.insert(index + 1, duplicated);
                        blocks.set(new_blocks);
                        active_block_index.set(index + 1);
                    }
                }
                "delete" => {
                    if new_blocks.len() > 1 && index < new_blocks.len() {
                        new_blocks.remove(index);
                        blocks.set(new_blocks);
                        
                        // Adjust active block index
                        let new_active = if index > 0 { index - 1 } else { 0 };
                        active_block_index.set(new_active);
                    }
                }
                "move-up" => {
                    if index > 0 && index < new_blocks.len() {
                        new_blocks.swap(index, index - 1);
                        blocks.set(new_blocks);
                        active_block_index.set(index - 1);
                    }
                }
                "move-down" => {
                    if index < new_blocks.len() - 1 {
                        new_blocks.swap(index, index + 1);
                        blocks.set(new_blocks);
                        active_block_index.set(index + 1);
                    }
                }
                _ => {}
            }
        })
    };

    // Handle focus on a block
    let on_focus_block = {
        let active_block_index = active_block_index.clone();
        Callback::from(move |index: usize| {
            active_block_index.set(index);
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
        let selection_info = selection_info.clone();
        let active_block_index = active_block_index.clone();

        Callback::from(move |(index, content): (usize, String)| {
            // Store selection before content update
            if let Some(selection) = window().get_selection().ok().flatten() {
                if selection.range_count() > 0 {
                    if let Ok(range) = selection.get_range_at(0) {
                        if let (Ok(start), Ok(end)) = (range.start_offset(), range.end_offset()) {
                            selection_info.set(Some((index, start as usize, end as usize)));
                        }
                    }
                }
            }

            /*
            TODO: Implement set_content() on the BlockTrait
            let mut new_blocks = (*blocks).clone();
            if index < new_blocks.len() {
                new_blocks[index].content = content.clone();
                blocks.set(new_blocks);
            }
            */
        })
    };

    // Callback to insert a new block at specific index
    let on_insert_block = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();

        Callback::from(move |index: usize| {
            let mut new_blocks = (*blocks).clone();

            // Create a new empty block
            let new_block = T::new_block();

            // Insert at the specified position
            new_blocks.insert(index, new_block);
            blocks.set(new_blocks);

            // Set focus to the new block
            active_block_index.set(index);

            // Schedule focus on the new block
            let index_clone = index;
            let closure = Closure::once_into_js(move || {
                if let Some(editor) =
                    document().get_element_by_id(&format!("block-{}", index_clone))
                {
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
            });

            let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                0,
            );
        })
    };

    // Handle keydown events - simplified to focus on core operations
    let on_keydown_block = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        let selection_info = selection_info.clone();

        Callback::from(move |e: KeyboardEvent| {
            let index = *active_block_index;
            let mut new_blocks = (*blocks).clone();
            let key = e.key();

            match key.as_str() {
                "Enter" => {
                    if !e.shift_key() && !e.ctrl_key() && !e.meta_key() {
                        e.prevent_default();

                        if index < new_blocks.len() {
                            // Get cursor position
                            /*
                            let mut cursor_pos = 0;
                            let mut content_before = String::new();
                            let mut content_after = String::new();

                            let current_content = &new_blocks[index].content;

                            if let Some(selection) = window().get_selection().ok().flatten() {
                                if selection.range_count() > 0 {
                                    if let Ok(range) = selection.get_range_at(0) {
                                        if let Ok(offset) = range.start_offset() {
                                            cursor_pos = offset as usize;
                                            if cursor_pos <= current_content.len() {
                                                content_before =
                                                    current_content[..cursor_pos].to_string();
                                                content_after =
                                                    current_content[cursor_pos..].to_string();
                                            } else {
                                                content_before = current_content.clone();
                                            }
                                        }
                                    }
                                }
                            }

                            // Update current block with content before cursor
                            new_blocks[index].content = content_before;
                            */
                            // Create and insert new block
                            let mut new_block = T::new_block();
                            // new_block.content = content_after;

                            // Insert the new block after current
                            new_blocks.insert(index + 1, new_block);
                            blocks.set(new_blocks.clone());

                            // Focus the new block
                            active_block_index.set(index + 1);

                            // Store selection info for the new block
                            selection_info.set(Some((index + 1, 0, 0)));
                        }
                    }
                }
                "Backspace" => {
                    // Check if we can delete this block
                    let can_delete = if index > 0 {
                        if let Some(block) = new_blocks.get(index) {
                            block.can_delete()
                        } else {
                            false
                        }
                    } else {
                        false // Can't delete first block
                    };

                    if can_delete {
                        e.prevent_default();
                        selection_info.set(Some((index - 1, 0, 0)));
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
                            on_blur: on_blur_block.clone(),
                            on_insert_block: on_insert_block.clone(),
                            on_block_action: on_block_action.clone(),
                            show_block_actions: props.show_block_actions,
                        };

                        html! {
                            <div key={format!("block-{}", index)}>
                                <EditorBlock<T> ..block_props />
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
