use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::DragEvent;
use yew::prelude::*;

// Position relative to a card
#[derive(Clone, Debug, PartialEq)]
pub enum DropPosition {
    Above,
    Below,
}

// Drag state for sharing across components
#[derive(Clone, Debug, PartialEq)]
pub struct DragState {
    pub dragging_card_id: Option<String>,
    pub dragging_card_title: Option<String>,
    pub dragging_card_content: Option<String>,
    pub hover_column_id: Option<String>,
    pub hover_card_id: Option<String>,
    pub drop_position: Option<DropPosition>,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            dragging_card_id: None,
            dragging_card_title: None,
            dragging_card_content: None,
            hover_column_id: None,
            hover_card_id: None,
            drop_position: None,
        }
    }
}

// Main Kanban component
#[derive(Properties, PartialEq)]
pub struct KanbanProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub allow_multiple_column_drops: bool,
    #[prop_or_default]
    pub ondragstart: Option<Callback<(String, Option<String>, Option<String>)>>,
    #[prop_or_default]
    pub ondragend: Option<Callback<()>>,
}

#[function_component(Kanban)]
pub fn kanban(props: &KanbanProps) -> Html {
    // Use hardcoded styles since this component may not be in the brandguide yet
    let kanban_container = "flex overflow-x-auto gap-6 p-6";
    
    let drag_state = use_state(DragState::default);
    
    let ondragstart = {
        let drag_state = drag_state.clone();
        let user_callback = props.ondragstart.clone();
        Callback::from(move |(id, title, content): (String, Option<String>, Option<String>)| {
            let mut state = (*drag_state).clone();
            state.dragging_card_id = Some(id.clone());
            state.dragging_card_title = title.clone();
            state.dragging_card_content = content.clone();
            drag_state.set(state);
            
            if let Some(callback) = &user_callback {
                callback.emit((id, title, content));
            }
        })
    };
    
    let ondragend = {
        let drag_state = drag_state.clone();
        let user_callback = props.ondragend.clone();
        Callback::from(move |_| {
            drag_state.set(DragState::default());
            if let Some(callback) = &user_callback {
                callback.emit(());
            }
        })
    };
    
    let onhover = {
        let drag_state = drag_state.clone();
        Callback::from(move |(column_id, card_id, position): (Option<String>, Option<String>, Option<DropPosition>)| {
            let mut state = (*drag_state).clone();
            
            // Only update if actually changed to prevent unnecessary re-renders
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

    html! {
        <ContextProvider<Rc<DragState>> context={Rc::new((*drag_state).clone())}>
            <ContextProvider<Callback<(String, Option<String>, Option<String>)>> context={ondragstart}>
                <ContextProvider<Callback<()>> context={ondragend}>
                    <ContextProvider<Callback<(Option<String>, Option<String>, Option<DropPosition>)>> context={onhover}>
                        <div class={classes!(kanban_container, props.class.clone())}>
                            { for props.children.iter() }
                        </div>
                    </ContextProvider<Callback<(Option<String>, Option<String>, Option<DropPosition>)>>>
                </ContextProvider<Callback<()>>>
            </ContextProvider<Callback<(String, Option<String>, Option<String>)>>>
        </ContextProvider<Rc<DragState>>>
    }
}

// Kanban Column component
#[derive(Properties, PartialEq)]
pub struct KanbanColumnProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<KanbanCard>,
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
    pub ondragenter: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondragleave: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondragover: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondrop: Option<Callback<(String, String, Option<String>)>>,
}

#[function_component(KanbanColumn)]
pub fn kanban_column(props: &KanbanColumnProps) -> Html {
    let drag_state = use_context::<Rc<DragState>>().unwrap_or_default();
    let onhover = use_context::<Callback<(Option<String>, Option<String>, Option<DropPosition>)>>();
    // Use hardcoded styles since these components may not be in the brandguide yet
    let kanban_column = "flex flex-col min-w-[300px] border rounded-md bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-700 shadow-sm";
    let kanban_column_header = "p-4 font-semibold border-b border-zinc-200 dark:border-zinc-700";
    let kanban_column_content = "p-3 flex-1 flex flex-col gap-3 min-h-[100px] overflow-y-auto";
    let kanban_column_over = "border-2 border-blue-500 dark:border-blue-400";

    let is_over = use_state(|| false);
    let column_id = props.id.clone();

    // Global drag state to properly track highlight state
    let drag_counter = use_state(|| 0_u32);

    let ondragenter = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let user_callback = props.ondragenter.clone();
        let column_id = column_id.clone();
        let onhover = onhover.clone();
        let drag_state = drag_state.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            // Increment counter for nested elements
            drag_counter.set(*drag_counter + 1);
            is_over.set(true);
            
            // Check if the event target is the column itself (not a card)
            let is_column_target = e.target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .map(|elem| elem.get_attribute("data-column-id").is_some())
                .unwrap_or(false);
            
            // Only update hover state if we're entering the column directly
            // This preserves card positions when moving between cards
            if is_column_target {
                if let Some(ref hover_cb) = onhover {
                    // Check if we're already in this column
                    if drag_state.hover_column_id != Some(column_id.to_string()) {
                        // Entering a new column, default to top
                        hover_cb.emit((Some(column_id.to_string()), None, None));
                    }
                    // If already in this column, don't update (preserve position)
                }
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragleave = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let user_callback = props.ondragleave.clone();
        let onhover = onhover.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            // Decrement counter and only remove highlight when we've left all nested elements
            let new_count = (*drag_counter).saturating_sub(1);
            drag_counter.set(new_count);

            if new_count == 0 {
                is_over.set(false);
                // Clear hover state when leaving column
                if let Some(ref hover_cb) = onhover {
                    hover_cb.emit((None, None, None));
                }
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragover = {
        let user_callback = props.ondragover.clone();
        let column_id = column_id.clone();
        let onhover = onhover.clone();
        let drag_state = drag_state.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            // Check if we're directly over the column (not a card)
            let is_column_target = e.target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .map(|elem| {
                    elem.get_attribute("data-column-id").is_some() && 
                    elem.get_attribute("data-card-id").is_none() &&
                    elem.get_attribute("data-ghost-card").is_none()
                })
                .unwrap_or(false);
            
            // If dragging over column background, maintain column hover but clear card
            if is_column_target && drag_state.hover_column_id == Some(column_id.to_string()) {
                if let Some(ref hover_cb) = onhover {
                    // Keep the column, but indicate no specific card
                    // This maintains the existing ghost position
                }
            }
            
            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondrop = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let column_id = column_id.clone();
        let user_callback = props.ondrop.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_over.set(false);
            drag_counter.set(0);

            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(card_id) = data_transfer.get_data("text/plain") {
                    // Try to determine the target card from the drop event target
                    let target_card_id = e.target()
                        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                        .and_then(|elem| {
                            // Find the closest card element
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
                }
            }
        })
    };

    let over_class = if *is_over { kanban_column_over } else { "" };

    // Sort children by order if provided
    let mut sorted_children: Vec<_> = props.children.iter().collect();
    sorted_children.sort_by_key(|child| child.props.order);

    html! {
        <div
            class={classes!(kanban_column, over_class, props.class.clone())}
            ondragenter={ondragenter}
            ondragleave={ondragleave}
            ondragover={ondragover}
            ondrop={ondrop}
            data-column-id={props.id.clone()}
        >
            <div class={classes!(kanban_column_header, props.header_class.clone())}>
                <div class="font-medium text-lg">{props.title.clone()}</div>
            </div>
            <div class={classes!(kanban_column_content, props.body_class.clone())}>
                {
                    {
                        let mut items = vec![];
                        let is_column_hovered = drag_state.hover_column_id == Some(column_id.to_string());
                        let has_dragging_card = drag_state.dragging_card_id.is_some();
                        
                        // Create ghost card element if we're dragging
                        let ghost_card = if has_dragging_card && is_column_hovered {
                            Some(html! {
                            <div class="border-2 border-dashed border-blue-400 dark:border-blue-500 rounded-md bg-blue-50 dark:bg-blue-900/20 p-4 opacity-60 transition-all duration-200 ease-in-out"
                                 data-ghost-card="true">
                                if let Some(ref title) = drag_state.dragging_card_title {
                                    <div class="font-medium pb-2">{title.clone()}</div>
                                }
                                if let Some(ref content) = drag_state.dragging_card_content {
                                    <div class="text-sm text-zinc-600 dark:text-zinc-400">{content.clone()}</div>
                                }
                            </div>
                            })
                        } else {
                            None
                        };
                        
                        // If no cards and hovering, show ghost at top
                        if sorted_children.is_empty() && ghost_card.is_some() {
                            items.push(ghost_card.unwrap());
                        } else {
                            // Process cards with ghost positioning
                            for (index, child) in sorted_children.iter().enumerate() {
                                let card_id = child.props.id.to_string();
                                let is_dragging_this = drag_state.dragging_card_id.as_ref() == Some(&card_id);
                                
                                // Determine if we should show ghost before this card
                                if let Some(ref ghost) = ghost_card {
                                    if !is_dragging_this {  // Don't show ghost near the card being dragged
                                        if let Some(ref hover_card_id) = drag_state.hover_card_id {
                                            if hover_card_id == &card_id {
                                                // Show ghost based on drop position
                                                if drag_state.drop_position == Some(DropPosition::Above) {
                                                    items.push(ghost.clone());
                                                }
                                            }
                                        } else if index == 0 && drag_state.hover_card_id.is_none() {
                                            // Default to top when hovering column but no specific card
                                            items.push(ghost.clone());
                                        }
                                    }
                                }
                                
                                // Add the actual card (unless it's being dragged)
                                if !is_dragging_this || !is_column_hovered {
                                    let mut props = (*child.props).clone();
                                    props.column_id = Some(column_id.clone());
                                    items.push(html! {
                                        <KanbanCard ..props>
                                            { for child.props.children.iter() }
                                        </KanbanCard>
                                    });
                                }
                                
                                // Check if we should show ghost after this card
                                if let Some(ref ghost) = ghost_card {
                                    if !is_dragging_this {
                                        if let Some(ref hover_card_id) = drag_state.hover_card_id {
                                            if hover_card_id == &card_id && drag_state.drop_position == Some(DropPosition::Below) {
                                                items.push(ghost.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        html! { <>{for items}</> }
                    }
                }
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
    pub column_id: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub description: Option<AttrValue>,  // Add explicit description for ghost preview
    #[prop_or_default]
    pub ondragstart: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondragend: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondragover: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondragenter: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub ondragleave: Option<Callback<DragEvent>>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub order: Option<usize>,
}

#[function_component(KanbanCard)]
pub fn kanban_card(props: &KanbanCardProps) -> Html {
    let global_ondragstart = use_context::<Callback<(String, Option<String>, Option<String>)>>();
    let global_ondragend = use_context::<Callback<()>>();
    let onhover = use_context::<Callback<(Option<String>, Option<String>, Option<DropPosition>)>>();
    let card_ref = use_node_ref();
    // Use hardcoded styles since these components may not be in the brandguide yet
    let kanban_card = "border border-zinc-200 dark:border-zinc-700 rounded-md bg-white dark:bg-zinc-800 p-4 cursor-grab active:cursor-grabbing shadow-sm";
    let kanban_card_title = "font-medium pb-2";
    let kanban_card_content = "text-sm text-zinc-600 dark:text-zinc-400";
    let kanban_card_dragging = "opacity-50 shadow-md";
    let kanban_card_drag_target = "border-t-2 border-blue-500 dark:border-blue-400";
    let kanban_drag_handle = "cursor-move text-zinc-400 hover:text-zinc-600 dark:text-zinc-500 dark:hover:text-zinc-300 transition-colors";

    let is_dragging = use_state(|| false);
    let is_drag_over = use_state(|| false);
    let drag_counter = use_state(|| 0_u32);
    let card_id = props.id.clone();

    let ondragstart = {
        let is_dragging = is_dragging.clone();
        let card_id = card_id.clone();
        let column_id = props.column_id.clone();
        let order = props.order.clone();
        let user_callback = props.ondragstart.clone();
        let global_ondragstart = global_ondragstart.clone();
        let title = props.title.clone();
        let description = props.description.clone();

        Callback::from(move |e: DragEvent| {
            if let Some(data_transfer) = e.data_transfer() {
                // Set the card ID as the drag data
                let _ = data_transfer.set_data("text/plain", &card_id);

                // Store original column ID if needed for reordering logic
                if let Some(col_id) = &column_id {
                    let _ = data_transfer.set_data("source-column", col_id);
                }

                // Store the order if available for proper positioning
                if let Some(order_val) = order {
                    let _ = data_transfer.set_data("order", &order_val.to_string());
                }
            }

            is_dragging.set(true);
            
            // Notify global drag state with card info
            if let Some(ref global_cb) = global_ondragstart {
                global_cb.emit((
                    card_id.to_string(), 
                    title.clone().map(|t| t.to_string()), 
                    description.clone().map(|d| d.to_string())
                ));
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragend = {
        let is_dragging = is_dragging.clone();
        let user_callback = props.ondragend.clone();
        let global_ondragend = global_ondragend.clone();

        Callback::from(move |e: DragEvent| {
            is_dragging.set(false);
            
            // Notify global drag end
            if let Some(ref global_cb) = global_ondragend {
                global_cb.emit(());
            }
            
            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragenter = {
        let is_drag_over = is_drag_over.clone();
        let drag_counter = drag_counter.clone();
        let user_callback = props.ondragenter.clone();
        let card_id = card_id.clone();
        let column_id = props.column_id.clone();
        let onhover = onhover.clone();
        let card_ref = card_ref.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            // Check if not dragging self
            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(dragged_id) = data_transfer.get_data("text/plain") {
                    if dragged_id == card_id {
                        return;
                    }
                }
            }

            drag_counter.set(*drag_counter + 1);
            is_drag_over.set(true);
            
            // Don't update if we're over the ghost card
            if let Some(target) = e.target() {
                if let Ok(elem) = target.dyn_into::<web_sys::Element>() {
                    if elem.get_attribute("data-ghost-card").is_some() {
                        return;  // Keep existing position when over ghost
                    }
                }
            }
            
            // Calculate position based on mouse Y coordinate
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
                Some(DropPosition::Above)  // Default to above
            };
            
            // Update hover state with position
            if let Some(ref hover_cb) = onhover {
                hover_cb.emit((column_id.clone().map(|c| c.to_string()), Some(card_id.to_string()), position));
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragleave = {
        let is_drag_over = is_drag_over.clone();
        let drag_counter = drag_counter.clone();
        let user_callback = props.ondragleave.clone();
        let onhover = onhover.clone();
        let column_id = props.column_id.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            let new_count = (*drag_counter).saturating_sub(1);
            drag_counter.set(new_count);

            if new_count == 0 {
                is_drag_over.set(false);
                // Keep column but clear card hover when leaving
                if let Some(ref hover_cb) = onhover {
                    hover_cb.emit((column_id.clone().map(|c| c.to_string()), None, None));
                }
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragover = {
        let user_callback = props.ondragover.clone();
        let card_ref = card_ref.clone();
        let card_id = card_id.clone();
        let column_id = props.column_id.clone();
        let onhover = onhover.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            // Don't update if we're over the ghost card
            if let Some(target) = e.target() {
                if let Ok(elem) = target.dyn_into::<web_sys::Element>() {
                    if elem.get_attribute("data-ghost-card").is_some() {
                        return;  // Keep existing position when over ghost
                    }
                }
            }
            
            // Update position while dragging over
            if let Some(card_elem) = card_ref.cast::<web_sys::HtmlElement>() {
                let rect = card_elem.get_bounding_client_rect();
                let mouse_y = e.client_y() as f64;
                let card_middle = rect.top() + (rect.height() / 2.0);
                
                let position = if mouse_y < card_middle {
                    Some(DropPosition::Above)
                } else {
                    Some(DropPosition::Below)
                };
                
                // Only emit if position changed to reduce re-renders
                if let Some(ref hover_cb) = onhover {
                    hover_cb.emit((column_id.clone().map(|c| c.to_string()), Some(card_id.to_string()), position));
                }
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let drag_class = if *is_dragging {
        kanban_card_dragging
    } else {
        ""
    };

    let drag_over_class = if *is_drag_over {
        kanban_card_drag_target
    } else {
        ""
    };

    html! {
        <div
            ref={card_ref}
            class={classes!(kanban_card, drag_class, drag_over_class, props.class.clone())}
            draggable="true"
            ondragstart={ondragstart}
            ondragend={ondragend}
            ondragenter={ondragenter}
            ondragleave={ondragleave}
            ondragover={ondragover}
            onclick={props.onclick.clone()}
            data-card-id={props.id.clone()}
            data-column-id={props.column_id.clone()}
            data-order={props.order.map(|o| o.to_string())}
        >
            <span class={kanban_drag_handle}>
                {"≡"}
            </span>
            if let Some(title) = &props.title {
                <div class={kanban_card_title}>
                    {title.clone()}
                </div>
            }
            <div class={kanban_card_content}>
                { 
                    if let Some(description) = &props.description {
                        html! { {description.clone()} }
                    } else {
                        html! { { for props.children.iter() } }
                    }
                }
            </div>
        </div>
    }
}
