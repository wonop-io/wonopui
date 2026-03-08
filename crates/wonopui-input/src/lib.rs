//! Input component for WonopUI.
//!
//! A text input component with various configuration options.
//! Styled to match shadcn/ui v4 design system.

use web_sys::HtmlInputElement;
use wonopui_core::*;

/// Default CSS classes for input styling.
/// Based on shadcn/ui v4 input component.
pub mod classes {
    /// Base input styles - matches shadcn v4 Input component.
    /// Uses the shadcn v4 focus pattern: focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    pub const BASE: &str = "flex h-10 w-full min-w-0 rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent dark:bg-zinc-950/30 px-3.5 py-2 text-base md:text-sm shadow-xs transition-[color,box-shadow] duration-200 outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-zinc-900 dark:file:text-zinc-50 placeholder:text-zinc-500 dark:placeholder:text-zinc-400 selection:bg-zinc-900 selection:text-zinc-50 dark:selection:bg-zinc-50 dark:selection:text-zinc-900 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 text-zinc-900 dark:text-zinc-50";
    
    /// Wrapper for input with prefix/suffix elements
    pub const INPUT_WRAPPER: &str = "relative flex items-center w-full";
    
    /// Input when used with prefix/suffix - no left/right padding handled by wrapper
    pub const INPUT_WITH_ADDONS: &str = "flex h-10 w-full min-w-0 rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent dark:bg-zinc-950/30 py-2 text-base md:text-sm shadow-xs transition-[color,box-shadow] duration-200 outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-zinc-900 dark:file:text-zinc-50 placeholder:text-zinc-500 dark:placeholder:text-zinc-400 selection:bg-zinc-900 selection:text-zinc-50 dark:selection:bg-zinc-50 dark:selection:text-zinc-900 focus-visible:border-zinc-950 dark:focus-visible:border-zinc-300 focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px] disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 text-zinc-900 dark:text-zinc-50";
    
    /// Prefix/suffix icon container
    pub const INPUT_ICON: &str = "absolute flex items-center justify-center text-zinc-500 dark:text-zinc-400 pointer-events-none";
    
    /// Left icon positioning
    pub const INPUT_ICON_LEFT: &str = "left-3";
    
    /// Right icon positioning
    pub const INPUT_ICON_RIGHT: &str = "right-3";
    
    /// Input group with start/end buttons
    pub const INPUT_GROUP: &str = "flex items-center";
    
    /// Start button in input group
    pub const INPUT_GROUP_START: &str = "inline-flex items-center justify-center h-10 px-3 border border-r-0 border-zinc-200 dark:border-zinc-800 rounded-l-md bg-zinc-50 dark:bg-zinc-900 text-zinc-600 dark:text-zinc-400 text-sm font-medium";
    
    /// End button in input group
    pub const INPUT_GROUP_END: &str = "inline-flex items-center justify-center h-10 px-3 border border-l-0 border-zinc-200 dark:border-zinc-800 rounded-r-md bg-zinc-50 dark:bg-zinc-900 text-zinc-600 dark:text-zinc-400 text-sm font-medium";
    
    /// Input when used within a group (no rounded corners on appropriate side)
    pub const INPUT_IN_GROUP_MIDDLE: &str = "rounded-none";
    pub const INPUT_IN_GROUP_START: &str = "rounded-r-none";
    pub const INPUT_IN_GROUP_END: &str = "rounded-l-none";
}

/// Properties for the Input component.
#[derive(Properties, PartialEq)]
pub struct InputProps {
    /// Current input value.
    #[prop_or_default]
    pub value: String,

    /// Callback fired on input events.
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,

    /// Callback fired with the text value on input.
    #[prop_or_default]
    pub ontext: Callback<String>,

    /// Callback fired on change events.
    #[prop_or_default]
    pub onchange: Callback<Event>,

    /// Callback fired on keypress events.
    #[prop_or_default]
    pub onkeypress: Callback<KeyboardEvent>,

    /// Callback fired on keydown events.
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,

    /// Callback fired on keyup events.
    #[prop_or_default]
    pub onkeyup: Callback<KeyboardEvent>,

    /// Callback fired on focus events.
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,

    /// Callback fired on blur events.
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,

    /// Placeholder text.
    #[prop_or_default]
    pub placeholder: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: Classes,

    /// Input ID attribute.
    #[prop_or_default]
    pub id: String,

    /// Input name attribute.
    #[prop_or_default]
    pub name: String,

    /// Input type (e.g., "text", "password", "email").
    #[prop_or("text".to_string())]
    pub kind: String,

    /// Maximum input length.
    #[prop_or_default]
    pub maxlength: Option<i32>,

    /// Whether the input is read-only.
    #[prop_or_default]
    pub readonly: bool,

    /// Minimum value (for number inputs).
    #[prop_or_default]
    pub min: Option<String>,

    /// Maximum value (for number inputs).
    #[prop_or_default]
    pub max: Option<String>,

    /// Step value (for number inputs).
    #[prop_or_default]
    pub step: Option<String>,

    /// Node reference for direct DOM access.
    #[prop_or_default]
    pub node_ref: NodeRef,

    /// Whether the input is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the input is required.
    #[prop_or_default]
    pub required: bool,

    /// Left/prefix icon (Html).
    #[prop_or_default]
    pub prefix_icon: Option<Html>,

    /// Right/suffix icon (Html).
    #[prop_or_default]
    pub suffix_icon: Option<Html>,

    /// Start addon element (button or text).
    #[prop_or_default]
    pub start_addon: Option<Html>,

    /// End addon element (button or text).
    #[prop_or_default]
    pub end_addon: Option<Html>,
}

/// A text input component with various configuration options.
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let ontext = use_callback(
        (props.ontext.clone(), props.oninput.clone()),
        |e: InputEvent, (ontext, oninput)| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ontext.emit(input.value());
            oninput.emit(e);
        },
    );

    let onchange = use_callback(props.onchange.clone(), |e, onchange| {
        onchange.emit(e);
    });

    let has_prefix = props.prefix_icon.is_some();
    let has_suffix = props.suffix_icon.is_some();
    let has_start = props.start_addon.is_some();
    let has_end = props.end_addon.is_some();
    let has_icons = has_prefix || has_suffix;
    let has_addons = has_start || has_end;

    // Determine input class based on addons
    let input_class = if has_addons {
        let base = classes::INPUT_WITH_ADDONS;
        let rounded = match (has_start, has_end) {
            (true, true) => classes::INPUT_IN_GROUP_MIDDLE,
            (true, false) => classes::INPUT_IN_GROUP_END,
            (false, true) => classes::INPUT_IN_GROUP_START,
            (false, false) => "",
        };
        let padding = match (has_prefix, has_suffix) {
            (true, true) => "pl-10 pr-10",
            (true, false) => "pl-10 pr-3.5",
            (false, true) => "pl-3.5 pr-10",
            (false, false) => "px-3.5",
        };
        classes!(base, rounded, padding, props.class.clone())
    } else if has_icons {
        let padding = match (has_prefix, has_suffix) {
            (true, true) => "pl-10 pr-10",
            (true, false) => "pl-10 pr-3.5",
            (false, true) => "pl-3.5 pr-10",
            (false, false) => "px-3.5",
        };
        classes!(classes::INPUT_WITH_ADDONS, padding, props.class.clone())
    } else {
        classes!(classes::BASE, props.class.clone())
    };

    let input_element = html! {
        <input
            data-slot="input"
            type={props.kind.clone()}
            class={input_class}
            value={props.value.clone()}
            oninput={ontext}
            onchange={onchange}
            onkeypress={props.onkeypress.clone()}
            onkeydown={props.onkeydown.clone()}
            onkeyup={props.onkeyup.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            placeholder={props.placeholder.clone()}
            id={props.id.clone()}
            name={props.name.clone()}
            maxlength={match props.maxlength {
                Some(v) => v.to_string(),
                None => "".to_string()
            }}
            readonly={props.readonly}
            min={props.min.clone()}
            max={props.max.clone()}
            step={props.step.clone()}
            ref={props.node_ref.clone()}
            disabled={props.disabled}
            required={props.required}
        />
    };

    // If we have prefix/suffix icons, wrap in a relative container
    let wrapped_input = if has_icons {
        html! {
            <div class={classes::INPUT_WRAPPER}>
                if let Some(prefix) = &props.prefix_icon {
                    <span class={classes!(classes::INPUT_ICON, classes::INPUT_ICON_LEFT)}>
                        { prefix.clone() }
                    </span>
                }
                { input_element }
                if let Some(suffix) = &props.suffix_icon {
                    <span class={classes!(classes::INPUT_ICON, classes::INPUT_ICON_RIGHT)}>
                        { suffix.clone() }
                    </span>
                }
            </div>
        }
    } else {
        input_element
    };

    // If we have start/end addons, wrap in a group
    if has_addons {
        html! {
            <div class={classes::INPUT_GROUP}>
                if let Some(start) = &props.start_addon {
                    <span class={classes::INPUT_GROUP_START}>
                        { start.clone() }
                    </span>
                }
                { wrapped_input }
                if let Some(end) = &props.end_addon {
                    <span class={classes::INPUT_GROUP_END}>
                        { end.clone() }
                    </span>
                }
            </div>
        }
    } else {
        wrapped_input
    }
}
