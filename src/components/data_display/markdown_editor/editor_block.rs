// editor_block.rs
use crate::components::data_display::contenteditable_commands::ContentEditableWithCommands;
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use wasm_bindgen::JsCast;
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
}

#[function_component(EditorBlock)]
pub fn editor_block<T: BlockTrait>(props: &EditorBlockProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let node_ref = use_node_ref();

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
        Callback::from(move |_: FocusEvent| {
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
        <div class="block-container">
            <div
                id={props.id.clone()}
                ref={node_ref}
                onkeydown={props.on_keydown.clone()}
                class={classes!(
                    "p-1",  // Removed mb-2 margin
                    if props.is_active { "border-zinc-500 border rounded" } else { "" }
                )}
            >
                {rendered}
            </div>
            <div
                class="rounded h-2 w-full cursor-text hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-colors"
                onclick={on_insert_block}
            />
        </div>
    }
}
