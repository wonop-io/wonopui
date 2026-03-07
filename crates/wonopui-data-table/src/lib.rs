//! Data Table component for WonopUI.
//!
//! A data table component with sorting, filtering, and pagination.
//! Note: This is a basic implementation. A full-featured data table
//! would include column resizing, row selection, and more.

pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the DataTable component (shadcn v4 style).
pub mod classes {
    /// Container styles - shadcn v4.
    pub const CONTAINER: &str = "relative w-full overflow-x-auto";
    
    /// Table styles - shadcn v4.
    pub const TABLE: &str = "w-full caption-bottom text-sm";
    
    /// Header styles - shadcn v4.
    pub const HEADER: &str = "[&_tr]:border-b";
    
    /// Header row styles.
    pub const HEADER_ROW: &str = "border-b border-zinc-200 dark:border-zinc-800 transition-colors";
    
    /// Header cell styles - shadcn v4 with proper text color.
    pub const HEADER_CELL: &str = "h-10 px-2 text-left align-middle font-medium text-zinc-950 dark:text-zinc-50 whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]";
    
    /// Sortable header cell styles - shadcn v4.
    pub const HEADER_CELL_SORTABLE: &str = "h-10 px-2 text-left align-middle font-medium text-zinc-950 dark:text-zinc-50 whitespace-nowrap cursor-pointer select-none hover:bg-zinc-100/50 dark:hover:bg-zinc-800/50 [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]";
    
    /// Body styles - shadcn v4.
    pub const BODY: &str = "[&_tr:last-child]:border-0";
    
    /// Row styles - shadcn v4 with hover and selection states.
    pub const ROW: &str = "border-b border-zinc-200 dark:border-zinc-800 transition-colors hover:bg-zinc-100/50 dark:hover:bg-zinc-800/50 data-[state=selected]:bg-zinc-100 dark:data-[state=selected]:bg-zinc-800";
    
    /// Selected row styles - shadcn v4.
    pub const ROW_SELECTED: &str = "border-b border-zinc-200 dark:border-zinc-800 transition-colors bg-zinc-100 dark:bg-zinc-800";
    
    /// Cell styles - shadcn v4.
    pub const CELL: &str = "p-2 align-middle text-zinc-900 dark:text-zinc-50 whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]";
    
    /// Footer styles - shadcn v4.
    pub const FOOTER: &str = "border-t border-zinc-200 dark:border-zinc-800 bg-zinc-100/50 dark:bg-zinc-800/50 font-medium text-zinc-900 dark:text-zinc-50 [&>tr]:last:border-b-0";
    
    /// Pagination container styles.
    pub const PAGINATION: &str = "flex items-center justify-between gap-2 p-4 text-sm text-zinc-500 dark:text-zinc-400";
    
    /// Pagination button styles - shadcn v4.
    pub const PAGINATION_BUTTON: &str = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-all outline-none h-9 px-3 hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:pointer-events-none disabled:opacity-50";
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
    // TODO: Implement sorting functionality
    let _sort_column = use_state(|| Option::<String>::None);
    let _sort_direction = use_state(|| SortDirection::Ascending);

    let page_size = props.page_size.unwrap_or(10);
    let total_pages = props.data.len().div_ceil(page_size);

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
                    <span class="text-sm text-zinc-500 dark:text-zinc-400">
                        { format!("Page {} of {}", *current_page + 1, total_pages) }
                    </span>
                    <div class="flex gap-2">
                        <button
                            class={classes::PAGINATION_BUTTON}
                            onclick={on_prev_page}
                            disabled={*current_page == 0}
                            type="button"
                        >
                            {"Previous"}
                        </button>
                        <button
                            class={classes::PAGINATION_BUTTON}
                            onclick={on_next_page}
                            disabled={*current_page >= total_pages - 1}
                            type="button"
                        >
                            {"Next"}
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
