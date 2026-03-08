use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(SettingsScreens)]
pub fn settings_screens() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Settings Screens"} />
            </Breadcrumb>
            <ExampleBlock title={"Settings Screen 1"}>
                <SettingsScreen1 />
            </ExampleBlock>
            <ExampleBlock title={"Settings Screen 2"}>
                <SettingsScreen2 />
            </ExampleBlock>
            <ExampleBlock title={"Settings Screen 3"}>
                <SettingsScreen3 />
            </ExampleBlock>
            <ExampleBlock title={"Settings Screen 4"}>
                <SettingsScreen4 />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(SettingsScreen1)]
pub fn settings_screen1() -> Html {
    html! {
        <div class="p-4 bg-gray-100 rounded-lg">
            <h2 class="text-xl font-bold mb-4">{"Settings Screen 1"}</h2>
            <div class="flex flex-col space-y-4">
                <label class="block">
                    <span class="text-gray-700">{"Username"}</span>
                    <input type="text" class="form-input mt-1 block w-full" placeholder="Enter your username" />
                </label>
                <label class="block">
                    <span class="text-gray-700">{"Email"}</span>
                    <input type="email" class="form-input mt-1 block w-full" placeholder="Enter your email" />
                </label>
            </div>
        </div>
    }
}

#[function_component(SettingsScreen2)]
pub fn settings_screen2() -> Html {
    html! {
        <div class="p-4 bg-gray-100 rounded-lg">
            <h2 class="text-xl font-bold mb-4">{"Settings Screen 2"}</h2>
            <div class="flex flex-col space-y-4">
                <label class="block">
                    <span class="text-gray-700">{"Password"}</span>
                    <input type="password" class="form-input mt-1 block w-full" placeholder="Enter your password" />
                </label>
                <label class="block">
                    <span class="text-gray-700">{"Confirm Password"}</span>
                    <input type="password" class="form-input mt-1 block w-full" placeholder="Confirm your password" />
                </label>
            </div>
        </div>
    }
}

#[function_component(SettingsScreen3)]
pub fn settings_screen3() -> Html {
    html! {
        <div class="p-4 bg-gray-100 rounded-lg">
            <h2 class="text-xl font-bold mb-4">{"Settings Screen 3"}</h2>
            <div class="flex flex-col space-y-4">
                <label class="block">
                    <span class="text-gray-700">{"Notification Preferences"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"Email Notifications"}</option>
                        <option>{"SMS Notifications"}</option>
                        <option>{"Push Notifications"}</option>
                    </select>
                </label>
            </div>
        </div>
    }
}

#[function_component(SettingsScreen4)]
pub fn settings_screen4() -> Html {
    html! {
        <div class="p-4 bg-gray-100 rounded-lg">
            <h2 class="text-xl font-bold mb-4">{"Settings Screen 4"}</h2>
            <div class="flex flex-col space-y-4">
                <label class="block">
                    <span class="text-gray-700">{"Language"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"English"}</option>
                        <option>{"Spanish"}</option>
                        <option>{"French"}</option>
                    </select>
                </label>
                <label class="block">
                    <span class="text-gray-700">{"Time Zone"}</span>
                    <select class="form-select mt-1 block w-full">
                        <option>{"GMT"}</option>
                        <option>{"PST"}</option>
                        <option>{"EST"}</option>
                    </select>
                </label>
            </div>
        </div>
    }
}
