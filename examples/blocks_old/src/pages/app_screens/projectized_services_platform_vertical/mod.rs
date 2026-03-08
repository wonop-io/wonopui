use crate::pages::example_block::ExampleBlock;
use yew::prelude::*;

use client_portal::ClientPortal;
use document_management::DocumentManagement;
use invoicing::Invoicing;
use package_subscription::PackageSubscription;
use project_overview::ProjectOverview;
use project_task_board::ProjectTaskBoard;
use resource_allocation::ResourceAllocation;
use task_templates::TaskTemplates;
use time_tracking::TimeTracking;
use wonopui::*;

#[function_component(ProjectizedServicesPlatformVertical)]
pub fn projectized_services_platform_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Projectized Services Platform"} />
            </Breadcrumb>

            <ExampleBlock title={"Project Overview"}>
                <ProjectOverview />
            </ExampleBlock>
            <ExampleBlock title={"Project Task Board"}>
                <ProjectTaskBoard />
            </ExampleBlock>
            <ExampleBlock title={"Task Templates"}>
                <TaskTemplates />
            </ExampleBlock>
            <ExampleBlock title={"Package Subscription"}>
                <PackageSubscription />
            </ExampleBlock>
            <ExampleBlock title={"Resource Allocation"}>
                <ResourceAllocation />
            </ExampleBlock>
            <ExampleBlock title={"Time Tracking"}>
                <TimeTracking />
            </ExampleBlock>
            <ExampleBlock title={"Client Portal"}>
                <ClientPortal />
            </ExampleBlock>
            <ExampleBlock title={"Invoicing"}>
                <Invoicing />
            </ExampleBlock>
            <ExampleBlock title={"Document Management"}>
                <DocumentManagement />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod client_portal;
pub mod document_management;
pub mod invoicing;
pub mod package_subscription;
pub mod project_overview;
pub mod project_task_board;
pub mod resource_allocation;
pub mod task_templates;
pub mod time_tracking;
