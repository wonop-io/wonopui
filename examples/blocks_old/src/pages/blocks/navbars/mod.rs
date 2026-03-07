use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use navbar1::Navbar1;
use navbar2::Navbar2;
use navbar3::Navbar3;
use navbar4::Navbar4;

#[function_component(Navbars)]
pub fn navbars() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Navbars"} />
            </Breadcrumb>
            <ExampleBlock title={"Navbar 1"}>
                <Navbar1 />
            </ExampleBlock>
            <ExampleBlock title={"Navbar 2"}>
                <Navbar2 />
            </ExampleBlock>
            <ExampleBlock title={"Navbar 3"}>
                <Navbar3 />
            </ExampleBlock>
            <ExampleBlock title={"Navbar 4"}>
                <Navbar4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod navbar1 {
    use super::*;

    #[function_component(Navbar1)]
    pub fn navbar1() -> Html {
        html! {
            <nav class="bg-gray-800 p-4">
                <div class="container mx-auto flex justify-between items-center">
                    <div class="text-white text-lg font-bold">{"Navbar 1"}</div>
                    <div class="space-x-4">
                        <a href="#" class="text-gray-300 hover:text-white">{"Home"}</a>
                        <a href="#" class="text-gray-300 hover:text-white">{"About"}</a>
                        <a href="#" class="text-gray-300 hover:text-white">{"Contact"}</a>
                    </div>
                </div>
            </nav>
        }
    }
}

pub mod navbar2 {
    use super::*;

    #[function_component(Navbar2)]
    pub fn navbar2() -> Html {
        html! {
            <nav class="bg-blue-600 p-4">
                <div class="container mx-auto flex justify-between items-center">
                    <div class="text-white text-lg font-bold">{"Navbar 2"}</div>
                    <div class="space-x-4">
                        <a href="#" class="text-gray-100 hover:text-white">{"Home"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Services"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Portfolio"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Contact"}</a>
                    </div>
                </div>
            </nav>
        }
    }
}

pub mod navbar3 {
    use super::*;

    #[function_component(Navbar3)]
    pub fn navbar3() -> Html {
        html! {
            <nav class="bg-green-600 p-4">
                <div class="container mx-auto flex justify-between items-center">
                    <div class="text-white text-lg font-bold">{"Navbar 3"}</div>
                    <div class="space-x-4">
                        <a href="#" class="text-gray-100 hover:text-white">{"Home"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Blog"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Projects"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Contact"}</a>
                    </div>
                </div>
            </nav>
        }
    }
}

pub mod navbar4 {
    use super::*;

    #[function_component(Navbar4)]
    pub fn navbar4() -> Html {
        html! {
            <nav class="bg-red-600 p-4">
                <div class="container mx-auto flex justify-between items-center">
                    <div class="text-white text-lg font-bold">{"Navbar 4"}</div>
                    <div class="space-x-4">
                        <a href="#" class="text-gray-100 hover:text-white">{"Home"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Shop"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Cart"}</a>
                        <a href="#" class="text-gray-100 hover:text-white">{"Contact"}</a>
                    </div>
                </div>
            </nav>
        }
    }
}
