use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Comboboxes)]
pub fn comboboxes() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Comboboxes"} />
            </Breadcrumb>
            <ExampleBlock title={"Combobox Example 1"} isolate={true}>
                <ComboboxExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Combobox Example 2"} isolate={true}>
                <ComboboxExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Combobox Example 3"} isolate={true}>
                <ComboboxExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Combobox Example 4"} isolate={true}>
                <ComboboxExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(ComboboxExample1)]
pub fn combobox_example1() -> Html {
    let options = vec![
        ("1".to_string(), "Option 1".to_string()),
        ("2".to_string(), "Option 2".to_string()),
        ("3".to_string(), "Option 3".to_string()),
    ];
    html! {
        <Combobox id="combobox1" options={options} on_select={Callback::from(|_| {})} disabled={false} />
    }
}

#[function_component(ComboboxExample2)]
pub fn combobox_example2() -> Html {
    let options = vec![
        ("apple".to_string(), "Apple".to_string()),
        ("banana".to_string(), "Banana".to_string()),
        ("cherry".to_string(), "Cherry".to_string()),
    ];
    html! {
        <Combobox id="combobox2" options={options} on_select={Callback::from(|_| {})} disabled={false} />
    }
}

#[function_component(ComboboxExample3)]
pub fn combobox_example3() -> Html {
    let options = vec![
        ("red".to_string(), "Red".to_string()),
        ("green".to_string(), "Green".to_string()),
        ("blue".to_string(), "Blue".to_string()),
    ];
    html! {
        <Combobox id="combobox3" options={options} on_select={Callback::from(|_| {})} disabled={false} />
    }
}

#[function_component(ComboboxExample4)]
pub fn combobox_example4() -> Html {
    let options = vec![
        ("usa".to_string(), "USA".to_string()),
        ("canada".to_string(), "Canada".to_string()),
        ("mexico".to_string(), "Mexico".to_string()),
    ];
    html! {
        <Combobox id="combobox4" options={options} on_select={Callback::from(|_| {})} disabled={false} />
    }
}
