use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Calendars)]
pub fn calendars() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Calendars"} />
            </Breadcrumb>
            <ExampleBlock title={"Calendar Example 1"} isolate={true}>
                <CalendarExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Calendar Example 2"} isolate={true}>
                <CalendarExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Calendar Example 3"} isolate={true}>
                <CalendarExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Calendar Example 4"} isolate={true}>
                <CalendarExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(CalendarExample1)]
pub fn calendar_example1() -> Html {
    html! {
        <Calendar year={2023} month={1} />
    }
}

#[function_component(CalendarExample2)]
pub fn calendar_example2() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <Calendar year={2023} month={2} />
        </div>
    }
}

#[function_component(CalendarExample3)]
pub fn calendar_example3() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <Calendar year={2023} month={3} />
        </div>
    }
}

#[function_component(CalendarExample4)]
pub fn calendar_example4() -> Html {
    html! {
        <div class="p-4 border rounded shadow">
            <Calendar year={2023} month={4} />
        </div>
    }
}
