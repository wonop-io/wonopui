//! Table component for wonopui
//!
//! A responsive table component with header, body, and footer support.

use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const TABLE_CONTAINER: &str = "overflow-x-auto";
    pub const TABLE: &str = "min-w-full divide-y divide-gray-200 dark:divide-zinc-700";
    pub const TABLE_HEAD: &str = "bg-gray-50 dark:bg-zinc-800";
    pub const TABLE_HEAD_ROW: &str = "";
    pub const TABLE_HEAD_CELL: &str = "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-zinc-400 uppercase tracking-wider";
    pub const TABLE_BODY: &str = "bg-white dark:bg-zinc-900 divide-y divide-gray-200 dark:divide-zinc-700";
    pub const TABLE_ROW: &str = "hover:bg-gray-50 dark:hover:bg-zinc-800 transition-colors";
    pub const TABLE_CELL: &str = "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100";
    pub const TABLE_FOOTER: &str = "bg-gray-50 dark:bg-zinc-800";
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub sticky_header: bool,
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let container_class = merge_classes(&[
        classes::TABLE_CONTAINER,
        if props.sticky_header { "sticky-header" } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div class={container_class}>
            <table class={classes::TABLE} id={props.id.clone()}>
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
    pub class: Classes,
    #[prop_or_default]
    pub sticky: bool,
}

#[function_component(TableHead)]
pub fn table_head(props: &TableHeadProps) -> Html {
    let head_class = merge_classes(&[
        classes::TABLE_HEAD,
        if props.sticky { "sticky top-0" } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <thead class={head_class}>
            { for props.children.iter() }
        </thead>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub head: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(TableRow)]
pub fn table_row(props: &TableRowProps) -> Html {
    let class = merge_classes(&[
        if props.head { classes::TABLE_HEAD_ROW } else { classes::TABLE_ROW },
        &props.class.to_string(),
    ]);

    html! {
        <tr class={class} onclick={props.onclick.clone()}>
            { for props.children.iter() }
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeadCellProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub colspan: Option<u32>,
    #[prop_or_default]
    pub rowspan: Option<u32>,
    #[prop_or_default]
    pub scope: Option<String>,
}

#[function_component(TableHeadCell)]
pub fn table_head_cell(props: &TableHeadCellProps) -> Html {
    let class = merge_classes(&[
        classes::TABLE_HEAD_CELL,
        &props.class.to_string(),
    ]);

    html! {
        <th
            class={class}
            colspan={props.colspan.map(|c| c.to_string())}
            rowspan={props.rowspan.map(|r| r.to_string())}
            scope={props.scope.clone()}
        >
            { for props.children.iter() }
        </th>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableCellProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub colspan: Option<u32>,
    #[prop_or_default]
    pub rowspan: Option<u32>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(TableCell)]
pub fn table_cell(props: &TableCellProps) -> Html {
    let class = merge_classes(&[
        classes::TABLE_CELL,
        &props.class.to_string(),
    ]);

    html! {
        <td
            class={class}
            colspan={props.colspan.map(|c| c.to_string())}
            rowspan={props.rowspan.map(|r| r.to_string())}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </td>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProps) -> Html {
    let class = merge_classes(&[
        classes::TABLE_BODY,
        &props.class.to_string(),
    ]);

    html! {
        <tbody class={class}>
           { for props.children.iter() }
        </tbody>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TableFooter)]
pub fn table_footer(props: &TableFooterProps) -> Html {
    let class = merge_classes(&[
        classes::TABLE_FOOTER,
        &props.class.to_string(),
    ]);

    html! {
        <tfoot class={class}>
            { for props.children.iter() }
        </tfoot>
    }
}
