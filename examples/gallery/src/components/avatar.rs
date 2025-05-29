use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;

#[function_component(AvatarThemeEditor)]
pub fn avatar_theme_editor() -> Html {
    let fields = vec![
        ("avatar_base".to_string(), "Base Avatar".to_string()),
        ("avatar_small".to_string(), "Small Avatar".to_string()),
        ("avatar_medium".to_string(), "Medium Avatar".to_string()),
        ("avatar_large".to_string(), "Large Avatar".to_string()),
    ];

    let preview = html! {
        <>
            <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Small Avatar" size={AvatarSize::Small} />
            <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Medium Avatar" size={AvatarSize::Medium} />
            <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Large Avatar" size={AvatarSize::Large} />
        </>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(AvatarDocumentation)]
pub fn avatar_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Avatar Component" }</h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Avatar component is used to display user profile images in different sizes. It supports small, medium, and large sizes, and allows for customization of the image URL and alt text." }</p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
            <ExampleCode
                preview={html! {
                    <>
                        <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Small Avatar" size={AvatarSize::Small} />
                        <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Medium Avatar" size={AvatarSize::Medium} />
                        <Avatar src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?q=80&w=3387&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="Large Avatar" size={AvatarSize::Large} />
                    </>
                }}
                customize={html! {
                    <AvatarThemeEditor />
                }}
                code={r#"
<Avatar src="https://example.com/avatar.jpg" alt="Small Avatar" size={AvatarSize::Small} />
<Avatar src="https://example.com/avatar.jpg" alt="Medium Avatar" size={AvatarSize::Medium} />
<Avatar src="https://example.com/avatar.jpg" alt="Large Avatar" size={AvatarSize::Large} />"#.to_string()}
            />

            <Features features={vec!["Customizable sizes", "Alt text support", "Responsive design"]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Avatar"
                description="Props for the Avatar component."
                props={vec![
                    ("src", "String", "The URL of the image to be displayed."),
                    ("alt", "String", "The alt text for the image."),
                    ("size", "AvatarSize", "The size of the avatar. Can be Small, Medium, or Large."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Ensure that the image URL is valid and accessible.".to_string(),
                    "Use appropriate alt text for accessibility.".to_string(),
                    "Choose the size that best fits your layout.".to_string(),
                    "The component automatically handles image loading and error states.".to_string(),
                    "For best results, use square images with a 1:1 aspect ratio.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Avatar".to_string()}
                class_descriptions={vec![
                    ("avatar_base".to_string(), "Base styles for all avatar sizes. Typically includes border-radius for circular shape.".to_string()),
                    ("avatar_small".to_string(), "Styles for small avatars. Usually defines a smaller width and height.".to_string()),
                    ("avatar_medium".to_string(), "Styles for medium avatars. Defines a medium width and height.".to_string()),
                    ("avatar_large".to_string(), "Styles for large avatars. Defines a larger width and height.".to_string()),
                ]}
            />
        </Container>
    }
}
