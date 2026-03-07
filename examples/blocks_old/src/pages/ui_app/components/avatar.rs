use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(AvatarDocumentation)]
pub fn avatar_documentation() -> Html {
    html! {
        <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Avatar Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Avatar component is used to display user profile images in different sizes. It supports small, medium, and large sizes, and allows for customization of the image URL and alt text." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <>
                        <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="First Example Avatar" size={AvatarSize::Small} />
                        <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Second Example Avatar" size={AvatarSize::Medium} />
                        <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Third Example Avatar" size={AvatarSize::Large} />
                    </>
                }}
                code={r#"
<div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
    <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="First Example Avatar" size={AvatarSize::Small} />
    <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Second Example Avatar" size={AvatarSize::Medium} />
    <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Third Example Avatar" size={AvatarSize::Large} />
</div>"#.to_string()}
            />

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
            <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Avatar" }</h3>
            <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the avatar component." }</p>
            <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                <li>{ "src: String - The URL of the image to be displayed." }</li>
                <li>{ "alt: String - The alt text for the image." }</li>
                <li>{ "size: AvatarSize - The size of the avatar. It can be one of the following: Small, Medium, Large." }</li>
            </ul>
        </div>
    }
}
