use super::layout1::AppLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(UserSettings)]
pub fn user_settings() -> Html {
    html! {
        <AppLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="hidden space-y-6 p-10 pb-16 md:block">
                <div class="space-y-0.5">
                    <H2>{"Settings"}</H2>
                    <Paragraph>{"Manage your account settings and set e-mail preferences."}</Paragraph>
                </div>
                <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full my-6"></div>
                <Tabs default_value="profile" class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
                    <TabsList class="flex space-x-2 lg:flex-col lg:space-x-0 lg:space-y-1">
                        <TabsTrigger value="profile">{"Profile"}</TabsTrigger>
                        <TabsTrigger value="account">{"Account"}</TabsTrigger>
                        <TabsTrigger value="appearance">{"Appearance"}</TabsTrigger>
                        <TabsTrigger value="notifications">{"Notifications"}</TabsTrigger>
                        <TabsTrigger value="display">{"Display"}</TabsTrigger>
                    </TabsList>
                    <TabsContent value="profile" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Profile"}</H3>
                                <Paragraph>{"This is how others will see you on the site."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <form class="space-y-8">
                                <div class="space-y-2">
                                    <Label for_id="username">{"Username"}</Label>
                                    <Input placeholder="wonop" id="username" name="username" />
                                    <Paragraph class="text-[0.8rem] text-muted-foreground">{"This is your public display name. It can be your real name or a pseudonym. You can only change this once every 30 days."}</Paragraph>
                                </div>
                                <div class="space-y-2">
                                    <Label for_id="email">{"Email"}</Label>
                                    <Input placeholder="example@example.com" id="email" name="email" />
                                    <Paragraph class="text-[0.8rem] text-muted-foreground">{"You can manage verified email addresses in your email settings."}</Paragraph>
                                </div>
                                <div class="space-y-2">
                                    <Label for_id="bio">{"Bio"}</Label>
                                    <Textarea placeholder="Tell us a little bit about yourself" name="bio" id="bio"></Textarea>
                                    <Paragraph class="text-[0.8rem] text-muted-foreground">{"You can @mention other users and organizations to link to them."}</Paragraph>
                                </div>
                                <div class="space-y-2">
                                    <Label for_id="urls">{"URLs"}</Label>
                                    <Input placeholder="https://example.com" id="urls" name="urls" />
                                    <Paragraph class="text-[0.8rem] text-muted-foreground">{"Add links to your website, blog, or social media profiles."}</Paragraph>
                                </div>
                                <Button /*type="submit"*/>{"Update profile"}</Button>
                            </form>
                        </div>
                    </TabsContent>
                    <TabsContent value="account" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Account"}</H3>
                                <Paragraph>{"Manage your account settings and set e-mail preferences."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <form class="space-y-8">
                                // <!-- Account form fields go here -->
                            </form>
                        </div>
                    </TabsContent>
                    <TabsContent value="appearance" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Appearance"}</H3>
                                <Paragraph>{"Customize the appearance of your profile."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <form class="space-y-8">
                                // <!-- Appearance form fields go here -->
                            </form>
                        </div>
                    </TabsContent>
                    <TabsContent value="notifications" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Notifications"}</H3>
                                <Paragraph>{"Configure how you receive notifications."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <form class="space-y-8">
                                <div class="space-y-3">
                                    <Label for_id=":r0:-form-item">{"Notify me about..."}</Label>
                                    <div role="radiogroup" aria-required="false" dir="ltr" class="gap-2 flex flex-col space-y-1" id=":r0:-form-item" aria-describedby=":r0:-form-item-description" aria-invalid="false" tabindex="0" style="outline: none;">
                                        <div class="flex items-center space-x-3 space-y-0">
                                            <button type="button" role="radio" aria-checked="false" data-state="unchecked" value="all" class="aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50" id=":r1:-form-item" aria-describedby=":r1:-form-item-description" aria-invalid="false" tabindex="-1" data-radix-collection-item=""></button>
                                            <input aria-hidden="true" tabindex="-1" type="radio" value="all" style="transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0px; width: 16px; height: 16px;" />
                                            <Label for_id=":r1:-form-item">{"All new messages"}</Label>
                                        </div>
                                        <div class="flex items-center space-x-3 space-y-0">
                                            <button type="button" role="radio" aria-checked="false" data-state="unchecked" value="mentions" class="aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50" id=":r3:-form-item" aria-describedby=":r3:-form-item-description" aria-invalid="false" tabindex="-1" data-radix-collection-item=""></button>
                                            <input aria-hidden="true" tabindex="-1" type="radio" value="mentions" style="transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0px; width: 16px; height: 16px;" />
                                            <Label for_id=":r3:-form-item">{"Direct messages and mentions"}</Label>
                                        </div>
                                        <div class="flex items-center space-x-3 space-y-0">
                                            <button type="button" role="radio" aria-checked="false" data-state="unchecked" value="none" class="aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50" id=":r5:-form-item" aria-describedby=":r5:-form-item-description" aria-invalid="false" tabindex="-1" data-radix-collection-item=""></button>
                                            <input aria-hidden="true" tabindex="-1" type="radio" value="none" style="transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0px; width: 16px; height: 16px;" />
                                            <Label for_id=":r5:-form-item">{"Nothing"}</Label>
                                        </div>
                                    </div>
                                </div>
                                <div>
                                    <H3 class="mb-4">{"Email Notifications"}</H3>
                                    <div class="space-y-4">
                                        <div class="space-y-2 flex flex-row items-center justify-between rounded-lg border p-4">
                                            <div class="space-y-0.5">
                                                <Label for_id=":r7:-form-item">{"Communication emails"}</Label>
                                                <Paragraph class="text-[0.8rem] text-muted-foreground">{"Receive emails about your account activity."}</Paragraph>
                                            </div>
                                            <SwitchButton checked={false} />
                                        </div>
                                        <div class="space-y-2 flex flex-row items-center justify-between rounded-lg border p-4">
                                            <div class="space-y-0.5">
                                                <Label for_id=":r8:-form-item">{"Marketing emails"}</Label>
                                                <Paragraph class="text-[0.8rem] text-muted-foreground">{"Receive emails about new products, features, and more."}</Paragraph>
                                            </div>
                                            <SwitchButton checked={false} />
                                        </div>
                                        <div class="space-y-2 flex flex-row items-center justify-between rounded-lg border p-4">
                                            <div class="space-y-0.5">
                                                <Label for_id=":r9:-form-item">{"Social emails"}</Label>
                                                <Paragraph class="text-[0.8rem] text-muted-foreground">{"Receive emails for friend requests, follows, and more."}</Paragraph>
                                            </div>
                                            <SwitchButton checked={true} />
                                        </div>
                                        <div class="space-y-2 flex flex-row items-center justify-between rounded-lg border p-4">
                                            <div class="space-y-0.5">
                                                <Label for_id=":ra:-form-item">{"Security emails"}</Label>
                                                <Paragraph class="text-[0.8rem] text-muted-foreground">{"Receive emails about your account activity and security."}</Paragraph>
                                            </div>
                                            <SwitchButton checked={true} disabled={true} />
                                        </div>
                                    </div>
                                </div>
                                <div class="flex flex-row items-start space-x-3 space-y-0">
                                    <Checkbox /><Label>{"Use different settings for my mobile devices"}</Label>
                                    <Paragraph class="text-[0.8rem] text-muted-foreground">{"You can manage your mobile notifications in the mobile settings page."}</Paragraph>
                                </div>
                                <Button /*type="submit"*/>{"Update notifications"}</Button>
                            </form>
                        </div>
                    </TabsContent>
                    <TabsContent value="display" class="flex-1 lg:max-w-2xl">
                        <div class="space-y-6">
                            <div>
                                <H3>{"Display"}</H3>
                                <Paragraph>{"Adjust display settings."}</Paragraph>
                            </div>
                            <div data-orientation="horizontal" role="none" class="shrink-0 bg-border h-[1px] w-full"></div>
                            <form class="space-y-8">
                                <div class="space-y-2">
                                    <div class="mb-4">
                                        <Label for_id="sidebar">{"Sidebar"}</Label>
                                        <Paragraph class="text-[0.8rem] text-muted-foreground">{"Select the items you want to display in the sidebar."}</Paragraph>
                                    </div>
                                    <Checkbox checked={true} /><Label>{"Recents"}</Label>
                                    <Checkbox checked={true} /><Label>{"Home"}</Label>
                                    <Checkbox /><Label>{"Applications"}</Label>
                                    <Checkbox /><Label>{"Desktop"}</Label>
                                    <Checkbox /><Label>{"Downloads"}</Label>
                                    <Checkbox /><Label>{"Documents"}</Label>
                                </div>
                                <Button /*type="submit"*/>{"Update display"}</Button>
                            </form>
                        </div>
                    </TabsContent>
                </Tabs>
            </div>
        </AppLayout>
    }
}
