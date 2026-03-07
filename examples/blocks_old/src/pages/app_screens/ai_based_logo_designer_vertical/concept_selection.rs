use super::layout::AILogoDesignerLayout;
use wonopui::LayoutState;
use yew::prelude::*;

#[function_component(ConceptSelection)]
pub fn concept_selection() -> Html {
    html! {
        <AILogoDesignerLayout initial_state={LayoutState { standard_menu_size:550,  sidebar_folded: false, ..LayoutState::new() }}>
            <div class="w-full h-full bg-white flex items-center justify-center p-16">
                {"Placeholder for ConceptSelection"}
            </div>
        </AILogoDesignerLayout>
    }
}
