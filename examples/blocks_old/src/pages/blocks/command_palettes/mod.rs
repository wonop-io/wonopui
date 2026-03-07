use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(CommandPalettes)]
pub fn command_palettes() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Command Palettes"} />
            </Breadcrumb>
            <ExampleBlock title={"Command Palette Example 1"} isolate={true}>
                <CommandPaletteExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Command Palette Example 2"} isolate={true}>
                <CommandPaletteExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Command Palette Example 3"} isolate={true}>
                <CommandPaletteExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Command Palette Example 4"} isolate={true}>
                <CommandPaletteExample4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(CommandPaletteExample1)]
pub fn command_palette_example1() -> Html {
    html! {

            <Command options={vec![
                ("1".to_string(), "Option 1".to_string(), None),
                ("2".to_string(), "Option 2".to_string(), None),
                ("3".to_string(), "Option 3".to_string(), None)
            ]} />
    }
}

#[function_component(CommandPaletteExample2)]
pub fn command_palette_example2() -> Html {
    html! {
            <Command options={vec![
                ("search_files".to_string(), "Search Files".to_string(), None),
                ("search_settings".to_string(), "Search Settings".to_string(), None),
                ("search_help".to_string(), "Search Help".to_string(), None)
            ]} />
    }
}

#[function_component(CommandPaletteExample3)]
pub fn command_palette_example3() -> Html {
    html! {
            <Command options={vec![
                ("run_build".to_string(), "Run Build".to_string(), None),
                ("run_tests".to_string(), "Run Tests".to_string(), None),
                ("run_linter".to_string(), "Run Linter".to_string(), None)
            ]} />
    }
}

#[function_component(CommandPaletteExample4)]
pub fn command_palette_example4() -> Html {
    html! {
            <Command options={vec![
                ("open_dashboard".to_string(), "Open Dashboard".to_string(), None),
                ("open_profile".to_string(), "Open Profile".to_string(), None),
                ("open_settings".to_string(), "Open Settings".to_string(), None)
            ]} />
    }
}
