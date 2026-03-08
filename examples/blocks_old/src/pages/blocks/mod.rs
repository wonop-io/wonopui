mod action_panels;
mod page_headings;
mod routes;
mod sidebar_layout;
mod stacked_layout;

mod alerts;
mod avatars;
mod badges;
mod breadcrumbs;
mod button_groups;
mod buttons;
mod calendars;
mod card_headings;
mod checkboxes;
mod comboboxes;
mod command_palettes;
mod containers;
mod description_lists;
mod detail_screens;
mod dialogs;
mod dividers;
mod dropdowns;
mod empty_states;
mod feeds;
mod form_layouts;
mod grid_lists;
mod home_screens;

mod input_groups;
mod list_containers;
mod media_objects;
mod multi_column_layouts;
mod navbars;
mod notifications;
mod pagination;
mod panels;
mod progress_bars;
mod radio_groups;

//
mod section_headings;
mod settings_screens;
mod sidebar_navigation;
mod sign_in_and_registration;
mod slide_overs;

mod stacked_layouts;
mod tables;
mod textareas;
mod vertical_navigation;
//mod stacked_lists;
//mod tabs;
//mod toggles;

use alerts::Alerts;
use avatars::Avatars;
use badges::Badges;
use breadcrumbs::Breadcrumbs;
use button_groups::ButtonGroups;
use buttons::Buttons;
use calendars::Calendars;
use card_headings::CardHeadings;
use checkboxes::Checkboxes;
use comboboxes::Comboboxes;
use command_palettes::CommandPalettes;
use containers::Containers;
use description_lists::DescriptionLists;
use detail_screens::DetailScreens;
use dialogs::Dialogs;
use dividers::Dividers;
use dropdowns::Dropdowns;
use empty_states::EmptyStates;
use feeds::Feeds;
use form_layouts::FormLayouts;
use grid_lists::GridLists;
use home_screens::HomeScreens;

use input_groups::InputGroups;
use list_containers::ListContainers;
use media_objects::MediaObjects;
use multi_column_layouts::MultiColumnLayouts;
use navbars::Navbars;
use notifications::Notifications;
use pagination::Paginations;
use panels::Panels;
use progress_bars::ProgressBars;
use radio_groups::RadioGroups;

use tables::Tables;
use textareas::Textareas;
use vertical_navigation::VerticalNavigation;

use action_panels::ActionPanels;
use page_headings::PageHeadings;
use routes::AppRoute;
pub use sidebar_layout::SidebarLayouts;
use stacked_layout::StackedLayouts;
use wonopui::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct GroupCardProps {
    pub title: String,
    #[prop_or_default]
    pub components: i32,
    #[prop_or_default]
    pub route: Option<AppRoute>,
    #[prop_or("bg-yellow-100".to_string())]
    pub bg_color: String,
}

#[function_component(GroupCard)]
pub fn group_card(props: &GroupCardProps) -> Html {
    let inner = html! {
        <>
            <Card class="h-32">
            </Card>
            <div class="text-zinc-500">{&props.title}</div>
            <div class="text-xs text-zinc-500">{props.components} {" components"}</div>
        </>
    };

    match &props.route {
        Some(route) => {
            html! {
                <Link<AppRoute> to={route.clone()} classes={classes!("rounded-md","cursor-pointer","hover:bg-zinc-100","space-y-2","p-2", props.bg_color.clone())}>
                    {inner}
                </Link<AppRoute>>
            }
        }
        _ => {
            html! {
                <div class="bg-red-100 rounded-md cursor-pointer hover:bg-zinc-100 space-y-2 p-2">
                    {inner}
                </div>
            }
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct BlockSectionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(BlockSection)]
pub fn block_section(props: &BlockSectionProps) -> Html {
    // let n = props.children.len() / 3 + 1;
    // row-span-1 row-span-2 row-span-3 row-span-4 row-span-5 row-span-6 row-span-7 row-span-8 row-span-9 row-span-10
    html! {
        <div class={"border-t p-12 w-full grid grid-cols-4 gap-4"}>
            <div class={format!("py-2 col-span-1")}>
                <div class="text-xl font-bold">{&props.title}</div>
                <div class="text-sm text-zinc-500">{"Blocks"}</div>
            </div>
            <div class="col-span-3 grid grid-cols-3 gap-4">
                {for props.children.iter()}
            </div>
        </div>
    }
}

#[function_component(BlocksRootView)]
pub fn blocks_root_view() -> Html {
    html! {
        <MainContent class="flex flex-col w-full space-y-4">
            <BlockSection title="Application Shells">
                <GroupCard title="Stacked Layouts" components={9} route={Some(AppRoute::StackedLayouts)} />
                <GroupCard title="Sidebar Layouts" components={8} route={Some(AppRoute::SidebarLayouts)} />
                <GroupCard title="Multi-Column Layouts" components={6} route={Some(AppRoute::MultiColumnLayouts)} />
            </BlockSection>

            <BlockSection title="Page Examples">
                <GroupCard title="Home Screens" components={2} route={Some(AppRoute::HomeScreens)} />
                <GroupCard title="Detail Screens" components={2} route={Some(AppRoute::DetailScreens)} />
                <GroupCard title="Settings Screens" components={2} />
            </BlockSection>


            <BlockSection title="Headings">
                <GroupCard title="Page Headings" components={13} route={Some(AppRoute::PageHeadings)} />
                <GroupCard title="Card Headings" components={6} route={Some(AppRoute::CardHeadings)} />
                <GroupCard title="Section Headings" components={10} />
            </BlockSection>

            <BlockSection title="Data Display">
                <GroupCard title="Description Lists" components={7} route={Some(AppRoute::DescriptionLists)} />
                <GroupCard title="Stats" components={5} />
                <GroupCard title="Calendars" components={8} route={Some(AppRoute::Calendars)} />
            </BlockSection>

            <BlockSection title="Lists">
                <GroupCard title="Stacked Lists" components={17} />
                <GroupCard title="Tables" components={20} route={Some(AppRoute::Tables)} />
                <GroupCard title="Grid Lists" components={7} route={Some(AppRoute::GridLists)} />
                <GroupCard title="Feeds" components={3} route={Some(AppRoute::Feeds)} />
            </BlockSection>

            <BlockSection title="Forms">
                <GroupCard title="Form Layouts" components={5} route={Some(AppRoute::FormLayouts)} />
                <GroupCard title="Input Groups" components={21} route={Some(AppRoute::InputGroups)} />
                <GroupCard title="Select Menus" components={7} />
                <GroupCard title="Sign-in and Registration" components={5} />
                <GroupCard title="Textareas" components={5} route={Some(AppRoute::Textareas)} />
                <GroupCard title="Radio Groups" components={12} route={Some(AppRoute::RadioGroups)} />
                <GroupCard title="Checkboxes" components={4} route={Some(AppRoute::Checkboxes)} />
                <GroupCard title="Toggles" components={5} />
                <GroupCard title="Action Panels" components={8} route={Some(AppRoute::ActionPanels)} />
                <GroupCard title="Comboboxes" components={5} route={Some(AppRoute::Comboboxes)} />
            </BlockSection>

            <BlockSection title="Feedback">
                <GroupCard title="Alerts" components={6} route={Some(AppRoute::Alerts)} />
                <GroupCard title="Empty States" components={6} route={Some(AppRoute::EmptyStates)} />
            </BlockSection>

            <BlockSection title="Navigation">
                <GroupCard title="Navbars" components={11} route={Some(AppRoute::Navbars)} />
                <GroupCard title="Pagination" components={3} route={Some(AppRoute::Pagination)} />
                <GroupCard title="Tabs" components={9} route={Some(AppRoute::Tabs)} />
                <GroupCard title="Vertical Navigation" components={6} route={Some(AppRoute::VerticalNavigation)} />
                <GroupCard title="Sidebar Navigation" components={5}  />
                <GroupCard title="Breadcrumbs" components={4} route={Some(AppRoute::Breadcrumbs)} />
                <GroupCard title="Progress Bars" components={8} route={Some(AppRoute::ProgressBars)} />
                <GroupCard title="Command Palettes" components={9} route={Some(AppRoute::CommandPalettes)} />
            </BlockSection>

            <BlockSection title="Overlays">
                <GroupCard title="Dialogs" components={6} route={Some(AppRoute::Dialogs)} />
                <GroupCard title="Slide-overs" components={12}  />
                <GroupCard title="Notifications" components={6} route={Some(AppRoute::Notifications)} />
            </BlockSection>

            <BlockSection title="Elements">
                <GroupCard title="Avatars" components={11} route={Some(AppRoute::Avatars)} />
                <GroupCard title="Badges" components={18} route={Some(AppRoute::Badges)} />
                <GroupCard title="Dropdowns" components={5} route={Some(AppRoute::Dropdowns)} />
                <GroupCard title="Buttons" components={10} route={Some(AppRoute::Buttons)} />
                <GroupCard title="Button Groups" components={5} route={Some(AppRoute::ButtonGroups)} />
            </BlockSection>

            <BlockSection title="Layout">
                <GroupCard title="Containers" components={5} route={Some(AppRoute::Containers)} />
                <GroupCard title="Panels" components={10} route={Some(AppRoute::Panels)} />
                <GroupCard title="List containers" components={7} route={Some(AppRoute::ListContainers)} />
                <GroupCard title="Media Objects" components={8} route={Some(AppRoute::MediaObjects)} />
                <GroupCard title="Dividers" components={8} route={Some(AppRoute::Dividers)} />
            </BlockSection>



        </MainContent>
    }
}

#[function_component(BlocksView)]
pub fn blocks_view() -> Html {
    let topbar = html! {
        <Topbar style={TopbarStyle::Sticky}>
            <div class="mx-auto w-full max-w-7xl flex justify-between">
                <svg viewBox="0 0 248 31" class="text-zinc-900 dark:text-white w-auto h-5">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M25.517 0C18.712 0 14.46 3.382 12.758 10.146c2.552-3.382 5.529-4.65 8.931-3.805 1.941.482 3.329 1.882 4.864 3.432 2.502 2.524 5.398 5.445 11.722 5.445 6.804 0 11.057-3.382 12.758-10.145-2.551 3.382-5.528 4.65-8.93 3.804-1.942-.482-3.33-1.882-4.865-3.431C34.736 2.92 31.841 0 25.517 0zM12.758 15.218C5.954 15.218 1.701 18.6 0 25.364c2.552-3.382 5.529-4.65 8.93-3.805 1.942.482 3.33 1.882 4.865 3.432 2.502 2.524 5.397 5.445 11.722 5.445 6.804 0 11.057-3.381 12.758-10.145-2.552 3.382-5.529 4.65-8.931 3.805-1.941-.483-3.329-1.883-4.864-3.432-2.502-2.524-5.398-5.446-11.722-5.446z" fill="#38bdf8"></path>
                </svg>
                <div>{"Login"}</div>
            </div>
        </Topbar>
    };

    let render = move |routes| match routes {
        AppRoute::SidebarLayouts => html! { <SidebarLayouts /> },
        AppRoute::AppHome => html! { <BlocksRootView /> },
        AppRoute::StackedLayouts => html! {   <StackedLayouts /> },
        AppRoute::MultiColumnLayouts => html! { <MultiColumnLayouts /> },
        AppRoute::PageHeadings => html! {  <PageHeadings /> },
        AppRoute::CardHeadings => html! {    <CardHeadings /> },
        AppRoute::SectionHeadings => html! {  <div>{"Placeholder"}</div> }, // <SectionHeadings /> },
        AppRoute::DescriptionLists => html! { <DescriptionLists /> },
        AppRoute::Stats => html! {  <div>{"Placeholder"}</div> }, // <Stats /> },
        AppRoute::Calendars => html! { <Calendars /> },
        AppRoute::StackedLists => html! {  <div>{"Placeholder"}</div> }, // <StackedLists /> },
        AppRoute::Tables => html! {  <Tables /> },
        AppRoute::GridLists => html! { <GridLists /> },
        AppRoute::Feeds => html! {  <Feeds /> },
        AppRoute::FormLayouts => html! { <FormLayouts /> },
        AppRoute::InputGroups => html! {   <InputGroups /> },
        AppRoute::SelectMenus => html! {  <div>{"Placeholder"}</div> }, // <SelectMenus /> },
        AppRoute::SignInAndRegistration => html! {  <div>{"Placeholder"}</div> }, // <SignInAndRegistration /> },
        AppRoute::Textareas => html! {   <Textareas /> },
        AppRoute::RadioGroups => html! {  <RadioGroups /> },
        AppRoute::Checkboxes => html! {   <Checkboxes /> },
        AppRoute::Toggles => html! {  <div>{"Placeholder"}</div> }, // <Toggles /> },
        AppRoute::ActionPanels => html! { <ActionPanels /> },
        AppRoute::Comboboxes => html! {  <Comboboxes /> },
        AppRoute::Alerts => html! {  <Alerts /> },
        AppRoute::EmptyStates => html! { <EmptyStates /> },
        AppRoute::Navbars => html! { <Navbars /> },
        AppRoute::Pagination => html! { <Paginations /> },
        AppRoute::Tabs => html! {   <div>{"Placeholder"}</div> }, //  <Tabs /> },
        AppRoute::VerticalNavigation => html! { <VerticalNavigation /> },
        AppRoute::SidebarNavigation => html! {  <div>{"Placeholder"}</div> }, // <SidebarNavigation /> },
        AppRoute::Breadcrumbs => html! {  <Breadcrumbs /> },
        AppRoute::ProgressBars => html! { <ProgressBars /> },
        AppRoute::CommandPalettes => html! { <CommandPalettes /> },
        AppRoute::Dialogs => html! {  <Dialogs /> },
        AppRoute::SlideOvers => html! {  <div>{"Placeholder"}</div> }, // <SlideOvers /> },
        AppRoute::Notifications => html! { <Notifications /> },
        AppRoute::Avatars => html! {  <Avatars /> },
        AppRoute::Badges => html! {  <Badges /> }, // <Badges /> },
        AppRoute::Dropdowns => html! {   <Dropdowns /> },
        AppRoute::Buttons => html! {  <Buttons /> },
        AppRoute::ButtonGroups => html! {  <ButtonGroups /> },
        AppRoute::Containers => html! {  <Containers /> },
        AppRoute::Panels => html! {   <Panels /> },
        AppRoute::ListContainers => html! { <ListContainers /> },
        AppRoute::MediaObjects => html! {  <MediaObjects /> },
        AppRoute::Dividers => html! {  <Dividers /> },
        AppRoute::HomeScreens => html! {  <HomeScreens /> },
        AppRoute::DetailScreens => html! {  <DetailScreens /> },
        AppRoute::SettingsScreens => html! {  <div>{"Placeholder"}</div> }, // <SettingsScreens /> },
    };

    html! {
        <Layout topbar={topbar}>
            <Switch<AppRoute> render={render} />
        </Layout>
    }
}
