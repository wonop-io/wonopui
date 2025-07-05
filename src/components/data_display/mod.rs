// Data display components

#[cfg(feature = "Avatar")]
mod avatar;
#[cfg(feature = "Card")]
mod card;
#[cfg(feature = "Carousel")]
mod carousel;
#[cfg(feature = "Kanban")]
mod kanban;
#[cfg(feature = "MarkdownEditor")]
mod markdown_editor;
#[cfg(feature = "Placeholder")]
mod placeholder;
#[cfg(feature = "Table")]
mod table;
// Re-enable when DataTable feature is added
// #[cfg(feature = "DataTable")]
// mod data_table;

#[cfg(feature = "Avatar")]
pub use avatar::{Avatar, AvatarSize};
#[cfg(feature = "Card")]
pub use card::{Card, CardContent, CardHeader, CardTitle};
#[cfg(feature = "Carousel")]
pub use carousel::{Carousel, CarouselItem};
#[cfg(feature = "Kanban")]
pub use kanban::{Kanban, KanbanCard, KanbanColumn};
#[cfg(feature = "MarkdownEditor")]
pub use markdown_editor::{Block, BlockTrait, MarkdownEditor};
#[cfg(feature = "Placeholder")]
pub use placeholder::Placeholder;
#[cfg(feature = "Table")]
pub use table::{Table, TableBody, TableCell, TableFooter, TableHead, TableRow};
// Re-enable when DataTable feature is added
// #[cfg(feature = "DataTable")]
// pub use data_table::DataTable;
