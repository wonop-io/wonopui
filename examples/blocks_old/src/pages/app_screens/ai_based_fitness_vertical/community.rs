use super::layout::AIFitnessLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(Community)]
pub fn community() -> Html {
    html! {
        <AIFitnessLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="space-y-6 p-10 pb-16">
                <div class="space-y-0.5">
                    <H2>{"Community"}</H2>
                    <Paragraph>{"Connect with a community for support and motivation."}</Paragraph>
                </div>
                <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full my-6"></div>
                <Tabs default_value="posts" class="flex flex-col space-y-8">
                    <TabsList class="flex space-x-2">
                        <TabsTrigger value="posts">{"Posts"}</TabsTrigger>
                        <TabsTrigger value="events">{"Events"}</TabsTrigger>
                        <TabsTrigger value="members">{"Members"}</TabsTrigger>
                    </TabsList>
                    <TabsContent value="posts" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Community Posts"}</H3>
                                <Paragraph>{"Share your experiences and get support from the community."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <div class="space-y-4">
                                <Card>
                                    <CardHeader>
                                        <H4>{"Post Title"}</H4>
                                        <Paragraph>{"Post content goes here."}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Comments and interactions."}</Paragraph>
                                    </CardContent>
                                </Card>
                            </div>
                        </div>
                    </TabsContent>
                    <TabsContent value="events" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Community Events"}</H3>
                                <Paragraph>{"Join events and activities organized by the community."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <div class="space-y-4">
                                <Card>
                                    <CardHeader>
                                        <H4>{"Event Title"}</H4>
                                        <Paragraph>{"Event details go here."}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Event date and time."}</Paragraph>
                                    </CardContent>
                                </Card>
                            </div>
                        </div>
                    </TabsContent>
                    <TabsContent value="members" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Community Members"}</H3>
                                <Paragraph>{"Meet and connect with other members of the community."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <div class="space-y-4">
                                <Card>
                                    <CardHeader>
                                        <H4>{"Member Name"}</H4>
                                        <Paragraph>{"Member details go here."}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Member bio and interactions."}</Paragraph>
                                    </CardContent>
                                </Card>
                            </div>
                        </div>
                    </TabsContent>
                </Tabs>
            </div>
        </AIFitnessLayout>
    }
}
