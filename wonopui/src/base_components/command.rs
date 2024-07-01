use crate::config::BRANDGUIDE;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CommandProps {
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub options: Vec<(String, String, Option<Html>)>, // (value, label, icon)
    #[prop_or_default]
    pub on_select: Callback<String>,
}

#[function_component(Command)]
pub fn command(props: &CommandProps) -> Html {
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

    let toggle_open = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    html! {
        <div class={classes!(BRANDGUIDE.command_container)}>
            <div class={classes!(BRANDGUIDE.command_input_wrapper)}>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={classes!(BRANDGUIDE.command_icon)}>
                    <circle cx="11" cy="11" r="8"></circle>
                    <path d="m21 21-4.3-4.3"></path>
                </svg>
                <input
                    class={classes!(BRANDGUIDE.command_input)}
                    placeholder={props.placeholder.clone()}
                    autocomplete="off"
                    autocorrect="off"
                    spellcheck="false"
                    aria-autocomplete="list"
                    role="combobox"
                    aria-expanded={open.to_string()}
                    onclick={toggle_open}
                />
            </div>
            {
                if *open {
                    html! {
                        <div class={classes!(BRANDGUIDE.command_list)} role="listbox">
                            { for props.options.iter().map(|(val, label, icon)| {
                                let on_select = on_select.clone();
                                let val = val.clone();
                                html! {
                                    <div
                                        class={classes!(BRANDGUIDE.command_item)}
                                        onclick={Callback::from(move |_| on_select.emit(val.clone()))}
                                        role="option"
                                    >
                                        if let Some(icon) = icon {
                                            {icon.clone()}
                                        }

                                        <span>{ label }</span>
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
// command_container: "flex h-full w-full flex-col overflow-hidden bg-white text-black rounded-lg border shadow-md",
// command_input_wrapper: "flex items-center border-b px-3",
// command_icon: "mr-2 h-4 w-4 shrink-0 opacity-50",
// command_input: "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder-gray-500 disabled:cursor-not-allowed disabled:opacity-50",
// command_list: "max-h-[300px] overflow-y-auto overflow-x-hidden",
// command_item: "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-gray-200",
// command_item_icon: "mr-2 h-4 w-4"
