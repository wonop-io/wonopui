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
pub use compat::{ContentEditableWithCommands, BlockTrait};
pub use compat::code_editor;
pub use compat::code_editor::{Diff, DiffType, Annotation, AnnotationType, TypeHint, CodeEditor, DiffView, DiffViewMode};

// Re-exports from real crates (with aliases for backward compatibility)
#[cfg(feature = "theme-provider")]
pub use wonopui_theme_provider::{ThemeProvider, BrandGuideType, BrandGuide, ClassesStr, BRANDGUIDE};

#[cfg(not(feature = "theme-provider"))]
pub use compat::{ThemeProvider, BrandGuideType, BrandGuide, ClassesStr, BRANDGUIDE};

#[cfg(feature = "component-editor")]
pub use wonopui_component_editor::ComponentEditor;

#[cfg(not(feature = "component-editor"))]
pub use compat::ComponentEditor;

// Prelude module for convenient imports
pub mod prelude {
    pub use wonopui_core::{merge_classes, Size, Variant, Status, Direction, Position};
    pub use yew::prelude::*;
    
    // Compatibility types
    pub use crate::compat::{
        ThemeProvider, BrandGuideType, BrandGuide, ClassesStr, BRANDGUIDE, ComponentEditor,
        ContentEditableWithCommands, BlockTrait,
    };
    pub use crate::compat::code_editor::{
        Diff, DiffType, Annotation, AnnotationType, TypeHint, CodeEditor, DiffView, DiffViewMode,
    };
    
    // ContextMenu extras are now in the real crate
    #[cfg(feature = "context-menu")]
    pub use wonopui_context_menu::{
        ContextMenuSub, ContextMenuSubTrigger, ContextMenuSubContent,
        ContextMenuCheckboxItem, ContextMenuRadioGroup, ContextMenuRadioItem,
    };
    
    // Alert extras are now in the real crate
    #[cfg(feature = "alert")]
    pub use wonopui_alert::{AlertTitle, AlertDescription};
    
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
    pub use wonopui_card::{Card, CardContent, CardHeader, CardTitle, CardProps};
    
    #[cfg(feature = "checkbox")]
    pub use wonopui_checkbox::{Checkbox, CheckboxProps};
    
    #[cfg(feature = "command")]
    pub use wonopui_command::{Command, CommandProps, CommandOption};
    
    #[cfg(feature = "col")]
    pub use wonopui_col::{Col, ColProps, ColGap, ColAlign, ColJustify, Row};
    
    #[cfg(feature = "collapsible")]
    pub use wonopui_collapsible::{Collapsible, CollapsibleTrigger, CollapsibleContent, CollapsibleHeader, CollapsibleTitle, CollapsibleItem};
    
    #[cfg(feature = "container")]
    pub use wonopui_container::{Container, ContainerProps, ContainerVariant};
    
    #[cfg(feature = "dialog")]
    pub use wonopui_dialog::{Dialog, DialogProvider, DialogTrigger, DialogHeader, DialogTitle, DialogBody, DialogFooter, DialogClose, use_dialog};
    
    #[cfg(feature = "divider")]
    pub use wonopui_divider::{Divider, DividerProps};
    
    #[cfg(feature = "drawer")]
    pub use wonopui_drawer::{DrawerProvider, DrawerTrigger, Drawer, DrawerHeader, DrawerTitle, DrawerDescription, DrawerContent, DrawerFooter, DrawerClose, DrawerSide};
    
    #[cfg(feature = "dropdown")]
    pub use wonopui_dropdown::{Dropdown, DropdownItem, DropdownPosition};
    
    #[cfg(feature = "input")]
    pub use wonopui_input::{Input, InputProps};
    
    #[cfg(feature = "label")]
    pub use wonopui_label::{Label, LabelProps};
    
    #[cfg(feature = "notification")]
    pub use wonopui_notification::{NotificationProvider, Notification, use_notify};
    
    #[cfg(feature = "pagination")]
    pub use wonopui_pagination::{Pagination, PaginationProps};
    
    #[cfg(feature = "popover")]
    pub use wonopui_popover::{Popover, PopoverTrigger, PopoverContent, PopoverPosition};
    
    #[cfg(feature = "select")]
    pub use wonopui_select::{Select, SelectProps, SelectOption};
    
    #[cfg(feature = "switch")]
    pub use wonopui_switch::{SwitchButton, SwitchButtonProps};
    
    #[cfg(feature = "table")]
    pub use wonopui_table::{Table, TableHead, TableBody, TableRow, TableCell, TableHeadCell, TableFooter};
    
    #[cfg(feature = "tabs")]
    pub use wonopui_tabs::{Tabs, TabsList, TabsTrigger, TabsContent, TabsProvider, TabsDirection, use_tabs};
    
    #[cfg(feature = "textarea")]
    pub use wonopui_textarea::{Textarea, TextareaProps};
    
    #[cfg(feature = "toggle")]
    pub use wonopui_toggle::{Toggle, ToggleProps, ToggleVariant, ToggleSize};
    
    #[cfg(feature = "typography")]
    pub use wonopui_typography::{Heading, HeadingLevel, HeadingProps, Paragraph, ParagraphProps, H1, H2, H3, H4, H5, H6, P};
    
    #[cfg(feature = "media-query")]
    pub use wonopui_media_query::use_media_query;
    
    #[cfg(feature = "layout")]
    pub use wonopui_layout::{Layout, LayoutProvider, LayoutContext, LayoutAction, LayoutState, LayoutDirection, SidebarPosition, use_layout};
    
    #[cfg(feature = "sidebar")]
    pub use wonopui_sidebar::{Sidebar, SidebarHeader, SidebarContent, SidebarFooter, SidebarHeading, SidebarNav, SidebarItem, SidebarMenu, SidebarLink};
    
    #[cfg(feature = "topbar")]
    pub use wonopui_topbar::{Topbar, TopbarStart, TopbarCenter, TopbarEnd, TopbarPosition};
    
    #[cfg(feature = "copy-button")]
    pub use wonopui_copy_button::CopyButton;
    
    #[cfg(feature = "group-button")]
    pub use wonopui_group_button::{GroupButton, GroupButtonTrigger, GroupButtonDirection};
}

// Re-export components at the crate root level
#[cfg(feature = "accordion")]
pub use wonopui_accordion::{self, Accordion, AccordionProps};

#[cfg(feature = "alert")]
pub use wonopui_alert::{self, Alert, AlertProps, AlertVariant, AlertType, AlertTitle, AlertDescription};

#[cfg(feature = "avatar")]
pub use wonopui_avatar::{self, Avatar, AvatarProps, AvatarSize};

#[cfg(feature = "badge")]
pub use wonopui_badge::{self, Badge, BadgeProps, BadgeVariant, BadgeType};

#[cfg(feature = "breadcrumb")]
pub use wonopui_breadcrumb::{self, Breadcrumb, BreadcrumbItem, BreadcrumbItemProps, BreadcrumbLink, BreadcrumbProps, BreadcrumbRouteItem};

#[cfg(feature = "browser-provider")]
pub use wonopui_browser_provider as browser_provider;

#[cfg(feature = "button")]
pub use wonopui_button::{self, Button, ButtonProps, ButtonSize, ButtonVariant};

#[cfg(feature = "calendar")]
pub use wonopui_calendar::{self as calendar, Calendar, CalendarProps};

#[cfg(feature = "card")]
pub use wonopui_card::{self, Card, CardContent, CardContentProps, CardHeader, CardHeaderProps, CardProps, CardTitle, CardTitleProps};

#[cfg(feature = "carousel")]
pub use wonopui_carousel::{self as carousel, Carousel, CarouselItem, CarouselProps, CarouselItemProps};

#[cfg(feature = "checkbox")]
pub use wonopui_checkbox::{self, Checkbox, CheckboxProps};

#[cfg(feature = "code-editor")]
pub use wonopui_code_editor::{self as code_editor_crate};

#[cfg(feature = "col")]
pub use wonopui_col::{self, Col, ColProps, ColGap, ColAlign, ColJustify, Row};

#[cfg(feature = "collapsible")]
pub use wonopui_collapsible::{self, Collapsible, CollapsibleContent, CollapsibleHeader, CollapsibleItem, CollapsibleTitle, CollapsibleTrigger};

#[cfg(feature = "color-picker")]
pub use wonopui_color_picker::{self as color_picker, ColorPicker, ColorPickerProps};

#[cfg(feature = "combobox")]
pub use wonopui_combobox::{self as combobox, Combobox, ComboboxProps, ComboboxItem};

#[cfg(feature = "command")]
pub use wonopui_command::{self as command, Command, CommandProps, CommandOption};

#[cfg(feature = "container")]
pub use wonopui_container::{self, Container, ContainerProps, ContainerVariant};

#[cfg(feature = "content")]
pub use wonopui_content::{self, MainContent, Content, MainContentProps, ContentProps};

#[cfg(feature = "context-menu")]
pub use wonopui_context_menu::{self, ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, ContextMenuLabel, ContextMenuShortcut, ContextMenuSub, ContextMenuSubTrigger, ContextMenuSubContent, ContextMenuCheckboxItem, ContextMenuRadioGroup, ContextMenuRadioItem};

#[cfg(feature = "copy-button")]
pub use wonopui_copy_button::{self as copy_button, CopyButton, CopyButtonProps};

#[cfg(feature = "dark-mode-provider")]
pub use wonopui_dark_mode_provider::{self, DarkModeProvider, DarkModeContext, ColorMode, use_dark_mode, use_is_dark, use_toggle_dark_mode};

#[cfg(feature = "data-table")]
pub use wonopui_data_table as data_table;

#[cfg(feature = "date-picker")]
pub use wonopui_date_picker as date_picker;

#[cfg(feature = "dialog")]
pub use wonopui_dialog::{self, Dialog, DialogBody, DialogClose, DialogFooter, DialogHeader, DialogProvider, DialogTitle, DialogTrigger, use_dialog};

#[cfg(feature = "diffview")]
pub use wonopui_diffview as diffview;

#[cfg(feature = "divider")]
pub use wonopui_divider::{self, Divider, DividerProps};

#[cfg(feature = "drag-point")]
pub use wonopui_drag_point::{self as drag_point, DragPoint, DragPointProps};

#[cfg(feature = "drawer")]
pub use wonopui_drawer::{self, Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerProvider, DrawerSide, DrawerTitle, DrawerTrigger};

#[cfg(feature = "dropdown")]
pub use wonopui_dropdown::{self, Dropdown, DropdownItem, DropdownPosition};

#[cfg(feature = "group-button")]
pub use wonopui_group_button::{self, GroupButton, GroupButtonTrigger, GroupButtonDirection, FlexDirection};

#[cfg(feature = "iframe")]
pub use wonopui_iframe::{self as iframe, Iframe, IframeProps};

#[cfg(feature = "input")]
pub use wonopui_input::{self, Input, InputProps};

#[cfg(feature = "kanban")]
pub use wonopui_kanban::{self as kanban, Kanban, KanbanColumn, KanbanCard, KanbanProps, KanbanColumnProps, KanbanCardProps};

#[cfg(feature = "label")]
pub use wonopui_label::{self, Label, LabelProps};

#[cfg(feature = "layout")]
pub use wonopui_layout::{self, Layout, LayoutProvider, LayoutDirection, LayoutState, LayoutAction, LayoutContext, SidebarPosition, use_layout};

#[cfg(feature = "markdown-editor")]
pub use wonopui_markdown_editor::{self as markdown_editor, MarkdownEditor, MarkdownEditorProps, EditorMode};

#[cfg(feature = "media-query")]
pub use wonopui_media_query::{self, use_media_query, breakpoints};

#[cfg(feature = "mention-input")]
pub use wonopui_mention_input::{self as mention_input, MentionInput, MentionInputProps};

#[cfg(feature = "multicol-sidebar")]
pub use wonopui_multicol_sidebar::{self as multicol_sidebar, MultiColumnSidebar, MultiColumnSidebarProps, SidebarColumn, SidebarColumnProps};

#[cfg(feature = "notification")]
pub use wonopui_notification::{self, Notification, NotificationProvider, use_notify, use_notification_context};

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
pub use wonopui_popover::{self, Popover, PopoverContent, PopoverPosition, PopoverState, PopoverTrigger};

#[cfg(feature = "resizable")]
pub use wonopui_resizable::{self as resizable, Resizable, ResizableProps};

#[cfg(feature = "select")]
pub use wonopui_select::{self, Select, SelectOption, SelectProps, SelectState};

#[cfg(feature = "selectable")]
pub use wonopui_selectable::{self as selectable, Selectable, SelectableArea, SelectableIndicator, SelectableProps, SelectableAreaProps, SelectableIndicatorProps};

#[cfg(feature = "sidebar")]
pub use wonopui_sidebar::{self, Sidebar, SidebarHeader, SidebarContent, SidebarFooter, SidebarHeading, SidebarNav, SidebarItem, SidebarMenu, SidebarLink, SidebarPosition as SidebarSide};

#[cfg(feature = "switch")]
pub use wonopui_switch::{self, SwitchButton, SwitchButtonProps};

#[cfg(feature = "table")]
pub use wonopui_table::{self, Table, TableBody, TableCell, TableFooter, TableHead, TableHeadCell, TableRow};

#[cfg(feature = "tabs")]
pub use wonopui_tabs::{self, Tabs, TabsContent, TabsDirection, TabsList, TabsProvider, TabsTrigger, use_tabs};

#[cfg(feature = "tag-input")]
pub use wonopui_tag_input::{self, TagInput, TagInputProps};

#[cfg(feature = "tailwind-color-picker")]
pub use wonopui_tailwind_color_picker::{self as tailwind_color_picker, TailwindColorPicker, TailwindColorPickerProps};

#[cfg(feature = "textarea")]
pub use wonopui_textarea::{self, Textarea, TextareaProps};

#[cfg(feature = "toggle")]
pub use wonopui_toggle::{self, Toggle, ToggleProps, ToggleSize, ToggleVariant};

#[cfg(feature = "topbar")]
pub use wonopui_topbar::{self, Topbar, TopbarStart, TopbarCenter, TopbarEnd, TopbarPosition, TopbarProps};

#[cfg(feature = "typography")]
pub use wonopui_typography::{self, Heading, HeadingLevel, HeadingProps, Paragraph, ParagraphProps, H1, H2, H3, H4, H5, H6, P};

#[cfg(feature = "window-provider")]
pub use wonopui_window_provider::{self as window_provider, WindowProvider, WindowProviderProps, use_window_context};
