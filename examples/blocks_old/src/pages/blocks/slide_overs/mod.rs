use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(SlideOvers)]
pub fn slide_overs() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Slide Overs"} />
            </Breadcrumb>
            <ExampleBlock title={"Slide Over 1"}>
                <SlideOver1 />
            </ExampleBlock>
            <ExampleBlock title={"Slide Over 2"}>
                <SlideOver2 />
            </ExampleBlock>
            <ExampleBlock title={"Slide Over 3"}>
                <SlideOver3 />
            </ExampleBlock>
            <ExampleBlock title={"Slide Over 4"}>
                <SlideOver4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(SlideOver1)]
pub fn slide_over1() -> Html {
    html! {
        <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
            <Drawer<String>>
                <DrawerHeader>
                    <DrawerTitle>{"Slide Over 1"}</DrawerTitle>
                    <DrawerClose<String>>{"Close"}</DrawerClose<String>>
                </DrawerHeader>
                <DrawerDescription>
                    <p>{"This is the content of Slide Over 1."}</p>
                </DrawerDescription>
                <DrawerFooter>
                    <DrawerClose<String>>
                        <Button>{"Close"}</Button>
                    </DrawerClose<String>>
                </DrawerFooter>
            </Drawer<String>>
        })}>
            <DrawerTrigger<String> drawer={"slide_over_1".to_string()}>
                <Button>{"Open Slide Over 1"}</Button>
            </DrawerTrigger<String>>
        </DrawerProvider<String>>
    }
}

#[function_component(SlideOver2)]
pub fn slide_over2() -> Html {
    html! {
        <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
            <Drawer<String>>
                <DrawerHeader>
                    <DrawerTitle>{"Slide Over 2"}</DrawerTitle>
                    <DrawerClose<String>>{"Close"}</DrawerClose<String>>
                </DrawerHeader>
                <DrawerDescription>
                    <p>{"This is the content of Slide Over 2."}</p>
                </DrawerDescription>
                <DrawerFooter>
                    <DrawerClose<String>>
                        <Button>{"Close"}</Button>
                    </DrawerClose<String>>
                </DrawerFooter>
            </Drawer<String>>
        })}>
            <DrawerTrigger<String> drawer={"slide_over_2".to_string()}>
                <Button>{"Open Slide Over 2"}</Button>
            </DrawerTrigger<String>>
        </DrawerProvider<String>>
    }
}

#[function_component(SlideOver3)]
pub fn slide_over3() -> Html {
    html! {
        <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
            <Drawer<String>>
                <DrawerHeader>
                    <DrawerTitle>{"Slide Over 3"}</DrawerTitle>
                    <DrawerClose<String>>{"Close"}</DrawerClose<String>>
                </DrawerHeader>
                <DrawerDescription>
                    <p>{"This is the content of Slide Over 3."}</p>
                </DrawerDescription>
                <DrawerFooter>
                    <DrawerClose<String>>
                        <Button>{"Close"}</Button>
                    </DrawerClose<String>>
                </DrawerFooter>
            </Drawer<String>>
        })}>
            <DrawerTrigger<String> drawer={"slide_over_3".to_string()}>
                <Button>{"Open Slide Over 3"}</Button>
            </DrawerTrigger<String>>
        </DrawerProvider<String>>
    }
}

#[function_component(SlideOver4)]
pub fn slide_over4() -> Html {
    html! {
        <DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
            <Drawer<String>>
                <DrawerHeader>
                    <DrawerTitle>{"Slide Over 4"}</DrawerTitle>
                    <DrawerClose<String>>{"Close"}</DrawerClose<String>>
                </DrawerHeader>
                <DrawerDescription>
                    <p>{"This is the content of Slide Over 4."}</p>
                </DrawerDescription>
                <DrawerFooter>
                    <DrawerClose<String>>
                        <Button>{"Close"}</Button>
                    </DrawerClose<String>>
                </DrawerFooter>
            </Drawer<String>>
        })}>
            <DrawerTrigger<String> drawer={"slide_over_4".to_string()}>
                <Button>{"Open Slide Over 4"}</Button>
            </DrawerTrigger<String>>
        </DrawerProvider<String>>
    }
}
