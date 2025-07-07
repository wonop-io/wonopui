#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::{BrandGuideType, ClassesContainer};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::CompositionEvent;
use web_sys::{Document, HtmlElement, Node, Range};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MentionInputProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or_default]
    pub start_token: Option<String>,
    #[prop_or_default]
    pub end_token: Option<String>,
    #[prop_or_default]
    pub candidates: Option<Callback<String, Vec<String>>>,
    #[prop_or_default]
    pub onupdate: Option<Callback<String>>,
    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(MentionInput)]
pub fn mention_input(props: &MentionInputProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = Rc::new(get_brandguide());

    let input_value = use_state(|| props.default_value.clone());
    let container_ref = use_node_ref();
    let editable_ref = use_node_ref();
    let suggestion_ref = use_node_ref();
    let cursor_position = use_state(|| (0, 0)); // (node_index, offset)
    let selection_data = use_state(|| (None::<web_sys::Selection>, None::<Range>)); // (selection, range)
    let is_composing = use_state(|| false);

    // Default tokens if not provided
    let start_token = props.start_token.clone().unwrap_or_else(|| "@".to_string());
    let end_token = props.end_token.clone().unwrap_or_else(|| " ".to_string());

    // Store the raw text content separate from the rendered HTML
    let raw_content = use_state(|| props.default_value.clone());

    // Effect for setting up focus event listener
    {
        let container_ref = container_ref.clone();
        let editable_ref = editable_ref.clone();
        use_effect_with((), move |_| {
            let mut cleanup_needed = false;
            let mut event_closure = None;

            if let Some(container) = container_ref.cast::<HtmlElement>() {
                let editable_ref = editable_ref.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                    if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                        let _ = editable.focus();
                    }
                }) as Box<dyn FnMut(_)>);

                container
                    .add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())
                    .unwrap();

                cleanup_needed = true;
                event_closure = Some(closure);
            }

            move || {
                if cleanup_needed {
                    if let Some(closure) = event_closure {
                        drop(closure);
                    }
                }
            }
        });
    }

    // Effect to initialize content from default_value
    {
        let editable_ref = editable_ref.clone();
        let default_value = props.default_value.clone();
        let input_value = input_value.clone();
        let start_token = start_token.clone();
        let end_token = end_token.clone();
        let brandguide = brandguide.clone();
        let raw_content = raw_content.clone();

        use_effect_with((), move |_| {
            if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                // Render the content with mentions as styled badges
                let rendered_content = render_content(
                    &default_value,
                    &start_token,
                    &end_token,
                    brandguide.tag_input_tag.to_string(),
                );

                // Set the HTML content instead of just text
                editable.set_inner_html(&rendered_content);
                input_value.set(default_value.clone());
                raw_content.set(default_value);
            }
            || {}
        });
    }

    // Effect to set up composition event listeners
    {
        let editable_ref = editable_ref.clone();
        let is_composing = is_composing.clone();
        let raw_content = raw_content.clone();
        let input_value = input_value.clone();
        let start_token = start_token.clone();
        let end_token = end_token.clone();
        let brandguide = brandguide.clone();
        let props_onupdate = props.onupdate.clone();

        use_effect_with(editable_ref.clone(), move |editable_ref| {
            let mut start_closure = None;
            let mut end_closure = None;

            if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                // Composition start handler
                let is_composing_clone = is_composing.clone();
                let start_handler = Closure::wrap(Box::new(move |_: web_sys::CompositionEvent| {
                    is_composing_clone.set(true);
                }) as Box<dyn FnMut(_)>);

                editable
                    .add_event_listener_with_callback(
                        "compositionstart",
                        start_handler.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                start_closure = Some(start_handler);

                // Composition end handler
                let is_composing_clone = is_composing.clone();
                let raw_content_clone = raw_content.clone();
                let input_value_clone = input_value.clone();
                let start_token_clone = start_token.clone();
                let end_token_clone = end_token.clone();
                let brandguide_clone = brandguide.clone();
                let props_onupdate_clone = props_onupdate.clone();
                let editable_ref_clone = editable_ref.clone();

                let end_handler = Closure::wrap(Box::new(move |_: web_sys::CompositionEvent| {
                    is_composing_clone.set(false);

                    // Process the completed composition
                    if let Some(editable) = editable_ref_clone.cast::<HtmlElement>() {
                        // Save selection if possible
                        let selection_opt = save_selection();

                        // Extract and process the text
                        let current_text = extract_raw_text(&editable);
                        raw_content_clone.set(current_text.clone());

                        // Render with highlighting
                        let rendered_content = render_content(
                            &current_text,
                            &start_token_clone,
                            &end_token_clone,
                            brandguide_clone.tag_input_tag.to_string(),
                        );

                        editable.set_inner_html(&rendered_content);

                        // Restore selection if possible
                        if let Some((selection, range)) = selection_opt {
                            restore_selection(&selection, &range);
                        }

                        // Update state and notify parent
                        input_value_clone.set(current_text.clone());
                        if let Some(ref onupdate) = props_onupdate_clone {
                            onupdate.emit(current_text);
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let editable_ref = editable_ref.clone();
                editable
                    .add_event_listener_with_callback(
                        "compositionend",
                        end_handler.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                end_closure = Some(end_handler);
            }

            let editable_ref = editable_ref.clone();
            move || {
                if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                    if let Some(closure) = start_closure {
                        editable
                            .remove_event_listener_with_callback(
                                "compositionstart",
                                closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }

                    if let Some(closure) = end_closure {
                        editable
                            .remove_event_listener_with_callback(
                                "compositionend",
                                closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                }
            }
        });
    }

    // Convert text with mentions to HTML with badge styling
    // This function hides tokens and styles mentions
    fn render_content(
        text: &str,
        start_token: &str,
        end_token: &str,
        tag_input_tag: String,
    ) -> String {
        let mut result = String::new();
        let mut remaining_text = text;

        while !remaining_text.is_empty() {
            if let Some(start_idx) = remaining_text.find(start_token) {
                // Add text before the mention
                if start_idx > 0 {
                    result.push_str(&remaining_text[..start_idx]);
                }

                // Look for the end token after the start token
                let mention_start = start_idx + start_token.len();
                if mention_start <= remaining_text.len() {
                    if let Some(end_idx) = remaining_text[mention_start..].find(end_token) {
                        let mention_text = &remaining_text[mention_start..mention_start + end_idx];

                        // Create a badge for the mention with data attributes to preserve tokens
                        result.push_str(&format!(
                            "<span class=\"{}\" data-mention=\"true\" data-start-token=\"{}\" data-end-token=\"{}\">{}</span>",
                            tag_input_tag,
                            start_token,
                            end_token,
                            mention_text
                        ));

                        // Update the remaining text to continue after the mention
                        let new_start = mention_start + end_idx + end_token.len();
                        remaining_text = &remaining_text[new_start..];
                    } else {
                        // No end token found, just add the start token as text and continue
                        result.push_str(start_token);
                        remaining_text = &remaining_text[start_idx + start_token.len()..];
                    }
                } else {
                    result.push_str(start_token);
                    break;
                }
            } else {
                // No more mentions, add the remaining text
                result.push_str(remaining_text);
                break;
            }
        }

        result
    }

    // Extract the raw text content from the HTML, including hidden tokens
    fn extract_raw_text(element: &HtmlElement) -> String {
        let mut result = String::new();

        let children = element.child_nodes();
        for i in 0..children.length() {
            if let Some(node) = children.get(i) {
                match node.node_type() {
                    // Text node
                    3 => {
                        if let Some(text) = node.text_content() {
                            result.push_str(&text);
                        }
                    }
                    // Element node
                    1 => {
                        if let Some(element) = node.dyn_ref::<HtmlElement>() {
                            if element.has_attribute("data-mention") {
                                // Get the tokens and mention text
                                let start_token = element
                                    .get_attribute("data-start-token")
                                    .unwrap_or_default();
                                let end_token =
                                    element.get_attribute("data-end-token").unwrap_or_default();
                                let mention_text = element.text_content().unwrap_or_default();

                                // Reconstruct with tokens
                                result.push_str(&start_token);
                                result.push_str(&mention_text);
                                result.push_str(&end_token);
                            } else {
                                // Regular element, get its text content
                                if let Some(text) = element.text_content() {
                                    result.push_str(&text);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        result
    }

    // Better selection preservation that tracks cursor position by character index
    fn get_cursor_position(editable: &HtmlElement) -> Option<usize> {
        let window = web_sys::window()?;
        let selection = window.get_selection().ok()??;
        if selection.range_count() == 0 {
            return None;
        }

        let range = selection.get_range_at(0).ok()?;
        let preselection_range = range.clone_range();
        preselection_range.select_node_contents(editable).ok()?;
        preselection_range
            .set_end(&range.start_container().ok()?, range.start_offset().ok()?)
            .ok()?;

        let position = preselection_range.to_string().length();
        Some(position as usize)
    }

    // Set cursor position by character index
    fn set_cursor_position(editable: &HtmlElement, position: usize) -> Option<()> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let selection = window.get_selection().ok()??;

        // Traverse the node tree to find the position
        let mut current_pos = 0;
        let mut found = false;
        let mut target_node: Option<Node> = None;
        let mut target_offset = 0;

        fn find_position(
            node: &Node,
            position: usize,
            current_pos: &mut usize,
            found: &mut bool,
            target_node: &mut Option<Node>,
            target_offset: &mut u32,
        ) -> Option<()> {
            if *found {
                return Some(());
            }

            match node.node_type() {
                // Text node
                3 => {
                    let text = node.text_content()?;
                    let text_len = text.len();

                    if *current_pos + text_len >= position {
                        // Found the node containing our position
                        *target_node = Some(node.clone());
                        *target_offset = (position - *current_pos) as u32;
                        *found = true;
                        return Some(());
                    }

                    *current_pos += text_len;
                }
                // Element node
                1 => {
                    let children = node.child_nodes();
                    for i in 0..children.length() {
                        if let Some(child) = children.get(i) {
                            find_position(
                                &child,
                                position,
                                current_pos,
                                found,
                                target_node,
                                target_offset,
                            )?;
                            if *found {
                                return Some(());
                            }
                        }
                    }

                    // If this is an element with no children, and we're at the right position
                    if children.length() == 0 && *current_pos == position {
                        *target_node = Some(node.clone());
                        *target_offset = 0;
                        *found = true;
                        return Some(());
                    }
                }
                _ => {}
            }

            Some(())
        }

        find_position(
            editable,
            position,
            &mut current_pos,
            &mut found,
            &mut target_node,
            &mut target_offset,
        )?;

        if let Some(node) = target_node {
            let range = document.create_range().ok()?;
            range.set_start(&node, target_offset).ok()?;
            range.set_end(&node, target_offset).ok()?;

            selection.remove_all_ranges().ok()?;
            selection.add_range(&range).ok()?;
        }

        Some(())
    }

    // Save and restore the caret position
    fn save_selection() -> Option<(web_sys::Selection, web_sys::Range)> {
        let window = web_sys::window()?;
        let selection = window.get_selection().ok()??;
        if selection.range_count() == 0 {
            return None;
        }
        let range = selection.get_range_at(0).ok()?;
        Some((selection, range))
    }

    fn restore_selection(selection: &web_sys::Selection, range: &web_sys::Range) {
        let _ = selection.remove_all_ranges();
        let _ = selection.add_range(range);
    }

    // Place the cursor at a specific position relative to a node
    fn place_cursor_at_node(node: &web_sys::Node, offset: u32) -> Option<()> {
        let window = web_sys::window()?;
        let selection = window.get_selection().ok()??;
        let document = window.document()?;

        let range = document.create_range().ok()?;
        let _ = range.set_start(node, offset);
        let _ = range.set_end(node, offset);

        let _ = selection.remove_all_ranges();
        let _ = selection.add_range(&range);

        Some(())
    }

    // Get the current node and offset where the cursor is
    fn get_cursor_node_and_offset() -> Option<(Node, u32)> {
        let window = web_sys::window()?;
        let selection = window.get_selection().ok()??;
        if selection.range_count() == 0 {
            return None;
        }
        let range = selection.get_range_at(0).ok()?;
        let node = range.start_container().ok()?;
        let offset = range.start_offset().ok()?;
        Some((node, offset))
    }

    let oninput = {
        let editable_ref = editable_ref.clone();
        let raw_content = raw_content.clone();
        let input_value = input_value.clone();
        let start_token = start_token.clone();
        let end_token = end_token.clone();
        let brandguide = brandguide.clone();
        let props_onupdate = props.onupdate.clone();
        let is_composing = is_composing.clone();

        Callback::from(move |e: InputEvent| {
            // Skip processing during IME composition
            if *is_composing {
                return;
            }

            if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                // Save cursor position by character index (more reliable than DOM position)
                let cursor_pos = get_cursor_position(&editable);

                // Extract the current text including hidden tokens
                let current_text = extract_raw_text(&editable);
                raw_content.set(current_text.clone());

                // Render the HTML with highlighted mentions
                let rendered_content = render_content(
                    &current_text,
                    &start_token,
                    &end_token,
                    brandguide.tag_input_tag.to_string(),
                );

                // Update the displayed content
                editable.set_inner_html(&rendered_content);

                // Restore cursor position
                if let Some(pos) = cursor_pos {
                    let _ = set_cursor_position(&editable, pos);
                }

                // Update state and notify parent
                input_value.set(current_text.clone());
                if let Some(ref onupdate) = props_onupdate {
                    onupdate.emit(current_text);
                }
            }
        })
    };

    let onkeydown = {
        let editable_ref = editable_ref.clone();
        let cursor_position = cursor_position.clone();

        Callback::from(move |e: KeyboardEvent| {
            if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                // Save cursor position info
                if let Some((node, offset)) = get_cursor_node_and_offset() {
                    // Find the index of this node in the parent's child list
                    if let Some(parent) = node.parent_node() {
                        let children = parent.child_nodes();
                        for i in 0..children.length() {
                            if let Some(child) = children.get(i) {
                                if child == node {
                                    cursor_position.set((i as usize, offset as usize));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        })
    };

    // Add click handler to maintain focus
    let onclick = {
        let editable_ref = editable_ref.clone();

        Callback::from(move |_: MouseEvent| {
            if let Some(editable) = editable_ref.cast::<HtmlElement>() {
                let _ = editable.focus();
            }
        })
    };

    html! {
        <div
            ref={container_ref}
            key={props.id.clone().unwrap_or("mention".to_string())}
            tabindex="0"
            class={classes!(&brandguide.tag_input_container)}
            onclick={onclick.clone()}
        >
            <div
                ref={editable_ref}
                contenteditable="true"
                class={classes!(&brandguide.tag_input_tags_container)}
                oninput={oninput}
                onkeydown={onkeydown}
                onclick={onclick}
                placeholder={props.placeholder.clone()}
            >
            </div>
        </div>
    }
}
