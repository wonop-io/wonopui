#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <div class={format!("{} {}", brandguide.table_container, props.class)}>
            <table class={&brandguide.table}>
                { for props.children.iter() }
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeadProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<TableRow>,
    #[prop_or_default]
    pub class: String,
}

#[function_component(TableHead)]
pub fn table_head(props: &TableHeadProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <thead class={format!("{} {}", brandguide.table_head, props.class)}>
            { for props.children.iter().map(|child| {
                html! {
                    <TableRow
                        class={child.props.class.clone()}
                        head={true}
                    >
                        { child.props.children.clone() }
                    </TableRow>
                }
            }) }
        </thead>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub head: bool,
}

#[function_component(TableRow)]
pub fn table_row(props: &TableRowProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let class = if props.head {
        &brandguide.table_head_row
    } else {
        &brandguide.table_row
    };
    html! {
        <tr class={format!("{} {}", class, props.class)}>
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
    #[prop_or_default]
    pub colspan: Option<u32>,
    #[prop_or_default]
    pub rowspan: Option<u32>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(TableCell)]
pub fn table_cell(props: &TableCellProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <td
            class={format!("{} {}", brandguide.table_cell, props.class)}
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
    pub class: String,
}

#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <tbody class={format!("{} {}", brandguide.table_body, props.class)}>
           { for props.children.iter() }
           /*
            { for props.children.iter().map(|child| {
                html! {
                    <TableRow
                        class={child.props.class.clone()}
                        head={false}
                    >
                        { child.props.children.clone() }
                    </TableRow>
                }
            }) }
            */
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
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <tfoot class={format!("{} {}", brandguide.table_footer, props.class)}>
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
