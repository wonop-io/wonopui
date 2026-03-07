mod app;
mod components;
mod layout;
mod routes;

use app::SidebarLayoutWithToolbar;
use components::{
    AccordionDocumentation, AlertDocumentation, AvatarDocumentation, BadgeDocumentation,
    BreadcrumbDocumentation, ButtonDocumentation, CardDocumentation, CarouselDocumentation,
    CheckboxDocumentation, ComboboxDocumentation, CommandDocumentation, DialogDocumentation,
    DrawerDocumentation, NotificationDocumentation, PopoverDocumentation, SelectDocumentation,
    TabsDocumentation, ToggleDocumentation,
};
use gloo_console as console;
use layout::Layout;
use routes::{AppRoute, Route};
use std::rc::Rc;
use web_sys::HtmlInputElement;
use wonopui::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(AppGalleryView)]
pub fn app_gallery_view() -> Html {
    let render = move |routes| match routes {
        AppRoute::AppLayoutWithToolbar => {
            html! { <SidebarLayoutWithToolbar /> }
        }
    };

    return html! {
        <Switch<AppRoute> render={render} />
    };
}

#[function_component(UiGalleryView)]
pub fn ui_gallery_view() -> Html {
    let render = move |routes| match routes {
        Route::GettingStarted => {
            html! { <div>{"hello"}</div> }
        }

        Route::AccordionExample => {
            html! { <AccordionDocumentation /> }
        }
        Route::AlertExamples => {
            html! { <AlertDocumentation /> }
        }
        Route::AvatarExamples => {
            html! { <AvatarDocumentation /> }
        }
        Route::BadgeExample => {
            html! { <BadgeDocumentation /> }
        }
        Route::ButtonExample => {
            html! { <ButtonDocumentation /> }
        }
        Route::CardExample => {
            html! { <CardDocumentation /> }
        }
        Route::CarouselExample => {
            html! { <CarouselDocumentation /> }
        }
        Route::BreadcrumbExample => {
            html! { <BreadcrumbDocumentation /> }
        }
        Route::CheckboxExample => {
            html! { <CheckboxDocumentation /> }
        }
        Route::ComboboxExample => {
            html! { <ComboboxDocumentation /> }
        }
        Route::CommandExample => {
            html! { <CommandDocumentation /> }
        }
        Route::PopoverExample => {
            html! { <PopoverDocumentation /> }
        }
        Route::DrawerExample => {
            html! { <DrawerDocumentation /> }
        }
        Route::DialogExample => {
            html! { <DialogDocumentation /> }
        }
        Route::NotificationExample => {
            html! { <NotificationDocumentation /> }
        }
        Route::ToggleExample => {
            html! { <ToggleDocumentation /> }
        }
        Route::SelectExample => {
            html! { <SelectDocumentation /> }
        }
        Route::TabsExample => {
            html! { <TabsDocumentation /> }
        }
    };

    return html! {
      <Layout header_title="Documentation" footer_year={2023}>
        <Switch<Route> render={render} />
      </Layout>
    };
}
