//! Table component for wonopui
//!
//! A responsive table component with header, body, and footer support.

use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for table styling (shadcn v4 style).
pub mod classes {
    /// Table container styles - shadcn v4.
    pub const TABLE_CONTAINER: &str = "relative w-full overflow-x-auto";
    
    /// Table element styles - shadcn v4 Table.
    pub const TABLE: &str = "w-full caption-bottom text-sm";
    
    /// Table header styles - shadcn v4 TableHeader.
    pub const TABLE_HEAD: &str = "[&_tr]:border-b";
    
    /// Table head row styles.
    pub const TABLE_HEAD_ROW: &str = "border-b border-zinc-200 dark:border-zinc-800 transition-colors";
    
    /// Table head cell styles - shadcn v4 TableHead with proper text color.
    pub const TABLE_HEAD_CELL: &str = "h-10 px-2 text-left align-middle font-medium text-zinc-950 dark:text-zinc-50 whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]";
    
    /// Table body styles - shadcn v4 TableBody.
    pub const TABLE_BODY: &str = "[&_tr:last-child]:border-0";
    
    /// Table row styles - shadcn v4 TableRow with hover and selection states.
    pub const TABLE_ROW: &str = "border-b border-zinc-200 dark:border-zinc-800 transition-colors hover:bg-zinc-100/50 dark:hover:bg-zinc-800/50 data-[state=selected]:bg-zinc-100 dark:data-[state=selected]:bg-zinc-800";
    
    /// Table cell styles - shadcn v4 TableCell.
    pub const TABLE_CELL: &str = "p-2 align-middle text-zinc-900 dark:text-zinc-50 whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]";
    
    /// Table footer styles - shadcn v4 TableFooter.
    pub const TABLE_FOOTER: &str = "border-t border-zinc-200 dark:border-zinc-800 bg-zinc-100/50 dark:bg-zinc-800/50 font-medium [&>tr]:last:border-b-0";
    
    /// Table caption styles - shadcn v4 TableCaption.
    pub const TABLE_CAPTION: &str = "mt-4 text-sm text-zinc-500 dark:text-zinc-400";
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
        if props.sticky_header {
            "sticky-header"
        } else {
            ""
        },
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="table-container" class={container_class}>
            <table data-slot="table" class={classes::TABLE} id={props.id.clone()}>
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
        <thead data-slot="table-header" class={head_class}>
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
        if props.head {
            classes::TABLE_HEAD_ROW
        } else {
            classes::TABLE_ROW
        },
        &props.class.to_string(),
    ]);

    html! {
        <tr data-slot="table-row" class={class} onclick={props.onclick.clone()}>
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
    let class = merge_classes(&[classes::TABLE_HEAD_CELL, &props.class.to_string()]);

    html! {
        <th
            data-slot="table-head"
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
    let class = merge_classes(&[classes::TABLE_CELL, &props.class.to_string()]);

    html! {
        <td
            data-slot="table-cell"
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
    let class = merge_classes(&[classes::TABLE_BODY, &props.class.to_string()]);

    html! {
        <tbody data-slot="table-body" class={class}>
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
    let class = merge_classes(&[classes::TABLE_FOOTER, &props.class.to_string()]);

    html! {
        <tfoot data-slot="table-footer" class={class}>
            { for props.children.iter() }
        </tfoot>
    }
}
