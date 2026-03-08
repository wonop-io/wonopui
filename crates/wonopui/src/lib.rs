//! # WonopUI
//!
//! A YEW UI framework for building flexible and customizable web applications
//! with Tailwind CSS styling.
//!
//! ## Features
//!
//! WonopUI provides a comprehensive set of UI components, each available as an
//! individual feature flag. Enable only the components you need to minimize
//! bundle size.
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wonopui = { version = "0.1", features = ["button", "input", "card"] }
//! ```
//!
//! Or enable all components:
//!
//! ```toml
//! [dependencies]
//! wonopui = { version = "0.1", features = ["everything"] }
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use wonopui::prelude::*;
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Card>
//!             <CardHeader>
//!                 <CardTitle>{"Hello WonopUI!"}</CardTitle>
//!             </CardHeader>
//!             <CardContent>
//!                 <Button variant={ButtonVariant::Primary}>
//!                     {"Click me"}
//!                 </Button>
//!             </CardContent>
//!         </Card>
//!     }
//! }
//! ```

// Re-export core utilities
pub use wonopui_core::*;

// Compatibility module for components not yet migrated
pub mod compat;

// Re-export compat types at top level for backward compatibility
// Note: ThemeProvider, BrandGuide, ComponentEditor are now in their own crates
// but we keep re-exporting them here for backward compatibility
pub use compat::{BlockTrait, ContentEditableWithCommands};

// Re-export code_editor module from compat for backward compatibility with wonopui::code_editor::* imports
// These are simplified stubs; use wonopui_code_editor directly for full features
pub use compat::code_editor;
pub use compat::code_editor::{Annotation, AnnotationType, Diff, DiffType, DiffViewMode, TypeHint};

// Re-export CodeEditor from the compat module (stub version)
// The full-featured version is in wonopui_code_editor
pub use compat::code_editor::CodeEditor;
pub use compat::code_editor::DiffView;

// Re-exports from real crates
#[cfg(feature = "theme-provider")]
pub use wonopui_theme_provider::{
    BrandGuide, BrandGuideType, ClassesStr, ThemeProvider, BRANDGUIDE,
};

#[cfg(feature = "component-editor")]
pub use wonopui_component_editor::ComponentEditor;

// Prelude module for convenient imports
pub mod prelude {
    pub use wonopui_core::{merge_classes, Direction, Position, Size, Status, Variant};
    pub use yew::prelude::*;

    // Compatibility types
    pub use crate::compat::{BlockTrait, ContentEditableWithCommands};

    // Re-exports from real crates
    #[cfg(feature = "theme-provider")]
    pub use wonopui_theme_provider::{
        BrandGuide, BrandGuideType, ClassesStr, ThemeProvider, BRANDGUIDE,
    };

    pub use crate::compat::code_editor::{
        Annotation, AnnotationType, CodeEditor, Diff, DiffType, DiffView, DiffViewMode, TypeHint,
    };
    #[cfg(feature = "component-editor")]
    pub use wonopui_component_editor::ComponentEditor;

    // ContextMenu extras are now in the real crate
    #[cfg(feature = "context-menu")]
    pub use wonopui_context_menu::{
        ContextMenuCheckboxItem, ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSub,
        ContextMenuSubContent, ContextMenuSubTrigger,
    };

    // Alert extras are now in the real crate
    #[cfg(feature = "alert")]
    pub use wonopui_alert::{AlertDescription, AlertTitle};

    #[cfg(feature = "accordion")]
    pub use wonopui_accordion::{Accordion, AccordionProps};

    #[cfg(feature = "alert")]
    pub use wonopui_alert::{Alert, AlertProps, AlertVariant};

    #[cfg(feature = "avatar")]
    pub use wonopui_avatar::{Avatar, AvatarProps, AvatarSize};

    #[cfg(feature = "badge")]
    pub use wonopui_badge::{Badge, BadgeProps, BadgeVariant};

    #[cfg(feature = "breadcrumb")]
    pub use wonopui_breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbProps};

    #[cfg(feature = "button")]
    pub use wonopui_button::{Button, ButtonProps, ButtonSize, ButtonVariant};

    #[cfg(feature = "card")]
    pub use wonopui_card::{Card, CardContent, CardHeader, CardProps, CardTitle};

    #[cfg(feature = "checkbox")]
    pub use wonopui_checkbox::{Checkbox, CheckboxProps};

    #[cfg(feature = "command")]
    pub use wonopui_command::{Command, CommandOption, CommandProps};

    #[cfg(feature = "confirm-dialog")]
    pub use wonopui_confirm_dialog::{ConfirmDialog, ConfirmDialogProps, ConfirmDialogVariant};

    #[cfg(feature = "col")]
    pub use wonopui_col::{Col, ColAlign, ColGap, ColJustify, ColProps, Row};

    #[cfg(feature = "collapsible")]
    pub use wonopui_collapsible::{
        Collapsible, CollapsibleContent, CollapsibleHeader, CollapsibleItem, CollapsibleTitle,
        CollapsibleTrigger,
    };

    #[cfg(feature = "container")]
    pub use wonopui_container::{Container, ContainerProps, ContainerVariant};

    #[cfg(feature = "dialog")]
    pub use wonopui_dialog::{
        use_dialog, Dialog, DialogBody, DialogClose, DialogFooter, DialogHeader, DialogProvider,
        DialogTitle, DialogTrigger,
    };

    #[cfg(feature = "divider")]
    pub use wonopui_divider::{Divider, DividerProps};

    #[cfg(feature = "drawer")]
    pub use wonopui_drawer::{
        Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
        DrawerProvider, DrawerSide, DrawerTitle, DrawerTrigger,
    };

    #[cfg(feature = "dropdown")]
    pub use wonopui_dropdown::{Dropdown, DropdownItem, DropdownPosition};

    #[cfg(feature = "error-boundary")]
    pub use wonopui_error_boundary::{ErrorBoundary, ErrorBoundaryProps};

    #[cfg(feature = "input")]
    pub use wonopui_input::{Input, InputProps};

    #[cfg(feature = "label")]
    pub use wonopui_label::{Label, LabelProps};

    #[cfg(feature = "markdown-renderer")]
    pub use wonopui_markdown_renderer::{MarkdownRenderer, MarkdownRendererProps};

    #[cfg(feature = "mermaid-diagram")]
    pub use wonopui_mermaid_diagram::{MermaidDiagram, MermaidDiagramProps};

    #[cfg(feature = "notification")]
    pub use wonopui_notification::{use_notify, Notification, NotificationProvider};

    #[cfg(feature = "pagination")]
    pub use wonopui_pagination::{Pagination, PaginationProps};

    #[cfg(feature = "popover")]
    pub use wonopui_popover::{Popover, PopoverContent, PopoverPosition, PopoverTrigger};

    #[cfg(feature = "progress")]
    pub use wonopui_progress::{Progress, ProgressProps, ProgressVariant, ProgressSize};

    #[cfg(feature = "select")]
    pub use wonopui_select::{Select, SelectOption, SelectProps};

    #[cfg(feature = "spinner")]
    pub use wonopui_spinner::{Spinner, SpinnerProps, SpinnerSize};

    #[cfg(feature = "status-dot")]
    pub use wonopui_status_dot::{StatusDot, StatusDotProps, StatusDotVariant, StatusDotSize};

    #[cfg(feature = "status-indicator")]
    pub use wonopui_status_indicator::{StatusIndicator, StatusIndicatorProps};

    #[cfg(feature = "switch")]
    pub use wonopui_switch::{SwitchButton, SwitchButtonProps};

    #[cfg(feature = "table")]
    pub use wonopui_table::{
        Table, TableBody, TableCell, TableFooter, TableHead, TableHeadCell, TableRow,
    };

    #[cfg(feature = "tabs")]
    pub use wonopui_tabs::{
        use_tabs, Tabs, TabsContent, TabsDirection, TabsList, TabsProvider, TabsTrigger,
    };

    #[cfg(feature = "textarea")]
    pub use wonopui_textarea::{Textarea, TextareaProps};

    #[cfg(feature = "toggle")]
    pub use wonopui_toggle::{Toggle, ToggleProps, ToggleSize, ToggleVariant};

    #[cfg(feature = "typography")]
    pub use wonopui_typography::{
        Heading, HeadingLevel, HeadingProps, Paragraph, ParagraphProps, H1, H2, H3, H4, H5, H6, P,
    };

    #[cfg(feature = "media-query")]
    pub use wonopui_media_query::use_media_query;

    #[cfg(feature = "layout")]
    pub use wonopui_layout::{
        use_layout, Layout, LayoutAction, LayoutContext, LayoutDirection, LayoutProvider,
        LayoutState, SidebarPosition,
    };

    #[cfg(feature = "sidebar")]
    pub use wonopui_sidebar::{
        Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarHeading, SidebarItem,
        SidebarLink, SidebarMenu, SidebarNav,
    };

    #[cfg(feature = "topbar")]
    pub use wonopui_topbar::{Topbar, TopbarCenter, TopbarEnd, TopbarPosition, TopbarStart};

    #[cfg(feature = "copy-button")]
    pub use wonopui_copy_button::CopyButton;

    #[cfg(feature = "group-button")]
    pub use wonopui_group_button::{GroupButton, GroupButtonDirection, GroupButtonTrigger};

    #[cfg(feature = "window-controls")]
    pub use wonopui_window_controls::{WindowControls, WindowControlsProps};

    #[cfg(feature = "empty-state")]
    pub use wonopui_empty_state::{EmptyState, EmptyStateProps, EmptyStateSize};

    #[cfg(feature = "form-field")]
    pub use wonopui_form_field::{FormField, FormFieldProps};

    #[cfg(feature = "icon-button")]
    pub use wonopui_icon_button::{
        IconButton, IconButtonColor, IconButtonProps, IconButtonSize, IconButtonVariant,
    };

    #[cfg(feature = "modal")]
    pub use wonopui_modal::{Modal, ModalProps, ModalSize};
}

// Re-export components at the crate root level
#[cfg(feature = "accordion")]
pub use wonopui_accordion::{self, Accordion, AccordionProps};

#[cfg(feature = "alert")]
pub use wonopui_alert::{
    self, Alert, AlertDescription, AlertProps, AlertTitle, AlertType, AlertVariant,
};

#[cfg(feature = "avatar")]
pub use wonopui_avatar::{self, Avatar, AvatarProps, AvatarSize};

#[cfg(feature = "badge")]
pub use wonopui_badge::{self, Badge, BadgeProps, BadgeType, BadgeVariant};

#[cfg(feature = "breadcrumb")]
pub use wonopui_breadcrumb::{
    self, Breadcrumb, BreadcrumbItem, BreadcrumbItemProps, BreadcrumbLink, BreadcrumbProps,
    BreadcrumbRouteItem,
};

#[cfg(feature = "browser-provider")]
pub use wonopui_browser_provider as browser_provider;

#[cfg(feature = "button")]
pub use wonopui_button::{self, Button, ButtonProps, ButtonSize, ButtonVariant};

#[cfg(feature = "calendar")]
pub use wonopui_calendar::{self as calendar, Calendar, CalendarProps};

#[cfg(feature = "card")]
pub use wonopui_card::{
    self, Card, CardContent, CardContentProps, CardHeader, CardHeaderProps, CardProps, CardTitle,
    CardTitleProps,
};

#[cfg(feature = "carousel")]
pub use wonopui_carousel::{
    self as carousel, Carousel, CarouselItem, CarouselItemProps, CarouselProps,
};

#[cfg(feature = "checkbox")]
pub use wonopui_checkbox::{self, Checkbox, CheckboxProps};

#[cfg(feature = "code-editor")]
pub use wonopui_code_editor::{self as code_editor_crate};

#[cfg(feature = "col")]
pub use wonopui_col::{self, Col, ColAlign, ColGap, ColJustify, ColProps, Row};

#[cfg(feature = "collapsible")]
pub use wonopui_collapsible::{
    self, Collapsible, CollapsibleContent, CollapsibleHeader, CollapsibleItem, CollapsibleTitle,
    CollapsibleTrigger,
};

#[cfg(feature = "color-picker")]
pub use wonopui_color_picker::{self as color_picker, ColorPicker, ColorPickerProps};

#[cfg(feature = "combobox")]
pub use wonopui_combobox::{self as combobox, Combobox, ComboboxItem, ComboboxProps};

#[cfg(feature = "command")]
pub use wonopui_command::{self as command, Command, CommandOption, CommandProps};

#[cfg(feature = "confirm-dialog")]
pub use wonopui_confirm_dialog::{self as confirm_dialog, ConfirmDialog, ConfirmDialogProps, ConfirmDialogVariant};

#[cfg(feature = "container")]
pub use wonopui_container::{self, Container, ContainerProps, ContainerVariant};

#[cfg(feature = "content")]
pub use wonopui_content::{self, Content, ContentProps, MainContent, MainContentProps};

#[cfg(feature = "context-menu")]
pub use wonopui_context_menu::{
    self, ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem,
    ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator,
    ContextMenuShortcut, ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger,
    ContextMenuTrigger,
};

#[cfg(feature = "copy-button")]
pub use wonopui_copy_button::{self as copy_button, CopyButton, CopyButtonProps};

#[cfg(feature = "dark-mode-provider")]
pub use wonopui_dark_mode_provider::{
    self, use_dark_mode, use_is_dark, use_toggle_dark_mode, ColorMode, DarkModeContext,
    DarkModeProvider,
};

#[cfg(feature = "data-table")]
pub use wonopui_data_table as data_table;

#[cfg(feature = "date-picker")]
pub use wonopui_date_picker::{self as date_picker, DatePicker, DatePickerProps};

#[cfg(feature = "dialog")]
pub use wonopui_dialog::{
    self, use_dialog, Dialog, DialogBody, DialogClose, DialogFooter, DialogHeader, DialogProvider,
    DialogTitle, DialogTrigger,
};

#[cfg(feature = "diffview")]
pub use wonopui_diffview as diffview;

#[cfg(feature = "divider")]
pub use wonopui_divider::{self, Divider, DividerProps};

#[cfg(feature = "drag-point")]
pub use wonopui_drag_point::{self as drag_point, DragPoint, DragPointProps};

#[cfg(feature = "drawer")]
pub use wonopui_drawer::{
    self, Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
    DrawerProvider, DrawerSide, DrawerTitle, DrawerTrigger,
};

#[cfg(feature = "dropdown")]
pub use wonopui_dropdown::{self, Dropdown, DropdownItem, DropdownPosition};

#[cfg(feature = "error-boundary")]
pub use wonopui_error_boundary::{self as error_boundary, ErrorBoundary, ErrorBoundaryProps};

#[cfg(feature = "group-button")]
pub use wonopui_group_button::{
    self, FlexDirection, GroupButton, GroupButtonDirection, GroupButtonTrigger,
};

#[cfg(feature = "iframe")]
pub use wonopui_iframe::{self as iframe, Iframe, IframeProps};

#[cfg(feature = "input")]
pub use wonopui_input::{self, Input, InputProps};

#[cfg(feature = "kanban")]
pub use wonopui_kanban::{
    self as kanban, Kanban, KanbanCard, KanbanCardProps, KanbanColumn, KanbanColumnProps,
    KanbanProps,
};

#[cfg(feature = "label")]
pub use wonopui_label::{self, Label, LabelProps};

#[cfg(feature = "layout")]
pub use wonopui_layout::{
    self, use_layout, Layout, LayoutAction, LayoutContext, LayoutDirection, LayoutProvider,
    LayoutState, SidebarPosition,
};

#[cfg(feature = "markdown-editor")]
pub use wonopui_markdown_editor::{
    self as markdown_editor, MarkdownEditor, MarkdownEditorProps,
};

// Re-export markdown editor types separately to avoid conflict with compat module
#[cfg(feature = "markdown-editor")]
pub mod markdown_editor_types {
    pub use wonopui_markdown_editor::{
        BlockTrait, ContentEditableWithCommands, ContentEditableWithCommandsProps,
    };
}

#[cfg(feature = "markdown-renderer")]
pub use wonopui_markdown_renderer::{self as markdown_renderer, MarkdownRenderer, MarkdownRendererProps};

#[cfg(feature = "media-query")]
pub use wonopui_media_query::{self, breakpoints, use_media_query};

#[cfg(feature = "mention-input")]
pub use wonopui_mention_input::{self as mention_input, MentionInput, MentionInputProps};

#[cfg(feature = "mermaid-diagram")]
pub use wonopui_mermaid_diagram::{self as mermaid_diagram, MermaidDiagram, MermaidDiagramProps};

#[cfg(feature = "multicol-sidebar")]
pub use wonopui_multicol_sidebar::{
    self as multicol_sidebar, MultiColumnSidebar, MultiColumnSidebarProps, SidebarColumn,
    SidebarColumnProps,
};

#[cfg(feature = "notification")]
pub use wonopui_notification::{
    self, use_notification_context, use_notify, Notification, NotificationProvider,
};

#[cfg(feature = "page-content")]
pub use wonopui_page_content::{self as page_content, PageContent, PageContentProps};

#[cfg(feature = "page-header")]
pub use wonopui_page_header::{self as page_header, PageHeader, PageHeaderProps};

#[cfg(feature = "pagination")]
pub use wonopui_pagination::{self, Pagination, PaginationProps};

#[cfg(feature = "paint-canvas")]
pub use wonopui_paint_canvas::{self as paint_canvas, PaintCanvas, PaintCanvasProps};

#[cfg(feature = "placeholder")]
pub use wonopui_placeholder::{self, Placeholder, PlaceholderProps};

#[cfg(feature = "popover")]
pub use wonopui_popover::{
    self, Popover, PopoverContent, PopoverPosition, PopoverState, PopoverTrigger,
};

#[cfg(feature = "progress")]
pub use wonopui_progress::{self as progress, Progress, ProgressProps, ProgressVariant, ProgressSize};

#[cfg(feature = "resizable")]
pub use wonopui_resizable::{self as resizable, Resizable, ResizableProps};

#[cfg(feature = "select")]
pub use wonopui_select::{self, Select, SelectOption, SelectProps, SelectState};

#[cfg(feature = "selectable")]
pub use wonopui_selectable::{
    self as selectable, Selectable, SelectableArea, SelectableAreaProps, SelectableIndicator,
    SelectableIndicatorProps, SelectableProps,
};

#[cfg(feature = "sidebar")]
pub use wonopui_sidebar::{
    self, Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarHeading, SidebarItem,
    SidebarLink, SidebarMenu, SidebarNav, SidebarPosition as SidebarSide,
};

#[cfg(feature = "spinner")]
pub use wonopui_spinner::{self as spinner, Spinner, SpinnerProps, SpinnerSize};

#[cfg(feature = "status-dot")]
pub use wonopui_status_dot::{self as status_dot, StatusDot, StatusDotProps, StatusDotVariant, StatusDotSize};

#[cfg(feature = "status-indicator")]
pub use wonopui_status_indicator::{self as status_indicator, StatusIndicator, StatusIndicatorProps};

#[cfg(feature = "switch")]
pub use wonopui_switch::{self, SwitchButton, SwitchButtonProps};

#[cfg(feature = "table")]
pub use wonopui_table::{
    self, Table, TableBody, TableCell, TableFooter, TableHead, TableHeadCell, TableRow,
};

#[cfg(feature = "tabs")]
pub use wonopui_tabs::{
    self, use_tabs, Tabs, TabsContent, TabsDirection, TabsList, TabsProvider, TabsTrigger,
};

#[cfg(feature = "tag-input")]
pub use wonopui_tag_input::{self, TagInput, TagInputProps};

#[cfg(feature = "tailwind-color-picker")]
pub use wonopui_tailwind_color_picker::{
    self as tailwind_color_picker, TailwindColorPicker, TailwindColorPickerProps,
};

#[cfg(feature = "textarea")]
pub use wonopui_textarea::{self, Textarea, TextareaProps};

#[cfg(feature = "toggle")]
pub use wonopui_toggle::{self, Toggle, ToggleProps, ToggleSize, ToggleVariant};

#[cfg(feature = "topbar")]
pub use wonopui_topbar::{
    self, Topbar, TopbarCenter, TopbarEnd, TopbarPosition, TopbarProps, TopbarStart,
};

#[cfg(feature = "typography")]
pub use wonopui_typography::{
    self, Heading, HeadingLevel, HeadingProps, Paragraph, ParagraphProps, H1, H2, H3, H4, H5, H6, P,
};

#[cfg(feature = "window-controls")]
pub use wonopui_window_controls::{self as window_controls, WindowControls, WindowControlsProps};

#[cfg(feature = "window-provider")]
pub use wonopui_window_provider::{
    self as window_provider, use_window_context, WindowProvider, WindowProviderProps,
};

#[cfg(feature = "empty-state")]
pub use wonopui_empty_state::{self as empty_state, EmptyState, EmptyStateProps, EmptyStateSize};

#[cfg(feature = "form-field")]
pub use wonopui_form_field::{self as form_field, FormField, FormFieldProps};

#[cfg(feature = "icon-button")]
pub use wonopui_icon_button::{
    self as icon_button, IconButton, IconButtonColor, IconButtonProps, IconButtonSize,
    IconButtonVariant,
};

#[cfg(feature = "modal")]
pub use wonopui_modal::{self as modal, Modal, ModalProps, ModalSize};