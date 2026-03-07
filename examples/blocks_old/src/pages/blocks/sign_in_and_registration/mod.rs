use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

#[function_component(SignInAndRegistration)]
pub fn sign_in_and_registration() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Sign In and Registration"} />
            </Breadcrumb>
            <ExampleBlock title={"Sign In Form"}>
                <SignInForm />
            </ExampleBlock>
            <ExampleBlock title={"Registration Form"}>
                <RegistrationForm />
            </ExampleBlock>
            <ExampleBlock title={"Forgot Password Form"}>
                <ForgotPasswordForm />
            </ExampleBlock>
            <ExampleBlock title={"Reset Password Form"}>
                <ResetPasswordForm />
            </ExampleBlock>
        </MainContent>
    }
}

#[function_component(SignInForm)]
pub fn sign_in_form() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Sign In"}</h2>
            <form>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="email">{"Email"}</label>
                    <input class="w-full p-2 border rounded" type="email" id="email" placeholder="Enter your email" />
                </div>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="password">{"Password"}</label>
                    <input class="w-full p-2 border rounded" type="password" id="password" placeholder="Enter your password" />
                </div>
                <Button>{"Sign In"}</Button>
            </form>
        </div>
    }
}

#[function_component(RegistrationForm)]
pub fn registration_form() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Register"}</h2>
            <form>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="username">{"Username"}</label>
                    <input class="w-full p-2 border rounded" type="text" id="username" placeholder="Enter your username" />
                </div>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="email">{"Email"}</label>
                    <input class="w-full p-2 border rounded" type="email" id="email" placeholder="Enter your email" />
                </div>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="password">{"Password"}</label>
                    <input class="w-full p-2 border rounded" type="password" id="password" placeholder="Enter your password" />
                </div>
                <Button>{"Register"}</Button>
            </form>
        </div>
    }
}

#[function_component(ForgotPasswordForm)]
pub fn forgot_password_form() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Forgot Password"}</h2>
            <form>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="email">{"Email"}</label>
                    <input class="w-full p-2 border rounded" type="email" id="email" placeholder="Enter your email" />
                </div>
                <Button>{"Send Reset Link"}</Button>
            </form>
        </div>
    }
}

#[function_component(ResetPasswordForm)]
pub fn reset_password_form() -> Html {
    html! {
        <div class="max-w-md mx-auto p-4 border rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"Reset Password"}</h2>
            <form>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="new_password">{"New Password"}</label>
                    <input class="w-full p-2 border rounded" type="password" id="new_password" placeholder="Enter your new password" />
                </div>
                <div class="mb-4">
                    <label class="block text-sm font-medium mb-1" for="confirm_password">{"Confirm Password"}</label>
                    <input class="w-full p-2 border rounded" type="password" id="confirm_password" placeholder="Confirm your new password" />
                </div>
                <Button>{"Reset Password"}</Button>
            </form>
        </div>
    }
}
