use crate::config::BRANDGUIDE;
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
}

#[function_component(Combobox)]
pub fn combobox(props: &ComboboxProps) -> Html {
    let open = use_state(|| false);
    let value = use_state(|| "".to_string());

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
        .unwrap_or_else(|| "Select option...".to_string());

    let toggle_open = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    html! {
        <div>
            <button
                id={props.id.clone()}
                class={classes!(
                    BRANDGUIDE.combobox_button,
                    if *open { BRANDGUIDE.combobox_button_open } else { "" },
                    if props.disabled { BRANDGUIDE.combobox_button_disabled } else { "" },
                )}
                role="combobox"
                aria-expanded={open.to_string()}
                onclick={toggle_open}
                disabled={props.disabled}
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
                        <div class={BRANDGUIDE.combobox_list}>
                            { for props.options.iter().map(|(val, label)| {
                                let on_select = on_select.clone();
                                let val = val.clone();
                                html! {
                                    <div
                                        class={classes!(
                                            BRANDGUIDE.combobox_item,
                                            if *value == val { BRANDGUIDE.combobox_item_selected } else { "" },
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
