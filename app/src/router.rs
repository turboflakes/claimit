use crate::app::App;
use crate::pages::page_not_found::PageNotFound;
use claimit_common::runtimes::support::SupportedRelayRuntime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeSet;
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

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Query {
    /// Specifies to which network [polkadot, kusama, paseo] the api will try to connect to
    #[serde(default)]
    pub chain: SupportedRelayRuntime,
    // Flag to allow light client connection to be used as default or not when launching the app
    #[serde(default = "default_light_client")]
    pub lc: bool,
    // Filter by Bounty IDs expected in a csv format
    #[serde(default = "BTreeSet::default")]
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    #[serde(serialize_with = "as_csv", deserialize_with = "from_csv")]
    pub bounties: BTreeSet<u32>,
}

fn default_light_client() -> bool {
    true
}

fn from_csv<'de, D>(deserializer: D) -> Result<BTreeSet<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let csv: Option<String> = Deserialize::deserialize(deserializer)?;
    let csv = csv.unwrap_or_default();
    let to_vec = csv
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    Ok(to_vec)
}

fn as_csv<S>(bounties: &BTreeSet<u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let csv_string = bounties.iter()
        .map(|bounty| bounty.to_string())
        .collect::<Vec<_>>()
        .join(",");

    serializer.serialize_str(&csv_string)
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
