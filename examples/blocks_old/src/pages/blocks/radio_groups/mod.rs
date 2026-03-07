use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use radio_group1::RadioGroup1;
use radio_group2::RadioGroup2;
use radio_group3::RadioGroup3;
use radio_group4::RadioGroup4;

#[function_component(RadioGroups)]
pub fn radio_groups() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Radio Groups"} />
            </Breadcrumb>
            <ExampleBlock title={"Radio Group 1"} isolate={true}>
                <RadioGroup1 />
            </ExampleBlock>
            <ExampleBlock title={"Radio Group 2"} isolate={true}>
                <RadioGroup2 />
            </ExampleBlock>
            <ExampleBlock title={"Radio Group 3"} isolate={true}>
                <RadioGroup3 />
            </ExampleBlock>
            <ExampleBlock title={"Radio Group 4"} isolate={true}>
                <RadioGroup4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod radio_group1 {
    use super::*;

    #[function_component(RadioGroup1)]
    pub fn radio_group1() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group1" class="form-radio" />
                    <span>{"Option 1"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group1" class="form-radio" />
                    <span>{"Option 2"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group1" class="form-radio" />
                    <span>{"Option 3"}</span>
                </label>
            </div>
        }
    }
}

pub mod radio_group2 {
    use super::*;

    #[function_component(RadioGroup2)]
    pub fn radio_group2() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group2" class="form-radio text-blue-600" />
                    <span>{"Option A"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group2" class="form-radio text-blue-600" />
                    <span>{"Option B"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group2" class="form-radio text-blue-600" />
                    <span>{"Option C"}</span>
                </label>
            </div>
        }
    }
}

pub mod radio_group3 {
    use super::*;

    #[function_component(RadioGroup3)]
    pub fn radio_group3() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group3" class="form-radio text-green-600" />
                    <span>{"Choice 1"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group3" class="form-radio text-green-600" />
                    <span>{"Choice 2"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group3" class="form-radio text-green-600" />
                    <span>{"Choice 3"}</span>
                </label>
            </div>
        }
    }
}

pub mod radio_group4 {
    use super::*;

    #[function_component(RadioGroup4)]
    pub fn radio_group4() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group4" class="form-radio text-red-600" />
                    <span>{"Selection 1"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group4" class="form-radio text-red-600" />
                    <span>{"Selection 2"}</span>
                </label>
                <label class="flex items-center space-x-2">
                    <input type="radio" name="group4" class="form-radio text-red-600" />
                    <span>{"Selection 3"}</span>
                </label>
            </div>
        }
    }
}
