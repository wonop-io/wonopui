#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub total_pages: usize,
    pub current_page: usize,
    pub on_page_change: Callback<usize>,
    #[prop_or_default]
    pub next: Option<Html>,
    #[prop_or_default]
    pub prev: Option<Html>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let PaginationProps {
        total_pages,
        current_page,
        on_page_change,
        next,
        prev,
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

    html! {
        <nav class={classes!(&brandguide.pagination_container)} aria-label="Pagination">
            <ul class={classes!(&brandguide.pagination_list)}>
                <li>
                    <button
                        class={classes!(&brandguide.pagination_item)}
                        onclick={{
                            let on_page_change = on_page_change.clone();
                            let current_page = current_page.clone();
                            on_page_change.reform(move |_| current_page.saturating_sub(1))}}
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
                                    <span class={classes!(&brandguide.pagination_item)}>{"..."}</span>
                                </li>
                            }
                        } else {
                            let is_current = page == *current_page;
                            let page_class = if is_current {
                                classes!(&brandguide.pagination_item_current)
                            } else {
                                classes!(&brandguide.pagination_item)
                            };

                            html! {
                                <li key={page}>
                                    <button
                                        class={page_class}
                                        aria-current={if is_current { "page" } else { "false" }}
                                        onclick={{
                                            let on_page_change = on_page_change.clone();
                                            on_page_change.reform(move |_| page)}}
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
                        class={classes!(&brandguide.pagination_item)}
                        onclick={{
                            let on_page_change = on_page_change.clone();
                            let current_page = current_page.clone();
                            let total_pages = total_pages.clone();
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

// Snippets to update brandguide:
// ("pagination_container".to_string(), "flex justify-center mt-8".to_string()),
// ("pagination_list".to_string(), "inline-flex items-center -space-x-px".to_string()),
// ("pagination_item".to_string(), "px-3 py-2 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700".to_string()),
// ("pagination_item_current".to_string(), "z-10 px-3 py-2 leading-tight text-blue-600 border border-blue-300 bg-blue-50 hover:bg-blue-100 hover:text-blue-700".to_string()),
//
// pub pagination_container: ClassesContainer<T>,
// pub pagination_list: ClassesContainer<T>,
// pub pagination_item: ClassesContainer<T>,
// pub pagination_item_current: ClassesContainer<T>,
//
// pagination_container: self.pagination_container.to_owned(),
// pagination_list: self.pagination_list.to_owned(),
// pagination_item: self.pagination_item.to_owned(),
// pagination_item_current: self.pagination_item_current.to_owned(),
//
// pagination_container: default_config_hm
// .get("pagination_container")
// .expect("Template parameter missing")
// .clone(),
// pagination_list: default_config_hm
// .get("pagination_list")
// .expect("Template parameter missing")
// .clone(),
// pagination_item: default_config_hm
// .get("pagination_item")
// .expect("Template parameter missing")
// .clone(),
// pagination_item_current: default_config_hm
// .get("pagination_item_current")
// .expect("Template parameter missing")
// .clone(),
