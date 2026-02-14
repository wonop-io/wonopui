use wonopui::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| {
            if *counter > 0 {
                counter.set(*counter - 1);
            }
        })
    };

    html! {
        <div class="p-8">
            <Heading level={HeadingLevel::H1}>{"WonopUI Basic Example"}</Heading>

            <Card>
                <CardHeader>
                    <CardTitle>{"Counter Example"}</CardTitle>
                </CardHeader>
                <CardContent>
                    <Paragraph>{format!("Current count: {}", *counter)}</Paragraph>

                    <div class="flex gap-2 mt-4">
                        <Button
                            variant={ButtonVariant::Primary}
                            onclick={increment}
                        >
                            {"Increment"}
                        </Button>

                        <Button
                            variant={ButtonVariant::Secondary}
                            onclick={decrement}
                            disabled={*counter == 0}
                        >
                            {"Decrement"}
                        </Button>
                    </div>

                    <div class="mt-4">
                        {if *counter >= 10 {
                            html! {
                                <Alert variant={AlertVariant::Success} title="Success!">
                                    {"You've reached 10 or more!"}
                                </Alert>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
