use crate::pages::example_block::ExampleBlock;
use admin_inventory_management::AdminInventoryManagement;
use admin_order_tracking::AdminOrderTracking;
use admin_sales_reports::AdminSalesReports;
use admin_store_configuration::AdminStoreConfiguration;
use admin_user_management::AdminUserManagement;
use item_scanner::ItemScanner;
use online_payment::OnlinePayment;
use order_confirmation::OrderConfirmation;
use scanned_items_list::ScannedItemsList;
use wonopui::*;
use yew::prelude::*;

#[function_component(ShoppingAppVertical)]
pub fn shopping_app_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Shopping App"} />
            </Breadcrumb>

            <ExampleBlock title={"Item Scanner"}>
                <ItemScanner />
            </ExampleBlock>
            <ExampleBlock title={"Scanned Items List"}>
                <ScannedItemsList />
            </ExampleBlock>
            <ExampleBlock title={"Online Payment"}>
                <OnlinePayment />
            </ExampleBlock>
            <ExampleBlock title={"Order Confirmation"}>
                <OrderConfirmation />
            </ExampleBlock>
            <ExampleBlock title={"Admin Inventory Management"}>
                <AdminInventoryManagement />
            </ExampleBlock>
            <ExampleBlock title={"Admin Sales Reports"}>
                <AdminSalesReports />
            </ExampleBlock>
            <ExampleBlock title={"Admin User Management"}>
                <AdminUserManagement />
            </ExampleBlock>
            <ExampleBlock title={"Admin Store Configuration"}>
                <AdminStoreConfiguration />
            </ExampleBlock>
            <ExampleBlock title={"Admin Order Tracking"}>
                <AdminOrderTracking />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod admin_inventory_management;
pub mod admin_order_tracking;
pub mod admin_sales_reports;
pub mod admin_store_configuration;
pub mod admin_user_management;
pub mod item_scanner;
pub mod online_payment;
pub mod order_confirmation;
pub mod scanned_items_list;
