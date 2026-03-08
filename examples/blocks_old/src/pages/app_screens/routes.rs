use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/screens")]
    AppScreensRootView,

    #[at("/screens/ai-fitness")]
    AiBasedFitnessVertical,

    #[at("/screens/ai-logo-designer")]
    AiBasedLogoDesignerVertical,

    #[at("/screens/ai-gpu-marketplace")]
    AiGpuMarketplace,

    #[at("/screens/shopping-app")]
    ShoppingAppVertical,

    #[at("/screens/landing-page")]
    LandingPage,

    #[at("/screens/app")]
    AppGenericScreens,

    #[at("/screens/social-media")]
    SocialMediaVertical,

    #[at("/screens/healthcare")]
    HealthcareVertical,

    #[at("/screens/education")]
    EducationVertical,

    #[at("/screens/finance")]
    FinanceVertical,

    #[at("/screens/e-commerce")]
    ECommerceVertical,

    #[at("/screens/projectized-services")]
    ProjectizedServicesPlatformVertical,
}
