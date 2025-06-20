use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    GettingStarted,

    #[at("/accordion")]
    AccordionExample,
    #[at("/alert")]
    AlertExample,
    #[at("/avatar")]
    AvatarExample,
    #[at("/badge")]
    BadgeExample,
    #[at("/breadcrumb")]
    BreadcrumbExample,
    #[at("/button")]
    ButtonExample,
    #[at("/calendar")]
    CalendarExample,
    #[at("/card")]
    CardExample,
    #[at("/carousel")]
    CarouselExample,
    #[at("/checkbox")]
    CheckboxExample,
    #[at("/col")]
    ColExample,
    #[at("/collapsible")]
    CollapsibleExample,
    #[at("/code-editor")]
    CodeEditorExample,
    #[at("/color-picker")]
    ColorPickerExample,
    #[at("/combobox")]
    ComboboxExample,
    #[at("/command")]
    CommandExample,
    #[at("/container")]
    ContainerExample,
    #[at("/content")]
    ContentExample,
    #[at("/context-menu")]
    ContextMenuExample,
    #[at("/copy-button")]
    CopyButtonExample,
    #[at("/data-table")]
    DataTableExample,
    #[at("/date-picker")]
    DatePickerExample,
    #[at("/dialog")]
    DialogExample,
    #[at("/divider")]
    DividerExample,
    #[at("/drag-point")]
    DragPointExample,
    #[at("/drawer")]
    DrawerExample,
    #[at("/dropdown")]
    DropdownExample,
    #[at("/group-button")]
    GroupButtonExample,
    #[at("/iframe")]
    IframeExample,
    #[at("/input")]
    InputExample,
    #[at("/kanban")]
    KanbanExample,
    #[at("/label")]
    LabelExample,
    #[at("/media-query")]
    MediaQueryExample,
    #[at("/multicol-sidebar")]
    MulticolSidebarExample,
    #[at("/notification")]
    NotificationExample,
    #[at("/page-header")]
    PageHeaderExample,
    #[at("/page-content")]
    PageContentExample,
    #[at("/pagination")]
    PaginationExample,
    #[at("/paint-canvas")]
    PaintCanvasExample,
    #[at("/placeholder")]
    PlaceholderExample,
    #[at("/popover")]
    PopoverExample,
    #[at("/resizable")]
    ResizableExample,
    #[at("/select")]
    SelectExample,
    #[at("/selectable")]
    SelectableExample,
    #[at("/sidebar")]
    SidebarExample,
    #[at("/switch")]
    SwitchExample,
    #[at("/table")]
    TableExample,
    #[at("/tabs")]
    TabsExample,
    #[at("/tag-input")]
    TagInputExample,
    #[at("/tailwind-color-picker")]
    TailwindColorPickerExample,
    #[at("/textarea")]
    TextareaExample,
    #[at("/toggle")]
    ToggleExample,
    #[at("/topbar")]
    TopbarExample,
    #[at("/typography")]
    TypographyExample,
    #[at("/window-provider")]
    WindowProviderExample,
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/app/sidebar")]
    AppLayoutWithToolbar,
}
