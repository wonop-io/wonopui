use std::collections::HashMap;
use web_sys::DragEvent;
use yew::prelude::*;

// Main Kanban component
#[derive(Properties, PartialEq)]
pub struct KanbanProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub allow_multiple_column_drops: bool,
}

#[function_component(Kanban)]
pub fn kanban(props: &KanbanProps) -> Html {
    // Use hardcoded styles since this component may not be in the brandguide yet
    let kanban_container = "flex overflow-x-auto gap-6 p-6";

    html! {
        <div class={classes!(kanban_container, props.class.clone())}>
            { for props.children.iter() }
        </div>
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
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            // Increment counter for nested elements
            drag_counter.set(*drag_counter + 1);
            is_over.set(true);

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragleave = {
        let is_over = is_over.clone();
        let drag_counter = drag_counter.clone();
        let user_callback = props.ondragleave.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            // Decrement counter and only remove highlight when we've left all nested elements
            let new_count = (*drag_counter).saturating_sub(1);
            drag_counter.set(new_count);

            if new_count == 0 {
                is_over.set(false);
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragover = {
        let user_callback = props.ondragover.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
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
                    // Get position data (target card id) if available
                    let target_card_id = if let Ok(target) = data_transfer.get_data("target-card") {
                        if !target.is_empty() && target != card_id {
                            Some(target)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

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
                { for sorted_children.into_iter().map(|child| {
                    let mut props = (*child.props).clone();
                    props.column_id = Some(column_id.clone());
                    html! {
                        <KanbanCard ..props>
                            { for child.props.children.iter() }
                        </KanbanCard>
                    }
                })}
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

                // Clear any target card from previous operations
                let _ = data_transfer.set_data("target-card", "");
            }

            is_dragging.set(true);

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragend = {
        let is_dragging = is_dragging.clone();
        let user_callback = props.ondragend.clone();

        Callback::from(move |e: DragEvent| {
            is_dragging.set(false);
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

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragleave = {
        let is_drag_over = is_drag_over.clone();
        let drag_counter = drag_counter.clone();
        let user_callback = props.ondragleave.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            let new_count = (*drag_counter).saturating_sub(1);
            drag_counter.set(new_count);

            if new_count == 0 {
                is_drag_over.set(false);
            }

            if let Some(callback) = &user_callback {
                callback.emit(e);
            }
        })
    };

    let ondragover = {
        let card_id = card_id.clone();
        let user_callback = props.ondragover.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            // Don't allow dropping on self
            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(dragged_id) = data_transfer.get_data("text/plain") {
                    if dragged_id != card_id {
                        // Mark which card we're hovering over for positional dropping
                        let _ = data_transfer.set_data("target-card", &card_id);
                    }
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
                { for props.children.iter() }
            </div>
        </div>
    }
}
