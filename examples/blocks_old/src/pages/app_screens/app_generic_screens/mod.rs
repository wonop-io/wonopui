use crate::pages::example_block::ExampleBlock;
use yew::prelude::*;

use admin_panel::AdminPanel;
use cart::Cart;
use checkout::Checkout;
use dashboard::Dashboard;
use login::Login;
use messages_chat::MessagesChat;
use notifications::Notifications;
use order_history::OrderHistory;
use product_detail::ProductDetail;
use product_list::ProductList;
use registration_signup::RegistrationSignup;
use reports_analytics::ReportsAnalytics;
use search_results::SearchResults;
use settings::Settings;
use stripe_subscription::StripeSubscription;
use user_detail::UserDetail;
use user_list::UserList;
use user_profile::UserProfile;
use wonopui::*;

#[function_component(AppGenericScreens)]
pub fn app_generic_screens() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"App Generic Screens"} />
            </Breadcrumb>

            <ExampleBlock title={"Login"}>
                <Login />
            </ExampleBlock>
            <ExampleBlock title={"Registration Signup"}>
                <RegistrationSignup />
            </ExampleBlock>
            <ExampleBlock title={"Dashboard"}>
                <Dashboard />
            </ExampleBlock>
            <ExampleBlock title={"User Profile"}>
                <UserProfile />
            </ExampleBlock>
            <ExampleBlock title={"User List"}>
                <UserList />
            </ExampleBlock>
            <ExampleBlock title={"User Detail"}>
                <UserDetail />
            </ExampleBlock>
            <ExampleBlock title={"Settings"}>
                <Settings />
            </ExampleBlock>
            <ExampleBlock title={"Notifications"}>
                <Notifications />
            </ExampleBlock>
            <ExampleBlock title={"Messages Chat"}>
                <MessagesChat />
            </ExampleBlock>
            <ExampleBlock title={"Search Results"}>
                <SearchResults />
            </ExampleBlock>
            <ExampleBlock title={"Product List"}>
                <ProductList />
            </ExampleBlock>
            <ExampleBlock title={"Product Detail"}>
                <ProductDetail />
            </ExampleBlock>
            <ExampleBlock title={"Cart"}>
                <Cart />
            </ExampleBlock>
            <ExampleBlock title={"Checkout"}>
                <Checkout />
            </ExampleBlock>
            <ExampleBlock title={"Order History"}>
                <OrderHistory />
            </ExampleBlock>
            <ExampleBlock title={"Reports Analytics"}>
                <ReportsAnalytics />
            </ExampleBlock>
            <ExampleBlock title={"Admin Panel"}>
                <AdminPanel />
            </ExampleBlock>
            <ExampleBlock title={"Stripe Subscription"}>
                <StripeSubscription />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod admin_panel;
pub mod cart;
pub mod checkout;
pub mod dashboard;
pub mod login;
pub mod messages_chat;
pub mod notifications;
pub mod order_history;
pub mod product_detail;
pub mod product_list;
pub mod registration_signup;
pub mod reports_analytics;
pub mod search_results;
pub mod settings;
pub mod stripe_subscription;
pub mod user_detail;
pub mod user_list;
pub mod user_profile;
