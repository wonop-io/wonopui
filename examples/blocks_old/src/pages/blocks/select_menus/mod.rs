use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use select_menu1::SelectMenu1;
use select_menu2::SelectMenu2;
use select_menu3::SelectMenu3;
use select_menu4::SelectMenu4;

#[function_component(SelectMenus)]
pub fn select_menus() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Select Menus"} />
            </Breadcrumb>
            <ExampleBlock title={"Select Menu 1"}>
                <SelectMenu1 />
            </ExampleBlock>
            <ExampleBlock title={"Select Menu 2"}>
                <SelectMenu2 />
            </ExampleBlock>
            <ExampleBlock title={"Select Menu 3"}>
                <SelectMenu3 />
            </ExampleBlock>
            <ExampleBlock title={"Select Menu 4"}>
                <SelectMenu4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod select_menu1 {
    use super::*;

    #[function_component(SelectMenu1)]
    pub fn select_menu1() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="block">
                    <span class="text-gray-700">{"Select an option"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"Option 1"}</option>
                        <option>{"Option 2"}</option>
                        <option>{"Option 3"}</option>
                    </select>
                </label>
            </div>
        }
    }
}

pub mod select_menu2 {
    use super::*;

    #[function_component(SelectMenu2)]
    pub fn select_menu2() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="block">
                    <span class="text-gray-700">{"Choose a number"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"One"}</option>
                        <option>{"Two"}</option>
                        <option>{"Three"}</option>
                    </select>
                </label>
            </div>
        }
    }
}

pub mod select_menu3 {
    use super::*;

    #[function_component(SelectMenu3)]
    pub fn select_menu3() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="block">
                    <span class="text-gray-700">{"Pick a fruit"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"Apple"}</option>
                        <option>{"Banana"}</option>
                        <option>{"Cherry"}</option>
                    </select>
                </label>
            </div>
        }
    }
}

pub mod select_menu4 {
    use super::*;

    #[function_component(SelectMenu4)]
    pub fn select_menu4() -> Html {
        html! {
            <div class="flex flex-col space-y-2">
                <label class="block">
                    <span class="text-gray-700">{"Select a color"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"Red"}</option>
                        <option>{"Green"}</option>
                        <option>{"Blue"}</option>
                    </select>
                </label>
            </div>
        }
    }
}
