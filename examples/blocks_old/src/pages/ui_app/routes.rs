use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/api/")]
    GettingStarted,

    #[at("/api/accordion")]
    AccordionExample,
    #[at("/api/alerts")]
    AlertExamples,
    #[at("/api/avatars")]
    AvatarExamples,
    #[at("/api/badge")]
    BadgeExample,
    #[at("/api/button")]
    ButtonExample,
    #[at("/api/card")]
    CardExample,
    #[at("/api/carousel")]
    CarouselExample,
    #[at("/api/breadcrumb")]
    BreadcrumbExample,
    #[at("/api/checkbox")]
    CheckboxExample,
    #[at("/api/combobox")]
    ComboboxExample,
    #[at("/api/command")]
    CommandExample,
    #[at("/api/popover")]
    PopoverExample,
    #[at("/api/drawer")]
    DrawerExample,
    #[at("/api/dialog")]
    DialogExample,
    #[at("/api/notification")]
    NotificationExample,
    #[at("/api/toggle")]
    ToggleExample,
    #[at("/api/select")]
    SelectExample,
    #[at("/api/tabs")]
    TabsExample,
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/app/sidebar")]
    AppLayoutWithToolbar,
}
