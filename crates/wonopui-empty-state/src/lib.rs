use yew::prelude::*;

/// EmptyState size variants
#[derive(Clone, PartialEq, Default)]
pub enum EmptyStateSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl EmptyStateSize {
    fn icon_size(&self) -> &'static str {
        match self {
            EmptyStateSize::Small => "h-8 w-8",
            EmptyStateSize::Medium => "h-12 w-12",
            EmptyStateSize::Large => "h-16 w-16",
        }
    }
    
    fn title_size(&self) -> &'static str {
        match self {
            EmptyStateSize::Small => "text-sm",
            EmptyStateSize::Medium => "text-base",
            EmptyStateSize::Large => "text-lg",
        }
    }
    
    fn padding(&self) -> &'static str {
        match self {
            EmptyStateSize::Small => "py-6",
            EmptyStateSize::Medium => "py-10",
            EmptyStateSize::Large => "py-16",
        }
    }
}

/// EmptyState props
#[derive(Properties, Clone, PartialEq)]
pub struct EmptyStateProps {
    /// Title text
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Description text
    #[prop_or_default]
    pub description: Option<AttrValue>,
    /// Icon element
    #[prop_or_default]
    pub icon: Option<Html>,
    /// Action element (typically a button)
    #[prop_or_default]
    pub action: Option<Html>,
    /// Size variant
    #[prop_or_default]
    pub size: EmptyStateSize,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// Children content
    #[prop_or_default]
    pub children: Html,
}

#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProps) -> Html {
    let EmptyStateProps {
        title,
        description,
        icon,
        action,
        size,
        class,
        children,
    } = props.clone();

    let container_classes = classes!(
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "text-center",
        size.padding(),
        class,
    );

    let icon_classes = classes!(
        "text-zinc-400",
        "dark:text-zinc-500",
        "mb-4",
        size.icon_size(),
    );

    let title_classes = classes!(
        "font-semibold",
        "text-zinc-900",
        "dark:text-white",
        size.title_size(),
    );

    html! {
        <div class={container_classes}>
            if let Some(icon_element) = icon {
                <div class={icon_classes}>
                    {icon_element}
                </div>
            }
            
            if let Some(title_text) = title {
                <h3 class={title_classes}>
                    {title_text}
                </h3>
            }
            
            if let Some(desc) = description {
                <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400 max-w-sm">
                    {desc}
                </p>
            }
            
            {children}
            
            if let Some(action_element) = action {
                <div class="mt-6">
                    {action_element}
                </div>
            }
        </div>
    }
}
