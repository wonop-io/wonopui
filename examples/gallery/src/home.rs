use wonopui::prelude::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <div class="py-12 text-center">
                <h1 class="text-4xl font-bold mb-4 text-zinc-900 dark:text-white">
                    { "WonopUI Component Gallery" }
                </h1>
                <p class="text-xl text-zinc-600 dark:text-zinc-400 mb-8">
                    { "A comprehensive collection of UI components built with Yew and Tailwind CSS" }
                </p>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 text-left mt-12">
                    <Card>
                        <CardHeader>
                            <CardTitle>{ "Form Components" }</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-zinc-600 dark:text-zinc-400">
                                { "Input, Select, Checkbox, Switch, and more form controls" }
                            </p>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>{ "Layout Components" }</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-zinc-600 dark:text-zinc-400">
                                { "Container, Card, Sidebar, and responsive layout helpers" }
                            </p>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>{ "Feedback Components" }</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-zinc-600 dark:text-zinc-400">
                                { "Alert, Spinner, Progress, and status indicators" }
                            </p>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </Container>
    }
}
