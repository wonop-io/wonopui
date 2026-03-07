use super::layout1::AppLayout;
use gloo_console as console;
use web_sys::HtmlElement;
use wonopcharts::*;
use wonopui::*;
use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    /*
    let node_chart_ref = use_node_ref();
    {

        let node_chart_ref = node_chart_ref.clone();
        use_effect_with((node_chart_ref), |(node_chart_ref)| {
                let div = node_chart_ref
                .cast::<HtmlElement>()
                .expect("div_ref not attached to div element");

                let labels = vec!["Jan".to_string(), "Feb".to_string(), "Mar".to_string(), "Apr".to_string(), "May".to_string(), "Jun".to_string(), "Jul".to_string(), "Aug".to_string(), "Sep".to_string(), "Oct".to_string(), "Nov".to_string(), "Dec".to_string(),];
                let series = vec![vec![100., 200., 300., 400., 500., 600., 700., 800., 900., 1000., 1100., 1200.]];

                let chart_data = create_line_chart_data(labels, series);
                // let chart_options = LineChartOptions::new(Some("200px".to_string()), Some("200px".to_string()), None);
                let chart = LineChart::new(&format!("#{}", div.id()), &chart_data, None, None);
        });
    }
    */

    let data = vec![
        (0.0, 0.0),
        (10.0, 10.0),
        (20.0, 20.0),
        (30.0, 10.0),
        (40.0, 30.0),
        (50.0, 20.0),
    ];

    html! {
        <AppLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <Tabs default_value="overview" class="w-full">
                <TabsList class="inline-flex h-9 rounded-lg bg-muted p-1 text-muted-foreground">
                    <TabsTrigger value="overview">{"Overview"}</TabsTrigger>
                    <TabsTrigger value="analytics"/* disabled=true*/>{"Analytics"}</TabsTrigger>
                    <TabsTrigger value="reports" /* disabled=true*/>{"Reports"}</TabsTrigger>
                    <TabsTrigger value="notifications" /* disabled=true*/>{"Notifications"}</TabsTrigger>
                </TabsList>
                <TabsContent value="overview" class="space-y-8">
                    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                        <Card>
                            <CardHeader>
                                <h3 class="tracking-tight text-sm font-medium">{"Total Revenue"}</h3>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="h-4 w-4 text-muted-foreground">
                                    <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
                                </svg>
                            </CardHeader>
                            <CardContent>
                                <div class="text-2xl font-bold">{"$45,231.89"}</div>
                                <p class="text-xs text-muted-foreground">{"+20.1% from last month"}</p>
                            </CardContent>
                        </Card>
                        <Card>
                            <CardHeader>
                                <h3 class="tracking-tight text-sm font-medium">{"Subscriptions"}</h3>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="h-4 w-4 text-muted-foreground">
                                    <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
                                    <circle cx="9" cy="7" r="4"></circle>
                                    <path d="M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75"></path>
                                </svg>
                            </CardHeader>
                            <CardContent>
                                <div class="text-2xl font-bold">{"+2350"}</div>
                                <p class="text-xs text-muted-foreground">{"+180.1% from last month"}</p>
                            </CardContent>
                        </Card>
                        <Card>
                            <CardHeader>
                                <h3 class="tracking-tight text-sm font-medium">{"Sales"}</h3>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="h-4 w-4 text-muted-foreground">
                                    <rect width="20" height="14" x="2" y="5" rx="2"></rect>
                                    <path d="M2 10h20"></path>
                                </svg>
                            </CardHeader>
                            <CardContent>
                                <div class="text-2xl font-bold">{"+12,234"}</div>
                                <p class="text-xs text-muted-foreground">{"+19% from last month"}</p>
                            </CardContent>
                        </Card>
                        <Card>
                            <CardHeader>
                                <h3 class="tracking-tight text-sm font-medium">{"Active Now"}</h3>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="h-4 w-4 text-muted-foreground">
                                    <path d="M22 12h-4l-3 9L9 3l-3 9H2"></path>
                                </svg>
                            </CardHeader>
                            <CardContent>
                                <div class="text-2xl font-bold">{"+573"}</div>
                                <p class="text-xs text-muted-foreground">{"+201 since last hour"}</p>
                            </CardContent>
                        </Card>
                    </div>

                    <div class="grid gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-7">
                        <Card class="col-span-1 sm:col-span-1 md:col-span-2 lg:col-span-4">
                            <div class="flex flex-col space-y-1.5 p-6">
                                <h3 class="font-semibold leading-none tracking-tight">{"Overview"}</h3>
                            </div>
                            <div class="p-6 pt-0 pl-2">
                                <Chart x={(0.0, 50.0)} y={(-20.0, 50.0)} height={300} width={500} padding={50}>
                                    <XAxis show={false} ticks={6} />
                                    <YAxis show={false} />
                                    // <LineChart data={data.clone()} width={5} />
                                    // <ScatterChart data={data.clone()} color={"red"} />
                                    <BarChart data={data} color={"black"} width={30}/>
                                </Chart>
                            </div>
                        </Card>
                        <Card class="col-span-1 sm:col-span-1 md:col-span-2 lg:col-span-3">
                            <div class="flex flex-col space-y-1.5 p-6">
                                <h3 class="font-semibold leading-none tracking-tight">{"Recent Sales"}</h3>
                                <p class="text-sm text-muted-foreground">{"You made 265 sales this month."}</p>
                            </div>
                            <div class="p-6 pt-0">
                                <div class="space-y-8">
                                    <div class="flex items-center">
                                        <span class="relative flex shrink-0 overflow-hidden rounded-full h-9 w-9">
                                            <img class="aspect-square h-full w-full" alt="Avatar" src="/assets/profile_lowres.png" />
                                        </span>
                                        <div class="ml-4 space-y-1">
                                            <p class="text-sm font-medium leading-none">{"Olivia Martin"}</p>
                                            <p class="text-sm text-muted-foreground">{"olivia.martin@email.com"}</p>
                                        </div>
                                        <div class="ml-auto font-medium">{"$1,999.00"}</div>
                                    </div>
                                    <div class="flex items-center">
                                        <span class="relative shrink-0 overflow-hidden rounded-full flex h-9 w-9 items-center justify-center space-y-0 border">
                                            <img class="aspect-square h-full w-full" alt="Avatar" src="/assets/profile_lowres.png" />
                                        </span>
                                        <div class="ml-4 space-y-1">
                                            <p class="text-sm font-medium leading-none">{"Jackson Lee"}</p>
                                            <p class="text-sm text-muted-foreground">{"jackson.lee@email.com"}</p>
                                        </div>
                                        <div class="ml-auto font-medium">{"$39.00"}</div>
                                    </div>
                                    <div class="flex items-center">
                                        <span class="relative flex shrink-0 overflow-hidden rounded-full h-9 w-9">
                                            <img class="aspect-square h-full w-full" alt="Avatar" src="/assets/profile_lowres.png" />
                                        </span>
                                        <div class="ml-4 space-y-1">
                                            <p class="text-sm font-medium leading-none">{"Isabella Nguyen"}</p>
                                            <p class="text-sm text-muted-foreground">{"isabella.nguyen@email.com"}</p>
                                        </div>
                                        <div class="ml-auto font-medium">{"$299.00"}</div>
                                    </div>
                                    <div class="flex items-center">
                                        <span class="relative flex shrink-0 overflow-hidden rounded-full h-9 w-9">
                                            <img class="aspect-square h-full w-full" alt="Avatar" src="/assets/profile_lowres.png" />
                                        </span>
                                        <div class="ml-4 space-y-1">
                                            <p class="text-sm font-medium leading-none">{"William Kim"}</p>
                                            <p class="text-sm text-muted-foreground">{"will@email.com"}</p>
                                        </div>
                                        <div class="ml-auto font-medium">{"$99.00"}</div>
                                    </div>
                                    <div class="flex items-center">
                                        <span class="relative flex shrink-0 overflow-hidden rounded-full h-9 w-9">
                                            <img class="aspect-square h-full w-full" alt="Avatar" src="/assets/profile_lowres.png" />
                                        </span>
                                        <div class="ml-4 space-y-1">
                                            <p class="text-sm font-medium leading-none">{"Sofia Davis"}</p>
                                            <p class="text-sm text-muted-foreground">{"sofia.davis@email.com"}</p>
                                        </div>
                                        <div class="ml-auto font-medium">{"$39.00"}</div>
                                    </div>
                                </div>
                            </div>
                        </Card>
                    </div>

                    /*
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Overview"}</CardTitle>
                        </CardHeader>
                        <CardContent>{"This is the overview content."}</CardContent>
                        <div>
                            <Chart x={(0.0, 50.0)} y={(-20.0, 50.0)} height={300} width={500} padding={50}>
                                <XAxis />
                                <YAxis />
                                <LineChart data={data.clone()} width={5} />
                                <ScatterChart data={data.clone()} color={"red"} />
                                <BarChart data={data} color={"blue"} />
                            </Chart>

                        </div>
                    </Card>
                    */
                </TabsContent>
                <TabsContent value="analytics">
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Analytics"}</CardTitle>
                        </CardHeader>
                        <CardContent>{"This is the analytics content."}</CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value="reports">
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Reports"}</CardTitle>
                        </CardHeader>
                        <CardContent>{"This is the reports content."}</CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value="notifications">
                    <Card>
                        <CardHeader>
                            <CardTitle>{"Notifications"}</CardTitle>
                        </CardHeader>
                        <CardContent>{"This is the notifications content."}</CardContent>
                    </Card>
                </TabsContent>
            </Tabs>
            /*
            <div class="grid grid-cols-2 gap-4">
                <Card>
                    <CardTitle>{"Revenue"}</CardTitle>
                </Card>
                <Card>
                    <CardTitle>{"Performance"}</CardTitle>
                </Card>
                <Card class="col-span-2">
                    <CardTitle>{"Performance"}</CardTitle>
                </Card>
                <Card class="col-span-2">
                    <CardTitle>{"Performance"}</CardTitle>
                </Card>
            </div>
            */
        </AppLayout>
    }
}
