// command_menu.rs
#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use web_sys::KeyboardEvent;
use yew::prelude::*;

use super::block::BlockTrait;

#[derive(Properties, PartialEq, Clone)]
pub struct CommandMenuProps<T: BlockTrait> {
    pub position: (i32, i32),
    pub search_query: String,
    pub options: Vec<(T, String, String, Option<Html>)>,
    pub selected_index: usize,
    pub on_select: Callback<T>,
    pub on_close: Callback<()>,
}

#[function_component(CommandMenu)]
pub fn command_menu<T: BlockTrait>(props: &CommandMenuProps<T>) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let position_style = format!(
        "position: absolute; left: {}px; top: {}px;",
        props.position.0, props.position.1
    );

    // Handle keyboard navigation for the command menu
    let on_keydown = {
        let on_select = props.on_select.clone();
        let on_close = props.on_close.clone();
        let options = props.options.clone();
        let selected_index = props.selected_index;

        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" => {
                e.prevent_default();
                if selected_index < options.len() {
                    let (block_type, _, _, _) = &options[selected_index];
                    on_select.emit(block_type.clone());
                }
            }
            "Escape" => {
                e.prevent_default();
                on_close.emit(());
            }
            _ => {}
        })
    };

    html! {
        <div
            class={classes!(&brandguide.markdown_command_menu_container)}
            style={position_style}
            onkeydown={on_keydown}
        >
            <div class={classes!(&brandguide.command_container)}>
                <div class={classes!(&brandguide.command_list)} role="listbox">
                    {
                        if props.options.is_empty() {
                            html! {
                                <div class={classes!(&brandguide.command_item)}>
                                    { "No results available" }
                                </div>
                            }
                        } else {
                            props.options.iter().enumerate().map(|(index, (block_type, _, label, icon))| {
                                let on_select = props.on_select.clone();
                                let block_type = block_type.clone();
                                let is_selected = index == props.selected_index;

                                html! {
                                    <div
                                        class={classes!(if is_selected { &brandguide.command_selected_item } else { &brandguide.command_item })}
                                        onclick={Callback::from(move |_| {
                                            on_select.emit(block_type.clone());
                                        })}
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
}
