use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use input_group1::InputGroup1;
use input_group2::InputGroup2;
use input_group3::InputGroup3;
use input_group4::InputGroup4;

#[function_component(InputGroups)]
pub fn input_groups() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Input Groups"} />
            </Breadcrumb>
            <ExampleBlock title={"Input Group 1"}>
                <InputGroup1 />
            </ExampleBlock>
            <ExampleBlock title={"Input Group 2"}>
                <InputGroup2 />
            </ExampleBlock>
            <ExampleBlock title={"Input Group 3"}>
                <InputGroup3 />
            </ExampleBlock>
            <ExampleBlock title={"Input Group 4"}>
                <InputGroup4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod input_group1 {
    use super::*;

    #[function_component(InputGroup1)]
    pub fn input_group1() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Input Group 1"}</h2>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Username"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your username" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Email"}</label>
                        <input type="email" class="mt-1 block w-full border rounded p-2" placeholder="Enter your email" />
                    </div>
                    <Button>{"Submit"}</Button>
                </div>
            </div>
        }
    }
}

pub mod input_group2 {
    use super::*;

    #[function_component(InputGroup2)]
    pub fn input_group2() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Input Group 2"}</h2>
                <div class="space-y-4">
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{"First Name"}</label>
                            <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="First name" />
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{"Last Name"}</label>
                            <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Last name" />
                        </div>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Phone Number"}</label>
                        <input type="tel" class="mt-1 block w-full border rounded p-2" placeholder="Enter your phone number" />
                    </div>
                    <Button>{"Submit"}</Button>
                </div>
            </div>
        }
    }
}

pub mod input_group3 {
    use super::*;

    #[function_component(InputGroup3)]
    pub fn input_group3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Input Group 3"}</h2>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Address"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your address" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"City"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your city" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Postal Code"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your postal code" />
                    </div>
                    <Button>{"Submit"}</Button>
                </div>
            </div>
        }
    }
}

pub mod input_group4 {
    use super::*;

    #[function_component(InputGroup4)]
    pub fn input_group4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Input Group 4"}</h2>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Full Name"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your full name" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Email"}</label>
                        <input type="email" class="mt-1 block w-full border rounded p-2" placeholder="Enter your email" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Password"}</label>
                        <input type="password" class="mt-1 block w-full border rounded p-2" placeholder="Enter your password" />
                    </div>
                    <Button>{"Register"}</Button>
                </div>
            </div>
        }
    }
}
