use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/blocks/")]
    AppHome,

    #[at("/blocks/sidebar-layouts")]
    SidebarLayouts,

    #[at("/blocks/stacked-layouts")]
    StackedLayouts,

    #[at("/blocks/multi-column-layouts")]
    MultiColumnLayouts,

    #[at("/blocks/page-headings")]
    PageHeadings,

    #[at("/blocks/card-headings")]
    CardHeadings,

    #[at("/blocks/section-headings")]
    SectionHeadings,

    #[at("/blocks/description-lists")]
    DescriptionLists,

    #[at("/blocks/stats")]
    Stats,

    #[at("/blocks/calendars")]
    Calendars,

    #[at("/blocks/stacked-lists")]
    StackedLists,

    #[at("/blocks/tables")]
    Tables,

    #[at("/blocks/grid-lists")]
    GridLists,

    #[at("/blocks/feeds")]
    Feeds,

    #[at("/blocks/form-layouts")]
    FormLayouts,

    #[at("/blocks/input-groups")]
    InputGroups,

    #[at("/blocks/select-menus")]
    SelectMenus,

    #[at("/blocks/sign-in-and-registration")]
    SignInAndRegistration,

    #[at("/blocks/textareas")]
    Textareas,

    #[at("/blocks/radio-groups")]
    RadioGroups,

    #[at("/blocks/checkboxes")]
    Checkboxes,

    #[at("/blocks/toggles")]
    Toggles,

    #[at("/blocks/action-panels")]
    ActionPanels,

    #[at("/blocks/comboboxes")]
    Comboboxes,

    #[at("/blocks/alerts")]
    Alerts,

    #[at("/blocks/empty-states")]
    EmptyStates,

    #[at("/blocks/navbars")]
    Navbars,

    #[at("/blocks/pagination")]
    Pagination,

    #[at("/blocks/tabs")]
    Tabs,

    #[at("/blocks/vertical-navigation")]
    VerticalNavigation,

    #[at("/blocks/sidebar-navigation")]
    SidebarNavigation,

    #[at("/blocks/breadcrumbs")]
    Breadcrumbs,

    #[at("/blocks/progress-bars")]
    ProgressBars,

    #[at("/blocks/command-palettes")]
    CommandPalettes,

    #[at("/blocks/dialogs")]
    Dialogs,

    #[at("/blocks/slide-overs")]
    SlideOvers,

    #[at("/blocks/notifications")]
    Notifications,

    #[at("/blocks/avatars")]
    Avatars,

    #[at("/blocks/badges")]
    Badges,

    #[at("/blocks/dropdowns")]
    Dropdowns,

    #[at("/blocks/buttons")]
    Buttons,

    #[at("/blocks/button-groups")]
    ButtonGroups,

    #[at("/blocks/containers")]
    Containers,

    #[at("/blocks/panels")]
    Panels,

    #[at("/blocks/list-containers")]
    ListContainers,

    #[at("/blocks/media-objects")]
    MediaObjects,

    #[at("/blocks/dividers")]
    Dividers,

    #[at("/blocks/home-screens")]
    HomeScreens,

    #[at("/blocks/detail-screens")]
    DetailScreens,

    #[at("/blocks/settings-screens")]
    SettingsScreens,
}
