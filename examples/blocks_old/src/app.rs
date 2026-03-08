use crate::router::Router;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Router />
        </BrowserRouter>
    }
}
