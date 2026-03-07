use super::layout1::AppLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(InvoiceDetail)]
pub fn invoice_detail() -> Html {
    let invoice_data = (
        "INV-001",
        "John Doe",
        "2023-10-01",
        "2023-10-15",
        vec![
            ("Item 1", 2, 50.0),
            ("Item 2", 1, 100.0),
            ("Item 3", 5, 20.0),
        ],
        270.0,
    );

    html! {
        <AppLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold">{"Invoice Details"}</h1>
                    <Button variant={ButtonVariant::Primary}>{"Download PDF"}</Button>
                </div>
                <div class="bg-white shadow-md rounded p-6">
                    <div class="mb-4">
                        <h2 class="text-xl font-semibold">{"Invoice #"}</h2>
                        <p>{invoice_data.0}</p>
                    </div>
                    <div class="mb-4">
                        <h2 class="text-xl font-semibold">{"Customer"}</h2>
                        <p>{invoice_data.1}</p>
                    </div>
                    <div class="mb-4">
                        <h2 class="text-xl font-semibold">{"Invoice Date"}</h2>
                        <p>{invoice_data.2}</p>
                    </div>
                    <div class="mb-4">
                        <h2 class="text-xl font-semibold">{"Due Date"}</h2>
                        <p>{invoice_data.3}</p>
                    </div>
                    <div class="mb-4">
                        <h2 class="text-xl font-semibold">{"Items"}</h2>
                        <Table>
                            <TableHead>
                                <TableRow>
                                    <TableCell>{"Description"}</TableCell>
                                    <TableCell>{"Quantity"}</TableCell>
                                    <TableCell>{"Price"}</TableCell>
                                    <TableCell>{"Total"}</TableCell>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                { for invoice_data.4.iter().map(|(desc, qty, price)| html! {
                                    <TableRow>
                                        <TableCell>{*desc}</TableCell>
                                        <TableCell>{*qty}</TableCell>
                                        <TableCell>{format!("${:.2}", *price)}</TableCell>
                                        <TableCell>{format!("${:.2}", *qty as f64 * price)}</TableCell>
                                    </TableRow>
                                }) }
                            </TableBody>
                        </Table>
                    </div>
                    <div class="flex justify-end mt-4">
                        <div class="text-right">
                            <h2 class="text-xl font-semibold">{"Total Amount"}</h2>
                            <p class="text-2xl font-bold">{format!("${:.2}", invoice_data.5)}</p>
                        </div>
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}
