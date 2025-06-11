#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use crate::config::ClassesStr;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ComboboxProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub options: Vec<(String, String)>, // (value, label)
    #[prop_or_default]
    pub on_select: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub name: Option<Strin>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub tabindex: Option<i32>,
}

#[function_component(Combobox)]
pub fn combobox(props: &ComboboxProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    let open = use_state(|| false);
    let value = use_state(|| props.value.clone().unwrap_or_default());

    // Update internal value when prop changes
    use_effect_with(props.value.clone(), {
        let value = value.clone();
        move |prop_value: &Option<String>| {
            if let Some(val) = prop_value {
                value.set(val.clone());
            }
            || ()
        }
    });

    let on_select = {
        let value = value.clone();
        let open = open.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |selected_value: String| {
            value.set(selected_value.clone());
            open.set(false);
            on_select.emit(selected_value);
        })
    };

    let selected_label = props
        .options
        .iter()
        .find(|(val, _)| val == value.as_str())
        .map(|(_, label)| label.clone())
        .unwrap_or_else(|| {
            props
                .placeholder
                .clone()
                .unwrap_or_else(|| "Select option...".to_string())
        });

    let toggle_open = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    // Custom style for width if specified
    let custom_style = props.width.as_ref().map(|w| format!("width: {};", w));

    html! {
        <div class={classes!("relative", props.class.clone())}>
            <button
                id={props.id.clone()}
                name={props.name.clone()}
                class={classes!(
                    &brandguide.combobox_button,
                    if *open { brandguide.combobox_button_open.clone() } else { ClassesStr::empty() },
                    if props.disabled { brandguide.combobox_button_disabled.clone() } else { ClassesStr::empty() },
                )}
                role="combobox"
                aria-expanded={open.to_string()}
                aria-label={props.aria_label.clone()}
                onclick={toggle_open}
                disabled={props.disabled}
                required={props.required}
                autofocus={props.autofocus}
                tabindex={props.tabindex.map(|t| t.to_string())}
                style={custom_style}
            >
                { selected_label }
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-chevrons-up-down ml-2 h-4 w-4 shrink-0 opacity-50">
                    <path d="m7 15 5 5 5-5"></path>
                    <path d="m7 9 5-5 5 5"></path>
                </svg>
            </button>
            {
                if *open {
                    html! {
                        <div class={&brandguide.combobox_list} style={custom_style}>
                            { for props.options.iter().map(|(val, label)| {
                                let on_select = on_select.clone();
                                let val = val.clone();
                                html! {
                                    <div
                                        class={classes!(
                                            &brandguide.combobox_item,
                                            if *value == val { brandguide.combobox_item_selected.clone() } else { ClassesStr::empty() },
                                        )}
                                        onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                    >
                                        { label }
                                    </div>
                                }
                            }) }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

// New entries in the brand guide:
// combobox_button: "inline-flex items-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2 w-[200px] justify-between",
// combobox_button_open: "bg-accent text-accent-foreground",
// combobox_button_disabled: "disabled:pointer-events-none disabled:opacity-50",
// combobox_list: "absolute mt-1 w-[200px] bg-background border border-input rounded-md shadow-lg",
// combobox_item: "px-4 py-2 cursor-pointer hover:bg-accent hover:text-accent-foreground",
// combobox_item_selected: "bg-accent text-accent-foreground"
