use super::layout::AIDecentralisedGpuMarketLayout;
use wonopui::LayoutState;
use yew::prelude::*;

#[function_component(PaymentIntegration)]
pub fn payment_integration() -> Html {
    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState {  sidebar_folded: false, ..LayoutState::new() }}>
            <div class="w-full h-full bg-white flex items-center justify-center p-16">
                {"Placeholder for PaymentIntegration"}
            </div>
        </AIDecentralisedGpuMarketLayout>
    }
}
