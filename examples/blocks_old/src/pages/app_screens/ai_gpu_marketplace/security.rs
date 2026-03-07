use super::layout::AIDecentralisedGpuMarketLayout;
use wonopui::LayoutState;
use yew::prelude::*;

#[function_component(Security)]
pub fn security() -> Html {
    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState {  sidebar_folded: false, ..LayoutState::new() }}>
            <div class="w-full h-full bg-white flex items-center justify-center p-16">
                {"Placeholder for Security"}
            </div>
        </AIDecentralisedGpuMarketLayout>
    }
}
