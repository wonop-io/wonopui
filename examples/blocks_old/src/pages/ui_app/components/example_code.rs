use wonopui::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExampleCodeProps {
    pub preview: Html,
    pub code: String,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ExampleCode)]
pub fn example_code(props: &ExampleCodeProps) -> Html {
    html! {
        <Tabs default_value="preview" class={classes!(props.class.clone(), "w-full", "mb-8")}>
            <TabsList>
                <TabsTrigger value="preview">{ "Preview" }</TabsTrigger>
                <TabsTrigger value="code">{ "Code" }</TabsTrigger>
            </TabsList>
            <TabsContent value="preview">
                { props.preview.clone() }
            </TabsContent>
            <TabsContent value="code">
                <pre class="font-mono p-4 rounded overflow-x-auto overflow-y-auto" style="max-height: 300px;">
                    <code>{ &props.code }</code>
                </pre>
            </TabsContent>
        </Tabs>
    }
}
