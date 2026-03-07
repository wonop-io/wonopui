use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Textareas)]
pub fn textareas() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Textareas"} />
            </Breadcrumb>
            <ExampleBlock title={"Textarea Example 1"}>
                <TextareaExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Textarea Example 2"}>
                <TextareaExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Textarea Example 3"}>
                <TextareaExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Textarea Example 4"}>
                <TextareaExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(TextareaExample1)]
pub fn textarea_example1() -> Html {
    html! {
        <div class="p-4">
            <label for="example1" class="block text-sm font-medium text-gray-700">{"Example 1"}</label>
            <textarea id="example1" name="example1" rows="4" class="mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></textarea>
        </div>
    }
}

#[function_component(TextareaExample2)]
pub fn textarea_example2() -> Html {
    html! {
        <div class="p-4">
            <label for="example2" class="block text-sm font-medium text-gray-700">{"Example 2"}</label>
            <textarea id="example2" name="example2" rows="4" class="mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></textarea>
        </div>
    }
}

#[function_component(TextareaExample3)]
pub fn textarea_example3() -> Html {
    html! {
        <div class="p-4">
            <label for="example3" class="block text-sm font-medium text-gray-700">{"Example 3"}</label>
            <textarea id="example3" name="example3" rows="4" class="mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></textarea>
        </div>
    }
}

#[function_component(TextareaExample4)]
pub fn textarea_example4() -> Html {
    html! {
        <div class="p-4">
            <label for="example4" class="block text-sm font-medium text-gray-700">{"Example 4"}</label>
            <textarea id="example4" name="example4" rows="4" class="mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"></textarea>
        </div>
    }
}
