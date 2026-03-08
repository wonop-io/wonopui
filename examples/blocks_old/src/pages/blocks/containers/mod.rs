use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use container1::Container1;
use container2::Container2;
use container3::Container3;
use container4::Container4;

#[function_component(Containers)]
pub fn containers() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Containers"} />
            </Breadcrumb>
            <ExampleBlock title={"Container 1"}>
                <Container1 />
            </ExampleBlock>
            <ExampleBlock title={"Container 2"}>
                <Container2 />
            </ExampleBlock>
            <ExampleBlock title={"Container 3"}>
                <Container3 />
            </ExampleBlock>
            <ExampleBlock title={"Container 4"}>
                <Container4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod container1 {
    use super::*;

    #[function_component(Container1)]
    pub fn container1() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Container 1"}</h2>
                <p>{"This is the first container example."}</p>
                <Button>{"Click Me"}</Button>
            </div>
        }
    }
}

pub mod container2 {
    use super::*;

    #[function_component(Container2)]
    pub fn container2() -> Html {
        let options = vec![
            ("1".to_string(), "Option 1".to_string()),
            ("2".to_string(), "Option 2".to_string()),
            ("3".to_string(), "Option 3".to_string()),
        ];
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Container 2"}</h2>
                <p>{"This is the second container example."}</p>
                <Combobox id="combobox2" options={options} on_select={Callback::from(|_| {})} disabled={false} />
            </div>
        }
    }
}

pub mod container3 {
    use super::*;

    #[function_component(Container3)]
    pub fn container3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Container 3"}</h2>
                <p>{"This is the third container example."}</p>
                <Checkbox id="option1" checked={false} on_toggle={Callback::from(|_| {})} />
                <label for="option1" class="checkbox-label">{"Option 1"}</label>
                <Checkbox id="option2" checked={false} on_toggle={Callback::from(|_| {})} />
                <label for="option2" class="checkbox-label">{"Option 2"}</label>
                <Checkbox id="option3" checked={false} on_toggle={Callback::from(|_| {})} />
                <label for="option3" class="checkbox-label">{"Option 3"}</label>
            </div>
        }
    }
}

pub mod container4 {
    use super::*;

    #[function_component(Container4)]
    pub fn container4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Container 4"}</h2>
                <p>{"This is the fourth container example."}</p>
                <Command options={vec![
                    ("1".to_string(), "Command 1".to_string(), None),
                    ("2".to_string(), "Command 2".to_string(), None),
                    ("3".to_string(), "Command 3".to_string(), None)
                ]} />
            </div>
        }
    }
}
