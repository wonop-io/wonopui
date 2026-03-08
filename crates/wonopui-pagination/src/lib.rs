//! Pagination component for wonopui
//!
//! A pagination component for navigating through pages of content.

use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for pagination styling (shadcn v4 style).
pub mod classes {
    /// Container styles - shadcn v4 Pagination.
    pub const PAGINATION_CONTAINER: &str = "mx-auto flex w-full justify-center";
    
    /// List styles - with better spacing.
    pub const PAGINATION_LIST: &str = "flex flex-row items-center gap-1.5";
    
    /// Base item styles - button ghost variant with focus ring.
    pub const PAGINATION_ITEM_BASE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-lg text-sm font-medium transition-all duration-200 outline-none focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 text-zinc-700 dark:text-zinc-300";
    
    /// Ghost variant (non-active).
    pub const PAGINATION_ITEM_GHOST: &str = "hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-950 dark:hover:text-zinc-50 size-10";
    
    /// Outline variant (active) - premium with better contrast.
    pub const PAGINATION_ITEM_OUTLINE: &str = "border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-zinc-950 dark:text-zinc-50 shadow-sm size-10";
    
    /// Navigation button styles (prev/next) - with minimum width to prevent overlap.
    pub const PAGINATION_NAV: &str = "gap-2 px-4 h-10 min-w-[5rem] sm:min-w-[6rem]";
    
    /// Ellipsis styles.
    pub const PAGINATION_ELLIPSIS: &str = "flex size-10 items-center justify-center text-zinc-400 dark:text-zinc-500";
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
    let nav_button_class = merge_classes(&[classes::PAGINATION_ITEM_BASE, classes::PAGINATION_ITEM_GHOST, classes::PAGINATION_NAV]);

    // Default prev/next icons using chevrons
    let default_prev = html! {
        <>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m15 18-6-6 6-6"/>
            </svg>
            <span class="hidden sm:block">{"Previous"}</span>
        </>
    };
    
    let default_next = html! {
        <>
            <span class="hidden sm:block">{"Next"}</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m9 18 6-6-6-6"/>
            </svg>
        </>
    };

    html! {
        <nav data-slot="pagination" role="navigation" aria-label="pagination" class={container_class}>
            <ul data-slot="pagination-content" class={classes::PAGINATION_LIST}>
                <li data-slot="pagination-item">
                    <button
                        aria-label="Go to previous page"
                        class={&nav_button_class}
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
                                <li data-slot="pagination-item">
                                    <span data-slot="pagination-ellipsis" aria-hidden="true" class={classes::PAGINATION_ELLIPSIS}>
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
                                merge_classes(&[classes::PAGINATION_ITEM_BASE, classes::PAGINATION_ITEM_OUTLINE])
                            } else {
                                merge_classes(&[classes::PAGINATION_ITEM_BASE, classes::PAGINATION_ITEM_GHOST])
                            };

                            html! {
                                <li data-slot="pagination-item" key={page}>
                                    <button
                                        data-slot="pagination-link"
                                        data-active={is_current.to_string()}
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
                <li data-slot="pagination-item">
                    <button
                        aria-label="Go to next page"
                        class={&nav_button_class}
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