use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
use yew::prelude::*;

#[function_component(WindowControlsDocumentation)]
pub fn window_controls_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "WindowControls Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "macOS-style window control buttons (traffic lights) for Tauri/desktop applications. Provides close, minimize, and maximize buttons." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center gap-8">
                        <div>
                            <p class="text-sm text-zinc-500 mb-2">{"Default"}</p>
                            <WindowControls />
                        </div>
                        <div>
                            <p class="text-sm text-zinc-500 mb-2">{"With callbacks"}</p>
                            <WindowControls
                                on_close={Callback::from(|_| ())}
                                on_minimize={Callback::from(|_| ())}
                                on_maximize={Callback::from(|_| ())}
                            />
                        </div>
                    </div>
                }}
                code={r#"
<WindowControls />

// With callbacks
<WindowControls
    on_close={Callback::from(|_| close_window())}
    on_minimize={Callback::from(|_| minimize_window())}
    on_maximize={Callback::from(|_| maximize_window())}
/>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Disabled Buttons" }</h2>
            <ExampleCode
                preview={html! {
                    <div class="flex items-center gap-8">
                        <div>
                            <p class="text-sm text-zinc-500 mb-2">{"Close disabled"}</p>
                            <WindowControls close_disabled={true} />
                        </div>
                        <div>
                            <p class="text-sm text-zinc-500 mb-2">{"All disabled"}</p>
                            <WindowControls
                                close_disabled={true}
                                minimize_disabled={true}
                                maximize_disabled={true}
                            />
                        </div>
                    </div>
                }}
                code={r#"
<WindowControls close_disabled={true} />

<WindowControls
    close_disabled={true}
    minimize_disabled={true}
    maximize_disabled={true}
/>"#.to_string()}
            />

            <Features features={vec![
                "macOS-style traffic light buttons",
                "Close, minimize, and maximize actions",
                "Individual button disable states",
                "Hover effects",
                "Accessible with ARIA labels"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="WindowControls"
                description="Props for the WindowControls component."
                props={vec![
                    ("on_close", "Option<Callback<()>>", "Callback when close is clicked"),
                    ("on_minimize", "Option<Callback<()>>", "Callback when minimize is clicked"),
                    ("on_maximize", "Option<Callback<()>>", "Callback when maximize is clicked"),
                    ("close_disabled", "bool", "Disable the close button. Default: false"),
                    ("minimize_disabled", "bool", "Disable the minimize button. Default: false"),
                    ("maximize_disabled", "bool", "Disable the maximize button. Default: false"),
                    ("show_icons", "bool", "Show icons on hover. Default: false"),
                    ("class", "Classes", "Additional CSS classes"),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Primarily designed for Tauri/desktop applications.".to_string(),
                    "In Tauri, use the Tauri API to implement window actions.".to_string(),
                    "Position these controls in the top-left of your app's title bar.".to_string(),
                ]}
            />
        </Container>
    }
}
