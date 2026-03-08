use crate::app_layout::AppLayout;
use crate::blocks;
use crate::components::*;
use crate::home::Home;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use wonopui::ThemeProvider;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Accordion => html! { <AccordionDocumentation /> },
        Route::Alert => html! { <AlertDocumentation /> },
        Route::Avatar => html! { <AvatarDocumentation /> },
        Route::Badge => html! { <BadgeDocumentation /> },
        Route::Breadcrumb => html! { <BreadcrumbDocumentation /> },
        Route::Button => html! { <ButtonDocumentation /> },
        Route::Calendar => html! { <CalendarDocumentation /> },
        Route::Card => html! { <CardDocumentation /> },
        Route::Carousel => html! { <CarouselDocumentation /> },
        Route::Checkbox => html! { <CheckboxDocumentation /> },
        Route::CodeEditor => html! { <CodeEditorDocumentation /> },
        Route::Col => html! { <ColDocumentation /> },
        Route::Collapsible => html! { <CollapsibleDocumentation /> },
        Route::ColorPicker => html! { <ColorPickerDocumentation /> },
        Route::Combobox => html! { <ComboboxDocumentation /> },
        Route::Command => html! { <CommandDocumentation /> },
        Route::ConfirmDialog => html! { <ConfirmDialogDocumentation /> },
        Route::Container => html! { <ContainerDocumentation /> },
        Route::Content => html! { <ContentDocumentation /> },
        Route::ContextMenu => html! { <ContextMenuDocumentation /> },
        Route::CopyButton => html! { <CopyButtonDocumentation /> },
        Route::DataTable => html! { <DataTableDocumentation /> },
        Route::DatePicker => html! { <DatePickerDocumentation /> },
        Route::Dialog => html! { <DialogDocumentation /> },
        Route::DiffView => html! { <DiffViewDocumentation /> },
        Route::Divider => html! { <DividerDocumentation /> },
        Route::DragPoint => html! { <DragPointDocumentation /> },
        Route::Drawer => html! { <DrawerDocumentation /> },
        Route::Dropdown => html! { <DropdownDocumentation /> },
        Route::EmptyState => html! { <EmptyStateDocumentation /> },
        Route::ErrorBoundary => html! { <ErrorBoundaryDocumentation /> },
        Route::FormField => html! { <FormFieldDocumentation /> },
        Route::GroupButton => html! { <GroupButtonDocumentation /> },
        Route::IconButton => html! { <IconButtonDocumentation /> },
        Route::Iframe => html! { <IframeDocumentation /> },
        Route::Input => html! { <InputDocumentation /> },
        Route::Kanban => html! { <KanbanDocumentation /> },
        Route::Label => html! { <LabelDocumentation /> },
        Route::MarkdownEditor => html! { <MarkdownEditorDocumentation /> },
        Route::MarkdownRenderer => html! { <MarkdownRendererDocumentation /> },
        Route::MediaQuery => html! { <MediaQueryDocumentation /> },
        Route::MentionInput => html! { <MentionInputDocumentation /> },
        Route::MermaidDiagram => html! { <MermaidDiagramDocumentation /> },
        Route::Modal => html! { <ModalDocumentation /> },
        Route::MulticolSidebar => html! { <MulticolSidebarDocumentation /> },
        Route::Notification => html! { <NotificationDocumentation /> },
        Route::PageContent => html! { <PageContentDocumentation /> },
        Route::PageHeader => html! { <PageHeaderDocumentation /> },
        Route::Pagination => html! { <PaginationDocumentation /> },
        Route::PaintCanvas => html! { <PaintCanvasDocumentation /> },
        Route::Placeholder => html! { <PlaceholderDocumentation /> },
        Route::Popover => html! { <PopoverDocumentation /> },
        Route::Progress => html! { <ProgressDocumentation /> },
        Route::Resizable => html! { <ResizableDocumentation /> },
        Route::Select => html! { <SelectDocumentation /> },
        Route::Selectable => html! { <SelectableDocumentation /> },
        Route::Sidebar => html! { <SidebarDocumentation /> },
        Route::Spinner => html! { <SpinnerDocumentation /> },
        Route::StatusDot => html! { <StatusDotDocumentation /> },
        Route::StatusIndicator => html! { <StatusIndicatorDocumentation /> },
        Route::Switch => html! { <SwitchDocumentation /> },
        Route::Table => html! { <TableDocumentation /> },
        Route::Tabs => html! { <TabsDocumentation /> },
        Route::TagInput => html! { <TagInputDocumentation /> },
        Route::TailwindColorPicker => html! { <TailwindColorPickerDocumentation /> },
        Route::Textarea => html! { <TextareaDocumentation /> },
        Route::Toggle => html! { <ToggleDocumentation /> },
        Route::Topbar => html! { <TopbarDocumentation /> },
        Route::TypographyExample => html! { <TypographyDocumentation /> },
        Route::UIBlocksExample => html! { <UIBlocksDocumentation /> },
        Route::WindowControlsExample => html! { <WindowControlsDocumentation /> },
        Route::WindowProviderExample => html! { <WindowProviderDocumentation /> },
        // Blocks routes
        Route::BlocksIndex => html! { <blocks::BlocksIndex /> },
        Route::BlocksElements => html! { <blocks::elements::ElementsBlocks /> },
        Route::BlocksForms => html! { <blocks::forms::FormsBlocks /> },
        Route::BlocksNavigation => html! { <blocks::navigation::NavigationBlocks /> },
        Route::BlocksFeedback => html! { <blocks::feedback::FeedbackBlocks /> },
        Route::BlocksApplicationShells => html! { <blocks::application_shells::ApplicationShellsBlocks /> },
        Route::BlocksPageExamples => html! { <blocks::page_examples::PageExamplesBlocks /> },
        Route::BlocksMarketing => html! { <blocks::marketing::MarketingBlocks /> },
        Route::BlocksHeadings => html! { <blocks::headings::HeadingsBlocks /> },
        Route::BlocksDataDisplay => html! { <blocks::data_display::DataDisplayBlocks /> },
        Route::BlocksLists => html! { <blocks::lists::ListsBlocks /> },
        Route::NotFound => html! { <div class="p-8">{"404 - Page Not Found"}</div> },
    }
}

#[function_component(App)]
pub fn app() -> Html {

    html! {
    <ThemeProvider>
        <BrowserRouter>
            <AppLayout>
                <Switch<Route> render={switch} />
            </AppLayout>
        </BrowserRouter>
    </ThemeProvider>
    }
}