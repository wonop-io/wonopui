use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(SectionHeadings)]
pub fn section_headings() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Section Headings"} />
            </Breadcrumb>
            <ExampleBlock title={"Section Heading 1"}>
                <SectionHeading1 />
            </ExampleBlock>
            <ExampleBlock title={"Section Heading 2"}>
                <SectionHeading2 />
            </ExampleBlock>
            <ExampleBlock title={"Section Heading 3"}>
                <SectionHeading3 />
            </ExampleBlock>
            <ExampleBlock title={"Section Heading 4"}>
                <SectionHeading4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(SectionHeading1)]
pub fn section_heading1() -> Html {
    html! {
        <div class="text-2xl font-bold">
            {"Section Heading 1"}
        </div>
    }
}

#[function_component(SectionHeading2)]
pub fn section_heading2() -> Html {
    html! {
        <div class="text-2xl font-bold text-blue-600">
            {"Section Heading 2"}
        </div>
    }
}

#[function_component(SectionHeading3)]
pub fn section_heading3() -> Html {
    html! {
        <div class="text-2xl font-bold text-center">
            {"Section Heading 3"}
        </div>
    }
}

#[function_component(SectionHeading4)]
pub fn section_heading4() -> Html {
    html! {
        <div class="text-2xl font-bold text-right">
            {"Section Heading 4"}
        </div>
    }
}
