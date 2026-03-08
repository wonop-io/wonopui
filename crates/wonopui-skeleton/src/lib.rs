//! Skeleton component for wonopui
//!
//! A skeleton loading placeholder that shows animated shimmer effects,
//! following shadcn/ui design patterns.

use wonopui_core::merge_classes;
use yew::prelude::*;

/// Skeleton variant styles
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SkeletonVariant {
    /// Default rectangular skeleton
    #[default]
    Default,
    /// Circular skeleton (for avatars)
    Circle,
    /// Rounded skeleton
    Rounded,
}

impl SkeletonVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "rounded-md",
            Self::Circle => "rounded-full",
            Self::Rounded => "rounded-lg",
        }
    }
}

pub mod classes {
    pub const SKELETON_BASE: &str = "animate-pulse bg-muted";
    pub const SKELETON_PRIMARY: &str = "animate-pulse bg-primary/10";
}

#[derive(Properties, PartialEq)]
pub struct SkeletonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub variant: SkeletonVariant,
    #[prop_or(true)]
    pub show: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProps) -> Html {
    if !props.show {
        return html! { <>{ for props.children.iter() }</> };
    }

    let base_class = merge_classes(&[
        classes::SKELETON_BASE,
        props.variant.class(),
        &props.class.to_string(),
    ]);

    let style = {
        let mut styles = Vec::new();
        if let Some(ref w) = props.width {
            styles.push(format!("width: {}", w));
        }
        if let Some(ref h) = props.height {
            styles.push(format!("height: {}", h));
        }
        if styles.is_empty() {
            None
        } else {
            Some(styles.join("; "))
        }
    };

    html! {
        <div class={base_class} style={style} aria-hidden="true" />
    }
}

#[derive(Properties, PartialEq)]
pub struct SkeletonTextProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(3)]
    pub lines: u8,
    #[prop_or("0.5rem".to_string())]
    pub gap: String,
}

#[function_component(SkeletonText)]
pub fn skeleton_text(props: &SkeletonTextProps) -> Html {
    let container_style = format!("display: flex; flex-direction: column; gap: {}", props.gap);
    
    html! {
        <div class={props.class.clone()} style={container_style}>
            { for (0..props.lines).map(|i| {
                let width = if i == props.lines - 1 { "60%" } else { "100%" };
                html! {
                    <Skeleton width={width.to_string()} height="1rem" />
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SkeletonCardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub show_image: bool,
    #[prop_or(false)]
    pub show_avatar: bool,
}

#[function_component(SkeletonCard)]
pub fn skeleton_card(props: &SkeletonCardProps) -> Html {
    let container_class = merge_classes(&[
        "rounded-lg border border-border p-4 space-y-4",
        &props.class.to_string(),
    ]);

    html! {
        <div class={container_class}>
            if props.show_image {
                <Skeleton width="100%" height="150px" variant={SkeletonVariant::Rounded} />
            }
            <div class="space-y-2">
                if props.show_avatar {
                    <div class="flex items-center gap-3">
                        <Skeleton variant={SkeletonVariant::Circle} width="40px" height="40px" />
                        <div class="flex-1 space-y-2">
                            <Skeleton width="120px" height="14px" />
                            <Skeleton width="80px" height="12px" />
                        </div>
                    </div>
                }
                <Skeleton width="80%" height="16px" />
                <Skeleton width="100%" height="14px" />
                <Skeleton width="60%" height="14px" />
            </div>
        </div>
    }
}
