use yew::prelude::*;

/// IconButton size variants
#[derive(Clone, PartialEq, Default)]
pub enum IconButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl IconButtonSize {
    fn class(&self) -> &'static str {
        match self {
            IconButtonSize::Small => "h-7 w-7",
            IconButtonSize::Medium => "h-9 w-9",
            IconButtonSize::Large => "h-11 w-11",
        }
    }
    
    fn icon_size(&self) -> &'static str {
        match self {
            IconButtonSize::Small => "h-4 w-4",
            IconButtonSize::Medium => "h-5 w-5",
            IconButtonSize::Large => "h-6 w-6",
        }
    }
}

/// IconButton variant (visual style)
#[derive(Clone, PartialEq, Default)]
pub enum IconButtonVariant {
    #[default]
    Ghost,
    Outline,
    Solid,
}

impl IconButtonVariant {
    fn class(&self, color: &IconButtonColor) -> String {
        match self {
            IconButtonVariant::Ghost => match color {
                IconButtonColor::Default => "hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white".to_string(),
                IconButtonColor::Primary => "hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-900 dark:text-white".to_string(),
                IconButtonColor::Danger => "hover:bg-red-50 dark:hover:bg-red-950 text-red-500 dark:text-red-400 hover:text-red-600 dark:hover:text-red-300".to_string(),
                IconButtonColor::Success => "hover:bg-emerald-50 dark:hover:bg-emerald-950 text-emerald-500 dark:text-emerald-400 hover:text-emerald-600 dark:hover:text-emerald-300".to_string(),
                IconButtonColor::Warning => "hover:bg-amber-50 dark:hover:bg-amber-950 text-amber-500 dark:text-amber-400 hover:text-amber-600 dark:hover:text-amber-300".to_string(),
            },
            IconButtonVariant::Outline => match color {
                IconButtonColor::Default => "border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-600 dark:text-zinc-400".to_string(),
                IconButtonColor::Primary => "border border-zinc-900 dark:border-white hover:bg-zinc-900 dark:hover:bg-white text-zinc-900 dark:text-white hover:text-white dark:hover:text-zinc-900".to_string(),
                IconButtonColor::Danger => "border border-red-500 dark:border-red-400 hover:bg-red-500 dark:hover:bg-red-400 text-red-500 dark:text-red-400 hover:text-white dark:hover:text-zinc-900".to_string(),
                IconButtonColor::Success => "border border-emerald-500 dark:border-emerald-400 hover:bg-emerald-500 dark:hover:bg-emerald-400 text-emerald-500 dark:text-emerald-400 hover:text-white dark:hover:text-zinc-900".to_string(),
                IconButtonColor::Warning => "border border-amber-500 dark:border-amber-400 hover:bg-amber-500 dark:hover:bg-amber-400 text-amber-500 dark:text-amber-400 hover:text-white dark:hover:text-zinc-900".to_string(),
            },
            IconButtonVariant::Solid => match color {
                IconButtonColor::Default => "bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-100".to_string(),
                IconButtonColor::Primary => "bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-100".to_string(),
                IconButtonColor::Danger => "bg-red-500 dark:bg-red-600 text-white hover:bg-red-600 dark:hover:bg-red-500".to_string(),
                IconButtonColor::Success => "bg-emerald-500 dark:bg-emerald-600 text-white hover:bg-emerald-600 dark:hover:bg-emerald-500".to_string(),
                IconButtonColor::Warning => "bg-amber-500 dark:bg-amber-600 text-white hover:bg-amber-600 dark:hover:bg-amber-500".to_string(),
            },
        }
    }
}

/// IconButton color variants
#[derive(Clone, PartialEq, Default)]
pub enum IconButtonColor {
    #[default]
    Default,
    Primary,
    Danger,
    Success,
    Warning,
}

/// IconButton props
#[derive(Properties, Clone, PartialEq)]
pub struct IconButtonProps {
    /// Icon element
    pub icon: Html,
    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    /// Size variant
    #[prop_or_default]
    pub size: IconButtonSize,
    /// Visual variant
    #[prop_or_default]
    pub variant: IconButtonVariant,
    /// Color variant
    #[prop_or_default]
    pub color: IconButtonColor,
    /// Whether the button is disabled
    #[prop_or_default]
    pub disabled: bool,
    /// Show loading spinner
    #[prop_or_default]
    pub loading: bool,
    /// Toggle/selected state (shows active styling)
    #[prop_or_default]
    pub active: bool,
    /// Accessible label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /// Title tooltip
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// Button type
    #[prop_or("button".into())]
    pub r#type: AttrValue,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let IconButtonProps {
        icon,
        onclick,
        size,
        variant,
        color,
        disabled,
        loading,
        active,
        aria_label,
        title,
        class,
        r#type,
    } = props.clone();

    let is_disabled = disabled || loading;

    let button_classes = classes!(
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-md",
        "transition-colors",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-zinc-400",
        "focus:ring-offset-2",
        "dark:focus:ring-offset-zinc-900",
        size.class(),
        variant.class(&color),
        if active {
            "bg-zinc-100 dark:bg-zinc-800 ring-2 ring-zinc-400"
        } else {
            ""
        },
        if is_disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-pointer"
        },
        class,
    );

    let icon_wrapper_classes = classes!(
        size.icon_size(),
    );

    // Spinner SVG for loading state
    let spinner = html! {
        <svg class={classes!(size.icon_size(), "animate-spin")} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
    };

    html! {
        <button
            type={r#type}
            class={button_classes}
            onclick={onclick}
            disabled={is_disabled}
            aria-label={aria_label}
            aria-pressed={if active { Some("true") } else { None }}
            title={title}
        >
            if loading {
                { spinner }
            } else {
                <span class={icon_wrapper_classes}>
                    {icon}
                </span>
            }
        </button>
    }
}
