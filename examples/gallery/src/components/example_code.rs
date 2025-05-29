use wonopui::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExampleCodeProps {
    pub preview: Html,
    pub code: String,
    #[prop_or_default]
    pub customize: Option<Html>,
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
                if props.customize.is_some() {
                    <TabsTrigger value="customize">{ "Customize" }</TabsTrigger>
                }
            </TabsList>
            <TabsContent value="preview">
                { props.preview.clone() }
            </TabsContent>
            <TabsContent value="code">
                <pre class="font-mono p-4 rounded overflow-x-auto overflow-y-auto" style="max-height: 300px;">
                    <code>{ &props.code }</code>
                </pre>
            </TabsContent>
            if let Some(customize) = &props.customize {
                <TabsContent value="customize">
                    { customize.clone() }
                </TabsContent>
            }
        </Tabs>
    }
}
