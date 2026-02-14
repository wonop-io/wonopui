//! Kanban component for WonopUI.
//!
//! A drag-and-drop Kanban board with columns and cards.

use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::DragEvent;
use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the Kanban component
pub mod classes {
    pub const CONTAINER: &str = "flex overflow-x-auto gap-6 p-6";
    pub const COLUMN: &str = "flex flex-col min-w-[300px] border rounded-md bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-700 shadow-sm";
    pub const COLUMN_HEADER: &str = "p-4 font-semibold border-b border-zinc-200 dark:border-zinc-700";
    pub const COLUMN_CONTENT: &str = "p-3 flex-1 flex flex-col gap-3 min-h-[100px] overflow-y-auto";
    pub const COLUMN_OVER: &str = "border-2 border-blue-500 dark:border-blue-400";
    pub const CARD: &str = "border border-zinc-200 dark:border-zinc-700 rounded-md bg-white dark:bg-zinc-800 p-4 cursor-grab active:cursor-grabbing shadow-sm";
    pub const CARD_TITLE: &str = "font-medium pb-2";
    pub const CARD_CONTENT: &str = "text-sm text-zinc-600 dark:text-zinc-400";
    pub const CARD_DRAGGING: &str = "opacity-50 shadow-md";
    pub const CARD_DRAG_TARGET: &str = "border-t-2 border-blue-500 dark:border-blue-400";
    pub const GHOST_CARD: &str = "border-2 border-dashed border-blue-400 dark:border-blue-500 rounded-md bg-blue-50 dark:bg-blue-900/20 p-4 opacity-60 transition-all duration-200 ease-in-out";
    pub const DRAG_HANDLE: &str = "cursor-move text-zinc-400 hover:text-zinc-600 dark:text-zinc-500 dark:hover:text-zinc-300 transition-colors";
}

/// Position relative to a card
#[derive(Clone, Debug, PartialEq)]
pub enum DropPosition {
    Above,
    Below,
}

/// Drag state for sharing across components
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DragState {
    pub dragging_card_id: Option<String>,
    pub dragging_card_title: Option<String>,
    pub dragging_card_content: Option<String>,
    pub source_column_id: Option<String>,
    pub hover_column_id: Option<String>,
    pub hover_card_id: Option<String>,
    pub drop_position: Option<DropPosition>,
}

/// Type alias for drag start callback
pub type DragStartCallback = Callback<(String, Option<String>, Option<String>, Option<String>)>;
/// Type alias for hover callback
pub type HoverCallback = Callback<(Option<String>, Option<String>, Option<DropPosition>)>;

// Main Kanban component
#[derive(Properties, PartialEq)]
pub struct KanbanProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub ondragstart: Option<DragStartCallback>,
    #[prop_or_default]
    pub ondragend: Option<Callback<()>>,
}

#[function_component(Kanban)]
pub fn kanban(props: &KanbanProps) -> Html {
    let drag_state = use_state(DragState::default);
    
    // Cleanup effect
    {
        let drag_state = drag_state.clone();
        use_effect_with((), move |_| {
            move || {
                drag_state.set(DragState::default());
            }
        });
    }
    
    let ondragstart: DragStartCallback = {
        let drag_state = drag_state.clone();
        let user_callback = props.ondragstart.clone();
        Callback::from(move |(id, title, content, column_id): (String, Option<String>, Option<String>, Option<String>)| {
            let mut state = (*drag_state).clone();
            state.dragging_card_id = Some(id.clone());
            state.dragging_card_title = title.clone();
            state.dragging_card_content = content.clone();
            state.source_column_id = column_id.clone();
            drag_state.set(state);
            
            if let Some(callback) = &user_callback {
                callback.emit((id, title, content, column_id));
            }
        })
    };
    
    let ondragend: Callback<()> = {
        let drag_state = drag_state.clone();
        let user_callback = props.ondragend.clone();
        Callback::from(move |_| {
            drag_state.set(DragState::default());
            
            if let Some(callback) = &user_callback {
                callback.emit(());
            }
        })
    };
    
    let onhover: HoverCallback = {
        let drag_state = drag_state.clone();
        Callback::from(move |(column_id, card_id, position): (Option<String>, Option<String>, Option<DropPosition>)| {
            let mut state = (*drag_state).clone();
            
            if state.hover_column_id != column_id || 
               state.hover_card_id != card_id || 
               state.drop_position != position {
                state.hover_column_id = column_id;
                state.hover_card_id = card_id;
                state.drop_position = position;
                drag_state.set(state);
            }
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <ContextProvider<Rc<DragState>> context={Rc::new((*drag_state).clone())}>
            <ContextProvider<DragStartCallback> context={ondragstart}>
                <ContextProvider<Callback<()>> context={ondragend}>
                    <ContextProvider<HoverCallback> context={onhover}>
                        <div class={container_class}>
                            { for props.children.iter() }
                        </div>
                    </ContextProvider<HoverCallback>>
                </ContextProvider<Callback<()>>>
            </ContextProvider<DragStartCallback>>
        </ContextProvider<Rc<DragState>>>
    }
}

// Kanban Column component
#[derive(Properties, PartialEq)]
pub struct KanbanColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub header_class: Classes,
    #[prop_or_default]
    pub body_class: Classes,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub ondrop: Option<Callback<(String, String, Option<String>)>>,
}

#[function_component(KanbanColumn)]
pub fn kanban_column(props: &KanbanColumnProps) -> Html {
    let drag_state = use_context::<Rc<DragState>>().unwrap_or_default();
    let onhover = use_context::<HoverCallback>();
    let global_ondragend = use_context::<Callback<()>>();

    let is_over = use_state(|| false);
    let drag_counter = use_state(|| 0_u32);
    let column_id = props.id.clone();

    let ondragenter = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let column_id = column_id.clone();
        let onhover = onhover.clone();
        let drag_state = drag_state.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            drag_counter.set(*drag_counter + 1);
            is_over.set(true);
            
            let is_column_target = e.target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .map(|elem| elem.get_attribute("data-column-id").is_some())
                .unwrap_or(false);
            
            if is_column_target {
                if let Some(ref hover_cb) = onhover {
                    if drag_state.hover_column_id != Some(column_id.to_string()) {
                        hover_cb.emit((Some(column_id.to_string()), None, None));
                    }
                }
            }
        })
    };

    let ondragleave = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let onhover = onhover.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            let new_count = (*drag_counter).saturating_sub(1);
            drag_counter.set(new_count);

            if new_count == 0 {
                is_over.set(false);
                if let Some(ref hover_cb) = onhover {
                    hover_cb.emit((None, None, None));
                }
            }
        })
    };

    let ondragover = {
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
        })
    };

    let ondrop = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let column_id = column_id.clone();
        let user_callback = props.ondrop.clone();
        let onhover = onhover.clone();
        let global_ondragend = global_ondragend.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            is_over.set(false);
            drag_counter.set(0);
            
            if let Some(ref hover_cb) = onhover {
                hover_cb.emit((None, None, None));
            }

            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(card_id) = data_transfer.get_data("text/plain") {
                    let target_card_id = e.target()
                        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        .and_then(|elem| {
                            let mut current = Some(elem);
                            while let Some(el) = current {
                                if el.get_attribute("data-card-id").is_some() {
                                    return el.get_attribute("data-card-id");
                                }
                                current = el.parent_element();
                            }
                            None
                        })
                        .filter(|id| id != &card_id);

                    if let Some(callback) = &user_callback {
                        callback.emit((card_id, column_id.to_string(), target_card_id));
                    }
                    
                    if let Some(ref global_cb) = global_ondragend {
                        global_cb.emit(());
                    }
                }
            }
        })
    };

    let column_class = merge_classes(&[
        classes::COLUMN,
        if *is_over { classes::COLUMN_OVER } else { "" },
        &props.class.to_string(),
    ]);

    let header_class = merge_classes(&[classes::COLUMN_HEADER, &props.header_class.to_string()]);
    let content_class = merge_classes(&[classes::COLUMN_CONTENT, &props.body_class.to_string()]);

    html! {
        <div
            class={column_class}
            ondragenter={ondragenter}
            ondragleave={ondragleave}
            ondragover={ondragover}
            ondrop={ondrop}
            data-column-id={props.id.clone()}
        >
            <div class={header_class}>
                <div class="font-medium text-lg">{ props.title.clone() }</div>
            </div>
            <div class={content_class}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

// Kanban Card component
#[derive(Clone, Properties, PartialEq)]
pub struct KanbanCardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    #[prop_or_default]
    pub column_id: Option<AttrValue>,
    #[prop_or_default]
    pub order: Option<i32>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(KanbanCard)]
pub fn kanban_card(props: &KanbanCardProps) -> Html {
    let global_ondragstart = use_context::<DragStartCallback>();
    let global_ondragend = use_context::<Callback<()>>();
    let onhover = use_context::<HoverCallback>();
    let card_ref = use_node_ref();

    let is_dragging = use_state(|| false);
    let is_drag_over = use_state(|| false);
    let drag_counter = use_state(|| 0_u32);
    let card_id = props.id.clone();

    let ondragstart = {
        let is_dragging = is_dragging.clone();
        let card_id = card_id.clone();
        let column_id = props.column_id.clone();
        let global_ondragstart = global_ondragstart.clone();
        let title = props.title.clone();
        let description = props.description.clone();

        Callback::from(move |e: DragEvent| {
            if let Some(data_transfer) = e.data_transfer() {
                let _ = data_transfer.set_data("text/plain", &card_id);
                if let Some(col_id) = &column_id {
                    let _ = data_transfer.set_data("source-column", col_id);
                }
            }

            is_dragging.set(true);
            
            if let Some(ref global_cb) = global_ondragstart {
                global_cb.emit((
                    card_id.to_string(), 
                    title.clone().map(|t| t.to_string()), 
                    description.clone().map(|d| d.to_string()),
                    column_id.clone().map(|c| c.to_string())
                ));
            }
        })
    };

    let ondragend = {
        let is_dragging = is_dragging.clone();
        let is_drag_over = is_drag_over.clone();
        let drag_counter = drag_counter.clone();
        let global_ondragend = global_ondragend.clone();

        Callback::from(move |_: DragEvent| {
            is_dragging.set(false);
            is_drag_over.set(false);
            drag_counter.set(0);
            
            if let Some(ref global_cb) = global_ondragend {
                global_cb.emit(());
            }
        })
    };

    let ondragenter = {
        let is_drag_over = is_drag_over.clone();
        let drag_counter = drag_counter.clone();
        let card_id = card_id.clone();
        let column_id = props.column_id.clone();
        let onhover = onhover.clone();
        let card_ref = card_ref.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(dragged_id) = data_transfer.get_data("text/plain") {
                    if dragged_id == card_id {
                        return;
                    }
                }
            }

            drag_counter.set(*drag_counter + 1);
            is_drag_over.set(true);
            
            let position = if let Some(card_elem) = card_ref.cast::<web_sys::HtmlElement>() {
                let rect = card_elem.get_bounding_client_rect();
                let mouse_y = e.client_y() as f64;
                let card_middle = rect.top() + (rect.height() / 2.0);
                
                if mouse_y < card_middle {
                    Some(DropPosition::Above)
                } else {
                    Some(DropPosition::Below)
                }
            } else {
                Some(DropPosition::Above)
            };
            
            if let Some(ref hover_cb) = onhover {
                hover_cb.emit((column_id.clone().map(|c| c.to_string()), Some(card_id.to_string()), position));
            }
        })
    };

    let ondragleave = {
        let is_drag_over = is_drag_over.clone();
        let drag_counter = drag_counter.clone();
        let onhover = onhover.clone();
        let column_id = props.column_id.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            let new_count = (*drag_counter).saturating_sub(1);
            drag_counter.set(new_count);

            if new_count == 0 {
                is_drag_over.set(false);
                if let Some(ref hover_cb) = onhover {
                    hover_cb.emit((column_id.clone().map(|c| c.to_string()), None, None));
                }
            }
        })
    };

    let ondragover = {
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
        })
    };

    let card_class = merge_classes(&[
        classes::CARD,
        if *is_dragging { classes::CARD_DRAGGING } else { "" },
        if *is_drag_over { classes::CARD_DRAG_TARGET } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div
            ref={card_ref}
            class={card_class}
            draggable="true"
            ondragstart={ondragstart}
            ondragend={ondragend}
            ondragenter={ondragenter}
            ondragleave={ondragleave}
            ondragover={ondragover}
            onclick={props.onclick.clone()}
            data-card-id={props.id.clone()}
            data-column-id={props.column_id.clone()}
        >
            <span class={classes::DRAG_HANDLE}>{"≡"}</span>
            if let Some(title) = &props.title {
                <div class={classes::CARD_TITLE}>{ title.clone() }</div>
            }
            <div class={classes::CARD_CONTENT}>
                { 
                    if let Some(description) = &props.description {
                        html! { { description.clone() } }
                    } else {
                        html! { { for props.children.iter() } }
                    }
                }
            </div>
        </div>
    }
}
