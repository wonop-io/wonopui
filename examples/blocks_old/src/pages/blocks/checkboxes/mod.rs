use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Checkboxes)]
pub fn checkboxes() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Checkboxes"} />
            </Breadcrumb>
            <ExampleBlock title={"Checkbox Example 1"} isolate={true}>
                <CheckboxExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Checkbox Example 2"} isolate={true}>
                <CheckboxExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Checkbox Example 3"} isolate={true}>
                <CheckboxExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Checkbox Example 4"} isolate={true}>
                <CheckboxExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(CheckboxExample1)]
pub fn checkbox_example1() -> Html {
    html! {
        <div class="space-x-2">
            <Checkbox id="option1" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="option1" class="checkbox-label">{"Option 1"}</label>
            <Checkbox id="option2" checked={true} on_toggle={Callback::from(|_| {})} />
            <label for="option2" class="checkbox-label">{"Option 2"}</label>
            <Checkbox id="option3" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="option3" class="checkbox-label">{"Option 3"}</label>
        </div>
    }
}

#[function_component(CheckboxExample2)]
pub fn checkbox_example2() -> Html {
    html! {
        <div class="space-x-2">
            <Checkbox id="terms" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="terms" class="checkbox-label">{"Accept Terms and Conditions"}</label>
        </div>
    }
}

#[function_component(CheckboxExample3)]
pub fn checkbox_example3() -> Html {
    html! {
        <div class="space-x-2">
            <Checkbox id="newsletter" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="newsletter" class="checkbox-label">{"Subscribe to newsletter"}</label>
            <Checkbox id="promos" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="promos" class="checkbox-label">{"Receive promotional emails"}</label>
        </div>
    }
}

#[function_component(CheckboxExample4)]
pub fn checkbox_example4() -> Html {
    html! {
        <div class="space-x-2">
            <Checkbox id="notifications" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="notifications" class="checkbox-label">{"Enable notifications"}</label>
            <Checkbox id="darkmode" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="darkmode" class="checkbox-label">{"Enable dark mode"}</label>
            <Checkbox id="autoupdates" checked={false} on_toggle={Callback::from(|_| {})} />
            <label for="autoupdates" class="checkbox-label">{"Enable auto-updates"}</label>
        </div>
    }
}
