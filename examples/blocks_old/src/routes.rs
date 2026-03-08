use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/blocks/")]
    BlocksRoot,
    #[at("/blocks/*")]
    Blocks,

    #[at("/screens/")]
    ScreensRoot,
    #[at("/screens/*")]
    Screens,

    #[at("/api/*")]
    UiApp,
    #[at("/app/*")]
    AppGallery,
}
