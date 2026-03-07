//! Feedback blocks - Dialogs, Slide-overs, Notifications, Empty States
//!
//! Reimplementation of blocks_old/dialogs, slide_overs, notifications, empty_states
//! using wonopui components with shadcn styling.

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_dialog::{Dialog, DialogProvider, DialogTrigger, DialogHeader, DialogTitle, DialogClose, DialogBody, DialogFooter};
use wonopui::wonopui_drawer::{DrawerProvider, DrawerTrigger, Drawer, DrawerHeader, DrawerTitle, DrawerDescription, DrawerContent, DrawerFooter, DrawerClose, DrawerSide};
use wonopui::wonopui_notification::{NotificationProvider, use_notify};
use wonopui::wonopui_input::Input;
use wonopui::wonopui_label::Label;
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardContent, CardDescription};
use crate::blocks::BlockPreview;

/// Feedback category page
#[function_component(FeedbackBlocks)]
pub fn feedback_blocks() -> Html {
    html! {
        <Container class="py-12">
            // Header
            <div class="mb-8">
                <nav class="mb-4">
                    <a href="/blocks" class="text-sm text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-200">
                        {"← Back to Blocks"}
                    </a>
                </nav>
                <h1 class="text-3xl font-bold tracking-tight text-zinc-900 dark:text-white">
                    {"Feedback"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Dialogs, slide-overs, notifications, and user feedback patterns."}
                </p>
            </div>
            
            // Blocks
            <div class="space-y-16">
                // Dialogs section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Dialogs"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Dialog"
                            description="Basic dialog with title and description."
                            code={DIALOG_SIMPLE_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <DialogSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Confirmation Dialog"
                            description="Dialog for confirming destructive actions."
                            code={DIALOG_CONFIRM_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <DialogConfirm />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Dialog with Form"
                            description="Dialog containing a form for user input."
                            code={DIALOG_FORM_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <DialogForm />
                        </BlockPreview>
                    </div>
                </div>
                
                // Slide-overs section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Slide-overs"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Slide-over from Right"
                            description="Panel that slides in from the right side."
                            code={SLIDEOVER_RIGHT_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <SlideoverRight />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Slide-over with Form"
                            description="Slide-over containing a detailed form."
                            code={SLIDEOVER_FORM_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <SlideoverForm />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Bottom Drawer"
                            description="Drawer that slides up from the bottom."
                            code={DRAWER_BOTTOM_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <DrawerBottom />
                        </BlockPreview>
                    </div>
                </div>
                
                // Notifications section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Notifications"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Toast Notification"
                            description="Simple toast notification with title and description."
                            code={NOTIFICATION_SIMPLE_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <NotificationSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Notification with Action"
                            description="Toast notification with an action button."
                            code={NOTIFICATION_ACTION_CODE}
                            min_height={200}
                            isolate={true}
                        >
                            <NotificationWithAction />
                        </BlockPreview>
                    </div>
                </div>
                
                // Empty States section
                <div>
                    <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Empty States"}</h2>
                    <div class="space-y-8">
                        <BlockPreview 
                            title="Simple Empty State"
                            description="Basic empty state with icon and message."
                            code={EMPTY_SIMPLE_CODE}
                            min_height={350}
                            isolate={true}
                        >
                            <EmptyStateSimple />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Empty State with Action"
                            description="Empty state with call-to-action button."
                            code={EMPTY_ACTION_CODE}
                            min_height={400}
                            isolate={true}
                        >
                            <EmptyStateWithAction />
                        </BlockPreview>
                        
                        <BlockPreview 
                            title="Empty State in Card"
                            description="Empty state displayed within a card container."
                            code={EMPTY_CARD_CODE}
                            min_height={450}
                            isolate={true}
                        >
                            <EmptyStateCard />
                        </BlockPreview>
                    </div>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Dialog Examples
// =============================================================================

#[function_component(DialogSimple)]
fn dialog_simple() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <DialogProvider>
                <DialogTrigger id="simple-dialog">
                    <Button variant={ButtonVariant::Primary}>{"Open Dialog"}</Button>
                </DialogTrigger>
                <Dialog id="simple-dialog">
                    <DialogHeader>
                        <DialogTitle>{"Edit profile"}</DialogTitle>
                        <DialogClose>{"×"}</DialogClose>
                    </DialogHeader>
                    <DialogBody>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Make changes to your profile here. Click save when you're done."}
                        </p>
                    </DialogBody>
                    <DialogFooter>
                        <DialogClose>
                            <Button variant={ButtonVariant::Secondary}>{"Cancel"}</Button>
                        </DialogClose>
                        <Button variant={ButtonVariant::Primary}>{"Save changes"}</Button>
                    </DialogFooter>
                </Dialog>
            </DialogProvider>
        </div>
    }
}

#[function_component(DialogConfirm)]
fn dialog_confirm() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <DialogProvider>
                <DialogTrigger id="confirm-dialog">
                    <Button variant={ButtonVariant::Danger}>{"Delete Account"}</Button>
                </DialogTrigger>
                <Dialog id="confirm-dialog">
                    <DialogHeader>
                        <DialogTitle>{"Are you absolutely sure?"}</DialogTitle>
                        <DialogClose>{"×"}</DialogClose>
                    </DialogHeader>
                    <DialogBody>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"This action cannot be undone. This will permanently delete your account and remove your data from our servers."}
                        </p>
                    </DialogBody>
                    <DialogFooter>
                        <DialogClose>
                            <Button variant={ButtonVariant::Secondary}>{"Cancel"}</Button>
                        </DialogClose>
                        <Button variant={ButtonVariant::Danger}>{"Yes, delete account"}</Button>
                    </DialogFooter>
                </Dialog>
            </DialogProvider>
        </div>
    }
}

#[function_component(DialogForm)]
fn dialog_form() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <DialogProvider>
                <DialogTrigger id="form-dialog">
                    <Button variant={ButtonVariant::Primary}>{"Edit Profile"}</Button>
                </DialogTrigger>
                <Dialog id="form-dialog">
                    <DialogHeader>
                        <DialogTitle>{"Edit profile"}</DialogTitle>
                        <DialogClose>{"×"}</DialogClose>
                    </DialogHeader>
                    <DialogBody>
                        <div class="space-y-4">
                            <div class="space-y-2">
                                <Label for_id="name">{"Name"}</Label>
                                <Input id="name" placeholder="Enter your name" />
                            </div>
                            <div class="space-y-2">
                                <Label for_id="email">{"Email"}</Label>
                                <Input id="email" kind="email" placeholder="Enter your email" />
                            </div>
                        </div>
                    </DialogBody>
                    <DialogFooter>
                        <DialogClose>
                            <Button variant={ButtonVariant::Secondary}>{"Cancel"}</Button>
                        </DialogClose>
                        <Button variant={ButtonVariant::Primary}>{"Save"}</Button>
                    </DialogFooter>
                </Dialog>
            </DialogProvider>
        </div>
    }
}

// =============================================================================
// Slide-over Examples (using Drawer)
// =============================================================================

#[derive(Clone, PartialEq)]
enum SlideoverType {
    Details,
}

#[function_component(SlideoverRight)]
fn slideover_right() -> Html {
    let render = Callback::from(|_drawer: SlideoverType| {
        html! {
            <Drawer<SlideoverType>>
                <DrawerHeader>
                    <DrawerTitle>{"Panel title"}</DrawerTitle>
                    <DrawerDescription>{"A description of the panel contents"}</DrawerDescription>
                </DrawerHeader>
                <DrawerContent>
                    <div class="space-y-4">
                        <p class="text-sm text-zinc-600 dark:text-zinc-400">
                            {"This is the slide-over panel content. You can put any content here - forms, lists, details, etc."}
                        </p>
                        <div class="rounded-lg bg-zinc-100 dark:bg-zinc-800 p-4">
                            <p class="text-sm text-zinc-600 dark:text-zinc-400">{"Some placeholder content..."}</p>
                        </div>
                    </div>
                </DrawerContent>
                <DrawerFooter>
                    <DrawerClose<SlideoverType>>
                        <Button variant={ButtonVariant::Primary} class="w-full">{"Submit"}</Button>
                    </DrawerClose<SlideoverType>>
                    <DrawerClose<SlideoverType>>
                        <Button variant={ButtonVariant::Secondary} class="w-full">{"Cancel"}</Button>
                    </DrawerClose<SlideoverType>>
                </DrawerFooter>
            </Drawer<SlideoverType>>
        }
    });
    
    html! {
        <div class="flex items-center justify-center p-8">
            <DrawerProvider<SlideoverType> side={DrawerSide::Right} {render}>
                <DrawerTrigger<SlideoverType> drawer={SlideoverType::Details}>
                    <Button variant={ButtonVariant::Primary}>{"Open Panel"}</Button>
                </DrawerTrigger<SlideoverType>>
            </DrawerProvider<SlideoverType>>
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum FormSlideoverType {
    Edit,
}

#[function_component(SlideoverForm)]
fn slideover_form() -> Html {
    let render = Callback::from(|_drawer: FormSlideoverType| {
        html! {
            <Drawer<FormSlideoverType>>
                <DrawerHeader>
                    <DrawerTitle>{"Edit Project"}</DrawerTitle>
                    <DrawerDescription>{"Make changes to your project settings."}</DrawerDescription>
                </DrawerHeader>
                <DrawerContent>
                    <div class="space-y-6">
                        <div class="space-y-2">
                            <Label for_id="project-name">{"Project name"}</Label>
                            <Input id="project-name" placeholder="My Project" />
                        </div>
                        <div class="space-y-2">
                            <Label for_id="project-desc">{"Description"}</Label>
                            <Input id="project-desc" placeholder="A short description..." />
                        </div>
                        <div class="space-y-2">
                            <Label for_id="project-url">{"Website URL"}</Label>
                            <Input id="project-url" kind="url" placeholder="https://example.com" />
                        </div>
                        <div class="pt-4 border-t border-zinc-200 dark:border-zinc-800">
                            <h4 class="text-sm font-medium text-zinc-900 dark:text-white mb-3">{"Team Members"}</h4>
                            <div class="space-y-2">
                                <div class="flex items-center justify-between p-2 rounded-lg bg-zinc-50 dark:bg-zinc-900">
                                    <div class="flex items-center gap-3">
                                        <div class="size-8 rounded-full bg-zinc-200 dark:bg-zinc-700"></div>
                                        <span class="text-sm text-zinc-900 dark:text-white">{"John Doe"}</span>
                                    </div>
                                    <span class="text-xs text-zinc-500">{"Owner"}</span>
                                </div>
                                <div class="flex items-center justify-between p-2 rounded-lg bg-zinc-50 dark:bg-zinc-900">
                                    <div class="flex items-center gap-3">
                                        <div class="size-8 rounded-full bg-zinc-200 dark:bg-zinc-700"></div>
                                        <span class="text-sm text-zinc-900 dark:text-white">{"Jane Smith"}</span>
                                    </div>
                                    <span class="text-xs text-zinc-500">{"Editor"}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </DrawerContent>
                <DrawerFooter>
                    <DrawerClose<FormSlideoverType>>
                        <Button variant={ButtonVariant::Primary} class="w-full">{"Save changes"}</Button>
                    </DrawerClose<FormSlideoverType>>
                    <DrawerClose<FormSlideoverType>>
                        <Button variant={ButtonVariant::Secondary} class="w-full">{"Cancel"}</Button>
                    </DrawerClose<FormSlideoverType>>
                </DrawerFooter>
            </Drawer<FormSlideoverType>>
        }
    });
    
    html! {
        <div class="flex items-center justify-center p-8">
            <DrawerProvider<FormSlideoverType> side={DrawerSide::Right} {render}>
                <DrawerTrigger<FormSlideoverType> drawer={FormSlideoverType::Edit}>
                    <Button variant={ButtonVariant::Primary}>{"Edit Project"}</Button>
                </DrawerTrigger<FormSlideoverType>>
            </DrawerProvider<FormSlideoverType>>
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum BottomDrawerType {
    Menu,
}

#[function_component(DrawerBottom)]
fn drawer_bottom() -> Html {
    let render = Callback::from(|_drawer: BottomDrawerType| {
        html! {
            <Drawer<BottomDrawerType>>
                <DrawerHeader>
                    <DrawerTitle>{"Quick Actions"}</DrawerTitle>
                    <DrawerDescription>{"Choose an action below"}</DrawerDescription>
                </DrawerHeader>
                <DrawerContent>
                    <div class="space-y-2">
                        <button class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-600 dark:text-zinc-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                            </svg>
                            <span class="text-sm font-medium text-zinc-900 dark:text-white">{"Create new"}</span>
                        </button>
                        <button class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-600 dark:text-zinc-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
                            </svg>
                            <span class="text-sm font-medium text-zinc-900 dark:text-white">{"Edit"}</span>
                        </button>
                        <button class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-600 dark:text-zinc-400">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M7.217 10.907a2.25 2.25 0 1 0 0 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186 9.566-5.314m-9.566 7.5 9.566 5.314m0 0a2.25 2.25 0 1 0 3.935 2.186 2.25 2.25 0 0 0-3.935-2.186Zm0-12.814a2.25 2.25 0 1 0 3.933-2.185 2.25 2.25 0 0 0-3.933 2.185Z" />
                            </svg>
                            <span class="text-sm font-medium text-zinc-900 dark:text-white">{"Share"}</span>
                        </button>
                        <button class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors text-red-600 dark:text-red-400">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                            </svg>
                            <span class="text-sm font-medium">{"Delete"}</span>
                        </button>
                    </div>
                </DrawerContent>
            </Drawer<BottomDrawerType>>
        }
    });
    
    html! {
        <div class="flex items-center justify-center p-8">
            <DrawerProvider<BottomDrawerType> side={DrawerSide::Bottom} {render}>
                <DrawerTrigger<BottomDrawerType> drawer={BottomDrawerType::Menu}>
                    <Button variant={ButtonVariant::Primary}>{"Open Bottom Drawer"}</Button>
                </DrawerTrigger<BottomDrawerType>>
            </DrawerProvider<BottomDrawerType>>
        </div>
    }
}

// =============================================================================
// Notification Examples
// =============================================================================

#[function_component(NotificationSimple)]
fn notification_simple() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <NotificationProvider>
                <NotificationTriggerSimple />
            </NotificationProvider>
        </div>
    }
}

#[function_component(NotificationTriggerSimple)]
fn notification_trigger_simple() -> Html {
    let notify = use_notify();
    
    let onclick = {
        let notify = notify.clone();
        Callback::from(move |_| {
            notify.emit((
                "Scheduled: Catch up".to_string(),
                "Friday, February 10, 2024 at 5:57 PM".to_string(),
                None,
            ));
        })
    };
    
    html! {
        <Button variant={ButtonVariant::Primary} {onclick}>{"Show Notification"}</Button>
    }
}

#[function_component(NotificationWithAction)]
fn notification_with_action() -> Html {
    html! {
        <div class="flex items-center justify-center p-8">
            <NotificationProvider>
                <NotificationTriggerAction />
            </NotificationProvider>
        </div>
    }
}

#[function_component(NotificationTriggerAction)]
fn notification_trigger_action() -> Html {
    let notify = use_notify();
    
    let onclick = {
        let notify = notify.clone();
        Callback::from(move |_| {
            let action = html! {
                <Button variant={ButtonVariant::Secondary} size={ButtonSize::Small}>{"Undo"}</Button>
            };
            notify.emit((
                "Event deleted".to_string(),
                "The event has been removed from your calendar.".to_string(),
                Some(action),
            ));
        })
    };
    
    html! {
        <Button variant={ButtonVariant::Danger} {onclick}>{"Delete Event"}</Button>
    }
}

// =============================================================================
// Empty State Examples
// =============================================================================

#[function_component(EmptyStateSimple)]
fn empty_state_simple() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="flex flex-col items-center justify-center py-12 text-center">
                <div class="rounded-full bg-zinc-100 dark:bg-zinc-800 p-4 mb-4">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8 text-zinc-400 dark:text-zinc-500">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H6.911a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661Z" />
                    </svg>
                </div>
                <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">{"No messages"}</h3>
                <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400 max-w-sm">
                    {"Your inbox is empty. When you receive new messages, they will appear here."}
                </p>
            </div>
        </div>
    }
}

#[function_component(EmptyStateWithAction)]
fn empty_state_with_action() -> Html {
    html! {
        <div class="w-full bg-white dark:bg-zinc-950 p-8">
            <div class="flex flex-col items-center justify-center py-12 text-center">
                <div class="rounded-full bg-zinc-100 dark:bg-zinc-800 p-4 mb-4">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8 text-zinc-400 dark:text-zinc-500">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                    </svg>
                </div>
                <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">{"No projects"}</h3>
                <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400 max-w-sm">
                    {"Get started by creating a new project."}
                </p>
                <Button variant={ButtonVariant::Primary} class="mt-6">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4 mr-2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                    {"New Project"}
                </Button>
            </div>
        </div>
    }
}

#[function_component(EmptyStateCard)]
fn empty_state_card() -> Html {
    html! {
        <div class="w-full bg-zinc-50 dark:bg-zinc-900 p-8 flex items-center justify-center">
            <Card class="w-full max-w-lg">
                <CardContent class="pt-6">
                    <div class="flex flex-col items-center justify-center py-8 text-center">
                        <div class="rounded-lg border-2 border-dashed border-zinc-300 dark:border-zinc-700 p-6 mb-4">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12 text-zinc-400 dark:text-zinc-500">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6 18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0 0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Z" />
                            </svg>
                        </div>
                        <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">{"No team members"}</h3>
                        <p class="mt-2 text-sm text-zinc-500 dark:text-zinc-400 max-w-xs">
                            {"You haven't added any team members to your workspace yet."}
                        </p>
                        <div class="flex gap-3 mt-6">
                            <Button variant={ButtonVariant::Secondary}>{"Import"}</Button>
                            <Button variant={ButtonVariant::Primary}>{"Add member"}</Button>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const DIALOG_SIMPLE_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_dialog::*;

html! {
    <DialogProvider>
        <DialogTrigger id="simple-dialog">
            <Button variant={ButtonVariant::Primary}>{"Open Dialog"}</Button>
        </DialogTrigger>
        <Dialog id="simple-dialog">
            <DialogHeader>
                <DialogTitle>{"Edit profile"}</DialogTitle>
                <DialogClose>{"×"}</DialogClose>
            </DialogHeader>
            <DialogBody>
                <p class="text-sm text-zinc-500">
                    {"Make changes to your profile here."}
                </p>
            </DialogBody>
            <DialogFooter>
                <DialogClose>
                    <Button variant={ButtonVariant::Secondary}>{"Cancel"}</Button>
                </DialogClose>
                <Button variant={ButtonVariant::Primary}>{"Save changes"}</Button>
            </DialogFooter>
        </Dialog>
    </DialogProvider>
}
"##;

const DIALOG_CONFIRM_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_dialog::*;

html! {
    <DialogProvider>
        <DialogTrigger id="confirm-dialog">
            <Button variant={ButtonVariant::Danger}>{"Delete Account"}</Button>
        </DialogTrigger>
        <Dialog id="confirm-dialog">
            <DialogHeader>
                <DialogTitle>{"Are you absolutely sure?"}</DialogTitle>
                <DialogClose>{"×"}</DialogClose>
            </DialogHeader>
            <DialogBody>
                <p class="text-sm text-zinc-500">
                    {"This action cannot be undone."}
                </p>
            </DialogBody>
            <DialogFooter>
                <DialogClose>
                    <Button variant={ButtonVariant::Secondary}>{"Cancel"}</Button>
                </DialogClose>
                <Button variant={ButtonVariant::Danger}>{"Yes, delete"}</Button>
            </DialogFooter>
        </Dialog>
    </DialogProvider>
}
"##;

const DIALOG_FORM_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_dialog::*;
use wonopui::wonopui_input::Input;
use wonopui::wonopui_label::Label;

html! {
    <DialogProvider>
        <DialogTrigger id="form-dialog">
            <Button variant={ButtonVariant::Primary}>{"Edit Profile"}</Button>
        </DialogTrigger>
        <Dialog id="form-dialog">
            <DialogHeader>
                <DialogTitle>{"Edit profile"}</DialogTitle>
                <DialogClose>{"×"}</DialogClose>
            </DialogHeader>
            <DialogBody>
                <div class="space-y-4">
                    <div class="space-y-2">
                        <Label for_id="name">{"Name"}</Label>
                        <Input id="name" placeholder="Enter your name" />
                    </div>
                    <div class="space-y-2">
                        <Label for_id="email">{"Email"}</Label>
                        <Input id="email" kind="email" placeholder="Enter email" />
                    </div>
                </div>
            </DialogBody>
            <DialogFooter>
                <DialogClose>
                    <Button variant={ButtonVariant::Secondary}>{"Cancel"}</Button>
                </DialogClose>
                <Button variant={ButtonVariant::Primary}>{"Save"}</Button>
            </DialogFooter>
        </Dialog>
    </DialogProvider>
}
"##;

const SLIDEOVER_RIGHT_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_drawer::*;

#[derive(Clone, PartialEq)]
enum SlideoverType { Details }

let render = Callback::from(|_drawer: SlideoverType| {
    html! {
        <Drawer<SlideoverType>>
            <DrawerHeader>
                <DrawerTitle>{"Panel title"}</DrawerTitle>
                <DrawerDescription>{"Panel description"}</DrawerDescription>
            </DrawerHeader>
            <DrawerContent>
                <p>{"Your content here..."}</p>
            </DrawerContent>
            <DrawerFooter>
                <DrawerClose<SlideoverType>>
                    <Button variant={ButtonVariant::Primary} class="w-full">
                        {"Submit"}
                    </Button>
                </DrawerClose<SlideoverType>>
            </DrawerFooter>
        </Drawer<SlideoverType>>
    }
});

html! {
    <DrawerProvider<SlideoverType> side={DrawerSide::Right} {render}>
        <DrawerTrigger<SlideoverType> drawer={SlideoverType::Details}>
            <Button variant={ButtonVariant::Primary}>{"Open Panel"}</Button>
        </DrawerTrigger<SlideoverType>>
    </DrawerProvider<SlideoverType>>
}
"##;

const SLIDEOVER_FORM_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_drawer::*;
use wonopui::wonopui_input::Input;
use wonopui::wonopui_label::Label;

#[derive(Clone, PartialEq)]
enum FormType { Edit }

let render = Callback::from(|_drawer: FormType| {
    html! {
        <Drawer<FormType>>
            <DrawerHeader>
                <DrawerTitle>{"Edit Project"}</DrawerTitle>
            </DrawerHeader>
            <DrawerContent>
                <div class="space-y-4">
                    <div>
                        <Label for_id="name">{"Name"}</Label>
                        <Input id="name" placeholder="Project name" />
                    </div>
                </div>
            </DrawerContent>
            <DrawerFooter>
                <DrawerClose<FormType>>
                    <Button variant={ButtonVariant::Primary}>{"Save"}</Button>
                </DrawerClose<FormType>>
            </DrawerFooter>
        </Drawer<FormType>>
    }
});

html! {
    <DrawerProvider<FormType> side={DrawerSide::Right} {render}>
        <DrawerTrigger<FormType> drawer={FormType::Edit}>
            <Button>{"Edit"}</Button>
        </DrawerTrigger<FormType>>
    </DrawerProvider<FormType>>
}
"##;

const DRAWER_BOTTOM_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_drawer::*;

#[derive(Clone, PartialEq)]
enum MenuType { Menu }

let render = Callback::from(|_drawer: MenuType| {
    html! {
        <Drawer<MenuType>>
            <DrawerHeader>
                <DrawerTitle>{"Quick Actions"}</DrawerTitle>
            </DrawerHeader>
            <DrawerContent>
                <div class="space-y-2">
                    <button class="w-full p-3 text-left rounded-lg hover:bg-zinc-100">
                        {"Create new"}
                    </button>
                    <button class="w-full p-3 text-left rounded-lg hover:bg-zinc-100">
                        {"Edit"}
                    </button>
                </div>
            </DrawerContent>
        </Drawer<MenuType>>
    }
});

html! {
    <DrawerProvider<MenuType> side={DrawerSide::Bottom} {render}>
        <DrawerTrigger<MenuType> drawer={MenuType::Menu}>
            <Button>{"Open Menu"}</Button>
        </DrawerTrigger<MenuType>>
    </DrawerProvider<MenuType>>
}
"##;

const NOTIFICATION_SIMPLE_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_notification::{NotificationProvider, use_notify};

#[function_component(NotificationDemo)]
fn notification_demo() -> Html {
    html! {
        <NotificationProvider>
            <NotificationTrigger />
        </NotificationProvider>
    }
}

#[function_component(NotificationTrigger)]
fn notification_trigger() -> Html {
    let notify = use_notify();
    
    let onclick = {
        let notify = notify.clone();
        Callback::from(move |_| {
            notify.emit((
                "Scheduled: Catch up".to_string(),
                "Friday, February 10 at 5:57 PM".to_string(),
                None,
            ));
        })
    };
    
    html! {
        <Button variant={ButtonVariant::Primary} {onclick}>
            {"Show Notification"}
        </Button>
    }
}
"##;

const NOTIFICATION_ACTION_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_notification::{NotificationProvider, use_notify};

#[function_component(NotificationTrigger)]
fn notification_trigger() -> Html {
    let notify = use_notify();
    
    let onclick = {
        let notify = notify.clone();
        Callback::from(move |_| {
            let action = html! {
                <Button variant={ButtonVariant::Secondary} size={ButtonSize::Small}>
                    {"Undo"}
                </Button>
            };
            notify.emit((
                "Event deleted".to_string(),
                "The event has been removed.".to_string(),
                Some(action),
            ));
        })
    };
    
    html! {
        <Button variant={ButtonVariant::Danger} {onclick}>
            {"Delete Event"}
        </Button>
    }
}
"##;

const EMPTY_SIMPLE_CODE: &str = r##"// Simple empty state
html! {
    <div class="flex flex-col items-center justify-center py-12 text-center">
        <div class="rounded-full bg-zinc-100 dark:bg-zinc-800 p-4 mb-4">
            // Inbox icon
            <svg class="size-8 text-zinc-400">...</svg>
        </div>
        <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">
            {"No messages"}
        </h3>
        <p class="mt-2 text-sm text-zinc-500">
            {"Your inbox is empty."}
        </p>
    </div>
}
"##;

const EMPTY_ACTION_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};

html! {
    <div class="flex flex-col items-center justify-center py-12 text-center">
        <div class="rounded-full bg-zinc-100 p-4 mb-4">
            <svg class="size-8 text-zinc-400">...</svg>
        </div>
        <h3 class="text-lg font-semibold">{"No projects"}</h3>
        <p class="mt-2 text-sm text-zinc-500">
            {"Get started by creating a new project."}
        </p>
        <Button variant={ButtonVariant::Primary} class="mt-6">
            <svg class="size-4 mr-2">...</svg>
            {"New Project"}
        </Button>
    </div>
}
"##;

const EMPTY_CARD_CODE: &str = r##"use wonopui::wonopui_button::{Button, ButtonVariant};
use wonopui::wonopui_card::{Card, CardContent};

html! {
    <Card class="w-full max-w-lg">
        <CardContent class="pt-6">
            <div class="flex flex-col items-center justify-center py-8 text-center">
                <div class="rounded-lg border-2 border-dashed border-zinc-300 p-6 mb-4">
                    <svg class="size-12 text-zinc-400">...</svg>
                </div>
                <h3 class="text-lg font-semibold">{"No team members"}</h3>
                <p class="mt-2 text-sm text-zinc-500">
                    {"Add team members to your workspace."}
                </p>
                <div class="flex gap-3 mt-6">
                    <Button variant={ButtonVariant::Secondary}>{"Import"}</Button>
                    <Button variant={ButtonVariant::Primary}>{"Add member"}</Button>
                </div>
            </div>
        </CardContent>
    </Card>
}
"##;
