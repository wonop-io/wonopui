use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(Dialogs)]
pub fn dialogs() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Dialogs"} />
            </Breadcrumb>
            <ExampleBlock title={"Dialog 1"}>
                <Dialog1 />
            </ExampleBlock>
            <ExampleBlock title={"Dialog 2"}>
                <Dialog2 />
            </ExampleBlock>
            <ExampleBlock title={"Dialog 3"}>
                <Dialog3 />
            </ExampleBlock>
            <ExampleBlock title={"Dialog 4"}>
                <Dialog4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(Dialog1)]
pub fn dialog1() -> Html {
    html! {
        <DialogProvider>
            <DialogTrigger id="dialog1">
                <Button>{"Open Dialog 1"}</Button>
            </DialogTrigger>
            <Dialog id="dialog1">
                <DialogHeader>
                    <DialogTitle>{"Dialog 1 Title"}</DialogTitle>
                    <DialogClose>{"X"}</DialogClose>
                </DialogHeader>
                <DialogBody>{"This is the description for Dialog 1."}</DialogBody>
                <DialogFooter>
                    <DialogClose>
                        <Button>{"Close"}</Button>
                    </DialogClose>
                </DialogFooter>
            </Dialog>
        </DialogProvider>
    }
}

#[function_component(Dialog2)]
pub fn dialog2() -> Html {
    html! {
        <DialogProvider>
            <DialogTrigger id="dialog2">
                <Button>{"Open Dialog 2"}</Button>
            </DialogTrigger>
            <Dialog id="dialog2">
                <DialogHeader>
                    <DialogTitle>{"Dialog 2 Title"}</DialogTitle>
                    <DialogClose>{"X"}</DialogClose>
                </DialogHeader>
                <DialogBody>{"This is the description for Dialog 2."}</DialogBody>
                <DialogFooter>
                    <DialogClose>
                        <Button>{"Close"}</Button>
                    </DialogClose>
                </DialogFooter>
            </Dialog>
        </DialogProvider>
    }
}

#[function_component(Dialog3)]
pub fn dialog3() -> Html {
    html! {
        <DialogProvider>
            <DialogTrigger id="dialog3">
                <Button>{"Open Dialog 3"}</Button>
            </DialogTrigger>
            <Dialog id="dialog3">
                <DialogHeader>
                    <DialogTitle>{"Dialog 3 Title"}</DialogTitle>
                    <DialogClose>{"X"}</DialogClose>
                </DialogHeader>
                <DialogBody>{"This is the description for Dialog 3."}</DialogBody>
                <DialogFooter>
                    <DialogClose>
                        <Button>{"Close"}</Button>
                    </DialogClose>
                </DialogFooter>
            </Dialog>
        </DialogProvider>
    }
}

#[function_component(Dialog4)]
pub fn dialog4() -> Html {
    html! {
        <DialogProvider>
            <DialogTrigger id="dialog4">
                <Button>{"Open Dialog 4"}</Button>
            </DialogTrigger>
            <Dialog id="dialog4">
                <DialogHeader>
                    <DialogTitle>{"Dialog 4 Title"}</DialogTitle>
                    <DialogClose>{"X"}</DialogClose>
                </DialogHeader>
                <DialogBody>{"This is the description for Dialog 4."}</DialogBody>
                <DialogFooter>
                    <DialogClose>
                        <Button>{"Close"}</Button>
                    </DialogClose>
                </DialogFooter>
            </Dialog>
        </DialogProvider>
    }
}
