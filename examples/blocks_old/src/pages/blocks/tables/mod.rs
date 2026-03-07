use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Tables)]
pub fn tables() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Tables"} />
            </Breadcrumb>
            <ExampleBlock title={"Table Example 1"} isolate={true}>
                <TableExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Table Example 2"} isolate={true}>
                <TableExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Table Example 3"} isolate={true}>
                <TableExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Table Example 4"} isolate={true}>
                <TableExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(TableExample1)]
pub fn table_example1() -> Html {
    html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>{"Name"}</TableCell>
                    <TableCell>{"Age"}</TableCell>
                    <TableCell>{"Location"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                <TableRow>
                    <TableCell>{"John Doe"}</TableCell>
                    <TableCell>{"30"}</TableCell>
                    <TableCell>{"New York"}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Jane Smith"}</TableCell>
                    <TableCell>{"25"}</TableCell>
                    <TableCell>{"Los Angeles"}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Sam Johnson"}</TableCell>
                    <TableCell>{"22"}</TableCell>
                    <TableCell>{"Chicago"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}

#[function_component(TableExample2)]
pub fn table_example2() -> Html {
    html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>{"Product"}</TableCell>
                    <TableCell>{"Price"}</TableCell>
                    <TableCell>{"Stock"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                <TableRow>
                    <TableCell>{"Laptop"}</TableCell>
                    <TableCell>{"$999"}</TableCell>
                    <TableCell><Badge badge_type={BadgeType::Success} label="Available" /></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Smartphone"}</TableCell>
                    <TableCell>{"$699"}</TableCell>
                    <TableCell><Badge badge_type={BadgeType::Error} label="Out of stock" /></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Tablet"}</TableCell>
                    <TableCell>{"$499"}</TableCell>
                    <TableCell>{"Available"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}

#[function_component(TableExample3)]
pub fn table_example3() -> Html {
    html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>{"Employee"}</TableCell>
                    <TableCell>{"Department"}</TableCell>
                    <TableCell>{"Role"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                <TableRow>
                    <TableCell>{"Alice Brown"}</TableCell>
                    <TableCell>{"HR"}</TableCell>
                    <TableCell>{"Manager"}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Bob White"}</TableCell>
                    <TableCell>{"IT"}</TableCell>
                    <TableCell>{"Developer"}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Charlie Green"}</TableCell>
                    <TableCell>{"Finance"}</TableCell>
                    <TableCell>{"Analyst"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}

#[function_component(TableExample4)]
pub fn table_example4() -> Html {
    html! {
        <Table>
            <TableHead>
                <TableRow>
                    <TableCell>{"Course"}</TableCell>
                    <TableCell>{"Instructor"}</TableCell>
                    <TableCell>{"Duration"}</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                <TableRow>
                    <TableCell>{"Math 101"}</TableCell>
                    <TableCell>{"Dr. Smith"}</TableCell>
                    <TableCell>{"3 months"}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"History 201"}</TableCell>
                    <TableCell>{"Prof. Johnson"}</TableCell>
                    <TableCell>{"4 months"}</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>{"Science 301"}</TableCell>
                    <TableCell>{"Dr. Brown"}</TableCell>
                    <TableCell>{"5 months"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}
