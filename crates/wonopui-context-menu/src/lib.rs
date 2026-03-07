//! ContextMenu component for wonopui
//!
//! A context menu that appears on right-click.

use gloo_timers::callback::Timeout;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

/// Default CSS classes for context menu styling (shadcn v4 style).
pub mod classes {
    /// Content container styles - shadcn v4 ContextMenuContent with animations.
    pub const CONTEXT_MENU_CONTENT: &str = "z-50 min-w-[8rem] overflow-hidden rounded-md border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 p-1 text-zinc-950 dark:text-zinc-50 shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";
    
    /// Item base styles - shadcn v4 ContextMenuItem with proper focus and gap.
    pub const CONTEXT_MENU_ITEM: &str = "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
    
    /// Item default state styles - shadcn v4 focus styles.
    pub const CONTEXT_MENU_ITEM_DEFAULT: &str = "text-zinc-900 dark:text-zinc-50 focus:bg-zinc-100 dark:focus:bg-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-800 [&_svg:not([class*='text-'])]:text-zinc-500 dark:[&_svg:not([class*='text-'])]:text-zinc-400";
    
    /// Destructive item variant.
    pub const CONTEXT_MENU_ITEM_DESTRUCTIVE: &str = "text-red-600 dark:text-red-400 focus:bg-red-50 dark:focus:bg-red-950/50 focus:text-red-600 dark:focus:text-red-400 [&_svg]:!text-red-600 dark:[&_svg]:!text-red-400";
    
    /// Item disabled state styles.
    pub const CONTEXT_MENU_ITEM_DISABLED: &str = "pointer-events-none opacity-50";
    
    /// Separator styles - shadcn v4 ContextMenuSeparator.
    pub const CONTEXT_MENU_SEPARATOR: &str = "-mx-1 my-1 h-px bg-zinc-200 dark:bg-zinc-800";
    
    /// Label styles - shadcn v4 ContextMenuLabel.
    pub const CONTEXT_MENU_LABEL: &str = "px-2 py-1.5 text-sm font-medium text-zinc-950 dark:text-zinc-50";
    
    /// Shortcut text styles - shadcn v4 ContextMenuShortcut.
    pub const CONTEXT_MENU_SHORTCUT: &str = "ml-auto text-xs tracking-widest text-zinc-500 dark:text-zinc-400";
    
    /// Checkbox/radio indicator container.
    pub const CONTEXT_MENU_INDICATOR: &str = "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center";
    
    /// SubTrigger styles - shadcn v4.
    pub const CONTEXT_MENU_SUB_TRIGGER: &str = "focus:bg-zinc-100 dark:focus:bg-zinc-800 data-[state=open]:bg-zinc-100 dark:data-[state=open]:bg-zinc-800";
}

#[derive(Clone, PartialEq)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: (i32, i32),
    pub toggle: Callback<(i32, i32)>,
    pub close: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ContextMenu)]
pub fn context_menu(props: &ContextMenuProps) -> Html {
    let is_open = use_state(|| false);
    let position = use_state(|| (0, 0));

    let toggle = {
        let is_open = is_open.clone();
        let position = position.clone();
        Callback::from(move |(x, y): (i32, i32)| {
            is_open.set(!*is_open);
            position.set((x, y));
        })
    };

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(false);
        })
    };

    let state = Rc::new(ContextMenuState {
        is_open: *is_open,
        position: *position,
        toggle: toggle.clone(),
        close: close.clone(),
    });

    let class = merge_classes(&["relative", &props.class.to_string()]);

    html! {
        <ContextProvider<Rc<ContextMenuState>> context={state}>
            <div data-slot="context-menu" class={class}>
                { for props.children.iter() }
            </div>
        </ContextProvider<Rc<ContextMenuState>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuTriggerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ContextMenuTrigger)]
pub fn context_menu_trigger(props: &ContextMenuTriggerProps) -> Html {
    let state = use_context::<Rc<ContextMenuState>>().expect("no context found");

    let oncontextmenu = {
        let toggle = state.toggle.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            toggle.emit((event.client_x() as i32, event.client_y() as i32));
        })
    };

    let class = merge_classes(&["cursor-pointer", &props.class.to_string()]);

    html! {
        <div data-slot="context-menu-trigger" {oncontextmenu} class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ContextMenuContent)]
pub fn context_menu_content(props: &ContextMenuContentProps) -> Html {
    let state = use_context::<Rc<ContextMenuState>>().expect("no context found");
    let menu_ref = use_node_ref();

    // Focus the menu when it opens
    {
        let is_open = state.is_open;
        let menu_ref = menu_ref.clone();
        use_effect_with(is_open, move |is_open| {
            if *is_open {
                if let Some(element) = menu_ref.cast::<web_sys::HtmlElement>() {
                    let _ = element.focus();
                }
            }
            || {}
        });
    }

    if !state.is_open {
        return html! {};
    }

    let style = format!(
        "position: fixed; left: {}px; top: {}px;",
        state.position.0, state.position.1
    );

    let onblur = {
        let close = state.close.clone();
        let menu_ref = menu_ref.clone();

        Callback::from(move |e: FocusEvent| {
            if let Some(related_target) = e.related_target() {
                let related_element: web_sys::Element = related_target.unchecked_into();
                if let Some(menu_element) = menu_ref.cast::<web_sys::Element>() {
                    if !menu_element.contains(Some(&related_element)) {
                        let close_callback = close.clone();
                        Timeout::new(50, move || {
                            close_callback.emit(());
                        })
                        .forget();
                    }
                }
            } else {
                let close_callback = close.clone();
                Timeout::new(50, move || {
                    close_callback.emit(());
                })
                .forget();
            }
        })
    };

    let class = merge_classes(&[classes::CONTEXT_MENU_CONTENT, &props.class.to_string()]);

    html! {
        <div
            data-slot="context-menu-content"
            data-state="open"
            ref={menu_ref}
            class={class}
            {style}
            tabindex="0"
            {onblur}
        >
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub inset: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ContextMenuItem)]
pub fn context_menu_item(props: &ContextMenuItemProps) -> Html {
    let state = use_context::<Rc<ContextMenuState>>().expect("no context found");

    let onclick = {
        let close = state.close.clone();
        let disabled = props.disabled;
        let user_onclick = props.onclick.clone();

        Callback::from(move |e: MouseEvent| {
            if !disabled {
                user_onclick.emit(e);
                let close_callback = close.clone();
                Timeout::new(50, move || {
                    close_callback.emit(());
                })
                .forget();
            }
        })
    };

    let class = merge_classes(&[
        classes::CONTEXT_MENU_ITEM,
        if props.disabled {
            classes::CONTEXT_MENU_ITEM_DISABLED
        } else {
            classes::CONTEXT_MENU_ITEM_DEFAULT
        },
        if props.inset { "pl-8" } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="context-menu-item" data-inset={props.inset.to_string()} class={class} role="menuitem" tabindex="-1" {onclick}>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(ContextMenuSeparator)]
pub fn context_menu_separator() -> Html {
    html! {
        <div data-slot="context-menu-separator" class={classes::CONTEXT_MENU_SEPARATOR} role="separator" />
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub inset: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ContextMenuLabel)]
pub fn context_menu_label(props: &ContextMenuLabelProps) -> Html {
    let class = merge_classes(&[
        classes::CONTEXT_MENU_LABEL,
        if props.inset { "pl-8" } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="context-menu-label" data-inset={props.inset.to_string()} class={class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuShortcutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ContextMenuShortcut)]
pub fn context_menu_shortcut(props: &ContextMenuShortcutProps) -> Html {
    html! {
        <span data-slot="context-menu-shortcut" class={classes::CONTEXT_MENU_SHORTCUT}>
            { for props.children.iter() }
        </span>
    }
}

// ============================================================================
// Submenu components
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct ContextMenuSubProps {
    #[prop_or_default]
    pub children: Children,
}

/// Container for a submenu
#[function_component(ContextMenuSub)]
pub fn context_menu_sub(props: &ContextMenuSubProps) -> Html {
    let is_open = use_state(|| false);

    let onmouseenter = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(true);
        })
    };

    let onmouseleave = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(false);
        })
    };

    html! {
        <ContextProvider<UseStateHandle<bool>> context={is_open}>
            <div data-slot="context-menu-sub" class="relative" {onmouseenter} {onmouseleave}>
                { for props.children.iter() }
            </div>
        </ContextProvider<UseStateHandle<bool>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuSubTriggerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub inset: bool,
    #[prop_or_default]
    pub class: Classes,
}

/// Trigger element for a submenu
#[function_component(ContextMenuSubTrigger)]
pub fn context_menu_sub_trigger(props: &ContextMenuSubTriggerProps) -> Html {
    let class = merge_classes(&[
        classes::CONTEXT_MENU_ITEM,
        classes::CONTEXT_MENU_ITEM_DEFAULT,
        classes::CONTEXT_MENU_SUB_TRIGGER,
        if props.inset { "pl-8" } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="context-menu-sub-trigger" data-inset={props.inset.to_string()} class={class}>
            { for props.children.iter() }
            <span class="ml-auto size-4">{"›"}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuSubContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// Content container for a submenu
#[function_component(ContextMenuSubContent)]
pub fn context_menu_sub_content(props: &ContextMenuSubContentProps) -> Html {
    let is_open = use_context::<UseStateHandle<bool>>();

    let should_show = is_open.map(|s| *s).unwrap_or(false);

    if !should_show {
        return html! {};
    }

    let class = merge_classes(&[
        "absolute left-full top-0 ml-1",
        classes::CONTEXT_MENU_CONTENT,
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="context-menu-sub-content" data-state="open" class={class}>
            { for props.children.iter() }
        </div>
    }
}

// ============================================================================
// Checkbox and Radio items
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct ContextMenuCheckboxItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

/// A checkbox item in the context menu
#[function_component(ContextMenuCheckboxItem)]
pub fn context_menu_checkbox_item(props: &ContextMenuCheckboxItemProps) -> Html {
    let state = use_context::<Rc<ContextMenuState>>().expect("no context found");

    let onclick = {
        let close = state.close.clone();
        let onchange = props.onchange.clone();
        let user_onclick = props.onclick.clone();
        let checked = props.checked;
        Callback::from(move |e: MouseEvent| {
            user_onclick.emit(e);
            onchange.emit(!checked);
            let close_callback = close.clone();
            Timeout::new(50, move || {
                close_callback.emit(());
            })
            .forget();
        })
    };

    let class = merge_classes(&[
        classes::CONTEXT_MENU_ITEM,
        classes::CONTEXT_MENU_ITEM_DEFAULT,
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="context-menu-checkbox-item" class={classes!(&class, "pl-8", "pr-2")} role="menuitemcheckbox" aria-checked={props.checked.to_string()} {onclick}>
            <span class={classes::CONTEXT_MENU_INDICATOR}>
                if props.checked {
                    {"✓"}
                }
            </span>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuRadioGroupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
}

/// Context for radio group state
#[derive(Clone, PartialEq)]
pub struct RadioGroupContext {
    pub value: String,
    pub onchange: Callback<String>,
}

/// A group of radio items in the context menu
#[function_component(ContextMenuRadioGroup)]
pub fn context_menu_radio_group(props: &ContextMenuRadioGroupProps) -> Html {
    let context = RadioGroupContext {
        value: props.value.clone(),
        onchange: props.onchange.clone(),
    };

    html! {
        <ContextProvider<RadioGroupContext> context={context}>
            <div data-slot="context-menu-radio-group" role="radiogroup">
                { for props.children.iter() }
            </div>
        </ContextProvider<RadioGroupContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuRadioItemProps {
    #[prop_or_default]
    pub children: Children,
    pub value: String,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

/// A radio item in the context menu
#[function_component(ContextMenuRadioItem)]
pub fn context_menu_radio_item(props: &ContextMenuRadioItemProps) -> Html {
    let state = use_context::<Rc<ContextMenuState>>().expect("no context found");
    let radio_context = use_context::<RadioGroupContext>();

    // Determine if this item is checked (either from prop or from radio group context)
    let is_checked = if let Some(ref ctx) = radio_context {
        ctx.value == props.value
    } else {
        props.checked
    };

    let onclick = {
        let close = state.close.clone();
        let user_onclick = props.onclick.clone();
        let value = props.value.clone();
        let radio_context = radio_context.clone();
        Callback::from(move |e: MouseEvent| {
            user_onclick.emit(e);
            if let Some(ref ctx) = radio_context {
                ctx.onchange.emit(value.clone());
            }
            let close_callback = close.clone();
            Timeout::new(50, move || {
                close_callback.emit(());
            })
            .forget();
        })
    };

    let class = merge_classes(&[
        classes::CONTEXT_MENU_ITEM,
        classes::CONTEXT_MENU_ITEM_DEFAULT,
        &props.class.to_string(),
    ]);

    html! {
        <div data-slot="context-menu-radio-item" class={classes!(&class, "pl-8", "pr-2")} role="menuitemradio" aria-checked={is_checked.to_string()} {onclick}>
            <span class={classes::CONTEXT_MENU_INDICATOR}>
                if is_checked {
                    {"●"}
                }
            </span>
            { for props.children.iter() }
        </div>
    }
}
