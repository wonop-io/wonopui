use wonopui::*;
use wonopui::code_editor::CodeEditor;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExampleCodeProps {
    pub preview: Html,
    pub code: String,
    #[prop_or_default]
    pub customize: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("rust".to_string())]
    pub language: String,
}

#[function_component(ExampleCode)]
pub fn example_code(props: &ExampleCodeProps) -> Html {
    // Use a light theme for light mode and dark theme for dark mode
    let theme = "InspiredGitHub".to_string(); // This is a light theme, could be made responsive to dark mode
    
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
                <div class="rounded-md border border-zinc-200 dark:border-zinc-700 overflow-hidden" style="max-height: 500px; overflow-y: auto;">
                    <CodeEditor
                        code={props.code.clone()}
                        language={props.language.clone()}
                        theme={theme}
                        show_line_numbers={true}
                        read_only={true}
                        font_size={14}
                        line_height={1.6}
                    />
                </div>
            </TabsContent>
            if let Some(customize) = &props.customize {
                <TabsContent value="customize">
                    { customize.clone() }
                </TabsContent>
            }
        </Tabs>
    }
}
