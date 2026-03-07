use super::layout::AIDecentralisedGpuMarketLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(ApiKeyManagement)]
pub fn api_key_management() -> Html {
    let api_keys = vec![
        ("Key 1", "2023-10-01"),
        ("Key 2", "2023-10-02"),
        ("Key 3", "2023-10-03"),
    ];

    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold">{"API Key Management"}</h1>
                    <Button variant={ButtonVariant::Primary}>{"Issue New API Key"}</Button>
                </div>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell>{"API Key"}</TableCell>
                            <TableCell>{"Creation Date"}</TableCell>
                            <TableCell class="text-right">{"Actions"}</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        { for api_keys.iter().map(|(key, date)| html! {
                            <TableRow>
                                <TableCell>{*key}</TableCell>
                                <TableCell>{*date}</TableCell>
                                <TableCell class="text-right flex space-x-2 justify-end items-center">
                                    <Button variant={ButtonVariant::Ghost}>{"Edit"}</Button>
                                    <Button variant={ButtonVariant::Primary}>{"Delete"}</Button>
                                </TableCell>
                            </TableRow>
                        }) }
                    </TableBody>
                </Table>
            </div>
        </AIDecentralisedGpuMarketLayout>
    }
}
