use crate::pages::example_block::ExampleBlock;
use yew::prelude::*;

use about_us::AboutUs;
use contact_us::ContactUs;
use error_pages::ErrorPages;
use faq_help_center::FaqHelpCenter;
use terms_and_conditions_privacy_policy::TermsAndConditionsPrivacyPolicy;
use wonopui::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Landing Page"} />
            </Breadcrumb>

            <ExampleBlock title={"FAQ Help Center"}>
                <FaqHelpCenter />
            </ExampleBlock>
            <ExampleBlock title={"Error Pages"}>
                <ErrorPages />
            </ExampleBlock>
            <ExampleBlock title={"About Us"}>
                <AboutUs />
            </ExampleBlock>
            <ExampleBlock title={"Contact Us"}>
                <ContactUs />
            </ExampleBlock>
            <ExampleBlock title={"Terms and Conditions / Privacy Policy"}>
                <TermsAndConditionsPrivacyPolicy />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod about_us;
pub mod contact_us;
pub mod error_pages;
pub mod faq_help_center;
pub mod terms_and_conditions_privacy_policy;
