use super::layout::AILogoDesignerLayout;
use wonopui::LayoutState;
use yew::prelude::*;

#[function_component(LogoEditor)]
pub fn logo_editor() -> Html {
    html! {
        <AILogoDesignerLayout initial_state={LayoutState { standard_menu_size:550,  sidebar_folded: false, ..LayoutState::new() }}>
            <div class="w-full h-full bg-white flex items-center justify-center p-16">
                <div class="flex flex-col items-center">
                    <div class="w-8 h-8 border-4"></div>
                    <div class="bg-white shadow-none" data-testid="editor-canvas-background"></div>
                    <div data-testid="editor-canvas-artboard">
                        <svg width="300" height="305" viewBox="0 0 312.5 318.16"></svg>
                    </div>
                </div>
            </div>
        </AILogoDesignerLayout>
    }
}
