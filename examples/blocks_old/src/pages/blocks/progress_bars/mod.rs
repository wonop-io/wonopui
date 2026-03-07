use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use progress_bar1::ProgressBar1;
use progress_bar2::ProgressBar2;
use progress_bar3::ProgressBar3;
use progress_bar4::ProgressBar4;

#[function_component(ProgressBars)]
pub fn progress_bars() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Progress Bars"} />
            </Breadcrumb>
            <ExampleBlock title={"Progress Bar 1"} isolate={true}>
                <ProgressBar1 />
            </ExampleBlock>
            <ExampleBlock title={"Progress Bar 2"} isolate={true}>
                <ProgressBar2 />
            </ExampleBlock>
            <ExampleBlock title={"Progress Bar 3"} isolate={true}>
                <ProgressBar3 />
            </ExampleBlock>
            <ExampleBlock title={"Progress Bar 4"} isolate={true}>
                <ProgressBar4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod progress_bar1 {
    use super::*;

    #[function_component(ProgressBar1)]
    pub fn progress_bar1() -> Html {
        html! {
            <div class="w-full bg-gray-200 rounded-full h-4">
                <div class="bg-blue-600 h-4 rounded-full" style="width: 25%"></div>
            </div>
        }
    }
}

pub mod progress_bar2 {
    use super::*;

    #[function_component(ProgressBar2)]
    pub fn progress_bar2() -> Html {
        html! {
            <div class="w-full bg-gray-200 rounded-full h-4">
                <div class="bg-green-600 h-4 rounded-full" style="width: 50%"></div>
            </div>
        }
    }
}

pub mod progress_bar3 {
    use super::*;

    #[function_component(ProgressBar3)]
    pub fn progress_bar3() -> Html {
        html! {
            <div class="w-full bg-gray-200 rounded-full h-4">
                <div class="bg-yellow-600 h-4 rounded-full" style="width: 75%"></div>
            </div>
        }
    }
}

pub mod progress_bar4 {
    use super::*;

    #[function_component(ProgressBar4)]
    pub fn progress_bar4() -> Html {
        html! {
            <div class="w-full bg-gray-200 rounded-full h-4">
                <div class="bg-red-600 h-4 rounded-full" style="width: 100%"></div>
            </div>
        }
    }
}
