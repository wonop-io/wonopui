// editor_block.rs
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;

use crate::block::BlockTrait;

#[derive(Properties, PartialEq)]
pub struct EditorBlockProps<T: BlockTrait> {
    pub id: String,
    pub index: usize,
    pub block: T,
    pub is_active: bool,
    pub on_focus: Callback<usize>,
    pub on_input: Callback<(usize, String)>,
    pub on_keydown: Callback<KeyboardEvent>,
    pub on_update_block_type: Callback<(usize, T)>,
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_insert_block: Callback<usize>,
    #[prop_or_default]
    pub on_block_action: Callback<(usize, String)>,
    #[prop_or(false)]
    pub show_block_actions: bool,
    #[prop_or(false)]
    pub is_selected: bool,
    #[prop_or_default]
    pub on_select: Callback<(usize, MouseEvent)>,
    #[prop_or_default]
    pub on_drag_start: Callback<(usize, web_sys::DragEvent)>,
    #[prop_or_default]
    pub on_drag_over: Callback<(usize, web_sys::DragEvent)>,
    #[prop_or_default]
    pub on_drop: Callback<(usize, web_sys::DragEvent)>,
    #[prop_or_default]
    pub on_drag_end: Callback<web_sys::DragEvent>,
    #[prop_or_default]
    pub on_drag_leave: Callback<web_sys::DragEvent>,
}

#[function_component(EditorBlock)]
pub fn editor_block<T: BlockTrait>(props: &EditorBlockProps<T>) -> Html {
    let node_ref = use_node_ref();
    let dropdown_open = use_state(|| false);

    // Close dropdown when clicking outside - properly managed closure to avoid memory leak
    {
        let dropdown_open = dropdown_open.clone();
        let is_open = *dropdown_open;

        use_effect_with(is_open, move |is_open| {
            let cleanup: Box<dyn FnOnce()> = if *is_open {
                let dropdown_open_clone = dropdown_open.clone();

                // Create closure and immediately get a raw pointer for cleanup
                let closure = Rc::new(Closure::wrap(Box::new(move |_e: web_sys::Event| {
                    dropdown_open_clone.set(false);
                }) as Box<dyn FnMut(web_sys::Event)>));

                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let _ = document.add_event_listener_with_callback(
                        "click",
                        closure.as_ref().as_ref().unchecked_ref(),
                    );
                }

                // Return cleanup that removes the listener
                let closure_for_cleanup = closure.clone();
                Box::new(move || {
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let _ = document.remove_event_listener_with_callback(
                            "click",
                            closure_for_cleanup.as_ref().as_ref().unchecked_ref(),
                        );
                    }
                    // closure is dropped here, cleaning up the Closure
                })
            } else {
                Box::new(|| {})
            };

            cleanup
        });
    }

    // Create callback for updating block type
    let update_block_callback = {
        let on_update_block_type = props.on_update_block_type.clone();
        let index = props.index;
        Callback::from(move |new_block_type: T| {
            on_update_block_type.emit((index, new_block_type));
        })
    };

    // Create focus callback
    let on_focus = {
        let on_focus = props.on_focus.clone();
        let index = props.index;
        Callback::from(move |e: FocusEvent| {
            e.stop_propagation();
            on_focus.emit(index);
        })
    };

    let on_click = {
        let on_focus = props.on_focus.clone();
        let on_select = props.on_select.clone();
        let index = props.index;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            
            // If shift or ctrl/cmd is held, handle selection
            if e.shift_key() || e.ctrl_key() || e.meta_key() {
                on_select.emit((index, e));
            } else {
                on_focus.emit(index);
            }
        })
    };

    // Create callback for inserting a new block
    let on_insert_block = {
        let on_insert_block = props.on_insert_block.clone();
        let index = props.index;
        Callback::from(move |_: MouseEvent| {
            on_insert_block.emit(index + 1); // Insert after current block
        })
    };

    // Toggle dropdown
    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            dropdown_open.set(!*dropdown_open);
        })
    };

    // Handle block action
    let on_action = {
        let on_block_action = props.on_block_action.clone();
        let index = props.index;
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |action: String| {
            dropdown_open.set(false);
            on_block_action.emit((index, action));
        })
    };

    // Wrap contenteditable in block rendering based on block type
    let rendered = props.block.render(
        update_block_callback,
        props.on_keydown.clone(),
        on_focus.clone(),
        props.on_blur.clone(),
        props.is_active,
    );

    // Handle selection toggle
    let on_select_click = {
        let on_select = props.on_select.clone();
        let index = props.index;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_select.emit((index, e));
        })
    };

    // Drag handlers
    let on_drag_start = {
        let on_drag_start = props.on_drag_start.clone();
        let index = props.index;
        Callback::from(move |e: web_sys::DragEvent| {
            on_drag_start.emit((index, e));
        })
    };

    let on_drag_over = {
        let on_drag_over = props.on_drag_over.clone();
        let index = props.index;
        Callback::from(move |e: web_sys::DragEvent| {
            on_drag_over.emit((index, e));
        })
    };

    let on_drop = {
        let on_drop = props.on_drop.clone();
        let index = props.index;
        Callback::from(move |e: web_sys::DragEvent| {
            on_drop.emit((index, e));
        })
    };

    let on_drag_end = {
        let on_drag_end = props.on_drag_end.clone();
        Callback::from(move |e: web_sys::DragEvent| {
            on_drag_end.emit(e);
        })
    };

    let on_drag_leave = {
        let on_drag_leave = props.on_drag_leave.clone();
        Callback::from(move |e: web_sys::DragEvent| {
            on_drag_leave.emit(e);
        })
    };

    // Check if this block is being dragged
    let is_dragging = use_state(|| false);
    
    let on_drag_start_with_style = {
        let on_drag_start = on_drag_start.clone();
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: web_sys::DragEvent| {
            is_dragging.set(true);
            on_drag_start.emit(e);
        })
    };

    let on_drag_end_with_style = {
        let on_drag_end = on_drag_end.clone();
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: web_sys::DragEvent| {
            is_dragging.set(false);
            on_drag_end.emit(e);
        })
    };

    // Lucide icons as inline SVG components
    let check_icon = html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
            <path d="M20 6 9 17l-5-5"/>
        </svg>
    };

    let grip_vertical_icon = html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="9" cy="12" r="1"/>
            <circle cx="9" cy="5" r="1"/>
            <circle cx="9" cy="19" r="1"/>
            <circle cx="15" cy="12" r="1"/>
            <circle cx="15" cy="5" r="1"/>
            <circle cx="15" cy="19" r="1"/>
        </svg>
    };

    let ellipsis_vertical_icon = html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="1"/>
            <circle cx="12" cy="5" r="1"/>
            <circle cx="12" cy="19" r="1"/>
        </svg>
    };

    html! {
        <div 
            class={classes!(
                // Base styles
                "group",
                "relative",
                "flex",
                "gap-1",
                "rounded-lg",
                "transition-colors",
                "duration-150",
                // Selection state
                if props.is_selected { 
                    "bg-blue-50 dark:bg-blue-950/30 ring-1 ring-blue-500/50" 
                } else { 
                    "" 
                },
                // Dragging state
                if *is_dragging { "opacity-50" } else { "" }
            )}
            ondragover={on_drag_over}
            ondrop={on_drop}
            ondragleave={on_drag_leave}
        >
            // Left toolbar - selection & drag handle
            <div class="flex flex-col items-center gap-0.5 pt-2.5 pl-1">
                // Selection checkbox
                <button
                    class="h-6 w-6 flex items-center justify-center rounded-md opacity-0 group-hover:opacity-100 transition-all duration-150 hover:bg-zinc-100 dark:hover:bg-zinc-800 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-400"
                    onclick={on_select_click}
                    title="Select block"
                >
                    <div class={classes!(
                        "h-4",
                        "w-4",
                        "shrink-0",
                        "rounded-sm",
                        "border",
                        "shadow-sm",
                        "transition-colors",
                        "duration-150",
                        if props.is_selected { 
                            "border-blue-600 bg-blue-600 text-white" 
                        } else { 
                            "border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-900" 
                        }
                    )}>
                        if props.is_selected {
                            <div class="flex items-center justify-center h-full">
                                {check_icon}
                            </div>
                        }
                    </div>
                </button>

                // Drag handle
                <div
                    draggable="true"
                    ondragstart={on_drag_start_with_style}
                    ondragend={on_drag_end_with_style}
                    class="h-6 w-6 flex items-center justify-center rounded-md opacity-0 group-hover:opacity-100 cursor-grab active:cursor-grabbing transition-all duration-150 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-400 hover:text-zinc-600 dark:text-zinc-500 dark:hover:text-zinc-300"
                    title="Drag to reorder"
                >
                    {grip_vertical_icon}
                </div>
            </div>

            <div class="flex items-start gap-1 flex-1 min-w-0">
                // Block actions dropdown
                if props.show_block_actions {
                    <div class="relative pt-2">
                        <button
                            class="h-6 w-6 flex items-center justify-center rounded-md opacity-0 group-hover:opacity-100 transition-all duration-150 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-400 hover:text-zinc-600 dark:text-zinc-500 dark:hover:text-zinc-300 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-400"
                            onclick={toggle_dropdown}
                            title="Block actions"
                        >
                            {ellipsis_vertical_icon}
                        </button>
                        
                        // Dropdown menu
                        if *dropdown_open {
                            <div class="absolute left-0 top-9 z-50 min-w-[160px] overflow-hidden rounded-md border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 p-1 shadow-lg">
                                {
                                    props.block.get_block_actions().iter().map(|(action, label)| {
                                        let action_clone = action.to_string();
                                        let on_action = on_action.clone();
                                        html! {
                                            <button
                                                class="relative flex w-full cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm text-zinc-700 dark:text-zinc-300 outline-none transition-colors hover:bg-zinc-100 dark:hover:bg-zinc-800 focus:bg-zinc-100 dark:focus:bg-zinc-800"
                                                onclick={Callback::from(move |_| {
                                                    on_action.emit(action_clone.clone());
                                                })}
                                            >
                                                {label}
                                            </button>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    </div>
                }
                
                // Main block content
                <div class="flex-1 min-w-0">
                    <div
                        id={props.id.clone()}
                        ref={node_ref}
                        onfocus={on_focus}
                        onclick={on_click}
                        onkeydown={props.on_keydown.clone()}
                        class={classes!(
                            "rounded-md",
                            "transition-all",
                            "duration-150",
                            if props.is_active { 
                                // Active block - subtle ring
                                "ring-1 ring-blue-500/40 bg-blue-50/50 dark:bg-blue-950/20" 
                            } else { 
                                // Hover state
                                "hover:bg-zinc-50 dark:hover:bg-zinc-800/50" 
                            }
                        )}
                    >
                        {rendered}
                    </div>
                </div>
            </div>
        </div>
    }
}
