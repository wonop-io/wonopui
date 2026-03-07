use crate::pages::example_block::ExampleBlock;
use flash_sales::FlashSales;
use reviews_and_ratings::ReviewsAndRatings;
use wishlist::Wishlist;
use wonopui::*;
use yew::prelude::*;

#[function_component(ECommerceVertical)]
pub fn e_commerce_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"E-Commerce"} />
            </Breadcrumb>

            <ExampleBlock title={"Wishlist"}>
                <Wishlist />
            </ExampleBlock>
            <ExampleBlock title={"Reviews and Ratings"}>
                <ReviewsAndRatings />
            </ExampleBlock>
            <ExampleBlock title={"Flash Sales"}>
                <FlashSales />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod flash_sales;
pub mod reviews_and_ratings;
pub mod wishlist;
