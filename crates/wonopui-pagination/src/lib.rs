//! Pagination component for wonopui
//!
//! A pagination component for navigating through pages of content.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    /// Container for the pagination nav
    pub const PAGINATION_CONTAINER: &str = "flex justify-center";
    
    /// The ul wrapper for pagination items
    pub const PAGINATION_LIST: &str = "flex flex-row items-center gap-1";
    
    /// Base pagination item (button style, ghost variant)
    pub const PAGINATION_ITEM: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 min-w-9 px-3";
    
    /// Current/active page item
    pub const PAGINATION_ITEM_CURRENT: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 min-w-9 px-3";
    
    /// Ellipsis span
    pub const PAGINATION_ELLIPSIS: &str = "flex h-9 w-9 items-center justify-center text-muted-foreground";
    
    /// Navigation buttons (prev/next)
    pub const PAGINATION_NAV: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 gap-1 px-2.5";
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

    let container_class = merge_classes(&[classes::PAGINATION_CONTAINER, &class.to_string()]);

    // Default prev/next icons using chevrons
    let default_prev = html! {
        <>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m15 18-6-6 6-6"/>
            </svg>
            <span>{"Previous"}</span>
        </>
    };
    
    let default_next = html! {
        <>
            <span>{"Next"}</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m9 18 6-6-6-6"/>
            </svg>
        </>
    };

    html! {
        <nav class={container_class} aria-label="pagination">
            <ul class={classes::PAGINATION_LIST}>
                <li>
                    <button
                        class={classes::PAGINATION_NAV}
                        aria-label="Go to previous page"
                        onclick={{
                            let on_page_change = on_page_change.clone();
                            let current_page = *current_page;
                            on_page_change.reform(move |_| current_page.saturating_sub(1).max(1))
                        }}
                        disabled={*current_page == 1}
                    >
                        {prev.clone().unwrap_or(default_prev)}
                    </button>
                </li>
                {
                    page_range.iter().map(|&page| {
                        if page == 0 {
                            html! {
                                <li>
                                    <span class={classes::PAGINATION_ELLIPSIS} aria-hidden="true">
                                        <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <circle cx="12" cy="12" r="1"/>
                                            <circle cx="19" cy="12" r="1"/>
                                            <circle cx="5" cy="12" r="1"/>
                                        </svg>
                                        <span class="sr-only">{"More pages"}</span>
                                    </span>
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
                                        aria-current={if is_current { Some("page") } else { None }}
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
                        class={classes::PAGINATION_NAV}
                        aria-label="Go to next page"
                        onclick={{
                            let on_page_change = on_page_change.clone();
                            let current_page = *current_page;
                            let total_pages = *total_pages;
                            on_page_change.reform(move |_| (current_page + 1).min(total_pages))
                        }}
                        disabled={*current_page == *total_pages}
                    >
                        {next.clone().unwrap_or(default_next)}
                    </button>
                </li>
            </ul>
        </nav>
    }
}
