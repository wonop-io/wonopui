#[cfg(not(feature = "ThemeProvider"))]
use crate::config::get_brandguide;
#[cfg(feature = "ThemeProvider")]
use crate::config::use_brandguide;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub for_id: String,
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    #[cfg(feature = "ThemeProvider")]
    let brandguide = use_brandguide();
    #[cfg(not(feature = "ThemeProvider"))]
    let brandguide = get_brandguide();
    html! {
        <>
            <label
                class={classes!(&brandguide.label_base, props.class.clone())}
                for={props.for_id.clone()}
            >
            {
                if let Some(description) = &props.description {
                    html! {
                        <>
                        <div>
                            {props.children.clone()}
                        </div>
                        <div class="text-xs text-gray-500 mt-1">
                            {description}
                        </div>
                        </>
                    }
                } else {
                    html!{ {props.children.clone()} }
                }
            }

            </label>

        </>
    }
}

// label_base: "px-4 py-2"
// label_description: "text-sm text-gray-500 mt-1"
