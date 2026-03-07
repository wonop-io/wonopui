use super::layout::AIDecentralisedGpuMarketLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(BackendRouting)]
pub fn backend_routing() -> Html {
    let backends = vec![
        ("Backend 1", "API Key 1"),
        ("Backend 2", "API Key 2"),
        ("Backend 3", "API Key 3"),
    ];

    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold">{"Backend Routing"}</h1>
                    <Button variant={ButtonVariant::Primary}>{"Add New Backend"}</Button>
                </div>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell>{"Backend"}</TableCell>
                            <TableCell>{"API Key"}</TableCell>
                            <TableCell class="text-right">{"Actions"}</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        { for backends.iter().map(|(backend, api_key)| html! {
                            <TableRow>
                                <TableCell>{*backend}</TableCell>
                                <TableCell>{*api_key}</TableCell>
                                <TableCell class="text-right">
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
