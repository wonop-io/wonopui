use crate::pages::example_block::ExampleBlock;
use appointment_booking::AppointmentBooking;
use medical_records::MedicalRecords;
use prescription_management::PrescriptionManagement;
use wonopui::*;
use yew::prelude::*;

#[function_component(HealthcareVertical)]
pub fn healthcare_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Healthcare"} />
            </Breadcrumb>

            <ExampleBlock title={"Appointment Booking"}>
                <AppointmentBooking />
            </ExampleBlock>
            <ExampleBlock title={"Medical Records"}>
                <MedicalRecords />
            </ExampleBlock>
            <ExampleBlock title={"Prescription Management"}>
                <PrescriptionManagement />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod appointment_booking;
pub mod medical_records;
pub mod prescription_management;
