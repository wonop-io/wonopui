//! Pagination component for wonopui
//!
//! A pagination component for navigating through pages of content.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const PAGINATION_CONTAINER: &str = "flex justify-center mt-8";
    pub const PAGINATION_LIST: &str = "inline-flex items-center -space-x-px";
    pub const PAGINATION_ITEM: &str = "px-3 py-2 leading-tight text-gray-500 dark:text-zinc-400 bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600 hover:bg-gray-100 dark:hover:bg-zinc-700 hover:text-gray-700 dark:hover:text-zinc-200 disabled:opacity-50 disabled:cursor-not-allowed";
    pub const PAGINATION_ITEM_CURRENT: &str = "z-10 px-3 py-2 leading-tight text-blue-600 dark:text-blue-400 border border-blue-300 dark:border-blue-600 bg-blue-50 dark:bg-blue-900/20 hover:bg-blue-100 dark:hover:bg-blue-900/40 hover:text-blue-700 dark:hover:text-blue-300";
    pub const PAGINATION_ELLIPSIS: &str = "px-3 py-2 leading-tight text-gray-500 dark:text-zinc-400 bg-white dark:bg-zinc-800 border border-gray-300 dark:border-zinc-600";
}

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub total_pages: usize,
    pub current_page: usize,
    pub on_page_change: Callback<usize>,
    #[prop_or_default]
    pub next: Option<Html>,
    #[prop_or_default]
    pub prev: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let PaginationProps {
        total_pages,
        current_page,
        on_page_change,
        next,
        prev,
        class,
    } = props;

    let page_range = if *total_pages <= 11 {
        (1..=*total_pages).collect::<Vec<_>>()
    } else if *current_page <= 5 {
        let mut range = (1..=7).collect::<Vec<_>>();
        range.push(0); // Placeholder for ellipsis
        range.extend(*total_pages - 1..=*total_pages);
        range
    } else if *current_page >= *total_pages - 4 {
        let mut range = vec![1, 2];
        range.push(0); // Placeholder for ellipsis
        range.extend(*total_pages - 6..=*total_pages);
        range
    } else {
        let mut range = vec![1, 2];
        range.push(0); // Placeholder for first ellipsis
        range.extend(*current_page - 2..=*current_page + 2);
        range.push(0); // Placeholder for second ellipsis
        range.extend(*total_pages - 1..=*total_pages);
        range
    };

    let container_class = merge_classes(&[
        classes::PAGINATION_CONTAINER,
        &class.to_string(),
    ]);

    html! {
        <nav class={container_class} aria-label="Pagination">
            <ul class={classes::PAGINATION_LIST}>
                <li>
                    <button
                        class={classes::PAGINATION_ITEM}
                        onclick={{
                            let on_page_change = on_page_change.clone();
                            let current_page = *current_page;
                            on_page_change.reform(move |_| current_page.saturating_sub(1).max(1))
                        }}
                        disabled={*current_page == 1}
                    >
                        {prev.clone().unwrap_or_else(|| html!("Prev"))}
                    </button>
                </li>
                {
                    page_range.iter().map(|&page| {
                        if page == 0 {
                            html! {
                                <li>
                                    <span class={classes::PAGINATION_ELLIPSIS}>{"..."}</span>
                                </li>
                            }
                        } else {
                            let is_current = page == *current_page;
                            let page_class = if is_current {
                                classes::PAGINATION_ITEM_CURRENT
                            } else {
                                classes::PAGINATION_ITEM
                            };

                            html! {
                                <li key={page}>
                                    <button
                                        class={page_class}
                                        aria-current={if is_current { "page" } else { "false" }}
                                        onclick={{
                                            let on_page_change = on_page_change.clone();
                                            on_page_change.reform(move |_| page)
                                        }}
                                    >
                                        {page}
                                    </button>
                                </li>
                            }
                        }
                    }).collect::<Html>()
                }
                <li>
                    <button
                        class={classes::PAGINATION_ITEM}
                        onclick={{
                            let on_page_change = on_page_change.clone();
                            let current_page = *current_page;
                            let total_pages = *total_pages;
                            on_page_change.reform(move |_| (current_page + 1).min(total_pages))
                        }}
                        disabled={*current_page == *total_pages}
                    >
                        {next.clone().unwrap_or_else(|| html!("Next"))}
                    </button>
                </li>
            </ul>
        </nav>
    }
}
