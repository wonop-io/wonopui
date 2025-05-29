use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(DataTableDocumentation)]
pub fn data_table_documentation() -> Html {
    /*
        html! {
            <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
                <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "DataTable Component" }</h1>
                <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                    { "The DataTable component is used for displaying tabular data. It supports sorting, filtering, and pagination functionalities to efficiently handle large datasets." }
                </p>

                <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
                <ExampleCode
                    preview={html! {
                        <DataTable
                            columns={vec!["Name".to_string(), "Age".to_string(), "Occupation".to_string()]}
                            data={vec![
                                vec!["Alice".to_string(), "30".to_string(), "Engineer".to_string()],
                                vec!["Bob".to_string(), "25".to_string(), "Designer".to_string()],
                                vec!["Charlie".to_string(), "35".to_string(), "Manager".to_string()],
                            ]}
                        />
                    }}
                    code={r#"
    <DataTable
        columns={vec!["Name".to_string(), "Age".to_string(), "Occupation".to_string()]}
        data={vec![
            vec!["Alice".to_string(), "30".to_string(), "Engineer".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "Designer".to_string()],
            vec!["Charlie".to_string(), "35".to_string(), "Manager".to_string()],
        ]}
    />"#.to_string()}
                />
                <Features features={vec!["DataTable"]} />

                <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                    { "API" }
                </h2>

                <ApiSection
                    title="DataTable"
                    description="Props for the DataTable component."
                    props={vec![
                        ("columns", "Vec<String>", "A vector of column names to display in the table header."),
                        ("data", "Vec<Vec<String>>", "A vector of rows, where each row is a vector of cell values."),
                    ]}
                />

                <NotesSection
                    title={"Notes".to_string()}
                    notes={vec![
                        "The DataTable component can manage large datasets efficiently.".to_string(),
                        "Sorting, filtering, and pagination functionalities need to be implemented as per the use case.".to_string(),
                        "The component utilizes a simple two-dimensional vector for data representation.".to_string(),
                    ]}
                />

                <StylingSection
                    component_name={"DataTable".to_string()}
                    class_descriptions={vec![
                        ("data_table".to_string(), "For the main data table container".to_string()),
                        ("data_table_header".to_string(), "For the table header row".to_string()),
                        ("data_table_row".to_string(), "For each data row".to_string()),
                        ("data_table_cell".to_string(), "For each data cell".to_string()),
                    ]}
                />

            </Container>
        }
        */
    html! {
        { "TODO: implement data table" }
    }
}
