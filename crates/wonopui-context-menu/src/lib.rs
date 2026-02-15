//! ContextMenu component for wonopui
//!
//! A context menu that appears on right-click.

use gloo_timers::callback::Timeout;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wonopui_core::merge_classes;
use yew::prelude::*;

pub mod classes {
    pub const CONTEXT_MENU_CONTENT: &str = "bg-white dark:bg-zinc-800 border border-gray-200 dark:border-zinc-700 rounded-md shadow-lg p-1 z-50 min-w-[8rem]";
    pub const CONTEXT_MENU_ITEM: &str =
        "flex items-center px-2 py-1.5 text-sm outline-none cursor-pointer rounded-sm";
    pub const CONTEXT_MENU_ITEM_DEFAULT: &str =
        "text-gray-700 dark:text-zinc-200 hover:bg-gray-100 dark:hover:bg-zinc-700";
    pub const CONTEXT_MENU_ITEM_DISABLED: &str =
        "text-gray-400 dark:text-zinc-500 cursor-not-allowed";
    pub const CONTEXT_MENU_SEPARATOR: &str = "h-px my-1 bg-gray-200 dark:bg-zinc-700";
    pub const CONTEXT_MENU_LABEL: &str = "px-2 py-1.5 text-sm text-gray-500 dark:text-zinc-400";
    pub const CONTEXT_MENU_SHORTCUT: &str = "ml-auto pl-5 text-xs text-gray-500 dark:text-zinc-400";
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
            <div class={class}>
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
            toggle.emit((event.client_x(), event.client_y()));
        })
    };

    let class = merge_classes(&["cursor-pointer", &props.class.to_string()]);

    html! {
        <div {oncontextmenu} class={class}>
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
        <div class={class} role="menuitem" tabindex="-1" {onclick}>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(ContextMenuSeparator)]
pub fn context_menu_separator() -> Html {
    html! {
        <div class={classes::CONTEXT_MENU_SEPARATOR} role="separator" />
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
        <div class={class}>
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
        <span class={classes::CONTEXT_MENU_SHORTCUT}>
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
            <div class="relative" {onmouseenter} {onmouseleave}>
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
        if props.inset { "pl-8" } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div class={class}>
            { for props.children.iter() }
            <span class="ml-auto">{"›"}</span>
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
        <div class={class}>
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
        <div class={class} role="menuitemcheckbox" aria-checked={props.checked.to_string()} {onclick}>
            <span class="w-4 h-4 mr-2 flex items-center justify-center">
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
            <div role="radiogroup">
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
        <div class={class} role="menuitemradio" aria-checked={is_checked.to_string()} {onclick}>
            <span class="w-4 h-4 mr-2 flex items-center justify-center">
                if is_checked {
                    {"●"}
                } else {
                    {"○"}
                }
            </span>
            { for props.children.iter() }
        </div>
    }
}
