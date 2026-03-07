//! Form blocks - Layouts, Input Groups, Sign-in/Registration, Checkboxes, Toggles
//! 
//! Comprehensive form patterns using WonopUI components

use yew::prelude::*;
use wonopui::*;
use wonopui::wonopui_button::{Button, ButtonVariant, ButtonSize};
use wonopui::wonopui_input::Input;
use wonopui::wonopui_label::Label;
use wonopui::wonopui_textarea::Textarea;
use wonopui::wonopui_select::{Select, SelectOption as SelectOpt};
use wonopui::wonopui_checkbox::Checkbox;
use wonopui::wonopui_switch::SwitchButton;
use wonopui::wonopui_toggle::{Toggle, ToggleVariant};
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
use wonopui::wonopui_divider::Divider;
use crate::blocks::BlockPreview;

/// Forms category page
#[function_component(FormsBlocks)]
pub fn forms_blocks() -> Html {
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
                    {"Forms"}
                </h1>
                <p class="mt-2 text-lg text-zinc-600 dark:text-zinc-400">
                    {"Form layouts, input groups, and authentication patterns using WonopUI components."}
                </p>
            </div>
            
            // Form Layouts Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Form Layouts"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Stacked Form"
                        description="Vertical form layout with labels above inputs."
                        code={STACKED_FORM_CODE}
                        min_height={500}
                        isolate={true}
                    >
                        <StackedForm />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Inline Form"
                        description="Horizontal form layout for simple filters or search."
                        code={INLINE_FORM_CODE}
                        min_height={150}
                        isolate={true}
                    >
                        <InlineForm />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Two-Column Form"
                        description="Grid-based form layout for longer forms."
                        code={TWO_COLUMN_FORM_CODE}
                        min_height={450}
                        isolate={true}
                    >
                        <TwoColumnForm />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Sectioned Form"
                        description="Form with distinct sections and dividers."
                        code={SECTIONED_FORM_CODE}
                        min_height={650}
                        isolate={true}
                    >
                        <SectionedForm />
                    </BlockPreview>
                </div>
            </div>
            
            // Input Groups Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Input Groups"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Input with Leading Icon"
                        description="Input field with an icon prefix."
                        code={INPUT_LEADING_ICON_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <InputLeadingIcon />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Input with Trailing Icon"
                        description="Input field with an icon suffix."
                        code={INPUT_TRAILING_ICON_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <InputTrailingIcon />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Input with Addon"
                        description="Input with text prefix or suffix addon."
                        code={INPUT_ADDON_CODE}
                        min_height={180}
                        isolate={true}
                    >
                        <InputWithAddon />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Input with Button"
                        description="Input combined with an action button."
                        code={INPUT_BUTTON_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <InputWithButton />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Input with Select"
                        description="Input combined with a select dropdown."
                        code={INPUT_SELECT_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <InputWithSelect />
                    </BlockPreview>
                </div>
            </div>
            
            // Sign-in/Registration Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Sign In & Registration"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Sign In Form"
                        description="Basic sign-in form with email and password."
                        code={SIGN_IN_FORM_CODE}
                        min_height={400}
                        isolate={true}
                    >
                        <SignInForm />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Registration Form"
                        description="User registration with validation hints."
                        code={REGISTRATION_FORM_CODE}
                        min_height={550}
                        isolate={true}
                    >
                        <RegistrationForm />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Social Sign In"
                        description="Sign-in with social provider buttons."
                        code={SOCIAL_SIGN_IN_CODE}
                        min_height={450}
                        isolate={true}
                    >
                        <SocialSignIn />
                    </BlockPreview>
                </div>
            </div>
            
            // Checkboxes Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Checkboxes"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Simple Checkbox"
                        description="Basic checkbox with label."
                        code={CHECKBOX_SIMPLE_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <CheckboxSimple />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Checkbox Group"
                        description="Multiple checkboxes for selecting options."
                        code={CHECKBOX_GROUP_CODE}
                        min_height={220}
                        isolate={true}
                    >
                        <CheckboxGroup />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Checkbox with Description"
                        description="Checkbox with supporting helper text."
                        code={CHECKBOX_DESCRIPTION_CODE}
                        min_height={200}
                        isolate={true}
                    >
                        <CheckboxWithDescription />
                    </BlockPreview>
                </div>
            </div>
            
            // Toggles Section
            <div class="mb-16">
                <h2 class="text-2xl font-bold mb-6 text-zinc-900 dark:text-white">{"Toggles & Switches"}</h2>
                <div class="space-y-12">
                    <BlockPreview 
                        title="Switch Toggle"
                        description="On/off switch for boolean settings."
                        code={SWITCH_SIMPLE_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <SwitchSimple />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Switch Group"
                        description="Multiple switches for settings."
                        code={SWITCH_GROUP_CODE}
                        min_height={280}
                        isolate={true}
                    >
                        <SwitchGroup />
                    </BlockPreview>
                    
                    <BlockPreview 
                        title="Button Toggle"
                        description="Pressable toggle button."
                        code={TOGGLE_BUTTON_CODE}
                        min_height={120}
                        isolate={true}
                    >
                        <ToggleButton />
                    </BlockPreview>
                </div>
            </div>
        </Container>
    }
}

// =============================================================================
// Form Layouts
// =============================================================================

#[function_component(StackedForm)]
fn stacked_form() -> Html {
    html! {
        <Card class="max-w-lg mx-auto">
            <CardHeader>
                <CardTitle>{"Create Account"}</CardTitle>
                <CardDescription>{"Enter your details to create a new account."}</CardDescription>
            </CardHeader>
            <CardContent>
                <form class="space-y-4">
                    <div>
                        <Label for_id="name">{"Full Name"}</Label>
                        <Input id="name" placeholder="John Doe" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="email">{"Email"}</Label>
                        <Input id="email" kind="email" placeholder="john@example.com" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="password">{"Password"}</Label>
                        <Input id="password" kind="password" placeholder="••••••••" class="mt-1.5" />
                        <p class="mt-1.5 text-sm text-zinc-500 dark:text-zinc-400">
                            {"Must be at least 8 characters."}
                        </p>
                    </div>
                    <div>
                        <Label for_id="bio">{"Bio"}</Label>
                        <Textarea id="bio" placeholder="Tell us about yourself..." class="mt-1.5" rows={3} />
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex justify-end gap-3">
                <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                <Button variant={ButtonVariant::Default}>{"Create Account"}</Button>
            </CardFooter>
        </Card>
    }
}

#[function_component(InlineForm)]
fn inline_form() -> Html {
    let options = vec![
        SelectOpt { value: "all".to_string(), label: "All".to_string() },
        SelectOpt { value: "electronics".to_string(), label: "Electronics".to_string() },
        SelectOpt { value: "clothing".to_string(), label: "Clothing".to_string() },
    ];
    
    html! {
        <div class="p-6 bg-white dark:bg-zinc-950 rounded-lg border border-zinc-200 dark:border-zinc-800">
            <form class="flex flex-wrap items-end gap-4">
                <div class="flex-1 min-w-[200px]">
                    <Label for_id="search">{"Search"}</Label>
                    <Input id="search" placeholder="Search products..." class="mt-1.5" />
                </div>
                <div class="w-40">
                    <Label for_id="category">{"Category"}</Label>
                    <Select<SelectOpt> 
                        options={options}
                        placeholder="Select..."
                        class="mt-1.5 w-full"
                    />
                </div>
                <Button variant={ButtonVariant::Default}>{"Search"}</Button>
            </form>
        </div>
    }
}

#[function_component(TwoColumnForm)]
fn two_column_form() -> Html {
    html! {
        <Card class="max-w-2xl mx-auto">
            <CardHeader>
                <CardTitle>{"Personal Information"}</CardTitle>
                <CardDescription>{"Update your personal details."}</CardDescription>
            </CardHeader>
            <CardContent>
                <form class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <Label for_id="first-name">{"First Name"}</Label>
                        <Input id="first-name" placeholder="John" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="last-name">{"Last Name"}</Label>
                        <Input id="last-name" placeholder="Doe" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="tc-email">{"Email"}</Label>
                        <Input id="tc-email" kind="email" placeholder="john@example.com" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="phone">{"Phone"}</Label>
                        <Input id="phone" kind="tel" placeholder="+1 (555) 000-0000" class="mt-1.5" />
                    </div>
                    <div class="md:col-span-2">
                        <Label for_id="address">{"Address"}</Label>
                        <Input id="address" placeholder="123 Main St" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="city">{"City"}</Label>
                        <Input id="city" placeholder="San Francisco" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="zip">{"ZIP Code"}</Label>
                        <Input id="zip" placeholder="94102" class="mt-1.5" />
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex justify-end gap-3">
                <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                <Button variant={ButtonVariant::Default}>{"Save Changes"}</Button>
            </CardFooter>
        </Card>
    }
}

#[function_component(SectionedForm)]
fn sectioned_form() -> Html {
    html! {
        <Card class="max-w-2xl mx-auto">
            <CardHeader>
                <CardTitle>{"Account Settings"}</CardTitle>
                <CardDescription>{"Manage your account preferences."}</CardDescription>
            </CardHeader>
            <CardContent class="space-y-6">
                // Profile Section
                <div>
                    <h3 class="text-sm font-medium text-zinc-900 dark:text-white mb-4">{"Profile"}</h3>
                    <div class="space-y-4">
                        <div>
                            <Label for_id="sf-name">{"Display Name"}</Label>
                            <Input id="sf-name" placeholder="Your display name" class="mt-1.5" />
                        </div>
                        <div>
                            <Label for_id="sf-email">{"Email"}</Label>
                            <Input id="sf-email" kind="email" placeholder="you@example.com" class="mt-1.5" />
                        </div>
                    </div>
                </div>
                
                <Divider />
                
                // Notifications Section
                <div>
                    <h3 class="text-sm font-medium text-zinc-900 dark:text-white mb-4">{"Notifications"}</h3>
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-zinc-900 dark:text-white">{"Email notifications"}</p>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Receive email about your account activity."}</p>
                            </div>
                            <SwitchButton checked={true} />
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm font-medium text-zinc-900 dark:text-white">{"Marketing emails"}</p>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Receive emails about new features and offers."}</p>
                            </div>
                            <SwitchButton />
                        </div>
                    </div>
                </div>
                
                <Divider />
                
                // Privacy Section
                <div>
                    <h3 class="text-sm font-medium text-zinc-900 dark:text-white mb-4">{"Privacy"}</h3>
                    <div class="space-y-4">
                        <div class="flex items-start gap-3">
                            <Checkbox id="public-profile" class="mt-1" />
                            <div>
                                <Label for_id="public-profile" class="!mb-0">{"Make profile public"}</Label>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Allow others to see your profile information."}</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-3">
                            <Checkbox id="show-activity" class="mt-1" checked={true} />
                            <div>
                                <Label for_id="show-activity" class="!mb-0">{"Show activity status"}</Label>
                                <p class="text-sm text-zinc-500 dark:text-zinc-400">{"Let others see when you're online."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </CardContent>
            <CardFooter class="flex justify-end gap-3">
                <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                <Button variant={ButtonVariant::Default}>{"Save Settings"}</Button>
            </CardFooter>
        </Card>
    }
}

// =============================================================================
// Input Groups
// =============================================================================

fn search_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-400">
            <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
        </svg>
    }
}

fn mail_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5 text-zinc-400">
            <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75" />
        </svg>
    }
}

fn check_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="size-5 text-green-500">
            <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
        </svg>
    }
}

#[function_component(InputLeadingIcon)]
fn input_leading_icon() -> Html {
    html! {
        <div class="max-w-sm mx-auto p-6 bg-white dark:bg-zinc-950 rounded-lg border border-zinc-200 dark:border-zinc-800">
            <Label for_id="search-input">{"Search"}</Label>
            <div class="relative mt-1.5">
                <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                    {search_icon()}
                </div>
                <Input id="search-input" placeholder="Search..." class="pl-10" />
            </div>
        </div>
    }
}

#[function_component(InputTrailingIcon)]
fn input_trailing_icon() -> Html {
    html! {
        <div class="max-w-sm mx-auto p-6 bg-white dark:bg-zinc-950 rounded-lg border border-zinc-200 dark:border-zinc-800">
            <Label for_id="email-verified">{"Email (verified)"}</Label>
            <div class="relative mt-1.5">
                <Input id="email-verified" kind="email" value="john@example.com" class="pr-10" />
                <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                    {check_icon()}
                </div>
            </div>
        </div>
    }
}

#[function_component(InputWithAddon)]
fn input_with_addon() -> Html {
    html! {
        <div class="max-w-md mx-auto p-6 bg-white dark:bg-zinc-950 rounded-lg border border-zinc-200 dark:border-zinc-800 space-y-4">
            // Leading addon
            <div>
                <Label for_id="website">{"Website"}</Label>
                <div class="flex mt-1.5">
                    <span class="inline-flex items-center px-3 text-sm text-zinc-500 bg-zinc-100 dark:bg-zinc-800 dark:text-zinc-400 border border-r-0 border-zinc-200 dark:border-zinc-700 rounded-l-md">
                        {"https://"}
                    </span>
                    <Input id="website" placeholder="www.example.com" class="rounded-l-none" />
                </div>
            </div>
            // Trailing addon
            <div>
                <Label for_id="price">{"Price"}</Label>
                <div class="flex mt-1.5">
                    <Input id="price" kind="number" placeholder="0.00" class="rounded-r-none" />
                    <span class="inline-flex items-center px-3 text-sm text-zinc-500 bg-zinc-100 dark:bg-zinc-800 dark:text-zinc-400 border border-l-0 border-zinc-200 dark:border-zinc-700 rounded-r-md">
                        {"USD"}
                    </span>
                </div>
            </div>
        </div>
    }
}

#[function_component(InputWithButton)]
fn input_with_button() -> Html {
    html! {
        <div class="max-w-md mx-auto p-6 bg-white dark:bg-zinc-950 rounded-lg border border-zinc-200 dark:border-zinc-800">
            <Label for_id="email-subscribe">{"Subscribe to newsletter"}</Label>
            <div class="flex mt-1.5">
                <Input id="email-subscribe" kind="email" placeholder="you@example.com" class="rounded-r-none" />
                <Button variant={ButtonVariant::Default} class="rounded-l-none">{"Subscribe"}</Button>
            </div>
        </div>
    }
}

#[function_component(InputWithSelect)]
fn input_with_select() -> Html {
    let country_codes = vec![
        SelectOpt { value: "+1".to_string(), label: "🇺🇸 +1".to_string() },
        SelectOpt { value: "+44".to_string(), label: "🇬🇧 +44".to_string() },
        SelectOpt { value: "+49".to_string(), label: "🇩🇪 +49".to_string() },
    ];
    
    html! {
        <div class="max-w-md mx-auto p-6 bg-white dark:bg-zinc-950 rounded-lg border border-zinc-200 dark:border-zinc-800">
            <Label for_id="phone-number">{"Phone Number"}</Label>
            <div class="flex mt-1.5">
                <Select<SelectOpt> 
                    options={country_codes}
                    selected={Some(SelectOpt { value: "+1".to_string(), label: "🇺🇸 +1".to_string() })}
                    class="w-28 rounded-r-none"
                />
                <Input id="phone-number" kind="tel" placeholder="(555) 000-0000" class="rounded-l-none flex-1" />
            </div>
        </div>
    }
}

// =============================================================================
// Sign-in / Registration
// =============================================================================

#[function_component(SignInForm)]
fn sign_in_form() -> Html {
    html! {
        <Card class="max-w-md mx-auto">
            <CardHeader class="text-center">
                <CardTitle class="text-2xl">{"Welcome back"}</CardTitle>
                <CardDescription>{"Enter your credentials to access your account."}</CardDescription>
            </CardHeader>
            <CardContent>
                <form class="space-y-4">
                    <div>
                        <Label for_id="signin-email">{"Email"}</Label>
                        <div class="relative mt-1.5">
                            <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                                {mail_icon()}
                            </div>
                            <Input id="signin-email" kind="email" placeholder="you@example.com" class="pl-10" />
                        </div>
                    </div>
                    <div>
                        <div class="flex items-center justify-between">
                            <Label for_id="signin-password">{"Password"}</Label>
                            <a href="#" class="text-sm text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">
                                {"Forgot password?"}
                            </a>
                        </div>
                        <Input id="signin-password" kind="password" placeholder="••••••••" class="mt-1.5" />
                    </div>
                    <div class="flex items-center gap-2">
                        <Checkbox id="remember" />
                        <Label for_id="remember" class="!mb-0 text-sm">{"Remember me"}</Label>
                    </div>
                    <Button variant={ButtonVariant::Default} class="w-full">{"Sign In"}</Button>
                </form>
            </CardContent>
            <CardFooter class="justify-center">
                <p class="text-sm text-zinc-500 dark:text-zinc-400">
                    {"Don't have an account? "}
                    <a href="#" class="text-zinc-900 hover:underline dark:text-white">{"Sign up"}</a>
                </p>
            </CardFooter>
        </Card>
    }
}

#[function_component(RegistrationForm)]
fn registration_form() -> Html {
    html! {
        <Card class="max-w-md mx-auto">
            <CardHeader class="text-center">
                <CardTitle class="text-2xl">{"Create an account"}</CardTitle>
                <CardDescription>{"Enter your details to get started."}</CardDescription>
            </CardHeader>
            <CardContent>
                <form class="space-y-4">
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <Label for_id="reg-first">{"First name"}</Label>
                            <Input id="reg-first" placeholder="John" class="mt-1.5" />
                        </div>
                        <div>
                            <Label for_id="reg-last">{"Last name"}</Label>
                            <Input id="reg-last" placeholder="Doe" class="mt-1.5" />
                        </div>
                    </div>
                    <div>
                        <Label for_id="reg-email">{"Email"}</Label>
                        <Input id="reg-email" kind="email" placeholder="you@example.com" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="reg-password">{"Password"}</Label>
                        <Input id="reg-password" kind="password" placeholder="••••••••" class="mt-1.5" />
                        <ul class="mt-2 text-sm text-zinc-500 dark:text-zinc-400 space-y-1">
                            <li class="flex items-center gap-2">
                                <span class="size-1.5 rounded-full bg-green-500"></span>
                                {"At least 8 characters"}
                            </li>
                            <li class="flex items-center gap-2">
                                <span class="size-1.5 rounded-full bg-zinc-300 dark:bg-zinc-600"></span>
                                {"One uppercase letter"}
                            </li>
                            <li class="flex items-center gap-2">
                                <span class="size-1.5 rounded-full bg-zinc-300 dark:bg-zinc-600"></span>
                                {"One number"}
                            </li>
                        </ul>
                    </div>
                    <div class="flex items-start gap-2">
                        <Checkbox id="terms" class="mt-1" />
                        <Label for_id="terms" class="!mb-0 text-sm">
                            {"I agree to the "}
                            <a href="#" class="text-zinc-900 hover:underline dark:text-white">{"Terms of Service"}</a>
                            {" and "}
                            <a href="#" class="text-zinc-900 hover:underline dark:text-white">{"Privacy Policy"}</a>
                        </Label>
                    </div>
                    <Button variant={ButtonVariant::Default} class="w-full">{"Create Account"}</Button>
                </form>
            </CardContent>
            <CardFooter class="justify-center">
                <p class="text-sm text-zinc-500 dark:text-zinc-400">
                    {"Already have an account? "}
                    <a href="#" class="text-zinc-900 hover:underline dark:text-white">{"Sign in"}</a>
                </p>
            </CardFooter>
        </Card>
    }
}

fn google_icon() -> Html {
    html! {
        <svg class="size-5" viewBox="0 0 24 24">
            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
        </svg>
    }
}

fn github_icon() -> Html {
    html! {
        <svg class="size-5" fill="currentColor" viewBox="0 0 24 24">
            <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/>
        </svg>
    }
}

#[function_component(SocialSignIn)]
fn social_sign_in() -> Html {
    html! {
        <Card class="max-w-md mx-auto">
            <CardHeader class="text-center">
                <CardTitle class="text-2xl">{"Sign in"}</CardTitle>
                <CardDescription>{"Choose your preferred sign-in method."}</CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="grid grid-cols-2 gap-3">
                    <Button variant={ButtonVariant::Outline} class="w-full">
                        {google_icon()}
                        <span class="ml-2">{"Google"}</span>
                    </Button>
                    <Button variant={ButtonVariant::Outline} class="w-full">
                        {github_icon()}
                        <span class="ml-2">{"GitHub"}</span>
                    </Button>
                </div>
                
                <div class="relative">
                    <div class="absolute inset-0 flex items-center">
                        <Divider />
                    </div>
                    <div class="relative flex justify-center text-xs uppercase">
                        <span class="bg-white dark:bg-zinc-950 px-2 text-zinc-500 dark:text-zinc-400">
                            {"Or continue with"}
                        </span>
                    </div>
                </div>
                
                <form class="space-y-4">
                    <div>
                        <Label for_id="social-email">{"Email"}</Label>
                        <Input id="social-email" kind="email" placeholder="you@example.com" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="social-password">{"Password"}</Label>
                        <Input id="social-password" kind="password" placeholder="••••••••" class="mt-1.5" />
                    </div>
                    <Button variant={ButtonVariant::Default} class="w-full">{"Sign In"}</Button>
                </form>
            </CardContent>
        </Card>
    }
}

// =============================================================================
// Code Constants
// =============================================================================

const STACKED_FORM_CODE: &str = r##"use wonopui::*;

html! {
    <Card class="max-w-lg mx-auto">
        <CardHeader>
            <CardTitle>{"Create Account"}</CardTitle>
            <CardDescription>{"Enter your details to create a new account."}</CardDescription>
        </CardHeader>
        <CardContent>
            <form class="space-y-4">
                <div>
                    <Label for_id="name">{"Full Name"}</Label>
                    <Input id="name" placeholder="John Doe" class="mt-1.5" />
                </div>
                <div>
                    <Label for_id="email">{"Email"}</Label>
                    <Input id="email" kind="email" placeholder="john@example.com" class="mt-1.5" />
                </div>
                <div>
                    <Label for_id="password">{"Password"}</Label>
                    <Input id="password" kind="password" class="mt-1.5" />
                    <p class="mt-1.5 text-sm text-zinc-500">{"Must be at least 8 characters."}</p>
                </div>
                <div>
                    <Label for_id="bio">{"Bio"}</Label>
                    <Textarea id="bio" placeholder="Tell us about yourself..." class="mt-1.5" rows={3} />
                </div>
            </form>
        </CardContent>
        <CardFooter class="flex justify-end gap-3">
            <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
            <Button variant={ButtonVariant::Default}>{"Create Account"}</Button>
        </CardFooter>
    </Card>
}
"##;

const INLINE_FORM_CODE: &str = r##"use wonopui::*;

html! {
    <form class="flex flex-wrap items-end gap-4">
        <div class="flex-1 min-w-[200px]">
            <Label for_id="search">{"Search"}</Label>
            <Input id="search" placeholder="Search products..." class="mt-1.5" />
        </div>
        <div class="w-40">
            <Label for_id="category">{"Category"}</Label>
            <Select id="category" class="mt-1.5">
                <SelectOption value="all">{"All"}</SelectOption>
                <SelectOption value="electronics">{"Electronics"}</SelectOption>
            </Select>
        </div>
        <Button variant={ButtonVariant::Default}>{"Search"}</Button>
    </form>
}
"##;

const TWO_COLUMN_FORM_CODE: &str = r##"use wonopui::*;

html! {
    <Card class="max-w-2xl mx-auto">
        <CardHeader>
            <CardTitle>{"Personal Information"}</CardTitle>
        </CardHeader>
        <CardContent>
            <form class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <Label for_id="first-name">{"First Name"}</Label>
                    <Input id="first-name" placeholder="John" class="mt-1.5" />
                </div>
                <div>
                    <Label for_id="last-name">{"Last Name"}</Label>
                    <Input id="last-name" placeholder="Doe" class="mt-1.5" />
                </div>
                <div class="md:col-span-2">
                    <Label for_id="address">{"Address"}</Label>
                    <Input id="address" placeholder="123 Main St" class="mt-1.5" />
                </div>
            </form>
        </CardContent>
        <CardFooter class="flex justify-end gap-3">
            <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
            <Button variant={ButtonVariant::Default}>{"Save Changes"}</Button>
        </CardFooter>
    </Card>
}
"##;

const SECTIONED_FORM_CODE: &str = r##"use wonopui::*;

html! {
    <Card>
        <CardHeader>
            <CardTitle>{"Account Settings"}</CardTitle>
        </CardHeader>
        <CardContent class="space-y-6">
            // Profile Section
            <div>
                <h3 class="text-sm font-medium mb-4">{"Profile"}</h3>
                <div class="space-y-4">
                    <div>
                        <Label for_id="name">{"Display Name"}</Label>
                        <Input id="name" class="mt-1.5" />
                    </div>
                </div>
            </div>
            
            <Divider />
            
            // Notifications Section
            <div>
                <h3 class="text-sm font-medium mb-4">{"Notifications"}</h3>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="text-sm font-medium">{"Email notifications"}</p>
                        <p class="text-sm text-zinc-500">{"Receive email updates."}</p>
                    </div>
                    <SwitchButton checked={true} />
                </div>
            </div>
        </CardContent>
    </Card>
}
"##;

const INPUT_LEADING_ICON_CODE: &str = r##"use wonopui::*;

html! {
    <div>
        <Label for_id="search">{"Search"}</Label>
        <div class="relative mt-1.5">
            <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                <SearchIcon />
            </div>
            <Input id="search" placeholder="Search..." class="pl-10" />
        </div>
    </div>
}
"##;

const INPUT_TRAILING_ICON_CODE: &str = r##"use wonopui::*;

html! {
    <div>
        <Label for_id="email">{"Email (verified)"}</Label>
        <div class="relative mt-1.5">
            <Input id="email" kind="email" value="john@example.com" class="pr-10" />
            <div class="absolute inset-y-0 right-0 flex items-center pr-3">
                <CheckIcon class="text-green-500" />
            </div>
        </div>
    </div>
}
"##;

const INPUT_ADDON_CODE: &str = r##"use wonopui::*;

html! {
    <div>
        <Label for_id="website">{"Website"}</Label>
        <div class="flex mt-1.5">
            <span class="inline-flex items-center px-3 text-sm text-zinc-500 bg-zinc-100 border border-r-0 rounded-l-md">
                {"https://"}
            </span>
            <Input id="website" placeholder="www.example.com" class="rounded-l-none" />
        </div>
    </div>
}
"##;

const INPUT_BUTTON_CODE: &str = r##"use wonopui::*;

html! {
    <div>
        <Label for_id="email">{"Subscribe to newsletter"}</Label>
        <div class="flex mt-1.5">
            <Input id="email" kind="email" placeholder="you@example.com" class="rounded-r-none" />
            <Button variant={ButtonVariant::Default} class="rounded-l-none">{"Subscribe"}</Button>
        </div>
    </div>
}
"##;

const INPUT_SELECT_CODE: &str = r##"use wonopui::*;

html! {
    <div>
        <Label for_id="phone">{"Phone Number"}</Label>
        <div class="flex mt-1.5">
            <Select class="w-24 rounded-r-none border-r-0">
                <SelectOption value="+1">{"🇺🇸 +1"}</SelectOption>
                <SelectOption value="+44">{"🇬🇧 +44"}</SelectOption>
            </Select>
            <Input id="phone" kind="tel" placeholder="(555) 000-0000" class="rounded-l-none" />
        </div>
    </div>
}
"##;

const SIGN_IN_FORM_CODE: &str = r##"use wonopui::*;

html! {
    <Card class="max-w-md mx-auto">
        <CardHeader class="text-center">
            <CardTitle class="text-2xl">{"Welcome back"}</CardTitle>
            <CardDescription>{"Enter your credentials to access your account."}</CardDescription>
        </CardHeader>
        <CardContent>
            <form class="space-y-4">
                <div>
                    <Label for_id="email">{"Email"}</Label>
                    <Input id="email" kind="email" placeholder="you@example.com" class="mt-1.5" />
                </div>
                <div>
                    <div class="flex items-center justify-between">
                        <Label for_id="password">{"Password"}</Label>
                        <a href="#" class="text-sm text-zinc-600">{"Forgot password?"}</a>
                    </div>
                    <Input id="password" kind="password" class="mt-1.5" />
                </div>
                <div class="flex items-center gap-2">
                    <Checkbox id="remember" />
                    <Label for_id="remember" class="!mb-0">{"Remember me"}</Label>
                </div>
                <Button variant={ButtonVariant::Default} class="w-full">{"Sign In"}</Button>
            </form>
        </CardContent>
    </Card>
}
"##;

const REGISTRATION_FORM_CODE: &str = r##"use wonopui::*;

html! {
    <Card class="max-w-md mx-auto">
        <CardHeader class="text-center">
            <CardTitle class="text-2xl">{"Create an account"}</CardTitle>
        </CardHeader>
        <CardContent>
            <form class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <Label for_id="first">{"First name"}</Label>
                        <Input id="first" placeholder="John" class="mt-1.5" />
                    </div>
                    <div>
                        <Label for_id="last">{"Last name"}</Label>
                        <Input id="last" placeholder="Doe" class="mt-1.5" />
                    </div>
                </div>
                <div>
                    <Label for_id="email">{"Email"}</Label>
                    <Input id="email" kind="email" class="mt-1.5" />
                </div>
                <div>
                    <Label for_id="password">{"Password"}</Label>
                    <Input id="password" kind="password" class="mt-1.5" />
                </div>
                <div class="flex items-start gap-2">
                    <Checkbox id="terms" class="mt-1" />
                    <Label for_id="terms" class="!mb-0 text-sm">
                        {"I agree to the Terms of Service"}
                    </Label>
                </div>
                <Button variant={ButtonVariant::Default} class="w-full">{"Create Account"}</Button>
            </form>
        </CardContent>
    </Card>
}
"##;

const SOCIAL_SIGN_IN_CODE: &str = r##"use wonopui::*;

html! {
    <Card class="max-w-md mx-auto">
        <CardHeader class="text-center">
            <CardTitle class="text-2xl">{"Sign in"}</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
            <div class="grid grid-cols-2 gap-3">
                <Button variant={ButtonVariant::Outline} class="w-full">
                    <GoogleIcon />
                    <span class="ml-2">{"Google"}</span>
                </Button>
                <Button variant={ButtonVariant::Outline} class="w-full">
                    <GitHubIcon />
                    <span class="ml-2">{"GitHub"}</span>
                </Button>
            </div>
            
            <div class="relative">
                <div class="absolute inset-0 flex items-center">
                    <Divider />
                </div>
                <div class="relative flex justify-center text-xs uppercase">
                    <span class="bg-white px-2 text-zinc-500">{"Or continue with"}</span>
                </div>
            </div>
            
            <form class="space-y-4">
                <div>
                    <Label for_id="email">{"Email"}</Label>
                    <Input id="email" kind="email" class="mt-1.5" />
                </div>
                <div>
                    <Label for_id="password">{"Password"}</Label>
                    <Input id="password" kind="password" class="mt-1.5" />
                </div>
                <Button variant={ButtonVariant::Default} class="w-full">{"Sign In"}</Button>
            </form>
        </CardContent>
    </Card>
}
"##;

// =============================================================================
// Checkbox Examples
// =============================================================================

#[function_component(CheckboxSimple)]
fn checkbox_simple() -> Html {
    html! {
        <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
            <div class="flex items-center space-x-2">
                <Checkbox id="terms" />
                <Label for_id="terms" class="!mb-0">{"Accept terms and conditions"}</Label>
            </div>
        </div>
    }
}

#[function_component(CheckboxGroup)]
fn checkbox_group() -> Html {
    html! {
        <div class="p-8 bg-white dark:bg-zinc-950">
            <fieldset class="max-w-sm mx-auto">
                <legend class="text-sm font-semibold text-zinc-900 dark:text-white mb-3">{"Notification preferences"}</legend>
                <div class="space-y-3">
                    <div class="flex items-center gap-3">
                        <Checkbox id="email_notif" checked={true} />
                        <Label for_id="email_notif" class="!mb-0">{"Email notifications"}</Label>
                    </div>
                    <div class="flex items-center gap-3">
                        <Checkbox id="sms_notif" />
                        <Label for_id="sms_notif" class="!mb-0">{"SMS notifications"}</Label>
                    </div>
                    <div class="flex items-center gap-3">
                        <Checkbox id="push_notif" checked={true} />
                        <Label for_id="push_notif" class="!mb-0">{"Push notifications"}</Label>
                    </div>
                </div>
            </fieldset>
        </div>
    }
}

#[function_component(CheckboxWithDescription)]
fn checkbox_with_description() -> Html {
    html! {
        <div class="p-8 bg-white dark:bg-zinc-950">
            <div class="max-w-sm mx-auto space-y-4">
                <div class="flex items-start gap-3">
                    <Checkbox id="marketing" class="mt-1" />
                    <div>
                        <Label for_id="marketing" class="!mb-0">{"Marketing emails"}</Label>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Receive emails about new products, features, and more."}
                        </p>
                    </div>
                </div>
                <div class="flex items-start gap-3">
                    <Checkbox id="security" checked={true} class="mt-1" />
                    <div>
                        <Label for_id="security" class="!mb-0">{"Security alerts"}</Label>
                        <p class="text-sm text-zinc-500 dark:text-zinc-400">
                            {"Get notified about important security updates."}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// Toggle/Switch Examples
// =============================================================================

#[function_component(SwitchSimple)]
fn switch_simple() -> Html {
    html! {
        <div class="flex items-center justify-center p-8 bg-white dark:bg-zinc-950">
            <div class="flex items-center space-x-3">
                <SwitchButton id="airplane" />
                <Label for_id="airplane" class="!mb-0">{"Airplane Mode"}</Label>
            </div>
        </div>
    }
}

#[function_component(SwitchGroup)]
fn switch_group() -> Html {
    html! {
        <div class="p-8 bg-white dark:bg-zinc-950">
            <Card class="max-w-sm mx-auto">
                <CardHeader>
                    <CardTitle>{"Settings"}</CardTitle>
                    <CardDescription>{"Manage your preferences"}</CardDescription>
                </CardHeader>
                <CardContent class="space-y-4">
                    <div class="flex items-center justify-between">
                        <div>
                            <Label for_id="dark_mode" class="!mb-0">{"Dark Mode"}</Label>
                            <p class="text-sm text-zinc-500">{"Use dark theme"}</p>
                        </div>
                        <SwitchButton id="dark_mode" />
                    </div>
                    <Divider />
                    <div class="flex items-center justify-between">
                        <div>
                            <Label for_id="notifications" class="!mb-0">{"Notifications"}</Label>
                            <p class="text-sm text-zinc-500">{"Enable notifications"}</p>
                        </div>
                        <SwitchButton id="notifications" checked={true} />
                    </div>
                    <Divider />
                    <div class="flex items-center justify-between">
                        <div>
                            <Label for_id="analytics" class="!mb-0">{"Analytics"}</Label>
                            <p class="text-sm text-zinc-500">{"Share usage data"}</p>
                        </div>
                        <SwitchButton id="analytics" />
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

#[function_component(ToggleButton)]
fn toggle_button() -> Html {
    html! {
        <div class="flex items-center justify-center gap-4 p-8 bg-white dark:bg-zinc-950">
            <Toggle>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m6-6H6" />
                </svg>
            </Toggle>
            <Toggle variant={ToggleVariant::Outline}>
                {"Bold"}
            </Toggle>
            <Toggle variant={ToggleVariant::Outline} pressed={true}>
                {"Italic"}
            </Toggle>
        </div>
    }
}

// =============================================================================
// Checkbox Code Constants
// =============================================================================

const CHECKBOX_SIMPLE_CODE: &str = r##"use wonopui::wonopui_checkbox::Checkbox;
use wonopui::wonopui_label::Label;

html! {
    <div class="flex items-center space-x-2">
        <Checkbox id="terms" />
        <Label for_id="terms">{"Accept terms and conditions"}</Label>
    </div>
}
"##;

const CHECKBOX_GROUP_CODE: &str = r##"use wonopui::wonopui_checkbox::Checkbox;
use wonopui::wonopui_label::Label;

html! {
    <fieldset>
        <legend class="text-sm font-semibold mb-3">{"Notification preferences"}</legend>
        <div class="space-y-3">
            <div class="flex items-center gap-3">
                <Checkbox id="email" checked={true} />
                <Label for_id="email">{"Email notifications"}</Label>
            </div>
            <div class="flex items-center gap-3">
                <Checkbox id="sms" />
                <Label for_id="sms">{"SMS notifications"}</Label>
            </div>
            <div class="flex items-center gap-3">
                <Checkbox id="push" checked={true} />
                <Label for_id="push">{"Push notifications"}</Label>
            </div>
        </div>
    </fieldset>
}
"##;

const CHECKBOX_DESCRIPTION_CODE: &str = r##"use wonopui::wonopui_checkbox::Checkbox;
use wonopui::wonopui_label::Label;

html! {
    <div class="flex items-start gap-3">
        <Checkbox id="marketing" class="mt-1" />
        <div>
            <Label for_id="marketing">{"Marketing emails"}</Label>
            <p class="text-sm text-zinc-500">
                {"Receive emails about new products, features, and more."}
            </p>
        </div>
    </div>
}
"##;

// =============================================================================
// Toggle/Switch Code Constants
// =============================================================================

const SWITCH_SIMPLE_CODE: &str = r##"use wonopui::wonopui_switch::SwitchButton;
use wonopui::wonopui_label::Label;

html! {
    <div class="flex items-center space-x-3">
        <SwitchButton id="airplane" />
        <Label for_id="airplane">{"Airplane Mode"}</Label>
    </div>
}
"##;

const SWITCH_GROUP_CODE: &str = r##"use wonopui::wonopui_switch::SwitchButton;
use wonopui::wonopui_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use wonopui::wonopui_divider::Divider;

html! {
    <Card>
        <CardHeader>
            <CardTitle>{"Settings"}</CardTitle>
            <CardDescription>{"Manage your preferences"}</CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
            <div class="flex items-center justify-between">
                <div>
                    <Label>{"Dark Mode"}</Label>
                    <p class="text-sm text-zinc-500">{"Use dark theme"}</p>
                </div>
                <SwitchButton />
            </div>
            <Divider />
            <div class="flex items-center justify-between">
                <div>
                    <Label>{"Notifications"}</Label>
                    <p class="text-sm text-zinc-500">{"Enable notifications"}</p>
                </div>
                <SwitchButton checked={true} />
            </div>
        </CardContent>
    </Card>
}
"##;

const TOGGLE_BUTTON_CODE: &str = r##"use wonopui::wonopui_toggle::{Toggle, ToggleVariant};

html! {
    <div class="flex items-center gap-4">
        <Toggle>
            // Plus icon
        </Toggle>
        <Toggle variant={ToggleVariant::Outline}>
            {"Bold"}
        </Toggle>
        <Toggle variant={ToggleVariant::Outline} pressed={true}>
            {"Italic"}
        </Toggle>
    </div>
}
"##;
