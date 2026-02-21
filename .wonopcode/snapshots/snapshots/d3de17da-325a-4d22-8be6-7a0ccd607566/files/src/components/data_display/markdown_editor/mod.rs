mod block;
mod editor_block;
mod history;
pub mod utils;

#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use history::EditorHistory;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{DragEvent, KeyboardEvent, MouseEvent};
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
    let selected_blocks = use_state(|| std::collections::HashSet::<usize>::new());
    let dragging_index = use_state(|| None::<usize>);
    let drop_indicator_index = use_state(|| None::<usize>);
    let last_selected_index = use_state(|| None::<usize>);

    // State for selection tracking
    let selection_info = use_state(|| None::<(usize, usize, usize)>);

    // History state for undo/redo
    let history = use_mut_ref(|| {
        let mut h = EditorHistory::new();
        if props.initial_content.is_empty() {
            h.initialize(vec![T::new_block()], 0);
        } else {
            h.initialize(props.initial_content.clone(), 0);
        }
        h
    });

    // Clipboard state for cut/paste of blocks
    let clipboard_blocks = use_state(|| Vec::<T>::new());

    // Notify parent about changes and record history
    let on_change = props.on_change.clone();
    let blocks_clone = blocks.clone();
    let history_clone = history.clone();
    let active_index_clone = active_block_index.clone();
    use_effect_with(blocks.clone(), move |_| {
        on_change.emit((*blocks_clone).clone());
        // Record state in history
        history_clone.borrow_mut().push(
            (*blocks_clone).clone(),
            *active_index_clone,
            "Edit",
        );
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

    // Handle blur-sm events
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

            // Update block content using set_content
            let mut new_blocks = (*blocks).clone();
            if index < new_blocks.len() {
                new_blocks[index] = new_blocks[index].set_content(content);
                blocks.set(new_blocks);
            }
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

    // Handle block selection with shift-click support
    let on_select_block = {
        let selected_blocks = selected_blocks.clone();
        let last_selected_index = last_selected_index.clone();
        let blocks = blocks.clone();
        Callback::from(move |(index, e): (usize, web_sys::MouseEvent)| {
            let mut selected = (*selected_blocks).clone();
            
            if e.shift_key() {
                // Shift-click: select range
                if let Some(last_idx) = *last_selected_index {
                    let start = last_idx.min(index);
                    let end = last_idx.max(index);
                    for i in start..=end {
                        if i < blocks.len() {
                            selected.insert(i);
                        }
                    }
                }
            } else if e.ctrl_key() || e.meta_key() {
                // Ctrl/Cmd-click: toggle individual selection
                if selected.contains(&index) {
                    selected.remove(&index);
                } else {
                    selected.insert(index);
                    last_selected_index.set(Some(index));
                }
            } else {
                // Regular click: single selection
                selected.clear();
                selected.insert(index);
                last_selected_index.set(Some(index));
            }
            
            selected_blocks.set(selected);
        })
    };

    // Shared state for RAF-based drag throttling to avoid excessive re-renders
    // Using Rc<Cell> to share state between callbacks without triggering Yew re-renders
    let pending_drop_indicator = use_mut_ref(|| None::<usize>);
    let raf_scheduled = use_mut_ref(|| false);
    
    // Handle drag start
    let on_drag_start = {
        let dragging_index = dragging_index.clone();
        Callback::from(move |(index, e): (usize, DragEvent)| {
            // Set data transfer to enable drag
            if let Some(dt) = e.data_transfer() {
                // Set the drag data (required for Firefox)
                let _ = dt.set_data("text/plain", &index.to_string());
                dt.set_effect_allowed("move");
            }
            dragging_index.set(Some(index));
        })
    };

    // Handle drag over - using RAF throttling for smooth updates
    let on_drag_over = {
        let drop_indicator_index = drop_indicator_index.clone();
        let dragging_index = dragging_index.clone();
        let pending_drop_indicator = pending_drop_indicator.clone();
        let raf_scheduled = raf_scheduled.clone();
        
        Callback::from(move |(index, e): (usize, DragEvent)| {
            e.prevent_default();
            
            // Only show indicator if we're actually dragging
            if dragging_index.is_none() {
                return;
            }
            
            if let Some(dt) = e.data_transfer() {
                dt.set_drop_effect("move");
            }
            
            // Calculate the new indicator position
            let new_indicator = if let Some(target) = e.current_target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                    let rect = element.get_bounding_client_rect();
                    let y = e.client_y() as f64;
                    let mid = rect.top() + (rect.height() / 2.0);
                    
                    if y < mid { index } else { index + 1 }
                } else {
                    index
                }
            } else {
                index
            };
            
            // Store the pending indicator value
            *pending_drop_indicator.borrow_mut() = Some(new_indicator);
            
            // Schedule RAF update if not already scheduled
            if !*raf_scheduled.borrow() {
                *raf_scheduled.borrow_mut() = true;
                
                let drop_indicator_index = drop_indicator_index.clone();
                let pending_drop_indicator = pending_drop_indicator.clone();
                let raf_scheduled = raf_scheduled.clone();
                
                let closure = Closure::once_into_js(move || {
                    *raf_scheduled.borrow_mut() = false;
                    
                    // Only update state if value actually changed
                    if let Some(new_val) = *pending_drop_indicator.borrow() {
                        // Compare with current value to avoid unnecessary renders
                        let current = *drop_indicator_index;
                        if current != Some(new_val) {
                            drop_indicator_index.set(Some(new_val));
                        }
                    }
                });
                
                let _ = window().request_animation_frame(closure.as_ref().unchecked_ref());
            }
        })
    };

    // Handle drag leave - clear indicator when leaving drop zone
    let on_drag_leave = {
        let drop_indicator_index = drop_indicator_index.clone();
        let pending_drop_indicator = pending_drop_indicator.clone();
        Callback::from(move |e: DragEvent| {
            // Only clear if leaving the container entirely (not entering a child)
            if let Some(related) = e.related_target() {
                if let Some(current) = e.current_target() {
                    if let (Ok(related_node), Ok(current_node)) = (
                        related.dyn_into::<web_sys::Node>(),
                        current.dyn_into::<web_sys::Node>()
                    ) {
                        // Check if related target is inside current target
                        if current_node.contains(Some(&related_node)) {
                            return; // Don't clear, we're still inside
                        }
                    }
                }
            }
            *pending_drop_indicator.borrow_mut() = None;
            drop_indicator_index.set(None);
        })
    };

    // Handle drag end - clean up state when drag operation ends (success or cancel)
    let on_drag_end = {
        let dragging_index = dragging_index.clone();
        let drop_indicator_index = drop_indicator_index.clone();
        let pending_drop_indicator = pending_drop_indicator.clone();
        let raf_scheduled = raf_scheduled.clone();
        Callback::from(move |_e: DragEvent| {
            // Clear all drag state
            *pending_drop_indicator.borrow_mut() = None;
            *raf_scheduled.borrow_mut() = false;
            dragging_index.set(None);
            drop_indicator_index.set(None);
        })
    };

    // Handle drop
    let on_drop = {
        let blocks = blocks.clone();
        let dragging_index = dragging_index.clone();
        let selected_blocks = selected_blocks.clone();
        let drop_indicator_index = drop_indicator_index.clone();
        let active_block_index = active_block_index.clone();
        let pending_drop_indicator = pending_drop_indicator.clone();
        let raf_scheduled = raf_scheduled.clone();
        Callback::from(move |(drop_index, e): (usize, DragEvent)| {
            e.prevent_default();
            e.stop_propagation();
            
            if let Some(drag_index) = *dragging_index {
                // Use the most up-to-date indicator position (pending or committed)
                let actual_drop_index = pending_drop_indicator.borrow()
                    .or(*drop_indicator_index)
                    .unwrap_or(drop_index);
                
                // Check if this is actually a move (not dropping on self or adjacent position that results in same order)
                let is_same_position = drag_index == actual_drop_index || 
                    (drag_index + 1 == actual_drop_index); // Dropping right after self is a no-op
                
                if !is_same_position {
                    let mut new_blocks = (*blocks).clone();
                    let dragged_block = new_blocks.remove(drag_index);
                    
                    let adjusted_drop_index = if drag_index < actual_drop_index {
                        actual_drop_index - 1
                    } else {
                        actual_drop_index
                    };
                    
                    new_blocks.insert(adjusted_drop_index, dragged_block);
                    blocks.set(new_blocks);
                    
                    // Update active block index to follow the dragged block
                    active_block_index.set(adjusted_drop_index);
                    
                    // Clear selection after drag
                    selected_blocks.set(HashSet::new());
                }
            }
            
            // Clean up all drag state
            *pending_drop_indicator.borrow_mut() = None;
            *raf_scheduled.borrow_mut() = false;
            dragging_index.set(None);
            drop_indicator_index.set(None);
        })
    };

    // Handle drop on container (between blocks or at edges)
    let on_container_drop = {
        let blocks = blocks.clone();
        let dragging_index = dragging_index.clone();
        let drop_indicator_index = drop_indicator_index.clone();
        let active_block_index = active_block_index.clone();
        let pending_drop_indicator = pending_drop_indicator.clone();
        let raf_scheduled = raf_scheduled.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            
            if let Some(drag_index) = *dragging_index {
                // Use the most up-to-date indicator position
                let target_index = pending_drop_indicator.borrow()
                    .or(*drop_indicator_index)
                    .unwrap_or(blocks.len());
                
                let is_same_position = drag_index == target_index || 
                    (drag_index + 1 == target_index);
                
                if !is_same_position {
                    let mut new_blocks = (*blocks).clone();
                    let dragged_block = new_blocks.remove(drag_index);
                    
                    let adjusted_index = if drag_index < target_index {
                        target_index - 1
                    } else {
                        target_index
                    };
                    
                    new_blocks.insert(adjusted_index, dragged_block);
                    blocks.set(new_blocks);
                    active_block_index.set(adjusted_index);
                }
            }
            
            // Clean up all drag state
            *pending_drop_indicator.borrow_mut() = None;
            *raf_scheduled.borrow_mut() = false;
            dragging_index.set(None);
            drop_indicator_index.set(None);
        })
    };

    // Handle drag over on container
    let on_container_drag_over = {
        let drop_indicator_index = drop_indicator_index.clone();
        let dragging_index = dragging_index.clone();
        let blocks = blocks.clone();
        let pending_drop_indicator = pending_drop_indicator.clone();
        let raf_scheduled = raf_scheduled.clone();
        
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            
            if dragging_index.is_none() {
                return;
            }
            
            if let Some(dt) = e.data_transfer() {
                dt.set_drop_effect("move");
            }
            
            // If dropping on the container but not on a block, set indicator to end
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                    // Check if we're on the container itself (not a block)
                    if element.class_list().contains("markdown-editor-blocks") {
                        let new_val = blocks.len();
                        *pending_drop_indicator.borrow_mut() = Some(new_val);
                        
                        // Schedule RAF update
                        if !*raf_scheduled.borrow() {
                            *raf_scheduled.borrow_mut() = true;
                            
                            let drop_indicator_index = drop_indicator_index.clone();
                            let pending_drop_indicator = pending_drop_indicator.clone();
                            let raf_scheduled = raf_scheduled.clone();
                            
                            let closure = Closure::once_into_js(move || {
                                *raf_scheduled.borrow_mut() = false;
                                if let Some(val) = *pending_drop_indicator.borrow() {
                                    let current = *drop_indicator_index;
                                    if current != Some(val) {
                                        drop_indicator_index.set(Some(val));
                                    }
                                }
                            });
                            
                            let _ = window().request_animation_frame(closure.as_ref().unchecked_ref());
                        }
                    }
                }
            }
        })
    };

    // Handle copy with markdown conversion through keyboard shortcut
    let handle_copy = {
        let blocks = blocks.clone();
        let selected_blocks = selected_blocks.clone();
        move || {
            let selected = (*selected_blocks).clone();
            if !selected.is_empty() {
                let mut markdown = String::new();
                let blocks_vec = (*blocks).clone();
                
                // Sort selected indices to maintain order
                let mut indices: Vec<_> = selected.iter().copied().collect();
                indices.sort();
                
                for index in indices {
                    if let Some(block) = blocks_vec.get(index) {
                        markdown.push_str(&block.to_markdown());
                        markdown.push_str("\n\n");
                    }
                }
                
                // Copy to clipboard using navigator.clipboard API
                let window = window();
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&markdown);
            }
        }
    };

    // Handle cut - remove selected blocks and store them
    let handle_cut = {
        let blocks = blocks.clone();
        let selected_blocks = selected_blocks.clone();
        let clipboard_blocks = clipboard_blocks.clone();
        let active_block_index = active_block_index.clone();
        move || {
            let selected = (*selected_blocks).clone();
            if !selected.is_empty() {
                let blocks_vec = (*blocks).clone();
                
                // Sort selected indices to maintain order
                let mut indices: Vec<_> = selected.iter().copied().collect();
                indices.sort();
                
                // Copy blocks to clipboard
                let cut_blocks: Vec<T> = indices.iter()
                    .filter_map(|&i| blocks_vec.get(i).cloned())
                    .collect();
                clipboard_blocks.set(cut_blocks);
                
                // Also copy markdown to system clipboard
                let mut markdown = String::new();
                for index in &indices {
                    if let Some(block) = blocks_vec.get(*index) {
                        markdown.push_str(&block.to_markdown());
                        markdown.push_str("\n\n");
                    }
                }
                let window = window();
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&markdown);
                
                // Remove selected blocks (in reverse order to preserve indices)
                let mut new_blocks = blocks_vec;
                for &index in indices.iter().rev() {
                    if new_blocks.len() > 1 {
                        new_blocks.remove(index);
                    }
                }
                
                // Ensure at least one block remains
                if new_blocks.is_empty() {
                    new_blocks.push(T::new_block());
                }
                
                blocks.set(new_blocks);
                selected_blocks.set(HashSet::new());
                
                // Adjust active block index
                let new_active = indices.first().copied().unwrap_or(0).min(blocks.len().saturating_sub(1));
                active_block_index.set(new_active);
            }
        }
    };

    // Handle paste blocks at current position
    let handle_paste_blocks = {
        let blocks = blocks.clone();
        let clipboard_blocks = clipboard_blocks.clone();
        let active_block_index = active_block_index.clone();
        move || {
            let paste_blocks = (*clipboard_blocks).clone();
            if !paste_blocks.is_empty() {
                let mut new_blocks = (*blocks).clone();
                let insert_at = *active_block_index + 1;
                
                for (i, block) in paste_blocks.into_iter().enumerate() {
                    new_blocks.insert(insert_at + i, block);
                }
                
                blocks.set(new_blocks);
                active_block_index.set(insert_at);
            }
        }
    };

    // Handle delete selected blocks
    let handle_delete_selected = {
        let blocks = blocks.clone();
        let selected_blocks = selected_blocks.clone();
        let active_block_index = active_block_index.clone();
        move || {
            let selected = (*selected_blocks).clone();
            if !selected.is_empty() {
                let mut new_blocks = (*blocks).clone();
                
                // Sort indices in reverse order to remove from end first
                let mut indices: Vec<_> = selected.iter().copied().collect();
                indices.sort();
                indices.reverse();
                
                for index in indices {
                    if new_blocks.len() > 1 && index < new_blocks.len() {
                        new_blocks.remove(index);
                    }
                }
                
                // Ensure at least one block remains
                if new_blocks.is_empty() {
                    new_blocks.push(T::new_block());
                }
                
                blocks.set(new_blocks.clone());
                selected_blocks.set(HashSet::new());
                
                // Adjust active block index
                let new_active = (*active_block_index).min(new_blocks.len().saturating_sub(1));
                active_block_index.set(new_active);
            }
        }
    };

    // Handle undo
    let handle_undo = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        let history = history.clone();
        move || {
            if let Some(entry) = history.borrow_mut().undo() {
                blocks.set(entry.blocks);
                active_block_index.set(entry.active_index);
            }
        }
    };

    // Handle redo
    let handle_redo = {
        let blocks = blocks.clone();
        let active_block_index = active_block_index.clone();
        let history = history.clone();
        move || {
            if let Some(entry) = history.borrow_mut().redo() {
                blocks.set(entry.blocks);
                active_block_index.set(entry.active_index);
            }
        }
    };

    // Handle select all blocks
    let handle_select_all = {
        let blocks = blocks.clone();
        let selected_blocks = selected_blocks.clone();
        move || {
            let all_indices: HashSet<usize> = (0..blocks.len()).collect();
            selected_blocks.set(all_indices);
        }
    };

    // Handle global keydown for copy, cut, paste, undo, redo, delete, select all
    let on_global_keydown = {
        let handle_copy = handle_copy.clone();
        let handle_cut = handle_cut.clone();
        let handle_paste_blocks = handle_paste_blocks.clone();
        let handle_delete_selected = handle_delete_selected.clone();
        let handle_undo = handle_undo.clone();
        let handle_redo = handle_redo.clone();
        let handle_select_all = handle_select_all.clone();
        let selected_blocks = selected_blocks.clone();
        let clipboard_blocks = clipboard_blocks.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            let is_mod = e.ctrl_key() || e.meta_key();
            
            if is_mod {
                match key.as_str() {
                    // Ctrl/Cmd+C - Copy selected blocks
                    "c" if !selected_blocks.is_empty() => {
                        e.prevent_default();
                        handle_copy();
                    }
                    // Ctrl/Cmd+X - Cut selected blocks
                    "x" if !selected_blocks.is_empty() => {
                        e.prevent_default();
                        handle_cut();
                    }
                    // Ctrl/Cmd+V - Paste blocks (only if we have blocks in clipboard)
                    "v" if !clipboard_blocks.is_empty() => {
                        e.prevent_default();
                        handle_paste_blocks();
                    }
                    // Ctrl/Cmd+Z - Undo
                    "z" if !e.shift_key() => {
                        e.prevent_default();
                        handle_undo();
                    }
                    // Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y - Redo
                    "z" if e.shift_key() => {
                        e.prevent_default();
                        handle_redo();
                    }
                    "y" => {
                        e.prevent_default();
                        handle_redo();
                    }
                    // Ctrl/Cmd+A - Select all blocks
                    "a" => {
                        e.prevent_default();
                        handle_select_all();
                    }
                    _ => {}
                }
            } else {
                match key.as_str() {
                    // Delete or Backspace - Delete selected blocks
                    "Delete" | "Backspace" if !selected_blocks.is_empty() => {
                        e.prevent_default();
                        handle_delete_selected();
                    }
                    // Escape - Clear selection
                    "Escape" => {
                        selected_blocks.set(HashSet::new());
                    }
                    _ => {}
                }
            }
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
                            // Get cursor position and split content
                            let current_content = new_blocks[index].get_content();
                            let mut content_before = current_content.clone();
                            let mut content_after = String::new();

                            if let Some(selection) = window().get_selection().ok().flatten() {
                                if selection.range_count() > 0 {
                                    if let Ok(range) = selection.get_range_at(0) {
                                        // Get the text content and cursor position
                                        if let Some(container) = range.start_container().ok() {
                                            let text = container.text_content().unwrap_or_default();
                                            if let Ok(offset) = range.start_offset() {
                                                let cursor_pos = offset as usize;
                                                // Handle UTF-8 properly by using char indices
                                                let chars: Vec<char> = text.chars().collect();
                                                if cursor_pos <= chars.len() {
                                                    content_before = chars[..cursor_pos].iter().collect();
                                                    content_after = chars[cursor_pos..].iter().collect();
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Update current block with content before cursor
                            new_blocks[index] = new_blocks[index].set_content(content_before);

                            // Create new block with content after cursor
                            let new_block = T::new_block_with_content(content_after);

                            // Insert the new block after current
                            new_blocks.insert(index + 1, new_block);
                            blocks.set(new_blocks.clone());

                            // Focus the new block
                            active_block_index.set(index + 1);

                            // Store selection info for the new block (cursor at start)
                            selection_info.set(Some((index + 1, 0, 0)));

                            // Schedule focus on the new block
                            let new_index = index + 1;
                            let closure = Closure::once_into_js(move || {
                                if let Some(editor) =
                                    document().get_element_by_id(&format!("block-{}", new_index))
                                {
                                    if let Some(editable) = editor
                                        .query_selector("[contenteditable='true']")
                                        .ok()
                                        .flatten()
                                    {
                                        if let Ok(html_element) =
                                            editable.dyn_into::<web_sys::HtmlElement>()
                                        {
                                            let _ = html_element.focus();
                                        }
                                    }
                                }
                            });

                            let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
                                closure.as_ref().unchecked_ref(),
                                0,
                            );
                        }
                    }
                }
                "Backspace" => {
                    // Check if we can delete this block (cursor at start and block is empty)
                    let can_delete = if index > 0 {
                        if let Some(block) = new_blocks.get(index) {
                            // Check if cursor is at the very beginning
                            let cursor_at_start = if let Some(selection) =
                                window().get_selection().ok().flatten()
                            {
                                if selection.range_count() > 0 {
                                    if let Ok(range) = selection.get_range_at(0) {
                                        range.start_offset().unwrap_or(1) == 0
                                            && range.collapsed()
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            };

                            cursor_at_start && block.can_delete()
                        } else {
                            false
                        }
                    } else {
                        false // Can't delete first block
                    };

                    if can_delete {
                        e.prevent_default();

                        // Get content from block being deleted to merge with previous
                        let deleted_content = new_blocks[index].get_content();
                        let prev_content = new_blocks[index - 1].get_content();
                        let merged_content = format!("{}{}", prev_content, deleted_content);

                        // Update previous block with merged content
                        new_blocks[index - 1] = new_blocks[index - 1].set_content(merged_content);

                        // Remove current block
                        new_blocks.remove(index);
                        blocks.set(new_blocks.clone());

                        // Set cursor position to end of previous content
                        selection_info.set(Some((index - 1, prev_content.len(), prev_content.len())));
                        active_block_index.set(index - 1);

                        // Focus previous block
                        let prev_index = index - 1;
                        let cursor_pos = prev_content.len();
                        let closure = Closure::once_into_js(move || {
                            if let Some(editor) =
                                document().get_element_by_id(&format!("block-{}", prev_index))
                            {
                                if let Some(editable) = editor
                                    .query_selector("[contenteditable='true']")
                                    .ok()
                                    .flatten()
                                {
                                    if let Ok(html_element) =
                                        editable.dyn_into::<web_sys::HtmlElement>()
                                    {
                                        let _ = html_element.focus();

                                        // Set cursor position at end of previous content
                                        if let Some(selection) =
                                            window().get_selection().ok().flatten()
                                        {
                                            if let Ok(range) = document().create_range() {
                                                if let Some(text_node) = html_element.first_child() {
                                                    let _ = range.set_start(
                                                        &text_node,
                                                        cursor_pos as u32,
                                                    );
                                                    range.collapse_with_to_start(true);
                                                    let _ = selection.remove_all_ranges();
                                                    let _ = selection.add_range(&range);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        });

                        let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            0,
                        );
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
                "Tab" => {
                    e.prevent_default();
                    
                    // Insert spaces at cursor position for indentation
                    let indent = if e.shift_key() { "" } else { "    " }; // 4 spaces or unindent
                    
                    if let Some(selection) = window().get_selection().ok().flatten() {
                        if selection.range_count() > 0 {
                            if let Ok(range) = selection.get_range_at(0) {
                                // For shift+tab, we could implement unindent
                                // For now, just insert spaces
                                if !e.shift_key() {
                                    if let Some(container) = range.start_container().ok() {
                                        let text = container.text_content().unwrap_or_default();
                                        if let Ok(offset) = range.start_offset() {
                                            let cursor_pos = offset as usize;
                                            let chars: Vec<char> = text.chars().collect();
                                            let before: String = chars[..cursor_pos.min(chars.len())].iter().collect();
                                            let after: String = chars[cursor_pos.min(chars.len())..].iter().collect();
                                            let new_content = format!("{}{}{}", before, indent, after);
                                            
                                            new_blocks[index] = new_blocks[index].set_content(new_content);
                                            blocks.set(new_blocks);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    // Handle Ctrl/Cmd + B/I/U for inline formatting
                    if e.ctrl_key() || e.meta_key() {
                        match key.as_str() {
                            "b" => {
                                e.prevent_default();
                                apply_inline_format("**", "**");
                            }
                            "i" => {
                                e.prevent_default();
                                apply_inline_format("*", "*");
                            }
                            "u" => {
                                e.prevent_default();
                                apply_inline_format("<u>", "</u>");
                            }
                            "`" => {
                                e.prevent_default();
                                apply_inline_format("`", "`");
                            }
                            _ => {}
                        }
                    }
                }
            }
        })
    };

    // Helper function to apply inline formatting to selected text
    fn apply_inline_format(prefix: &str, suffix: &str) {
        if let Some(selection) = window().get_selection().ok().flatten() {
            if selection.range_count() > 0 {
                if let Ok(range) = selection.get_range_at(0) {
                    // Get the selected text - convert JsString to String
                    let selected_js_text = range.to_string();
                    let selected_text: String = selected_js_text.into();
                    
                    if !selected_text.is_empty() {
                        // Wrap selected text with formatting
                        let formatted = format!("{}{}{}", prefix, selected_text, suffix);
                        
                        // Insert the formatted text
                        let _ = range.delete_contents();
                        let text_node = document().create_text_node(&formatted);
                        let _ = range.insert_node(&text_node);
                    }
                }
            }
        }
    }

    html! {
        <div 
            class={classes!(&brandguide.markdown_editor_container, props.class.clone())}
            onkeydown={on_global_keydown}
            tabindex="0"
        >
            <div 
                class={classes!(&brandguide.markdown_editor_blocks_container, "markdown-editor-blocks")}
                ondragover={on_container_drag_over}
                ondrop={on_container_drop}
            >
                {
                    blocks.iter().enumerate().map(|(index, block)| {
                        let is_selected = selected_blocks.contains(&index);
                        let show_drop_indicator = drop_indicator_index.map_or(false, |idx| idx == index);
                        
                        let block_props = EditorBlockProps {
                            id: format!("block-{}", index),
                            index,
                            block: block.clone(),
                            is_active: index == *active_block_index,
                            is_selected,
                            on_focus: on_focus_block.clone(),
                            on_input: on_input_block.clone(),
                            on_keydown: on_keydown_block.clone(),
                            on_update_block_type: on_update_block_type.clone(),
                            on_blur: on_blur_block.clone(),
                            on_insert_block: on_insert_block.clone(),
                            on_block_action: on_block_action.clone(),
                            on_select: on_select_block.clone(),
                            on_drag_start: on_drag_start.clone(),
                            on_drag_over: on_drag_over.clone(),
                            on_drop: on_drop.clone(),
                            on_drag_end: on_drag_end.clone(),
                            on_drag_leave: on_drag_leave.clone(),
                            show_block_actions: props.show_block_actions,
                        };
                        
                        html! {
                            <>
                                // Drop indicator line
                                if show_drop_indicator {
                                    <div class="h-1 bg-blue-500 rounded-full mx-2 my-1 animate-pulse" />
                                }
                                
                                <div key={format!("block-{}", index)}>
                                    <EditorBlock<T> ..block_props />
                                </div>
                                
                                // Show drop indicator at the end if needed
                                if blocks.len() == index + 1 && drop_indicator_index.map_or(false, |idx| idx == index + 1) {
                                    <div class="h-1 bg-blue-500 rounded-full mx-2 my-1 animate-pulse" />
                                }
                            </>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
