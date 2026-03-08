use super::layout::AIFitnessLayout;
use web_sys::HtmlElement;
use wonopcharts::*;
use wonopui::*;
use yew::prelude::*;

#[function_component(ActivityLog)]
pub fn activity_log() -> Html {
    let data = vec![
        (0.0, 0.0),
        (10.0, 10.0),
        (20.0, 20.0),
        (30.0, 10.0),
        (40.0, 30.0),
        (50.0, 20.0),
    ];

    html! {
        <AIFitnessLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="space-y-6 p-10 pb-16">
                <div class="space-y-0.5">
                    <H2>{"Activity Log"}</H2>
                    <Paragraph>{"Logs daily activities and workouts for review and analysis."}</Paragraph>
                </div>
                <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full my-6"></div>
                <Tabs default_value="daily" class="flex flex-col space-y-8">
                    <TabsList class="flex space-x-2">
                        <TabsTrigger value="daily">{"Daily"}</TabsTrigger>
                        <TabsTrigger value="weekly">{"Weekly"}</TabsTrigger>
                        <TabsTrigger value="monthly">{"Monthly"}</TabsTrigger>
                    </TabsList>
                    <TabsContent value="daily" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Daily Activity"}</H3>
                                <Paragraph>{"Review your daily activities and workouts."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <div class="space-y-4">
                                <Chart x={(0.0, 50.0)} y={(-20.0, 50.0)} height={300} width={500} padding={50}>
                                    <XAxis show={true} ticks={6} />
                                    <YAxis show={true} />
                                    <LineChart data={data.clone()} width={5} />
                                </Chart>
                                <Card>
                                    <CardHeader>
                                        <H4>{"Morning Run"}</H4>
                                        <Paragraph>{"5 km in 30 minutes"}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Calories burned: 300"}</Paragraph>
                                    </CardContent>
                                </Card>
                                <Card>
                                    <CardHeader>
                                        <H4>{"Yoga Session"}</H4>
                                        <Paragraph>{"1 hour of Vinyasa Yoga"}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Calories burned: 200"}</Paragraph>
                                    </CardContent>
                                </Card>
                            </div>
                        </div>
                    </TabsContent>
                    <TabsContent value="weekly" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Weekly Activity"}</H3>
                                <Paragraph>{"Review your weekly activities and workouts."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <div class="space-y-4">
                                <Card>
                                    <CardHeader>
                                        <H4>{"Total Distance"}</H4>
                                        <Paragraph>{"30 km"}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Calories burned: 2000"}</Paragraph>
                                    </CardContent>
                                </Card>
                                <Card>
                                    <CardHeader>
                                        <H4>{"Total Workouts"}</H4>
                                        <Paragraph>{"5 sessions"}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Average duration: 45 minutes"}</Paragraph>
                                    </CardContent>
                                </Card>
                            </div>
                        </div>
                    </TabsContent>
                    <TabsContent value="monthly" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Monthly Activity"}</H3>
                                <Paragraph>{"Review your monthly activities and workouts."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <div class="space-y-4">
                                <Card>
                                    <CardHeader>
                                        <H4>{"Total Distance"}</H4>
                                        <Paragraph>{"120 km"}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Calories burned: 8000"}</Paragraph>
                                    </CardContent>
                                </Card>
                                <Card>
                                    <CardHeader>
                                        <H4>{"Total Workouts"}</H4>
                                        <Paragraph>{"20 sessions"}</Paragraph>
                                    </CardHeader>
                                    <CardContent>
                                        <Paragraph>{"Average duration: 50 minutes"}</Paragraph>
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
