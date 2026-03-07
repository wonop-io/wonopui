use super::layout::AIDecentralisedGpuMarketLayout;
use wonopui::LayoutState;
use wonopui::*;
use yew::prelude::*;

#[function_component(TrainingDataCollection)]
pub fn training_data_collection() -> Html {
    let data_buckets = vec![
        ("Bucket 1", "API Key 1", 10, 2),
        ("Bucket 2", "API Key 2", 15, 5),
        ("Bucket 3", "API Key 3", 20, 3),
    ];

    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold">{"Training Data Collection"}</h1>
                    <Button variant={ButtonVariant::Primary}>{"Create New Bucket"}</Button>
                </div>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell>{"Data Bucket"}</TableCell>
                            <TableCell>{"API Key"}</TableCell>
                            <TableCell>{"Positive Examples"}</TableCell>
                            <TableCell>{"Negative Examples"}</TableCell>
                            <TableCell class="text-right">{"Actions"}</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        { for data_buckets.iter().map(|(bucket, api_key, positive, negative)| html! {
                            <TableRow>
                                <TableCell>{*bucket}</TableCell>
                                <TableCell>{*api_key}</TableCell>
                                <TableCell>{*positive}</TableCell>
                                <TableCell>{*negative}</TableCell>
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
