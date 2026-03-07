use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(CardHeadings)]
pub fn card_headings() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Card Headings"} />
            </Breadcrumb>
            <ExampleBlock title={"Card Heading Example 1"} isolate={true}>
                <CardHeadingExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Card Heading Example 2"} isolate={true}>
                <CardHeadingExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Card Heading Example 3"} isolate={true}>
                <CardHeadingExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Card Heading Example 4"} isolate={true}>
                <CardHeadingExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(CardHeadingExample1)]
pub fn card_heading_example1() -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{"Card Heading 1"}</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{"This is the content of Card Heading Example 1."}</p>
            </CardContent>
        </Card>
    }
}

#[function_component(CardHeadingExample2)]
pub fn card_heading_example2() -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{"Card Heading 2"}</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{"This is the content of Card Heading Example 2."}</p>
            </CardContent>
        </Card>
    }
}

#[function_component(CardHeadingExample3)]
pub fn card_heading_example3() -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{"Card Heading 3"}</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{"This is the content of Card Heading Example 3."}</p>
            </CardContent>
        </Card>
    }
}

#[function_component(CardHeadingExample4)]
pub fn card_heading_example4() -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{"Card Heading 4"}</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{"This is the content of Card Heading Example 4."}</p>
            </CardContent>
        </Card>
    }
}
