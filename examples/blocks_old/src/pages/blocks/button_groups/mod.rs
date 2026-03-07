use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(ButtonGroups)]
pub fn button_groups() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Button Groups"} />
            </Breadcrumb>
            <ExampleBlock title={"Button Group Example 1"} isolate={true}>
                <ButtonGroupExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Button Group Example 2"} isolate={true}>
                <ButtonGroupExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Button Group Example 3"} isolate={true}>
                <ButtonGroupExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Button Group Example 4"} isolate={true}>
                <ButtonGroupExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(ButtonGroupExample1)]
pub fn button_group_example1() -> Html {
    html! {
        <div class="flex space-x-2">
            <Button variant={ButtonVariant::Primary}>{"Button 1"}</Button>
            <Button variant={ButtonVariant::Secondary}>{"Button 2"}</Button>
            <Button variant={ButtonVariant::Danger}>{"Button 3"}</Button>
        </div>
    }
}

#[function_component(ButtonGroupExample2)]
pub fn button_group_example2() -> Html {
    html! {
        <div class="flex flex-col space-y-2">
        <Button variant={ButtonVariant::Primary}>{"Button 1"}</Button>
        <Button variant={ButtonVariant::Secondary}>{"Button 2"}</Button>
        <Button variant={ButtonVariant::Danger}>{"Button 3"}</Button>
    </div>
    }
}

#[function_component(ButtonGroupExample3)]
pub fn button_group_example3() -> Html {
    html! {
        <div class="flex space-x-2">
        <Button variant={ButtonVariant::Primary}>{"Button 1"}</Button>
        <Button variant={ButtonVariant::Secondary}>{"Button 2"}</Button>
        <Button variant={ButtonVariant::Danger}>{"Button 3"}</Button>
    </div>
    }
}

#[function_component(ButtonGroupExample4)]
pub fn button_group_example4() -> Html {
    html! {
        <div class="flex space-x-2">
        <Button variant={ButtonVariant::Primary}>{"Button 1"}</Button>
        <Button variant={ButtonVariant::Secondary}>{"Button 2"}</Button>
        <Button variant={ButtonVariant::Danger}>{"Button 3"}</Button>
    </div>
    }
}
