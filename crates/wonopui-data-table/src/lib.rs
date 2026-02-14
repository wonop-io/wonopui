//! Data Table component for WonopUI.
//!
//! A data table component with sorting, filtering, and pagination.
//! Note: This is a basic implementation. A full-featured data table
//! would include column resizing, row selection, and more.

use yew::prelude::*;
pub use wonopui_core::merge_classes;

/// CSS classes for the DataTable component
pub mod classes {
    pub const CONTAINER: &str = "w-full overflow-auto";
    pub const TABLE: &str = "w-full caption-bottom text-sm";
    pub const HEADER: &str = "border-b";
    pub const HEADER_ROW: &str = "";
    pub const HEADER_CELL: &str = "h-12 px-4 text-left align-middle font-medium text-muted-foreground";
    pub const HEADER_CELL_SORTABLE: &str = "h-12 px-4 text-left align-middle font-medium text-muted-foreground cursor-pointer hover:bg-accent";
    pub const BODY: &str = "";
    pub const ROW: &str = "border-b transition-colors hover:bg-muted/50";
    pub const ROW_SELECTED: &str = "border-b transition-colors bg-muted";
    pub const CELL: &str = "p-4 align-middle";
    pub const FOOTER: &str = "border-t bg-muted/50 font-medium";
    pub const PAGINATION: &str = "flex items-center justify-between p-4";
}

/// Column definition for the data table
#[derive(Clone, PartialEq)]
pub struct Column<T: Clone + PartialEq> {
    pub key: String,
    pub header: String,
    pub sortable: bool,
    pub render: Callback<T, Html>,
}

#[derive(Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Properties, PartialEq)]
pub struct DataTableProps<T: Clone + PartialEq + 'static> {
    pub columns: Vec<Column<T>>,
    pub data: Vec<T>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_row_click: Option<Callback<T>>,
    #[prop_or_default]
    pub selected_row: Option<usize>,
    #[prop_or_default]
    pub page_size: Option<usize>,
}

#[function_component(DataTable)]
pub fn data_table<T: Clone + PartialEq + 'static>(props: &DataTableProps<T>) -> Html {
    let current_page = use_state(|| 0usize);
    let sort_column = use_state(|| Option::<String>::None);
    let sort_direction = use_state(|| SortDirection::Ascending);

    let page_size = props.page_size.unwrap_or(10);
    let total_pages = (props.data.len() + page_size - 1) / page_size;
    
    let start_idx = *current_page * page_size;
    let end_idx = std::cmp::min(start_idx + page_size, props.data.len());
    let page_data = &props.data[start_idx..end_idx];

    let on_prev_page = {
        let current_page = current_page.clone();
        Callback::from(move |_: MouseEvent| {
            if *current_page > 0 {
                current_page.set(*current_page - 1);
            }
        })
    };

    let on_next_page = {
        let current_page = current_page.clone();
        Callback::from(move |_: MouseEvent| {
            if *current_page < total_pages - 1 {
                current_page.set(*current_page + 1);
            }
        })
    };

    let container_class = merge_classes(&[classes::CONTAINER, &props.class.to_string()]);

    html! {
        <div class={container_class}>
            <table class={classes::TABLE}>
                <thead class={classes::HEADER}>
                    <tr class={classes::HEADER_ROW}>
                        { for props.columns.iter().map(|col| {
                            let header_class = if col.sortable {
                                classes::HEADER_CELL_SORTABLE
                            } else {
                                classes::HEADER_CELL
                            };
                            html! {
                                <th class={header_class}>
                                    { &col.header }
                                </th>
                            }
                        }) }
                    </tr>
                </thead>
                <tbody class={classes::BODY}>
                    { for page_data.iter().enumerate().map(|(idx, row)| {
                        let row_idx = start_idx + idx;
                        let is_selected = props.selected_row == Some(row_idx);
                        let row_class = if is_selected {
                            classes::ROW_SELECTED
                        } else {
                            classes::ROW
                        };
                        
                        let on_click = {
                            let row = row.clone();
                            let on_row_click = props.on_row_click.clone();
                            Callback::from(move |_: MouseEvent| {
                                if let Some(callback) = &on_row_click {
                                    callback.emit(row.clone());
                                }
                            })
                        };
                        
                        html! {
                            <tr class={row_class} onclick={on_click}>
                                { for props.columns.iter().map(|col| {
                                    html! {
                                        <td class={classes::CELL}>
                                            { col.render.emit(row.clone()) }
                                        </td>
                                    }
                                }) }
                            </tr>
                        }
                    }) }
                </tbody>
            </table>
            
            if props.page_size.is_some() && total_pages > 1 {
                <div class={classes::PAGINATION}>
                    <span class="text-sm text-muted-foreground">
                        { format!("Page {} of {}", *current_page + 1, total_pages) }
                    </span>
                    <div class="flex gap-2">
                        <button
                            class="px-3 py-1 border rounded hover:bg-accent disabled:opacity-50"
                            onclick={on_prev_page}
                            disabled={*current_page == 0}
                        >
                            {"Previous"}
                        </button>
                        <button
                            class="px-3 py-1 border rounded hover:bg-accent disabled:opacity-50"
                            onclick={on_next_page}
                            disabled={*current_page >= total_pages - 1}
                        >
                            {"Next"}
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
