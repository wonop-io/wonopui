use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    html! {
        <div class={format!("{} {}", BRANDGUIDE.table_container, props.class)}>
            <table class={BRANDGUIDE.table.clone()}>
                { for props.children.iter() }
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeadProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(TableHead)]
pub fn table_head(props: &TableHeadProps) -> Html {
    html! {
        <thead class={format!("{} {}", BRANDGUIDE.table_head, props.class)}>
            { for props.children.iter() }
        </thead>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(TableRow)]
pub fn table_row(props: &TableRowProps) -> Html {
    html! {
        <tr class={format!("{} {}", BRANDGUIDE.table_row, props.class)}>
            { for props.children.iter() }
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableCellProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(TableCell)]
pub fn table_cell(props: &TableCellProps) -> Html {
    html! {
        <td class={format!("{} {}", BRANDGUIDE.table_cell, props.class)}>
            { for props.children.iter() }
        </td>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProps) -> Html {
    html! {
        <tbody class={format!("{} {}", BRANDGUIDE.table_body, props.class)}>
            { for props.children.iter() }
        </tbody>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(TableFooter)]
pub fn table_footer(props: &TableFooterProps) -> Html {
    html! {
        <tfoot class={format!("{} {}", BRANDGUIDE.table_footer, props.class)}>
            { for props.children.iter() }
        </tfoot>
    }
}

// New entries in the brand guide:
// table_container: "overflow-x-auto"
// table: "min-w-full bg-white border"
// table_head: ""
// table_row: ""
// table_cell: "py-2 px-4 border-b"
// table_body: ""
// table_footer: ""
