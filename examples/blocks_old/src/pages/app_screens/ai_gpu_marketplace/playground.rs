use super::layout::AIDecentralisedGpuMarketLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(Playground)]
pub fn playground() -> Html {
    html! {
        <AIDecentralisedGpuMarketLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="bg-white dark:bg-zinc-800 text-black dark:text-white min-h-screen">
                <div class="flex flex-col space-y-4 p-4">
                    <div class="flex items-end justify-start">
                        <div class="text-sm max-w-xs lg:max-w-md bg-gray-200 dark:bg-zinc-500 text-black dark:text-white p-2 rounded-lg">
                            {"Hello! How can I help you today?"}
                        </div>
                    </div>
                    <div class="flex items-end justify-end">
                        <div class="text-sm max-w-xs lg:max-w-md bg-gray-300 dark:bg-gray-400 p-2 rounded-lg">
                            {"I'm looking for information on the latest product updates."}
                        </div>
                    </div>
                    <div class="flex items-end justify-start">
                        <div class="text-sm max-w-xs lg:max-w-md bg-gray-200 dark:bg-zinc-500 text-black dark:text-white p-2 rounded-lg">
                            {"Sure, I can help with that. Which product are you interested in?"}
                        </div>
                    </div>
                    <div class="flex items-end justify-end">
                        <div class="text-sm max-w-xs lg:max-w-md bg-gray-300 dark:bg-gray-400 p-2 rounded-lg">
                            {"I'm interested in the new version of your main software."}
                        </div>
                    </div>
                </div>
                <div class="flex flex-row sticky inset-x-0 bottom-0 h-16 border-t border-gray-300 dark:border-zinc-700 px-2 py-3 bg-gray-100 dark:bg-zinc-800 space-x-2">
                    <input class="px-2 border border-gray-300 dark:border-zinc-700 rounded flex-grow outline-none bg-transparent text-black dark:text-white" placeholder="Type a message..."/>
                    <Button>
                        {"Send"}
                    </Button>
                </div>
            </div>
        </AIDecentralisedGpuMarketLayout>
    }
}
