//! Component Editor for WonopUI Gallery
//!
//! A component for displaying component demos with preview, code, and configuration options.

use yew::prelude::*;

/// Field definition for ComponentEditor
#[derive(Clone, PartialEq)]
pub struct ComponentField {
    pub name: String,
    pub value: Html,
}

#[derive(Properties, PartialEq)]
pub struct ComponentEditorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub description: String,
    #[prop_or_default]
    pub code: String,
    /// Preview content to show
    #[prop_or_default]
    pub preview: Html,
    /// Fields to display (label/value pairs) - accepts Html values
    #[prop_or_default]
    pub fields: Vec<(String, String)>,
}

/// ComponentEditor displays a component demo with preview and code
#[function_component(ComponentEditor)]
pub fn component_editor(props: &ComponentEditorProps) -> Html {
    let class = format!("space-y-4 {}", props.class.to_string());

    html! {
        <div class={class}>
            if !props.title.is_empty() {
                <h3 class="text-lg font-semibold">{ &props.title }</h3>
            }
            if !props.description.is_empty() {
                <p class="text-gray-600 dark:text-gray-400">{ &props.description }</p>
            }
            // Preview section
            <div class="border rounded-lg p-4 bg-white dark:bg-zinc-800">
                { props.preview.clone() }
                { for props.children.iter() }
            </div>
            // Fields section
            if !props.fields.is_empty() {
                <div class="grid gap-4">
                    { for props.fields.iter().map(|(label, value)| html! {
                        <div class="flex items-center gap-4">
                            <label class="font-medium text-sm min-w-32">{ label }</label>
                            <div>{ value }</div>
                        </div>
                    })}
                </div>
            }
            if !props.code.is_empty() {
                <pre class="bg-gray-100 dark:bg-zinc-900 p-4 rounded-lg overflow-x-auto text-sm">
                    <code>{ &props.code }</code>
                </pre>
            }
        </div>
    }
}
