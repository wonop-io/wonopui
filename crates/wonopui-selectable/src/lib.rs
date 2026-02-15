//! Selectable component for WonopUI.
//!
//! A component that enables selection of child elements with visual indicators.

use std::rc::Rc;
pub use wonopui_core::merge_classes;
use yew::prelude::*;

/// CSS classes for the Selectable component
pub mod classes {
    pub const INDICATOR: &str = "border-2 border-blue-500 dark:border-blue-400 pointer-events-none";
    pub const SELECTABLE: &str = "cursor-pointer";
    pub const SELECTED: &str = "ring-2 ring-blue-500 dark:ring-blue-400";
}

/// The selected area dimensions
#[derive(Clone, PartialEq, Default)]
pub struct SelectedArea {
    pub top: u64,
    pub left: u64,
    pub width: u64,
    pub height: u64,
}

/// State for the selectable context
#[derive(Clone, PartialEq)]
pub struct SelectableState {
    pub selected_id: Option<String>,
    pub hover_id: Option<String>,
    pub onselect: Callback<Option<String>>,
    pub selected_area: Option<SelectedArea>,
    pub select_mode: bool,
}

impl Default for SelectableState {
    fn default() -> Self {
        Self {
            selected_id: None,
            hover_id: None,
            onselect: Callback::noop(),
            selected_area: None,
            select_mode: false,
        }
    }
}

pub enum SelectableAction {
    SetSelectedId(Option<String>),
    SetHoverId(Option<String>),
    SetSelectedArea(Option<SelectedArea>),
    SetSelectMode(bool),
}

impl Reducible for SelectableState {
    type Action = SelectableAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SelectableAction::SetSelectedId(id) => {
                self.onselect.emit(id.clone());
                Rc::new(SelectableState {
                    selected_id: id,
                    ..(*self).clone()
                })
            }
            SelectableAction::SetHoverId(id) => Rc::new(SelectableState {
                hover_id: id,
                ..(*self).clone()
            }),
            SelectableAction::SetSelectedArea(area) => Rc::new(SelectableState {
                selected_area: area,
                ..(*self).clone()
            }),
            SelectableAction::SetSelectMode(mode) => Rc::new(SelectableState {
                select_mode: mode,
                ..(*self).clone()
            }),
        }
    }
}

pub type SelectableContext = UseReducerHandle<SelectableState>;

#[derive(Properties, PartialEq)]
pub struct SelectableAreaProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onselect: Callback<Option<String>>,
    #[prop_or_default]
    pub select_mode: bool,
}

/// Container that provides selection context
#[function_component(SelectableArea)]
pub fn selectable_area(props: &SelectableAreaProps) -> Html {
    let select_mode = props.select_mode;
    let state = use_reducer(|| SelectableState {
        selected_id: None,
        hover_id: None,
        onselect: props.onselect.clone(),
        selected_area: None,
        select_mode,
    });

    {
        let state = state.clone();
        use_effect_with(select_mode, move |select_mode| {
            state.dispatch(SelectableAction::SetSelectMode(*select_mode));
            || {}
        });
    }

    html! {
        <ContextProvider<SelectableContext> context={state}>
            { for props.children.iter() }
        </ContextProvider<SelectableContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectableIndicatorProps {
    #[prop_or_default]
    pub class: Classes,
}

/// Visual indicator that shows the selected element's bounds
#[function_component(SelectableIndicator)]
pub fn selectable_indicator(props: &SelectableIndicatorProps) -> Html {
    let state = use_context::<SelectableContext>().expect("SelectableContext not found");

    match &state.selected_area {
        Some(area) => {
            let style = format!(
                "position: fixed; top: {}px; left: {}px; width: {}px; height: {}px; z-index: 1000; pointer-events: none;",
                area.top, area.left, area.width, area.height
            );
            let indicator_class = merge_classes(&[classes::INDICATOR, &props.class.to_string()]);
            html! { <div style={style} class={indicator_class} /> }
        }
        None => html! {},
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectableProps {
    pub id: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// An item that can be selected
#[function_component(Selectable)]
pub fn selectable(props: &SelectableProps) -> Html {
    let state = use_context::<SelectableContext>().expect("SelectableContext not found");
    let node_ref = use_node_ref();

    let is_selected = state.selected_id.as_ref() == Some(&props.id);

    let onclick = {
        let state = state.clone();
        let id = props.id.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if state.select_mode {
                state.dispatch(SelectableAction::SetSelectedId(Some(id.clone())));

                // Update selected area
                if let Some(element) = node_ref.cast::<web_sys::HtmlElement>() {
                    let rect = element.get_bounding_client_rect();
                    state.dispatch(SelectableAction::SetSelectedArea(Some(SelectedArea {
                        top: rect.top() as u64,
                        left: rect.left() as u64,
                        width: rect.width() as u64,
                        height: rect.height() as u64,
                    })));
                }
            }
        })
    };

    let item_class = merge_classes(&[
        classes::SELECTABLE,
        if is_selected { classes::SELECTED } else { "" },
        &props.class.to_string(),
    ]);

    html! {
        <div ref={node_ref} class={item_class} onclick={onclick} data-selectable-id={props.id.clone()}>
            { for props.children.iter() }
        </div>
    }
}
