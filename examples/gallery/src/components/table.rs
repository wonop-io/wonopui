use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(TableThemeEditor)]
pub fn table_theme_editor() -> Html {
    let fields = vec![
        ("table_container".to_string(), "Table Container".to_string()),
        ("table".to_string(), "Table".to_string()),
        ("table_head".to_string(), "Table Head".to_string()),
        ("table_row".to_string(), "Table Row".to_string()),
        ("table_cell".to_string(), "Table Cell".to_string()),
        ("table_body".to_string(), "Table Body".to_string()),
        ("table_footer".to_string(), "Table Footer".to_string()),
    ];

    let preview = html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>{"Header 1"}</TableCell>
                    <TableCell>{"Header 2"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                <TableRow>
                    <TableCell>{"Data 1"}</TableCell>
                    <TableCell>{"Data 2"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(TableDemo)]
pub fn table_demo() -> Html {
    let data = use_state(|| {
        vec![
            ("Bob".to_string(), 42, "Engineer".to_string()),
            ("Alice".to_string(), 35, "Designer".to_string()),
            ("Charlie".to_string(), 21, "Manager".to_string()),
        ]
    });

    let sort_by_name = {
        let data = data.clone();
        Callback::from(move |_| {
            let mut new_data = (*data).clone();
            new_data.sort_by(|a, b| a.0.cmp(&b.0));
            data.set(new_data);
        })
    };

    let sort_by_age = {
        let data = data.clone();
        Callback::from(move |_| {
            let mut new_data = (*data).clone();
            new_data.sort_by(|a, b| a.1.cmp(&b.1));
            data.set(new_data);
        })
    };

    html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell onclick={sort_by_name} class="cursor-pointer">{"Name ↕"}</TableCell>
                    <TableCell onclick={sort_by_age} class="cursor-pointer">{"Age ↕"}</TableCell>
                    <TableCell>{"Occupation"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                {data.iter().map(|(name, age, occupation)| {
                    html! {
                        <TableRow>
                            <TableCell>{name.clone()}</TableCell>
                            <TableCell>{age.clone()}</TableCell>
                            <TableCell>{occupation.clone()}</TableCell>
                        </TableRow>
                    }
                }).collect::<Html>()}
            </TableBody>
            <TableFooter>
                <TableRow>
                    <TableCell colspan={3}>{"Total Employees: 3"}</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
    }
}

#[function_component(TableVariations)]
pub fn table_variations() -> Html {
    html! {
        <>
            <h3 class="text-xl font-semibold mb-2">{"Basic Table"}</h3>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableCell>{"Name"}</TableCell>
                        <TableCell>{"Age"}</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell>{"John"}</TableCell>
                        <TableCell>{"30"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>{"Jane"}</TableCell>
                        <TableCell>{"25"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <h3 class="text-xl font-semibold mt-4 mb-2">{"Table with Footer"}</h3>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableCell>{"Item"}</TableCell>
                        <TableCell>{"Price"}</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell>{"Apple"}</TableCell>
                        <TableCell>{"$1.00"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>{"Orange"}</TableCell>
                        <TableCell>{"$0.75"}</TableCell>
                    </TableRow>
                </TableBody>
                <TableFooter>
                    <TableRow>
                        <TableCell>{"Total"}</TableCell>
                        <TableCell>{"$1.75"}</TableCell>
                    </TableRow>
                </TableFooter>
            </Table>

            <h3 class="text-xl font-semibold mt-4 mb-2">{"Table with Colspan"}</h3>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableCell colspan={2}>{"Employee Information"}</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell>{"Name"}</TableCell>
                        <TableCell>{"Alice"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>{"Position"}</TableCell>
                        <TableCell>{"Developer"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </>
    }
}

#[function_component(TableDocumentation)]
pub fn table_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Table Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Table component is used to display tabular data in an organized manner. It provides subcomponents for table head, body, row, cell, and footer to help structure your data properly." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Examples" }</h2>
            <ExampleCode
                preview={html! { <TableVariations /> }}
                customize={html! { <TableThemeEditor /> }}
                code={r#"
// Basic Table
<Table>
    <TableHead>
        <TableRow>
            <TableCell>{"Name"}</TableCell>
            <TableCell>{"Age"}</TableCell>
        </TableRow>
    </TableHead>
    <TableBody>
        <TableRow>
            <TableCell>{"John"}</TableCell>
            <TableCell>{"30"}</TableCell>
        </TableRow>
        <TableRow>
            <TableCell>{"Jane"}</TableCell>
            <TableCell>{"25"}</TableCell>
        </TableRow>
    </TableBody>
</Table>

// Table with Footer
<Table>
    <TableHead>
        <TableRow>
            <TableCell>{"Item"}</TableCell>
            <TableCell>{"Price"}</TableCell>
        </TableRow>
    </TableHead>
    <TableBody>
        <TableRow>
            <TableCell>{"Apple"}</TableCell>
            <TableCell>{"$1.00"}</TableCell>
        </TableRow>
        <TableRow>
            <TableCell>{"Orange"}</TableCell>
            <TableCell>{"$0.75"}</TableCell>
        </TableRow>
    </TableBody>
    <TableFooter>
        <TableRow>
            <TableCell>{"Total"}</TableCell>
            <TableCell>{"$1.75"}</TableCell>
        </TableRow>
    </TableFooter>
</Table>

// Table with Colspan
<Table>
    <TableHead>
        <TableRow>
            <TableCell colspan={2}>{"Employee Information"}</TableCell>
        </TableRow>
    </TableHead>
    <TableBody>
        <TableRow>
            <TableCell>{"Name"}</TableCell>
            <TableCell>{"Alice"}</TableCell>
        </TableRow>
        <TableRow>
            <TableCell>{"Position"}</TableCell>
            <TableCell>{"Developer"}</TableCell>
        </TableRow>
    </TableBody>
</Table>
"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 mt-8 text-zinc-900 dark:text-white">{ "Interactive Demo" }</h2>
            <ExampleCode
                preview={html! { <TableDemo /> }}
                code={r#"
#[function_component(TableDemo)]
pub fn table_demo() -> Html {
    let data = use_state(|| vec![
        ("Alice".to_string(), 28, "Engineer".to_string()),
        ("Bob".to_string(), 35, "Designer".to_string()),
        ("Charlie".to_string(), 42, "Manager".to_string()),
    ]);

    let sort_by_name = {
        let data = data.clone();
        Callback::from(move |_| {
            let mut new_data = (*data).clone();
            new_data.sort_by(|a, b| a.0.cmp(&b.0));
            data.set(new_data);
        })
    };

    let sort_by_age = {
        let data = data.clone();
        Callback::from(move |_| {
            let mut new_data = (*data).clone();
            new_data.sort_by(|a, b| a.1.cmp(&b.1));
            data.set(new_data);
        })
    };

    html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell><button onclick={sort_by_name}>{"Name ↕"}</button></TableCell>
                    <TableCell><button onclick={sort_by_age}>{"Age ↕"}</button></TableCell>
                    <TableCell>{"Occupation"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                {data.iter().map(|(name, age, occupation)| {
                    html! {
                        <TableRow>
                            <TableCell>{name.clone()}</TableCell>
                            <TableCell>{age.clone()}</TableCell>
                            <TableCell>{occupation.clone()}</TableCell>
                        </TableRow>
                    }
                }).collect::<Html>()}
            </TableBody>
            <TableFooter>
                <TableRow>
                    <TableCell colspan={3}>{"Total Employees: 3"}</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
    }
}"#.to_string()}
            />

            <Features features={vec!["Table", "TableHead", "TableBody", "TableRow", "TableCell", "TableFooter"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Table"
                description="Props for the Table component."
                props={vec![
                    ("children", "Children", "The table rows, head, body, and footer components to be rendered inside the table."),
                    ("class", "String", "Additional CSS classes to apply to the table container."),
                ]}
            />

            <ApiSection
                title="TableHead"
                description="Props for the TableHead component."
                props={vec![
                    ("children", "Children", "The table rows to be rendered inside the table head."),
                    ("class", "String", "Additional CSS classes to apply to the table head."),
                ]}
            />

            <ApiSection
                title="TableRow"
                description="Props for the TableRow component."
                props={vec![
                    ("children", "Children", "The table cells to be rendered inside the table row."),
                    ("class", "String", "Additional CSS classes to apply to the table row."),
                ]}
            />

            <ApiSection
                title="TableCell"
                description="Props for the TableCell component."
                props={vec![
                    ("children", "Children", "The content to be rendered inside the table cell."),
                    ("class", "String", "Additional CSS classes to apply to the table cell."),
                    ("colspan", "Option<u32>", "Number of columns the cell should span."),
                    ("rowspan", "Option<u32>", "Number of rows the cell should span."),
                    ("onclick", "Option<Callback<MouseEvent>>", "Optional callback for click events on the cell."),
                ]}
            />

            <ApiSection
                title="TableBody"
                description="Props for the TableBody component."
                props={vec![
                    ("children", "Children", "The table rows to be rendered inside the table body."),
                    ("class", "String", "Additional CSS classes to apply to the table body."),
                ]}
            />

            <ApiSection
                title="TableFooter"
                description="Props for the TableFooter component."
                props={vec![
                    ("children", "Children", "The table rows to be rendered inside the table footer."),
                    ("class", "String", "Additional CSS classes to apply to the table footer."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "The Table component and its subcomponents use classes from the BRANDGUIDE object for consistent styling.".to_string(),
                    "You can apply additional CSS classes to customize the appearance of any part of the table.".to_string(),
                    "The Table component is responsive by default, with horizontal scrolling on smaller screens.".to_string(),
                    "Use TableHead for the header row, TableBody for the main content, and TableFooter for summary or additional information.".to_string(),
                    "TableCell components can span multiple columns or rows using the 'colspan' and 'rowspan' attributes.".to_string(),
                    "For accessibility, consider using proper header cells (th) for column and row headers.".to_string(),
                    "The interactive demo demonstrates how to implement sortable columns using state and callbacks.".to_string(),
                    "You can create complex tables with nested structures, merged cells, and dynamic content.".to_string(),
                    "When working with large datasets, consider implementing pagination or virtual scrolling for better performance.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Table".to_string()}
                class_descriptions={vec![
                    ("table_container".to_string(), "For the main table container, typically includes overflow handling for responsiveness".to_string()),
                    ("table".to_string(), "For the table element, includes basic table styling such as width and borders".to_string()),
                    ("table_head".to_string(), "For the table head element, often includes different background color and font weight".to_string()),
                    ("table_row".to_string(), "For the table row element, may include hover effects or alternating background colors".to_string()),
                    ("table_cell".to_string(), "For the table cell element, includes padding, border styling, and text alignment".to_string()),
                    ("table_body".to_string(), "For the table body element, may include styles for alternating row colors".to_string()),
                    ("table_footer".to_string(), "For the table footer element, often styled similarly to the header or with a distinct appearance".to_string()),
                ]}
            />
        </Container>
    }
}
