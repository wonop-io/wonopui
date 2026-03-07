use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(BadgeExample1)]
pub fn badge_example1() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Badge badge_type={BadgeType::Success} label="Primary" />
        </div>
    }
}

#[function_component(BadgeExample2)]
pub fn badge_example2() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Badge badge_type={BadgeType::Warning} label="Secondary" />
        </div>
    }
}

#[function_component(BadgeExample3)]
pub fn badge_example3() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Badge badge_type={BadgeType::Info} label="Success" />
        </div>
    }
}

#[function_component(BadgeExample4)]
pub fn badge_example4() -> Html {
    html! {
        <div class="w-full h-full bg-white flex items-center justify-center p-16">
            <Badge badge_type={BadgeType::Error} label="Danger" />
        </div>
    }
}

#[function_component(Badges)]
pub fn badges() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Badges"} />
            </Breadcrumb>
            <ExampleBlock title={"Primary"}>
                <BadgeExample1 />
            </ExampleBlock>
            <ExampleBlock title={"Secondary"}>
                <BadgeExample2 />
            </ExampleBlock>
            <ExampleBlock title={"Success"}>
                <BadgeExample3 />
            </ExampleBlock>
            <ExampleBlock title={"Danger"}>
                <BadgeExample4 />
            </ExampleBlock>
        </MainContent>
    }
}
