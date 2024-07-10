use crate::runtimes::{kusama, polkadot, support::SupportedRelayRuntime};
use crate::types::child_bounties::ChildBounties;
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, PolkadotConfig};
use yew_agent::prelude::oneshot;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Query {
    FetchChildBounties(SupportedRelayRuntime),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    ChildBounties(ChildBounties),
}

#[oneshot(StorageQueries)]
pub async fn storage_queries(q: Query) -> Response {
    match q {
        Query::FetchChildBounties(runtime) => {
            let api = OnlineClient::<PolkadotConfig>::from_url(runtime.default_rpc_url())
                .await
                .expect("expect valid RPC connection");
            let cbs = match runtime {
                SupportedRelayRuntime::Kusama => kusama::fetch_child_bounties(&api.clone()).await,
                SupportedRelayRuntime::Polkadot => {
                    polkadot::fetch_child_bounties(&api.clone()).await
                }
            }
            .unwrap();
            Response::ChildBounties(cbs)
        }
    }
}
