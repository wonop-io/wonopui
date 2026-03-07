use crate::pages::example_block::ExampleBlock;
use friend_requests::FriendRequests;
use news_feed::NewsFeed;
use stories::Stories;
use wonopui::*;
use yew::prelude::*;

#[function_component(SocialMediaVertical)]
pub fn social_media_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Social Media"} />
            </Breadcrumb>

            <ExampleBlock title={"News Feed"}>
                <NewsFeed />
            </ExampleBlock>
            <ExampleBlock title={"Friend Requests"}>
                <FriendRequests />
            </ExampleBlock>
            <ExampleBlock title={"Stories"}>
                <Stories />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod friend_requests;
pub mod news_feed;
pub mod stories;
