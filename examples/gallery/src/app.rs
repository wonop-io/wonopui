use crate::app_layout::AppLayout;
use crate::components::*;
use crate::routes::{AppRoute, Route};
use gloo_console as console;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use wonopui::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(GettingStarted)]
pub fn getting_started() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "Getting Started with Wonop UI" }</H1>
            <Paragraph>{ "Wonop UI is a parameterized UI framework that leverages Tailwind CSS for use with the Yew framework in Rust." }</Paragraph>
            <H2>{ "Installation" }</H2>
            <Paragraph>{ "To use Wonop UI in your Yew project, add the following to your Cargo.toml:" }</Paragraph>
            <pre>
                { "[dependencies]\nwonopui = { git = \"https://github.com/wonop-io/wonopui.git\", branch=\"main\" }" }
            </pre>
            <H2>{ "Initializing Tailwind CSS" }</H2>
            <Paragraph>{ "Run the following command to initialize Tailwind CSS:" }</Paragraph>
            <pre>{ "npx tailwindcss init" }</pre>
            <Paragraph>{ "Add the following to your tailwind.config.js:" }</Paragraph>
            <pre>
                { "module.exports = {\n  content: [\n    \"./src/**/*.rs\",\n    \"./target/wonopui.json\",\n    \"./target/tailwindcss.txt\",\n    \"./target/**/wonopui.json\",\n    \"./target/**/tailwindcss.txt\",    \n  ],\n  theme: {\n    extend: {},\n  },\n  plugins: [],\n};" }
            </pre>
            <H2>{ "Adding Tailwind CSS to your HTML" }</H2>
            <Paragraph>{ "Add the following to your index.html:" }</Paragraph>
            <pre>{ "<link data-trunk rel=\"tailwind-css\" href=\"tailwind.css\" />" }</pre>
            <Paragraph>{ "With these steps completed, you're ready to start using Wonop UI in your Yew project!" }</Paragraph>
        </Container>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let render = move |routes| match routes {
        Route::GettingStarted => {
            html! { <GettingStarted /> }
        }
        Route::AccordionExample => {
            html! { <AccordionDocumentation /> }
        }
        Route::AlertExample => {
            html! { <AlertDocumentation /> }
        }
        Route::AvatarExample => {
            html! { <AvatarDocumentation /> }
        }
        Route::BadgeExample => {
            html! { <BadgeDocumentation /> }
        }
        Route::BreadcrumbExample => {
            html! { <BreadcrumbDocumentation /> }
        }
        Route::ButtonExample => {
            html! { <ButtonDocumentation /> }
        }
        Route::CalendarExample => {
            html! { <CalendarDocumentation /> }
        }
        Route::CardExample => {
            html! { <CardDocumentation /> }
        }
        Route::CarouselExample => {
            html! { <CarouselDocumentation /> }
        }
        Route::CheckboxExample => {
            html! { <CheckboxDocumentation /> }
        }
        Route::CodeEditorExample => {
            html! { <CodeEditorDocumentation /> }
        }
        Route::ColExample => {
            html! { <ColDocumentation /> }
        }
        Route::CollapsibleExample => {
            html! { <CollapsibleDocumentation /> }
        }
        Route::ColorPickerExample => {
            html! { <ColorPickerDocumentation /> }
        }
        Route::ComboboxExample => {
            html! { <ComboboxDocumentation /> }
        }
        Route::CommandExample => {
            html! { <CommandDocumentation /> }
        }
        Route::ContainerExample => {
            html! { <ContainerDocumentation /> }
        }
        Route::ContentExample => {
            html! { <ContentDocumentation /> }
        }
        Route::ContextMenuExample => {
            html! { <ContextMenuDocumentation /> }
        }
        Route::CopyButtonExample => {
            html! { <CopyButtonDocumentation /> }
        }
        Route::DataTableExample => {
            html! { <DataTableDocumentation /> }
        }
        Route::DatePickerExample => {
            html! { <DatePickerDocumentation /> }
        }
        Route::DialogExample => {
            html! { <DialogDocumentation /> }
        }
        Route::DividerExample => {
            html! { <DividerDocumentation /> }
        }
        Route::DragPointExample => {
            html! { <DragPointDocumentation /> }
        }
        Route::DrawerExample => {
            html! { <DrawerDocumentation /> }
        }
        Route::DropdownExample => {
            html! { <DropdownDocumentation /> }
        }
        Route::GroupButtonExample => {
            html! { <GroupButtonDocumentation /> }
        }
        Route::IframeExample => {
            html! { <IframeDocumentation /> }
        }
        Route::InputExample => {
            html! { <InputDocumentation /> }
        }
        Route::LabelExample => {
            html! { <LabelDocumentation /> }
        }
        Route::MediaQueryExample => {
            html! { <MediaQueryDocumentation /> }
        }
        Route::MulticolSidebarExample => {
            html! { <MulticolSidebarDocumentation /> }
        }
        Route::NotificationExample => {
            html! { <NotificationDocumentation /> }
        }
        Route::PageContentExample => {
            html! { <PageContentDocumentation /> }
        }
        Route::PageHeaderExample => {
            html! { <PageHeaderDocumentation /> }
        }
        Route::PaginationExample => {
            html! { <PaginationDocumentation /> }
        }
        Route::PaintCanvasExample => {
            html! { <PaintCanvasDocumentation /> }
        }
        Route::PlaceholderExample => {
            html! { <PlaceholderDocumentation /> }
        }
        Route::PopoverExample => {
            html! { <PopoverDocumentation /> }
        }
        Route::ResizableExample => {
            html! { <ResizableDocumentation /> }
        }
        Route::SelectExample => {
            html! { <SelectDocumentation /> }
        }
        Route::SelectableExample => {
            html! { <SelectableDocumentation /> }
        }
        Route::SidebarExample => {
            html! { <SidebarDocumentation /> }
        }
        Route::SwitchExample => {
            html! { <SwitchDocumentation /> }
        }
        Route::TableExample => {
            html! { <TableDocumentation /> }
        }
        Route::TabsExample => {
            html! { <TabsDocumentation /> }
        }
        Route::TagInputExample => {
            html! { <TagInputDocumentation /> }
        }
        Route::TailwindColorPickerExample => {
            html! { <TailwindColorPickerDocumentation /> }
        }
        Route::TextareaExample => {
            html! { <TextareaDocumentation /> }
        }
        Route::ToggleExample => {
            html! { <ToggleDocumentation /> }
        }
        Route::TopbarExample => {
            html! { <TopbarDocumentation /> }
        }
        Route::TypographyExample => {
            html! { <TypographyDocumentation /> }
        }
        Route::WindowProviderExample => {
            html! { <WindowProviderDocumentation /> }
        }
        Route::KanbanExample => {
            html! { <KanbanDocumentation /> }
        }
    };

    return html! {
    <ThemeProvider>
        <BrowserRouter>
            <AppLayout>
                <Switch<Route> render={render} />
            </AppLayout>
        </BrowserRouter>
    </ThemeProvider>
    };
}