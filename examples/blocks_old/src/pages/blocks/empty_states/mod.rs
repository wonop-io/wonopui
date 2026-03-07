use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use empty_state1::EmptyState1;
use empty_state2::EmptyState2;
use empty_state3::EmptyState3;
use empty_state4::EmptyState4;

#[function_component(EmptyStates)]
pub fn empty_states() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Empty States"} />
            </Breadcrumb>
            <ExampleBlock title={"Empty State 1"}>
                <EmptyState1 />
            </ExampleBlock>
            <ExampleBlock title={"Empty State 2"}>
                <EmptyState2 />
            </ExampleBlock>
            <ExampleBlock title={"Empty State 3"}>
                <EmptyState3 />
            </ExampleBlock>
            <ExampleBlock title={"Empty State 4"}>
                <EmptyState4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod empty_state1 {
    use super::*;

    #[function_component(EmptyState1)]
    pub fn empty_state1() -> Html {
        html! {
            <div class="flex flex-col items-center justify-center h-full p-4">
                <h2 class="text-2xl font-bold mb-4">{"No Data Available"}</h2>
                <p class="text-gray-600 mb-4">{"There is currently no data to display. Please check back later."}</p>
                <Button>{"Refresh"}</Button>
            </div>
        }
    }
}

pub mod empty_state2 {
    use super::*;

    #[function_component(EmptyState2)]
    pub fn empty_state2() -> Html {
        html! {
            <div class="flex flex-col items-center justify-center h-full p-4">
                <h2 class="text-2xl font-bold mb-4">{"No Results Found"}</h2>
                <p class="text-gray-600 mb-4">{"We couldn't find any results for your search. Try adjusting your search criteria."}</p>
                <Button>{"Try Again"}</Button>
            </div>
        }
    }
}

pub mod empty_state3 {
    use super::*;

    #[function_component(EmptyState3)]
    pub fn empty_state3() -> Html {
        html! {
            <div class="flex flex-col items-center justify-center h-full p-4">
                <h2 class="text-2xl font-bold mb-4">{"No Notifications"}</h2>
                <p class="text-gray-600 mb-4">{"You have no new notifications at this time."}</p>
                <Button>{"Check Again"}</Button>
            </div>
        }
    }
}

pub mod empty_state4 {
    use super::*;

    #[function_component(EmptyState4)]
    pub fn empty_state4() -> Html {
        html! {
            <div class="flex flex-col items-center justify-center h-full p-4">
                <h2 class="text-2xl font-bold mb-4">{"No Messages"}</h2>
                <p class="text-gray-600 mb-4">{"You have no new messages. Start a new conversation to see messages here."}</p>
                <Button>{"New Message"}</Button>
            </div>
        }
    }
}
