use crate::pages::example_block::ExampleBlock;
use gloo_console as console;
use wonopui::*;
use yew::prelude::*;

#[function_component(PageHeadings)]
pub fn page_headings() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Page Headings"} />
            </Breadcrumb>
            <ExampleBlock title={"Heading 1"} isolate={true}>
                <Heading1 />
            </ExampleBlock>
            <ExampleBlock title={"Heading 2"} isolate={true}>
                <Heading2 />
            </ExampleBlock>
            <ExampleBlock title={"Heading 3"} isolate={false}>
                <Heading3 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(Heading1)]
pub fn heading1() -> Html {
    html! {
        <div class="flex w-full md:space-x-4 flex-col-reverse md:flex-row md:justify-between md:items-center">
            <H2>
                {"Company Wiki Page"}
            </H2>
            <div class="flex mb-4 space-x-2 md:mb-0">
                <Button>
                    {"Edit"}
                </Button>
                <Button variant={ButtonVariant::Primary}>
                    {"Publish"}
                </Button>
            </div>
        </div>
    }
}

#[function_component(Heading2)]
pub fn heading2() -> Html {
    let more_items = vec![
        DropdownItemProps {
            label: "API Documentation".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Usage Examples".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Support".to_string(),
            ..Default::default()
        },
    ];
    html! {
        <div class="lg:flex lg:items-start lg:justify-between w-full">
            <div class="min-w-0 flex-1">
                <H2 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
                    {"Deploy LLM API Endpoint"}
                </H2>
                <div class="mt-1 flex flex-col sm:mt-2 sm:flex-row sm:space-x-2 items-center">
                    <Badge badge_type={BadgeType::Warning} label="llama-3" />
                    <Badge badge_type={BadgeType::Success} label="Active" />
                    <div class="flex items-center text-sm space-x-2">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5" />
                        </svg>
                        <span>
                            {"Last used: October 1, 2023"}
                        </span>
                    </div>
                </div>
            </div>
            <div class="mt-5 flex lg:ml-4 lg:mt-0 space-x-2">
                <Button variant={ButtonVariant::Ghost}>
                    {"Edit"}
                </Button>
                <Button variant={ButtonVariant::Ghost}>
                    {"Test"}
                </Button>
                <Dropdown items={more_items}>
                    <Button variant={ButtonVariant::Ghost}>
                        {"More"}
                    </Button>
                </Dropdown>
                <Button variant={ButtonVariant::Primary}>
                    {"Deploy"}
                </Button>
            </div>
        </div>
    }
}

#[function_component(Heading3)]
pub fn heading3() -> Html {
    let more_items = vec![
        DropdownItemProps {
            label: "Item A".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Item B".to_string(),
            ..Default::default()
        },
        DropdownItemProps {
            label: "Item C".to_string(),
            ..Default::default()
        },
    ];

    html! {
        <div class="bg-white">
            <div>
                <img class="h-48 w-full object-cover lg:h-64" src="https://images.unsplash.com/photo-1444628838545-ac4016a5418a?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1950&q=80" alt=""/>
            </div>
            <MainContent>
                <div class="mx-6 -mt-12 sm:-mt-16 sm:flex sm:items-end sm:space-x-5">
                    <div class="flex justify-between w-full items-end">
                        <Avatar size={AvatarSize::Large} src="https://images.unsplash.com/photo-1463453091185-61582044d556?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80" alt="Ricardo Cooper"/>
                        <div class="mb-12">
                            <Dropdown items={more_items} position={PopoverPosition::SouthEnd}>
                                <Button variant={ButtonVariant::Ghost}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
                                </svg>

                                </Button>
                            </Dropdown>
                        </div>
                    </div>
                </div>
                <div class="mx-6 mt-6 hidden min-w-0 flex-1 sm:block md:-mt-16 md:ml-48">
                    <H1>{"John doe"}</H1>
                </div>
            </MainContent>
        </div>
    }
}
