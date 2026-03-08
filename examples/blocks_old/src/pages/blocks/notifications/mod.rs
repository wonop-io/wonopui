use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Notifications)]
pub fn notifications() -> Html {
    let notify = use_notify();

    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Notifications"} />
            </Breadcrumb>
            <ExampleBlock title={"Information Notification"}>
                <Button onclick={
                    let notify = notify.clone();
                    Callback::from(move |_| {
                        notify.emit((
                            "Information".to_string(),
                            "This is an informational notification.".to_string(),
                            None
                        ));
                    })
                }>
                    {"Show Information Notification"}
                </Button>
            </ExampleBlock>
            <ExampleBlock title={"Success Notification"}>
                <Button onclick={
                    let notify = notify.clone();
                    Callback::from(move |_| {
                        notify.emit((
                            "Success".to_string(),
                            "This is a success notification.".to_string(),
                            None
                        ));
                    })
                }>
                    {"Show Success Notification"}
                </Button>
            </ExampleBlock>
            <ExampleBlock title={"Warning Notification"}>
                <Button onclick={
                    let notify = notify.clone();
                    Callback::from(move |_| {
                        notify.emit((
                            "Warning".to_string(),
                            "This is a warning notification.".to_string(),
                            None
                        ));
                    })
                }>
                    {"Show Warning Notification"}
                </Button>
            </ExampleBlock>
            <ExampleBlock title={"Error Notification"}>
                <Button onclick={
                    let notify = notify.clone();
                    Callback::from(move |_| {
                        notify.emit((
                            "Error".to_string(),
                            "This is an error notification.".to_string(),
                            None
                        ));
                    })
                }>
                    {"Show Error Notification"}
                </Button>
            </ExampleBlock>
        </MainContent>
    }
}
