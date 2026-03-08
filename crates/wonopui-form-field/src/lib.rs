use yew::prelude::*;

/// FormField props
#[derive(Properties, Clone, PartialEq)]
pub struct FormFieldProps {
    /// Label text for the field
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Description text (shown below label)
    #[prop_or_default]
    pub description: Option<AttrValue>,
    /// Error message (shown below input)
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// Whether the field is required
    #[prop_or_default]
    pub required: bool,
    /// The form input element(s)
    #[prop_or_default]
    pub children: Html,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// Field ID for accessibility
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[function_component(FormField)]
pub fn form_field(props: &FormFieldProps) -> Html {
    let FormFieldProps {
        label,
        description,
        error,
        required,
        children,
        class,
        id,
    } = props.clone();

    let has_error = error.is_some();
    
    let container_classes = classes!(
        "space-y-2",
        class,
    );

    let label_classes = classes!(
        "block",
        "text-sm",
        "font-medium",
        if has_error {
            "text-red-600 dark:text-red-400"
        } else {
            "text-zinc-700 dark:text-zinc-300"
        },
    );

    html! {
        <div class={container_classes}>
            if let Some(label_text) = label {
                <label class={label_classes} for={id.clone()}>
                    {label_text}
                    if required {
                        <span class="text-red-500 ml-1">{"*"}</span>
                    }
                </label>
            }
            
            if let Some(desc) = description {
                <p class="text-sm text-zinc-500 dark:text-zinc-400">
                    {desc}
                </p>
            }
            
            <div>
                {children}
            </div>
            
            if let Some(err) = error {
                <p class="text-sm text-red-600 dark:text-red-400">
                    {err}
                </p>
            }
        </div>
    }
}
