use crate::config::ClassesStr;
use gloo_utils::document;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: (i32, i32),
    pub toggle: Callback<(i32, i32)>,
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuProps {
    #[prop_or_default]
    pub children: Children,
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

    let state = Rc::new(ContextMenuState {
        is_open: *is_open,
        position: *position,
        toggle: toggle.clone(),
    });

    html! {
        <ContextProvider<Rc<ContextMenuState>> context={state}>
            <div class="relative">
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

    html! {
        <div {oncontextmenu} class={classes!(props.class.clone(), "cursor-pointer")}>
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

    if !state.is_open {
        return html! {};
    }

    let style = format!(
        "position: fixed; left: {}px; top: {}px;",
        state.position.0, state.position.1
    );

    html! {
        <div class={classes!(props.class.clone(), "bg-white","dark:bg-zinc-800","border","border-gray-200","dark:border-gray-700","rounded-md","shadow-lg","p-1","z-50")} {style}>
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
}

#[function_component(ContextMenuItem)]
pub fn context_menu_item(props: &ContextMenuItemProps) -> Html {
    let class = classes!(
        "flex",
        "items-center",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        if props.inset { "pl-8" } else { "" },
        if props.disabled {
            "text-gray-400 dark:text-gray-500 cursor-not-allowed"
        } else {
            "text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-zinc-700 cursor-pointer"
        }
    );

    html! {
        <div {class} role="menuitem" tabindex="-1">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuSubProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ContextMenuSub)]
pub fn context_menu_sub(props: &ContextMenuSubProps) -> Html {
    html! {
        <div class="relative">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuSubTriggerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub inset: bool,
}

#[function_component(ContextMenuSubTrigger)]
pub fn context_menu_sub_trigger(props: &ContextMenuSubTriggerProps) -> Html {
    let class = classes!(
        "flex",
        "items-center",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "text-gray-700",
        "dark:text-gray-200",
        "hover:bg-gray-100",
        "dark:hover:bg-zinc-700",
        "cursor-pointer",
        if props.inset { "pl-8" } else { "" }
    );

    html! {
        <div {class} role="menuitem" tabindex="-1">
            { for props.children.iter() }
            <span class="ml-auto pl-2">{ "▶" }</span>
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

#[function_component(ContextMenuSubContent)]
pub fn context_menu_sub_content(props: &ContextMenuSubContentProps) -> Html {
    html! {
        <div class={classes!(props.class.clone(), "absolute","left-full","top-0","min-w-[8rem]","bg-white","dark:bg-zinc-800","border","border-gray-200","dark:border-gray-700","rounded-md","shadow-lg","p-1","z-50")}>
            { for props.children.iter() }
        </div>
    }
}

#[function_component(ContextMenuSeparator)]
pub fn context_menu_separator() -> Html {
    html! {
        <div class="h-px my-1 bg-gray-200 dark:bg-gray-700" role="separator" />
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuCheckboxItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub checked: bool,
}

#[function_component(ContextMenuCheckboxItem)]
pub fn context_menu_checkbox_item(props: &ContextMenuCheckboxItemProps) -> Html {
    let class = classes!(
        "flex",
        "items-center",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "text-gray-700",
        "dark:text-gray-200",
        "hover:bg-gray-100",
        "dark:hover:bg-zinc-700",
        "cursor-pointer"
    );

    html! {
        <div {class} role="menuitemcheckbox" aria-checked={props.checked.to_string()}>
            <span class="mr-2">
                if props.checked {
                    { "✓" }
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
    pub value: String,
}

#[function_component(ContextMenuRadioGroup)]
pub fn context_menu_radio_group(props: &ContextMenuRadioGroupProps) -> Html {
    html! {
        <div role="group">
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub inset: bool,
}

#[function_component(ContextMenuLabel)]
pub fn context_menu_label(props: &ContextMenuLabelProps) -> Html {
    let class = classes!(
        "px-2",
        "py-1.5",
        "text-sm",
        "text-gray-500",
        "dark:text-gray-400",
        if props.inset { "pl-8" } else { "" }
    );

    html! {
        <div {class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContextMenuRadioItemProps {
    #[prop_or_default]
    pub children: Children,
    pub value: String,
}

#[function_component(ContextMenuRadioItem)]
pub fn context_menu_radio_item(props: &ContextMenuRadioItemProps) -> Html {
    let radio_group =
        use_context::<Rc<ContextMenuRadioGroupProps>>().expect("no radio group context found");
    let is_selected = props.value == radio_group.value;

    let class = classes!(
        "flex",
        "items-center",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "text-gray-700",
        "dark:text-gray-200",
        "hover:bg-gray-100",
        "dark:hover:bg-zinc-700",
        "cursor-pointer"
    );

    html! {
        <div {class} role="menuitemradio" aria-checked={is_selected.to_string()}>
            <span class="mr-2">
                if is_selected {
                    { "●" }
                }
            </span>
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
        <span class="ml-auto pl-5 text-xs text-gray-500 dark:text-gray-400">
            { for props.children.iter() }
        </span>
    }
}
