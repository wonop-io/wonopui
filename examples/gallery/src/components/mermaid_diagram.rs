use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(MermaidDiagramDocumentation)]
pub fn mermaid_diagram_documentation() -> Html {
    let flowchart_code = r#"graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[OK]
    B -->|No| D[Cancel]"#;

    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "MermaidDiagram Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "Renders Mermaid.js diagrams from code. Supports flowcharts, sequence diagrams, and more." }</p>

            <div class="mb-6 p-4 bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg">
                <p class="text-amber-800 dark:text-amber-200 text-sm">
                    <strong>{"Note:"}</strong>{" This component requires Mermaid.js to be loaded in your HTML."}
                </p>
            </div>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="border rounded-lg p-4 bg-zinc-50 dark:bg-zinc-800">
                        <MermaidDiagram code={flowchart_code.to_string()} />
                    </div>
                }}
                code={r##"let code = r#"graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[OK]
    B -->|No| D[Cancel]"#;

<MermaidDiagram code={code.to_string()} />"##.to_string()}
            />

            <Features features={vec![
                "Supports all Mermaid diagram types",
                "Theme customization",
                "Automatic rendering on code change",
                "Error handling with fallback"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="MermaidDiagram"
                description="Props for the MermaidDiagram component."
                props={vec![
                    ("code", "String", "The Mermaid diagram code"),
                    ("id", "Option<String>", "Unique ID for the diagram instance"),
                    ("theme", "String", "Mermaid theme: default, dark, forest, neutral. Default: 'default'"),
                    ("class", "Classes", "Additional CSS classes"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Mermaid.js must be loaded in your page for diagrams to render.".to_string(),
                    "The component re-renders when the code prop changes.".to_string(),
                    "Use unique IDs when rendering multiple diagrams.".to_string(),
                ]}
            />
        </Container>
    }
}
