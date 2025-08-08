// editor_block.rs
use crate::components::data_display::contenteditable_commands::ContentEditableWithCommands;
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;

use super::block::BlockTrait;
use super::utils::{document, window};

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
}

#[function_component(EditorBlock)]
pub fn editor_block<T: BlockTrait>(props: &EditorBlockProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let node_ref = use_node_ref();
    let dropdown_open = use_state(|| false);

    // Close dropdown when clicking outside
    {
        let dropdown_open = dropdown_open.clone();
        use_effect_with(dropdown_open.clone(), move |dropdown_open| {
            let cleanup: Box<dyn FnOnce()> = if **dropdown_open {
                let dropdown_open_clone = dropdown_open.clone();
                let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                    // Prevent the dropdown toggle button from closing the dropdown
                    e.stop_propagation();
                    dropdown_open_clone.set(false);
                }) as Box<dyn FnMut(web_sys::Event)>);
                
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let _ = document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                }
                
                let closure = Box::leak(Box::new(closure));
                
                Box::new(move || {
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let _ = document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                    }
                })
            } else {
                Box::new(|| {})
            };
            cleanup
        });
    }

    // Create callbacks for the ContentEditableWithCommands component
    let on_input = {
        let on_input = props.on_input.clone();
        let index = props.index;
        Callback::from(move |content: String| {
            on_input.emit((index, content));
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
        let index = props.index;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_focus.emit(index);
        })
    };

    // Create callback for inserting a new block
    let on_insert_block = {
        let on_insert_block = props.on_insert_block.clone();
        let index = props.index;
        Callback::from(move |_| {
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

    // Get command options for this block type
    let command_options = {
        // Get block types that can be used as options
        let block_types = T::search(None);

        // Convert to command options format
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

    // Wrap contenteditable in block rendering based on block type
    let rendered = props.block.render(
        update_block_callback,
        props.on_keydown.clone(),
        on_focus.clone(),
        props.on_blur.clone(),
        props.is_active,
    );

    html! {
        <div class="block-container group relative">
            <div class="flex items-start gap-2">
                // Optional dropdown button for block actions
                if props.show_block_actions {
                    <div class="relative">
                        <button
                            class="opacity-0 group-hover:opacity-100 transition-opacity duration-200 p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
                            onclick={toggle_dropdown}
                            title="Block actions"
                        >
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z" />
                            </svg>
                        </button>
                        
                        // Dropdown menu
                        if *dropdown_open {
                            <div class="absolute left-0 top-8 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-xl min-w-[150px] z-50 animate-in fade-in-0 zoom-in-95 duration-200">
                                <div class="p-1">
                                    {
                                        props.block.get_block_actions().iter().map(|(action, label)| {
                                            let action_clone = action.to_string();
                                            let on_action = on_action.clone();
                                            html! {
                                                <button
                                                    class="w-full text-left px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md transition-colors duration-150 flex items-center gap-2"
                                                    onclick={Callback::from(move |_| {
                                                        on_action.emit(action_clone.clone());
                                                    })}
                                                >
                                                    <span class="flex-1">{label}</span>
                                                </button>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
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
                            "p-2", "rounded-md", "transition-all", "duration-200",
                            if props.is_active { 
                                "ring-2 ring-blue-500/50 bg-blue-50/50 dark:bg-blue-950/20" 
                            } else { 
                                "hover:bg-gray-50 dark:hover:bg-gray-800/50" 
                            }
                        )}
                    >
                        {rendered}
                    </div>
                </div>
            </div>
            
            // Insert block area
            <div
                class="mt-2 rounded h-2 w-full cursor-text opacity-0 group-hover:opacity-100 hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-all duration-200"
                onclick={on_insert_block}
                title="Click to add a block below"
            />
        </div>
    }
}
