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
    pub class: Classes,
    #[prop_or_default]
    pub sticky_header: bool,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub border: Option<String>,
    #[prop_or_default]
    pub cellspacing: Option<String>,
    #[prop_or_default]
    pub cellpadding: Option<String>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let container_class = if props.sticky_header {
        classes!(
            brandguide.table_container,
            "sticky-header",
            props.class.clone()
        )
    } else {
        classes!(brandguide.table_container, props.class.clone())
    };

    html! {
        <div class={container_class}>
            <table
                class={&brandguide.table}
                id={props.id.clone()}
                width={props.width.clone()}
                border={props.border.clone()}
                cellspacing={props.cellspacing.clone()}
                cellpadding={props.cellpadding.clone()}
            >
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
    pub class: Classes,
    #[prop_or_default]
    pub sticky: bool,
}

#[function_component(TableHead)]
pub fn table_head(props: &TableHeadProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();

    let head_class = if props.sticky {
        classes!(brandguide.table_head, "sticky-top", props.class.clone())
    } else {
        classes!(brandguide.table_head, props.class.clone())
    };

    html! {
        <thead class={head_class}>
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
    pub class: Classes,
    #[prop_or_default]
    pub head: bool,
    #[prop_or_default]
    pub align: Option<String>,
    #[prop_or_default]
    pub valign: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
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
        <tr
            class={classes!(class, props.class.clone())}
            align={props.align.clone()}
            valign={props.valign.clone()}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </tr>
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
    pub width: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub align: Option<String>,
    #[prop_or_default]
    pub valign: Option<String>,
    #[prop_or_default]
    pub nowrap: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub onmouseover: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub onmouseout: Option<Callback<MouseEvent>>,
}

#[function_component(TableCell)]
pub fn table_cell(props: &TableCellProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <td
            class={classes!(brandguide.table_cell, props.class.clone())}
            colspan={props.colspan.map(|c| c.to_string())}
            rowspan={props.rowspan.map(|r| r.to_string())}
            width={props.width.clone()}
            height={props.height.clone()}
            align={props.align.clone()}
            valign={props.valign.clone()}
            nowrap={props.nowrap.then_some("nowrap")}
            onclick={props.onclick.clone()}
            onmouseover={props.onmouseover.clone()}
            onmouseout={props.onmouseout.clone()}
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
    #[prop_or_default]
    pub align: Option<String>,
    #[prop_or_default]
    pub valign: Option<String>,
}

#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <tbody
            class={classes!(brandguide.table_body, props.class.clone())}
            align={props.align.clone()}
            valign={props.valign.clone()}
        >
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
    pub class: Classes,
    #[prop_or_default]
    pub align: Option<String>,
    #[prop_or_default]
    pub valign: Option<String>,
}

#[function_component(TableFooter)]
pub fn table_footer(props: &TableFooterProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <tfoot
            class={classes!(brandguide.table_footer, props.class.clone())}
            align={props.align.clone()}
            valign={props.valign.clone()}
        >
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
