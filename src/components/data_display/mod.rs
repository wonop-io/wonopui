// Data display components

#[cfg(feature = "Avatar")]
mod avatar;
#[cfg(feature = "Card")]
mod card;
#[cfg(feature = "Kanban")]
mod kanban;
#[cfg(feature = "Table")]
mod table;
#[cfg(feature = "Carousel")]
mod carousel;
#[cfg(feature = "Placeholder")]
mod placeholder;
// Re-enable when DataTable feature is added
// #[cfg(feature = "DataTable")]
// mod data_table;

#[cfg(feature = "Avatar")]
pub use avatar::{Avatar, AvatarSize};
#[cfg(feature = "Card")]
pub use card::{Card, CardContent, CardHeader, CardTitle};
#[cfg(feature = "Kanban")]
pub use kanban::{Kanban, KanbanColumn, KanbanCard};
#[cfg(feature = "Table")]
pub use table::{Table, TableBody, TableCell, TableFooter, TableHead, TableRow};
#[cfg(feature = "Carousel")]
pub use carousel::{Carousel, CarouselItem};
#[cfg(feature = "Placeholder")]
pub use placeholder::Placeholder;
// Re-enable when DataTable feature is added
// #[cfg(feature = "DataTable")]
// pub use data_table::DataTable;