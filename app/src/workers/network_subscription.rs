use crate::providers::network::SubscriptionId;
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use std::time::Duration;

use futures::sink::SinkExt;
use futures::{FutureExt, StreamExt};
use log::error;
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, PolkadotConfig};
use yew::platform::time::sleep;
use yew_agent::{prelude::reactor, reactor::ReactorScope};

pub type BlockNumber = u32;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Input {
    Start(SubscriptionId, SupportedRelayRuntime),
    Finish,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Output {
    Active(SubscriptionId),
    BlockNumber(SubscriptionId, BlockNumber),
    Err(SubscriptionId),
}

#[reactor(BlockSubscription)]
pub async fn block_subscription(mut scope: ReactorScope<Input, Output>) {
    'outer: while let Some(input) = scope.next().await {
        if let Input::Start(sub_id, runtime) = input {
            let api = OnlineClient::<PolkadotConfig>::from_url(runtime.default_rpc_url())
                .await
                .expect("expect valid RPC connection");

            // Inform the reactor is active
            if scope.send(Output::Active(sub_id)).await.is_err() {
                // sender closed, the bridge is disconnected
                break;
            }

            match api.blocks().subscribe_finalized().await {
                Ok(mut blocks_sub) => {
                    while let Some(result) = blocks_sub.next().await {
                        match result {
                            Ok(block) => {
                                if scope
                                    .send(Output::BlockNumber(sub_id, block.number().into()))
                                    .await
                                    .is_err()
                                {
                                    // sender closed, the bridge is disconnected
                                    break 'outer;
                                }
                            }
                            Err(e) => error!("{}", e),
                        }

                        // Wait for Finish signal to break or continue
                        futures::select! {
                            m = scope.next() => {
                                if m == Some(Input::Finish) {
                                    break 'outer;
                                }
                            },
                            _ = sleep(Duration::from_millis(100)).fuse() => {},
                        }
                    }
                }
                Err(e) => {
                    error!("error: {:?}", e);
                    if scope.send(Output::Err(sub_id)).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}
