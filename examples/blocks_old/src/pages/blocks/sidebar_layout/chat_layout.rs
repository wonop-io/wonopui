use crate::pages::example_block::ExampleBlock;
use gloo_console as console;
use wonopui::*;
use yew::prelude::*;

#[function_component(MenuButton)]
pub fn menu_button() -> Html {
    let layout_context = use_context::<LayoutContext>().expect("LayoutContext not found");
    let mobile_menu_open = layout_context.mobile_menu_open; // Use sidebar_folded from LayoutContext
    let open_mobile_menu = {
        let layout_context = layout_context.clone();
        Callback::from(move |_| {
            println!("Mobile menu open");
            layout_context.dispatch(LayoutAction::SetMobileMenuOpen(!mobile_menu_open));
        })
    };
    html! {
        <div onclick={open_mobile_menu} class="border rounded h-8 w-8 block lg:hidden bg-zinc-700 z-10 opacity-25 hover:opacity-100 cursor-pointer">
        </div>
    }
}

#[function_component(ChatList)]
pub fn chat_list() -> Html {
    html! {
        <Sidebar>
            <div class="px-4 h-12 w-full absolute block lg:hidden top-0 left-0 bg-zinc-700 z-10 border-b border-zinc-900 flex flex-row justify-between items-center">
                <MenuButton />
            </div>
            <SidebarMenu>
                <SidebarItem label={"Chat 1"} />
                <SidebarItem label={"Chat 2"} />
                <SidebarItem label={"Chat 3"} />
            </SidebarMenu>
        </Sidebar>
    }
}
#[function_component(ChatLayout)]
pub fn chat_layout() -> Html {
    html! {
        <div class="bg-zinc-800 text-white min-h-screen">
        <LayoutProvider>
            <Layout sidebar={html!{
                <ChatList />
            }}>
                <MainContent class="min-h-screen">

                    <div class="px-4 h-12 w-full fixed block lg:hidden top-0 left-0 bg-zinc-700 z-10 border-b border-zinc-800 flex flex-row justify-between items-center">
                        <MenuButton />
                    </div>
                    <div class="mt-12 md:mt-10 lg:mt-8 p-4 md:p-6 lg:p-8">
                        <div class="flex flex-col space-y-4">
                            <div class="flex items-end justify-start">
                                <div class="text-sm max-w-xs lg:max-w-md bg-zinc-500 text-white p-2 rounded-lg">
                                    {"Hello! How can I help you today?"}
                                </div>
                            </div>
                            <div class="flex items-end justify-end">
                                <div class="text-sm max-w-xs lg:max-w-md bg-gray-400 p-2 rounded-lg">
                                    {"I'm looking for information on the latest product updates."}
                                </div>
                            </div>
                            <div class="flex items-end justify-start">
                                <div class="text-sm max-w-xs lg:max-w-md bg-zinc-500 text-white p-2 rounded-lg">
                                    {"Sure, I can help with that. Which product are you interested in?"}
                                </div>
                            </div>
                            <div class="flex items-end justify-end">
                                <div class="text-sm max-w-xs lg:max-w-md bg-gray-400 p-2 rounded-lg">
                                    {"I'm interested in the new version of your main software."}
                                </div>
                            </div>
                            <div class="flex items-end justify-start">
                                <div class="text-sm max-w-xs lg:max-w-md bg-zinc-500 text-white p-2 rounded-lg">
                                    {"Hello! How can I help you today?"}
                                </div>
                            </div>
                            <div class="flex items-end justify-end">
                                <div class="text-sm max-w-xs lg:max-w-md bg-gray-400 p-2 rounded-lg">
                                    {"I'm looking for information on the latest product updates."}
                                </div>
                            </div>
                            <div class="flex items-end justify-start">
                                <div class="text-sm max-w-xs lg:max-w-md bg-zinc-500 text-white p-2 rounded-lg">
                                    {"Sure, I can help with that. Which product are you interested in?"}
                                </div>
                            </div>
                            <div class="flex items-end justify-end">
                                <div class="text-sm max-w-xs lg:max-w-md bg-gray-400 p-2 rounded-lg">
                                    {"I'm interested in the new version of your main software."}
                                </div>
                            </div>
                            <div class="flex items-end justify-start">
                                <div class="text-sm max-w-xs lg:max-w-md bg-zinc-500 text-white p-2 rounded-lg">
                                    {"Hello! How can I help you today?"}
                                </div>
                            </div>
                            <div class="flex items-end justify-end">
                                <div class="text-sm max-w-xs lg:max-w-md bg-gray-400 p-2 rounded-lg">
                                    {"I'm looking for information on the latest product updates."}
                                </div>
                            </div>
                            <div class="flex items-end justify-start">
                                <div class="text-sm max-w-xs lg:max-w-md bg-zinc-500 text-white p-2 rounded-lg">
                                    {"Sure, I can help with that. Which product are you interested in?"}
                                </div>
                            </div>
                            <div class="flex items-end justify-end">
                                <div class="text-sm max-w-xs lg:max-w-md bg-gray-400 p-2 rounded-lg">
                                    {"I'm interested in the new version of your main software."}
                                </div>
                            </div>
                        </div>
                    </div>
                </MainContent>
                <div class="flex flex-row sticky inset-x-0 bottom-0 h-16 border-t border-zinc-700 px-2 py-3 bg-zinc-800 space-x-2">
                    <input class="px-2 border border-zinc-700 rounded flex-grow outline-none bg-transparent text-white" placeholder="Type a message..."/>
                    <Button>
                        {"Send"}
                    </Button>
                </div>
            </Layout>
        </LayoutProvider>
        </div>
    }
}
