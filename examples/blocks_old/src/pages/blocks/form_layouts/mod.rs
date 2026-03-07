use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use form_layout1::FormLayout1;
use form_layout2::FormLayout2;
use form_layout3::FormLayout3;
use form_layout4::FormLayout4;

#[function_component(FormLayouts)]
pub fn form_layouts() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Form Layouts"} />
            </Breadcrumb>
            <ExampleBlock title={"Form Layout 1"} isolate={true}>
                <FormLayout1 />
            </ExampleBlock>
            <ExampleBlock title={"Form Layout 2"} isolate={true}>
                <FormLayout2 />
            </ExampleBlock>
            <ExampleBlock title={"Form Layout 3"} isolate={true}>
                <FormLayout3 />
            </ExampleBlock>
            <ExampleBlock title={"Form Layout 4"} isolate={true}>
                <FormLayout4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod form_layout1 {
    use super::*;

    #[function_component(FormLayout1)]
    pub fn form_layout1() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Form Layout 1"}</h2>
                <form class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Name"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your name" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Email"}</label>
                        <input type="email" class="mt-1 block w-full border rounded p-2" placeholder="Enter your email" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Password"}</label>
                        <input type="password" class="mt-1 block w-full border rounded p-2" placeholder="Enter your password" />
                    </div>
                    <Button>{"Submit"}</Button>
                </form>
            </div>
        }
    }
}

pub mod form_layout2 {
    use super::*;

    #[function_component(FormLayout2)]
    pub fn form_layout2() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Form Layout 2"}</h2>
                <form class="space-y-4">
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
                        <label class="block text-sm font-medium text-gray-700">{"Email"}</label>
                        <input type="email" class="mt-1 block w-full border rounded p-2" placeholder="Enter your email" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Phone Number"}</label>
                        <input type="tel" class="mt-1 block w-full border rounded p-2" placeholder="Enter your phone number" />
                    </div>
                    <Button>{"Submit"}</Button>
                </form>
            </div>
        }
    }
}

pub mod form_layout3 {
    use super::*;

    #[function_component(FormLayout3)]
    pub fn form_layout3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Form Layout 3"}</h2>
                <form class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Username"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your username" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Email"}</label>
                        <input type="email" class="mt-1 block w-full border rounded p-2" placeholder="Enter your email" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Password"}</label>
                        <input type="password" class="mt-1 block w-full border rounded p-2" placeholder="Enter your password" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Confirm Password"}</label>
                        <input type="password" class="mt-1 block w-full border rounded p-2" placeholder="Confirm your password" />
                    </div>
                    <Button>{"Register"}</Button>
                </form>
            </div>
        }
    }
}

pub mod form_layout4 {
    use super::*;

    #[function_component(FormLayout4)]
    pub fn form_layout4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold mb-4">{"Form Layout 4"}</h2>
                <form class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Full Name"}</label>
                        <input type="text" class="mt-1 block w-full border rounded p-2" placeholder="Enter your full name" />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">{"Email"}</label>
                        <input type="email" class="mt-1 block w-full border rounded p-2" placeholder="Enter your email" />
                    </div>
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
                </form>
            </div>
        }
    }
}
