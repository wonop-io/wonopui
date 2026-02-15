//! Input component for WonopUI.
//!
//! A text input component with various configuration options.

use web_sys::HtmlInputElement;
use wonopui_core::*;

/// Default CSS classes for input styling.
pub mod classes {
    /// Base input styles.
    pub const BASE: &str = "rounded-md border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 w-full px-3.5 py-2.5 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out text-zinc-800 dark:text-zinc-100";
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

    html! {
        <input
            type={props.kind.clone()}
            class={classes!(classes::BASE, props.class.clone())}
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
    }
}
