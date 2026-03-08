use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Modal size variants
#[derive(Clone, PartialEq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
    FullScreen,
}

impl ModalSize {
    fn class(&self) -> &'static str {
        match self {
            ModalSize::Small => "sm:max-w-sm",
            ModalSize::Medium => "sm:max-w-lg",
            ModalSize::Large => "sm:max-w-2xl",
            ModalSize::FullScreen => "sm:max-w-full sm:m-4",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    /// Whether the modal is open
    pub open: bool,
    /// Callback when the modal requests to close
    pub on_close: Callback<()>,
    /// Modal title
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Modal description/subtitle
    #[prop_or_default]
    pub description: Option<AttrValue>,
    /// Modal content
    #[prop_or_default]
    pub children: Html,
    /// Footer content (typically buttons)
    #[prop_or_default]
    pub footer: Option<Html>,
    /// Modal size
    #[prop_or_default]
    pub size: ModalSize,
    /// Whether clicking the backdrop closes the modal
    #[prop_or(true)]
    pub close_on_backdrop: bool,
    /// Whether pressing Escape closes the modal
    #[prop_or(true)]
    pub close_on_escape: bool,
    /// Whether to show the close button (X) in the header
    #[prop_or(true)]
    pub show_close_button: bool,
    /// Additional CSS classes for the modal panel
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps {
        open,
        on_close,
        title,
        description,
        children,
        footer,
        size,
        close_on_backdrop,
        close_on_escape,
        show_close_button,
        class,
    } = props.clone();

    let modal_ref = use_node_ref();
    
    // Handle escape key
    {
        let on_close = on_close.clone();
        let close_on_escape = close_on_escape;
        let open = open;
        use_effect_with((open, close_on_escape), move |(open, close_on_escape)| {
            if !*open || !*close_on_escape {
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            }
            
            let on_close = on_close.clone();
            let listener = gloo::events::EventListener::new(
                &gloo::utils::document(),
                "keydown",
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                    if event.key() == "Escape" {
                        on_close.emit(());
                    }
                },
            );
            
            Box::new(move || drop(listener)) as Box<dyn FnOnce()>
        });
    }

    // Focus trap - trap Tab key within the modal
    {
        let modal_ref = modal_ref.clone();
        let open = open;
        use_effect_with(open, move |open| {
            if !*open {
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            }
            
            let modal_ref = modal_ref.clone();
            
            // Focus the first focusable element when modal opens
            if let Some(modal) = modal_ref.cast::<web_sys::HtmlElement>() {
                let focusable = modal.query_selector(
                    "button, [href], input, select, textarea, [tabindex]:not([tabindex=\"-1\"])"
                );
                if let Ok(Some(first)) = focusable {
                    if let Some(el) = first.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = el.focus();
                    }
                }
            }
            
            let listener = gloo::events::EventListener::new(
                &gloo::utils::document(),
                "keydown",
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                    if event.key() != "Tab" {
                        return;
                    }
                    
                    if let Some(modal) = modal_ref.cast::<web_sys::HtmlElement>() {
                        let focusable_elements = modal.query_selector_all(
                            "button, [href], input, select, textarea, [tabindex]:not([tabindex=\"-1\"])"
                        );
                        
                        if let Ok(elements) = focusable_elements {
                            let len = elements.length();
                            if len == 0 {
                                return;
                            }
                            
                            let first = elements.get(0);
                            let last = elements.get(len - 1);
                            
                            let active = gloo::utils::document().active_element();
                            
                            if event.shift_key() {
                                // Shift+Tab: if on first element, go to last
                                if let (Some(first), Some(active)) = (&first, &active) {
                                    // Compare by casting both to Element
                                    if let Some(first_el) = first.dyn_ref::<web_sys::Element>() {
                                        if first_el == active {
                                            event.prevent_default();
                                            if let Some(last) = &last {
                                                if let Some(el) = last.dyn_ref::<web_sys::HtmlElement>() {
                                                    let _ = el.focus();
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Tab: if on last element, go to first
                                if let (Some(last), Some(active)) = (&last, &active) {
                                    if let Some(last_el) = last.dyn_ref::<web_sys::Element>() {
                                        if last_el == active {
                                            event.prevent_default();
                                            if let Some(first) = &first {
                                                if let Some(el) = first.dyn_ref::<web_sys::HtmlElement>() {
                                                    let _ = el.focus();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            );
            
            Box::new(move || drop(listener)) as Box<dyn FnOnce()>
        });
    }

    let handle_backdrop_click = {
        let on_close = on_close.clone();
        let close_on_backdrop = close_on_backdrop;
        Callback::from(move |_| {
            if close_on_backdrop {
                on_close.emit(());
            }
        })
    };

    let handle_panel_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    let handle_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    if !open {
        return html! {};
    }

    let size_class = size.class();
    let panel_classes = classes!(
        "relative",
        "transform",
        "overflow-hidden",
        "rounded-lg",
        "bg-white",
        "dark:bg-zinc-900",
        "text-left",
        "shadow-xl",
        "transition-all",
        "w-full",
        size_class,
        class,
    );

    let has_header = title.is_some() || description.is_some() || show_close_button;

    html! {
        <div
            class="fixed inset-0 z-50 overflow-y-auto"
            role="dialog"
            aria-modal="true"
        >
            // Backdrop with animation
            <div
                class="fixed inset-0 bg-black/50 transition-opacity animate-in fade-in duration-200"
                onclick={handle_backdrop_click}
            />

            // Modal container
            <div class="flex min-h-full items-center justify-center p-4">
                <div 
                    ref={modal_ref}
                    class={classes!(panel_classes, "animate-in", "fade-in", "zoom-in-95", "duration-200")} 
                    onclick={handle_panel_click}
                >
                    // Header
                    if has_header {
                        <div class="px-6 pt-6 pb-4 flex items-start justify-between">
                            <div class="flex-1">
                                if let Some(title) = title {
                                    <h3 class="text-lg font-semibold text-zinc-900 dark:text-white">
                                        {title}
                                    </h3>
                                }
                                if let Some(description) = description {
                                    <p class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
                                        {description}
                                    </p>
                                }
                            </div>
                            if show_close_button {
                                <button
                                    type="button"
                                    class="ml-4 rounded-md p-1 text-zinc-400 hover:text-zinc-500 dark:text-zinc-500 dark:hover:text-zinc-400 focus:outline-none focus:ring-2 focus:ring-zinc-400 focus:ring-offset-2 dark:focus:ring-offset-zinc-900 transition-colors"
                                    onclick={handle_close_click}
                                    aria-label="Close"
                                >
                                    <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            }
                        </div>
                    }

                    // Content
                    <div class="px-6 py-4">
                        {children}
                    </div>

                    // Footer
                    if let Some(footer) = footer {
                        <div class="px-6 py-4 bg-zinc-50 dark:bg-zinc-800/50 flex justify-end gap-3">
                            {footer}
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
