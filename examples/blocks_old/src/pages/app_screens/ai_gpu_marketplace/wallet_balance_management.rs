use super::layout::AIDecentralisedGpuMarketLayout;
use wonopcharts::*;
use wonopui::LayoutState;
use wonopui::*;
use yew::prelude::*;

#[function_component(WalletBalanceManagement)]
pub fn wallet_balance_management() -> Html {
    let wallet_data = (
        "WALLET-001",
        "John Doe",
        "2023-10-01",
        vec![
            ("Transaction 1", "2023-10-01", 0.5, "BTC", 25000.0),
            ("Transaction 2", "2023-10-02", 1.0, "ETH", 3000.0),
            ("Transaction 3", "2023-10-03", 2.5, "LTC", 150.0),
        ],
        4.0,
        28375.0,
    );

    let portfolio_data = vec![
        (0.0, 20000.0),
        (10.0, 22000.0),
        (20.0, 25000.0),
        (30.0, 27000.0),
        (40.0, 28375.0),
    ];

    let wallet_items = vec![
        DropdownItemProps {
            label: "WALLET-001".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "WALLET-002".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "WALLET-003".to_string(),
            ..Default::default()
        },
    ];

    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <Dropdown items={wallet_items} position={PopoverPosition::EastEnd}>
                        <Button variant={ButtonVariant::Ghost}>{wallet_data.0}</Button>
                    </Dropdown>
                    <Button variant={ButtonVariant::Danger}>{"Remove"}</Button>
                </div>
                <div class="grid gap-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3">
                    <Card>
                        <CardHeader>
                            <h3 class="tracking-tight text-sm font-medium">{"Receive Tokens"}</h3>
                        </CardHeader>
                        <CardContent>
                            <Button variant={ButtonVariant::Primary}>{"Generate Address"}</Button>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <h3 class="tracking-tight text-sm font-medium">{"Send Tokens"}</h3>
                        </CardHeader>
                        <CardContent>
                            <Button variant={ButtonVariant::Primary}>{"Send"}</Button>
                        </CardContent>
                    </Card>
                    <Card class="sm:col-span-2 md:col-span-1">
                        <CardHeader>
                            <h3 class="tracking-tight text-sm font-medium">{"Portfolio Overview"}</h3>
                        </CardHeader>
                        <CardContent>
                            <div class="text-lg font-bold">{format!("${:.2}", wallet_data.5)}</div>
                        </CardContent>
                    </Card>

                    <Card class="sm:col-span-2 md:col-span-3">
                        <CardHeader>
                            <h3 class="tracking-tight text-sm font-medium">{"Portfolio Overview"}</h3>
                        </CardHeader>
                        <CardContent>
                            <Chart x={(0.0, 50.0)} y={(0.0, 30000.0)} height={300} width={500} padding={50}>
                                <XAxis show={true} ticks={6} />
                                <YAxis show={true} />
                                <BarChart data={portfolio_data} width={30} />
                            </Chart>
                        </CardContent>
                    </Card>
                </div>

                <div class="mt-4">
                    <H2>{"Transactions"}</H2>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableCell>{"Description"}</TableCell>
                                <TableCell>{"Date"}</TableCell>
                                <TableCell>{"Amount"}</TableCell>
                                <TableCell>{"Currency"}</TableCell>
                                <TableCell>{"Value (USD)"}</TableCell>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            { for wallet_data.3.iter().map(|(desc, date, amount, currency, value)| html! {
                                <TableRow>
                                    <TableCell>{*desc}</TableCell>
                                    <TableCell>{*date}</TableCell>
                                    <TableCell>{format!("{:.2}", *amount)}</TableCell>
                                    <TableCell>{*currency}</TableCell>
                                    <TableCell>{format!("${:.2}", *value)}</TableCell>
                                </TableRow>
                            }) }
                        </TableBody>
                    </Table>
                </div>
            </div>
        </AIDecentralisedGpuMarketLayout>
    }
}
