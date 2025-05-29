use wonopui::prelude::{
    Alert, AlertType, Button, ButtonSize, ButtonVariant, Card, CardContent, CardHeader, CardTitle,
    Paragraph, ThemeProvider, H1,
};
use yew::prelude::*;

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
        <ThemeProvider>
            <div class="p-8">
                <H1>{"WonopUI Basic Example"}</H1>

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
                                    <Alert alert_type={AlertType::Success}>
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
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
