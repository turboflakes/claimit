use crate::app::App;
use crate::pages::page_not_found::PageNotFound;
use crate::runtimes::support::SupportedRelayRuntime;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Routes {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub chain: SupportedRelayRuntime,
}

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Index => {
            html! { <App /> }
        }
        Routes::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
