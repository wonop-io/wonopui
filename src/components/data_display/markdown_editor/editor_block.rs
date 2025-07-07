// editor_block.rs
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::BrandGuideType;
use wasm_bindgen::JsCast;
use web_sys::FocusEvent;
use yew::prelude::*;

use super::block::{Block, BlockTrait};
use super::utils::{document, window};

// Properties for a single editor block
#[derive(Properties, PartialEq)]
pub struct EditorBlockProps<T: BlockTrait> {
    pub id: String,
    pub index: usize,
    pub block: Block<T>,
    pub is_active: bool,
    pub on_focus: Callback<usize>,
    pub on_input: Callback<(usize, String)>,
    pub on_keydown: Callback<KeyboardEvent>,
    pub on_update_block_type: Callback<(usize, T)>,
    pub on_blur: Callback<FocusEvent>,
}

// Component for a single editor block
#[function_component(EditorBlock)]
pub fn editor_block<T: BlockTrait>(props: &EditorBlockProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    // Create callback for updating block type
    let update_block_callback = {
        let on_update_block_type = props.on_update_block_type.clone();
        let index = props.index;
        Callback::from(move |new_block_type: T| {
            on_update_block_type.emit((index, new_block_type));
        })
    };

    // Create callbacks for events that will be passed to the rendered block
    let on_focus = {
        let on_focus = props.on_focus.clone();
        let index = props.index;
        Callback::from(move |_: FocusEvent| {
            on_focus.emit(index);
        })
    };

    let on_input = {
        let on_input = props.on_input.clone();
        let index = props.index;
        Callback::from(move |content: String| {
            on_input.emit((index, content));
        })
    };

    let on_keydown = props.on_keydown.clone();
    let on_blur = props.on_blur.clone();

    // Create child focus callback
    let on_child_focus = {
        let on_focus = on_focus.clone();
        Callback::from(move |e: FocusEvent| {
            on_focus.emit(e);
        })
    };

    // Create child blur callback
    let on_child_blur = {
        let on_blur = on_blur.clone();
        Callback::from(move |e: FocusEvent| {
            on_blur.emit(e);
        })
    };

    // Render the block using the BlockTrait's render method
    let rendered = props.block.block_type.render(
        props.block.content.clone(),
        update_block_callback,
        on_keydown.clone(),
        on_child_focus,
        on_child_blur,
        props.is_active,
    );

    html! {
        <div
            id={props.id.clone()}
            class={classes!(
                &brandguide.markdown_editor_block,
                if props.is_active { &brandguide.markdown_editor_block_active } else { &brandguide.markdown_editor_block }
            )}
            onfocus={on_focus}
            onkeydown={on_keydown}
            onblur={on_blur.clone()}
        >
            { rendered }
        </div>
    }
}
