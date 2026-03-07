use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(AvatarExample1)]
pub fn avatar_example1() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Avatar size={AvatarSize::Small} src="https://via.placeholder.com/150" alt="Avatar 1" />
        </div>
    }
}

#[function_component(AvatarExample2)]
pub fn avatar_example2() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Avatar size={AvatarSize::Medium} src="https://via.placeholder.com/150" alt="Avatar 2" />
        </div>

    }
}

#[function_component(AvatarExample3)]
pub fn avatar_example3() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Avatar size={AvatarSize::Large} src="https://via.placeholder.com/150" alt="Avatar 3" />
        </div>

    }
}

#[function_component(AvatarExample4)]
pub fn avatar_example4() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Avatar size={AvatarSize::Large} src="https://via.placeholder.com/150" alt="Avatar 4" />
            <Avatar size={AvatarSize::Large} src="https://via.placeholder.com/150" alt="Avatar 4" />
            <Avatar size={AvatarSize::Large} src="https://via.placeholder.com/150" alt="Avatar 4" />

        </div>
    }
}

#[function_component(Avatars)]
pub fn avatars() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Avatars"} />
            </Breadcrumb>
            <ExampleBlock title={"Small Avatar"}>
                <AvatarExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Medium Avatar"}>
                <AvatarExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Large Avatar"}>
                <AvatarExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Extra Large Avatar"}>
                <AvatarExample4 />
            </ExampleBlock>
        </MainContent>
    }
}
