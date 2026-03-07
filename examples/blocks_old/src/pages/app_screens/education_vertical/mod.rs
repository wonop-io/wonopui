use crate::pages::example_block::ExampleBlock;
use yew::prelude::*;

use assignment_submission::AssignmentSubmission;
use course_catalog::CourseCatalog;
use gradebook::Gradebook;
use wonopui::*;

#[function_component(EducationVertical)]
pub fn education_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Education"} />
            </Breadcrumb>

            <ExampleBlock title={"Course Catalog"}>
                <CourseCatalog />
            </ExampleBlock>
            <ExampleBlock title={"Assignment Submission"}>
                <AssignmentSubmission />
            </ExampleBlock>
            <ExampleBlock title={"Gradebook"}>
                <Gradebook />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod assignment_submission;
pub mod course_catalog;
pub mod gradebook;
