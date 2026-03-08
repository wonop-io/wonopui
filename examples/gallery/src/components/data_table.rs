use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::*;
use wonopui::data_table::{Column, DataTable};
use yew::prelude::*;

/// Sample data structure for the table
#[derive(Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
    department: String,
}

#[function_component(DataTableDemo)]
fn data_table_demo() -> Html {
    let data = vec![
        Person { name: "Alice".to_string(), age: 30, occupation: "Engineer".to_string(), department: "R&D".to_string() },
        Person { name: "Bob".to_string(), age: 25, occupation: "Designer".to_string(), department: "Product".to_string() },
        Person { name: "Charlie".to_string(), age: 35, occupation: "Manager".to_string(), department: "Engineering".to_string() },
        Person { name: "Diana".to_string(), age: 28, occupation: "Analyst".to_string(), department: "Finance".to_string() },
        Person { name: "Eve".to_string(), age: 32, occupation: "Developer".to_string(), department: "Engineering".to_string() },
    ];

    let columns: Vec<Column<Person>> = vec![
        Column {
            key: "name".to_string(),
            header: "Name".to_string(),
            sortable: true,
            render: Callback::from(|p: Person| html! { <span class="font-medium">{ p.name }</span> }),
        },
        Column {
            key: "age".to_string(),
            header: "Age".to_string(),
            sortable: true,
            render: Callback::from(|p: Person| html! { { p.age } }),
        },
        Column {
            key: "occupation".to_string(),
            header: "Occupation".to_string(),
            sortable: false,
            render: Callback::from(|p: Person| html! { { p.occupation } }),
        },
        Column {
            key: "department".to_string(),
            header: "Department".to_string(),
            sortable: false,
            render: Callback::from(|p: Person| html! {
                <span class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300">
                    { p.department }
                </span>
            }),
        },
    ];

    html! {
        <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 overflow-hidden">
            <DataTable<Person>
                columns={columns}
                data={data}
            />
        </div>
    }
}

#[function_component(DataTableDocumentation)]
pub fn data_table_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "DataTable Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The DataTable component is used for displaying tabular data. It supports sorting, filtering, and pagination functionalities to efficiently handle large datasets." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <DataTableDemo />
                }}
                code={r#"
#[derive(Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
    department: String,
}

let data = vec![
    Person { name: "Alice".to_string(), age: 30, occupation: "Engineer".to_string(), department: "R&D".to_string() },
    Person { name: "Bob".to_string(), age: 25, occupation: "Designer".to_string(), department: "Product".to_string() },
    Person { name: "Charlie".to_string(), age: 35, occupation: "Manager".to_string(), department: "Engineering".to_string() },
];

let columns: Vec<Column<Person>> = vec![
    Column {
        key: "name".to_string(),
        header: "Name".to_string(),
        sortable: true,
        render: Callback::from(|p: Person| html! { <span class="font-medium">{ p.name }</span> }),
    },
    Column {
        key: "age".to_string(),
        header: "Age".to_string(),
        sortable: true,
        render: Callback::from(|p: Person| html! { { p.age } }),
    },
    Column {
        key: "occupation".to_string(),
        header: "Occupation".to_string(),
        sortable: false,
        render: Callback::from(|p: Person| html! { { p.occupation } }),
    },
];

html! {
    <DataTable<Person>
        columns={columns}
        data={data}
    />
}"#.to_string()}
            />
            <Features features={vec!["DataTable"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="DataTable"
                description="Props for the DataTable component."
                props={vec![
                    ("columns", "Vec<Column<T>>", "A vector of column definitions with key, header, sortable flag, and render callback."),
                    ("data", "Vec<T>", "A vector of data items to display in the table."),
                    ("on_row_click", "Option<Callback<T>>", "Optional callback when a row is clicked."),
                    ("selected_row", "Option<usize>", "Index of the currently selected row."),
                    ("page_size", "Option<usize>", "Number of items per page for pagination."),
                ]}
            />

            <NotesSection
                title={"Notes".to_string()}
                notes={vec![
                    "The DataTable component can manage large datasets efficiently with pagination.".to_string(),
                    "Columns can be marked as sortable for future sorting functionality.".to_string(),
                    "The render callback allows full customization of cell content.".to_string(),
                    "Row selection is supported via on_row_click and selected_row props.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"DataTable".to_string()}
                class_descriptions={vec![
                    ("CONTAINER".to_string(), "For the main data table container".to_string()),
                    ("TABLE".to_string(), "For the table element".to_string()),
                    ("HEADER".to_string(), "For the thead element".to_string()),
                    ("HEADER_ROW".to_string(), "For the header row".to_string()),
                    ("HEADER_CELL".to_string(), "For header cells".to_string()),
                    ("ROW".to_string(), "For data rows".to_string()),
                    ("CELL".to_string(), "For data cells".to_string()),
                ]}
            />

        </Container>
    }
}
