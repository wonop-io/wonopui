mod layout;

use crate::pages::example_block::ExampleBlock;
use concept_selection::ConceptSelection;
use logo_editor::LogoEditor;
use logo_selection::LogoSelection;
use wonopui::*;
use yew::prelude::*;

#[function_component(AiBasedLogoDesignerVertical)]
pub fn ai_based_logo_designer_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"AI Based Logo Designer"} />
            </Breadcrumb>
            <ExampleBlock title={"Concept Selection"}>
                <ConceptSelection />
            </ExampleBlock>
            <ExampleBlock title={"Logo Selection"}>
                <LogoSelection />
            </ExampleBlock>
            <ExampleBlock title={"Logo Editor"}>
                <LogoEditor />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod concept_selection;
pub mod logo_editor;
pub mod logo_selection;
