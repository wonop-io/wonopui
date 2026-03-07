mod ai_based_fitness_vertical;
mod ai_based_logo_designer_vertical;
mod ai_gpu_marketplace;
mod app_generic_screens;
mod e_commerce_vertical;
mod education_vertical;
mod finance_vertical;
mod healthcare_vertical;
mod landing_page;
mod projectized_services_platform_vertical;
mod shopping_app_vertical;
mod social_media_vertical;

pub use ai_based_fitness_vertical::AiBasedFitnessVertical;
pub use ai_based_logo_designer_vertical::AiBasedLogoDesignerVertical;
pub use ai_gpu_marketplace::AiGpuMarketplace;
pub use app_generic_screens::AppGenericScreens;
pub use e_commerce_vertical::ECommerceVertical;
pub use education_vertical::EducationVertical;
pub use finance_vertical::FinanceVertical;
pub use healthcare_vertical::HealthcareVertical;
pub use landing_page::LandingPage;
pub use projectized_services_platform_vertical::ProjectizedServicesPlatformVertical;
pub use shopping_app_vertical::ShoppingAppVertical;
pub use social_media_vertical::SocialMediaVertical;

mod routes;

use routes::AppRoute;
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

#[function_component(AppScreensRootView)]
pub fn app_screens_root_view() -> Html {
    html! {
        <MainContent class="flex flex-col w-full space-y-4">
            <BlockSection title="Apps">
                <GroupCard title="AI-Based Fitness Vertical" components={8} route={AppRoute::AiBasedFitnessVertical} />
                <GroupCard title="AI-Based Logo Designer Vertical" components={3} route={AppRoute::AiBasedLogoDesignerVertical} />
                <GroupCard title="AI GPU Marketplace" components={10} route={AppRoute::AiGpuMarketplace} />
                <GroupCard title="Shopping App Vertical" components={9} route={AppRoute::ShoppingAppVertical} />
                <GroupCard title="Landing Page" components={5} route={AppRoute::LandingPage} />
                <GroupCard title="App: Generic screens" components={18} route={AppRoute::AppGenericScreens} />
                <GroupCard title="Social Media Vertical" components={3} route={AppRoute::SocialMediaVertical} />
                <GroupCard title="Healthcare Vertical" components={3} route={AppRoute::HealthcareVertical} />
                <GroupCard title="Education Vertical" components={3} route={AppRoute::EducationVertical} />
                <GroupCard title="Finance Vertical" components={3} route={AppRoute::FinanceVertical} />
                <GroupCard title="E-commerce Vertical" components={3} route={AppRoute::ECommerceVertical} />
                <GroupCard title="Projectized Services Platform Vertical" components={9} route={AppRoute::ProjectizedServicesPlatformVertical} />
            </BlockSection>
        </MainContent>
    }
}

#[function_component(AppScreensView)]
pub fn app_screens_view() -> Html {
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
        AppRoute::AppScreensRootView => html! { <AppScreensRootView /> },
        AppRoute::AiBasedFitnessVertical => html! { <AiBasedFitnessVertical /> },
        AppRoute::AiBasedLogoDesignerVertical => html! { <AiBasedLogoDesignerVertical /> },
        AppRoute::AiGpuMarketplace => html! { <AiGpuMarketplace /> },
        AppRoute::ShoppingAppVertical => html! { <ShoppingAppVertical /> },
        AppRoute::LandingPage => html! { <LandingPage /> },
        AppRoute::AppGenericScreens => html! { <AppGenericScreens /> },
        AppRoute::SocialMediaVertical => html! { <SocialMediaVertical /> },
        AppRoute::HealthcareVertical => html! { <HealthcareVertical /> },
        AppRoute::EducationVertical => html! { <EducationVertical /> },
        AppRoute::FinanceVertical => html! { <FinanceVertical /> },
        AppRoute::ECommerceVertical => html! { <ECommerceVertical /> },
        AppRoute::ProjectizedServicesPlatformVertical => {
            html! { <ProjectizedServicesPlatformVertical /> }
        }
    };

    html! {
        <Layout topbar={topbar}>
            <Switch<AppRoute> render={render} />
        </Layout>
    }
}
