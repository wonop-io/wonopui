use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use media_object1::MediaObject1;
use media_object2::MediaObject2;
use media_object3::MediaObject3;
use media_object4::MediaObject4;

#[function_component(MediaObjects)]
pub fn media_objects() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Media Objects"} />
            </Breadcrumb>
            <ExampleBlock title={"Media Object 1"}>
                <MediaObject1 />
            </ExampleBlock>
            <ExampleBlock title={"Media Object 2"}>
                <MediaObject2 />
            </ExampleBlock>
            <ExampleBlock title={"Media Object 3"}>
                <MediaObject3 />
            </ExampleBlock>
            <ExampleBlock title={"Media Object 4"}>
                <MediaObject4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod media_object1 {
    use super::*;

    #[function_component(MediaObject1)]
    pub fn media_object1() -> Html {
        html! {
            <div class="flex items-center p-4 border rounded shadow">
                <Avatar size={AvatarSize::Large} src="https://via.placeholder.com/150" alt="Avatar 3" />
                <div class="ml-4">
                    <h2 class="text-xl font-bold">{"Media Object 1"}</h2>
                    <p>{"This is a description for media object 1."}</p>
                </div>
            </div>
        }
    }
}

pub mod media_object2 {
    use super::*;

    #[function_component(MediaObject2)]
    pub fn media_object2() -> Html {
        html! {
            <div class="flex items-center p-4 border rounded shadow">
                <Avatar size={AvatarSize::Medium} src="https://via.placeholder.com/150" alt="Avatar 3" />
                <div class="ml-4">
                    <h2 class="text-xl font-bold">{"Media Object 2"}</h2>
                    <p>{"This is a description for media object 2."}</p>
                </div>
            </div>
        }
    }
}

pub mod media_object3 {
    use super::*;

    #[function_component(MediaObject3)]
    pub fn media_object3() -> Html {
        html! {
            <div class="flex items-center p-4 border rounded shadow">
                <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 3" />
                <div class="ml-4">
                    <h2 class="text-xl font-bold">{"Media Object 3"}</h2>
                    <p>{"This is a description for media object 3."}</p>
                </div>
            </div>
        }
    }
}

pub mod media_object4 {
    use super::*;

    #[function_component(MediaObject4)]
    pub fn media_object4() -> Html {
        html! {
            <div class="flex items-center p-4 border rounded shadow">
                <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 3" />
                <div class="ml-4">
                    <h2 class="text-xl font-bold">{"Media Object 4"}</h2>
                    <p>{"This is a description for media object 4."}</p>
                </div>
            </div>
        }
    }
}
