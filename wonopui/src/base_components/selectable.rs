use yew::prelude::*;
use yew::virtual_dom::VNode;
use std::rc::Rc;
use yew::html::IntoPropValue;
use web_sys::HtmlElement;
use web_sys::{ResizeObserver,MutationObserver};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::virtual_dom::VTag;

#[derive(Clone, PartialEq)]
pub struct SelectedArea {
    pub top: u64,
    pub left: u64,
    pub width: u64,
    pub height: u64,    
}

#[derive(Clone, PartialEq)]
pub struct SelectableState {
    pub selected_id: Option<String>,
    pub hover_id: Option<String>,
    pub onselect: Callback<Option<String>>,
    pub selected_area: Option<SelectedArea>,
    pub select_mode: bool,
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
            },
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

#[derive(Properties, PartialEq)]
pub struct SelectableProps {
    pub id: String,
    pub children: VNode,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("div".to_string())]
    pub tag: String,
    #[prop_or(false)]
    pub select_mode: bool,
    #[prop_or_default]
    pub style: String,
}

#[derive(Properties, PartialEq)]
pub struct SelectableIndicatorProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SelectableIndicator)]
pub fn selectable_indicator(props: &SelectableIndicatorProps) -> Html {
    let state = use_context::<UseReducerHandle<SelectableState>>().expect("SelectableContext not found");

    match &state.selected_area {
        Some(area) => {
            let style = format!(
                "position: fixed; top: {}px; left: {}px; width: {}px; height: {}px; z-index: 1000; pointer-events: none;",
                area.top, area.left, area.width, area.height
            );
            html! { <div style={style} class={classes!("outline","outline-2","outline-zinc-400", "outline-dashed", props.class.clone())} /> }
        }
        None => {
            html! {}
        }
    }
}


#[derive(Properties, PartialEq)]
pub struct SelectableAreaProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onselect: Callback<Option<String>>,
    #[prop_or_default]
    pub select_mode: bool,
}

#[function_component(SelectableArea)]
pub fn selectable_area(props: &SelectableAreaProps) -> Html {
    let select_mode = props.select_mode;
    let state = use_reducer(|| SelectableState {
        selected_id: None,
        hover_id: None,
        onselect: props.onselect.clone(),
        selected_area: None,
        select_mode
    });

    {
        let state = state.clone();
        use_effect_with(
            (select_mode,),
            move |(select_mode,)| {
                state.dispatch(SelectableAction::SetSelectMode(*select_mode));
                || {}
            },
        );
    }

    html! {
        <ContextProvider<UseReducerHandle<SelectableState>> context={state}>
            { for props.children.iter() }
        </ContextProvider<UseReducerHandle<SelectableState>>>
    }
}

fn get_bounding_client_rect(node: &HtmlElement) -> SelectedArea {
    let rect = node.get_bounding_client_rect();

    let top = rect.top() as u64;
    let left = rect.left() as u64;
    let width = rect.width() as u64;
    let height = rect.height() as u64;

    SelectedArea {
        top,
        left,
        width,
        height,
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectableVTagProps {
    pub node: Box<VTag>,
    pub node_ref: NodeRef,
    pub onclick: Callback<MouseEvent>,
    pub id: String,
    pub onmouseenter: Callback<MouseEvent>,
    pub onmouseleave: Callback<MouseEvent>,
}

#[function_component(SelectableVTag)]
pub fn selectable_vtag(props: &SelectableVTagProps) -> Html {
    let state = use_context::<UseReducerHandle<SelectableState>>().expect("SelectableContext not found");    
    let mut node = props.node.clone();
    let node_ref = props.node_ref.clone();
    let onclick = props.onclick.clone();
    let onmouseenter = props.onmouseenter.clone();
    let onmouseleave = props.onmouseleave.clone();

    {
        let state = state.clone();
        let id = props.id.clone();
        let node_ref = node_ref.clone();
        use_effect_with(
            (node_ref.clone(), node.clone(), id.clone()),
            move |(node_ref,node,id)| {
                if Some(id) == state.selected_id.as_ref() {
                    if let Some(element) = node_ref.cast::<HtmlElement>() {
                        let area = get_bounding_client_rect(&element);
                        state.dispatch(SelectableAction::SetSelectedArea(Some(area)));
                    }                
                }
                move || {}
            }
        )
    }

    node.add_listener(yew::html::onclick::Wrapper::__macro_new(onclick.clone()).unwrap());
    node.add_listener(yew::html::onmouseenter::Wrapper::__macro_new(onmouseenter.clone()).unwrap());
    node.add_listener(yew::html::onmouseleave::Wrapper::__macro_new(onmouseleave.clone()).unwrap());
    node.node_ref = node_ref;

    VNode::VTag(node)
}

#[function_component(Selectable)]
pub fn selectable(props: &SelectableProps) -> Html {
    let state = use_context::<UseReducerHandle<SelectableState>>().expect("SelectableContext not found");
    if !state.select_mode {
        return html! {
            <@{props.tag.clone()} class={props.class.clone()} style={props.style.clone()} key="selectable">
            { props.children.clone() }
            </@>
        }
    }
    let selected = state.selected_id == Some(props.id.clone());
    let hovered = state.hover_id == Some(props.id.clone());
    let node_ref = use_node_ref();

    let onclick = {
        let state = state.clone();
        let id = props.id.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            if selected {
                state.dispatch(SelectableAction::SetSelectedId(None));
                state.dispatch(SelectableAction::SetSelectedArea(None));
            } else {
                state.dispatch(SelectableAction::SetSelectedId(Some(id.clone())));
                if let Some(element) = node_ref.cast::<HtmlElement>() {
                    let area = get_bounding_client_rect(&element);
                    state.dispatch(SelectableAction::SetSelectedArea(Some(area)));
                }
            }
        })
    };

    let onmouseenter = {
        let state = state.clone();
        let id = props.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            state.dispatch(SelectableAction::SetHoverId(Some(id.clone())));
        })
    };
    
    let onmouseleave = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            if hovered {
                state.dispatch(SelectableAction::SetHoverId(None));
            }
        })
    };


    {
        let state = state.clone();
        let id = props.id.clone();
        let node_ref = node_ref.clone();

        use_effect_with(
            (node_ref.clone(),),
            move |(node_ref,)| {
                let node_ref = node_ref.clone();
                let node = node_ref.cast::<HtmlElement>();
                let rs_obs =if let Some(element) = node {
                    let state_clone = state.clone();
                    let cb = {
                        move || {
                            if let Some(element) = node_ref.cast::<HtmlElement>() {
                                let area = get_bounding_client_rect(&element);
                                state_clone.dispatch(SelectableAction::SetSelectedArea(Some(area)));
                            }
                        }
                    };                    
                    let callback = Closure::<dyn Fn()>::new(cb);
                    let resize_observer = MutationObserver::new(callback.as_ref().unchecked_ref()).unwrap();
                    resize_observer.observe(&element);
                    callback.forget();
                    Some(resize_observer)
                } else {
                    None
                };
                move || {
                    match rs_obs {
                        Some(rs) => {
                            rs.disconnect();
                        },
                        None => (),
                    }
                }
            },
        );
    }
    
    let mut classes = props.class.clone();
    classes.push("hover:outline");
    classes.push("hover:outline-dashed");
    classes.push("hover:outline-2");
    classes.push("hover:outline-blue-500");    
    if selected {
        classes.push("outline");
        classes.push("outline-2");
        classes.push("outline-blue-500");
    }
    classes.push("cursor-pointer");

    html! {
        <@{props.tag.clone()} class={classes} style={props.style.clone()} {onclick} {onmouseenter} {onmouseleave} ref={node_ref}  key="selectable">
        { props.children.clone() }
        </@>
    }
}
 
/*
#[hook]
pub fn use_selectable_context() -> UseReducerHandle<SelectableState> {
    use_context::<UseReducerHandle<SelectableState>>().expect("SelectableContext not found")
}
*/