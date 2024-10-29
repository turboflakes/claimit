use claimit_common::errors::ClaimitError;
use claimit_common::runtimes::support::SupportedRelayRuntime;
use claimit_common::types::{
    child_bounties::ChildBountiesIds,
    network::SubscriptionId,
    worker::{Input, Output, SignerAddress},
};
use claimit_kusama::kusama;
use claimit_kusama_people::kusama_people;
use claimit_polkadot::polkadot;
use claimit_polkadot_people::polkadot_people;
use claimit_paseo::paseo;
use claimit_paseo_people::paseo_people;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use log::{error, warn};
use subxt::{
    backend::chain_head::{ChainHeadBackend, ChainHeadBackendBuilder}, lightclient::LightClient, utils::AccountId32, OnlineClient,
    PolkadotConfig,
};
use yew::platform::{
    pinned::mpsc::{unbounded, UnboundedSender},
    spawn_local,
};
use yew_agent::{prelude::reactor, reactor::ReactorScope};

type Client = OnlineClient<PolkadotConfig>;
use Client as RelayClient;
use Client as PeopleClient;

#[reactor(Worker)]
pub async fn worker(mut scope: ReactorScope<Input, Output>) {
    'outer: while let Some(input) = scope.next().await {
        if let Input::Start(sub_id, runtime, use_light_client) = input {
            // Create API clients
            let (relay_api, people_api) = create_api_clients(runtime, use_light_client)
                .await
                .expect("expect valid API clients");

            // Create unbounded channel to facilitate communication between the reactor and all background tasks
            let (tx_inner_output, mut rx_inner_output) = unbounded::<Output>();

            // Subscribe to relay finalized block
            subscribe_finalized_block(&relay_api.clone(), sub_id, tx_inner_output.clone());

            // Inform caller the API is ready and active
            if scope.send(Output::Active(sub_id)).await.is_err() {
                // sender closed, the bridge is disconnected
                break;
            }

            loop {
                // Wait for Finish signal to break or continue
                futures::select! {
                    a = scope.next() => {
                        match a {
                            Some(Input::Finish) =>  {
                                warn!("Finish API worker");
                                break 'outer;
                            },
                            Some(Input::FetchChildBounties) => {
                                fetch_child_bounties(&relay_api.clone(), runtime.clone(), tx_inner_output.clone());
                            }
                            Some(Input::FetchAccountBalance(account_id)) => {
                                fetch_account_balance(&relay_api.clone(), account_id.clone(), runtime.clone(), tx_inner_output.clone());
                            }
                            Some(Input::FetchAccountIdentity(account_id)) => {
                                fetch_account_identity(&people_api.clone(), account_id.clone(), runtime.clone(), tx_inner_output.clone());
                            }
                            Some(Input::CreatePayloadTx(child_bounty_ids, signer_address)) => {
                                create_payload_tx(&relay_api.clone(), child_bounty_ids.clone(), signer_address.clone(), runtime.clone(), tx_inner_output.clone());
                            }
                            Some(Input::SignAndSubmitTx(child_bounty_ids, signer_address, signature)) => {
                                sign_and_submit_tx(&relay_api.clone(), child_bounty_ids.clone(), signer_address.clone(), signature.clone(), runtime.clone(), tx_inner_output.clone());
                            }
                            _ => ()
                        }
                    },
                    b = rx_inner_output.next() => {
                        match b {
                            Some(data) => {
                                if scope
                                    .send(data)
                                    .await
                                    .is_err()
                                {
                                    break 'outer;
                                }
                            }
                            _ => ()
                        }
                    },
                }
            }
        }
    }
}

/// Create API clients
pub async fn create_api_clients(
    runtime: SupportedRelayRuntime,
    use_light_client: bool,
) -> Result<(RelayClient, PeopleClient), ClaimitError> {
    if use_light_client {
        // Initiate light client (smoldot)
        let (lc, rpc) = LightClient::relay_chain(runtime.chain_specs())
            .expect("expect valid smoldot connection");

        // NOTE: The latest RPC specs are implemented via UnstableBackend in Subxt which is the preferred way to connect to smoldot v0.18
        // let (unstable_backend, mut driver) = ChainHeadBackend::builder().build(rpc);
        
        // // Unstable backend needs manually driving at the moment see here:
        // // https://github.com/paritytech/subxt/issues/1453#issuecomment-2011922808
        // spawn_local(async move {
        //     while let Some(val) = driver.next().await {
        //         if let Err(e) = val {
        //             // Something went wrong driving unstable backend.
        //             error!("error driving unstable backend: {:?}", e);
        //             break;
        //         }
        //     }
        // });

        // https://github.com/paritytech/subxt/blob/master/subxt/examples/setup_rpc_chainhead_backend.rs
        let backend: ChainHeadBackend<PolkadotConfig> =
            ChainHeadBackendBuilder::default().build_with_background_driver(rpc.clone());
        
        // Create client from unstable backend (ie using new RPCs).
        let relay_api = Client::from_backend(backend.into())
            .await
            .expect("expect valid RPC connection");

        // let relay_api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc.clone())
        //     .await
        //     .expect("expect valid RPC connection");

        let people_rpc = lc
            .parachain(runtime.chain_specs_people())
            .expect("expect valid smoldot connection");

        let people_api = Client::from_rpc_client(people_rpc)
            .await
            .expect("expect valid RPC connection");

        Ok((relay_api, people_api))
    } else {
        // Initiate RPC client from default RPCs provider
        let relay_api = Client::from_url(runtime.default_rpc_url())
            .await
            .expect("expect valid RPC connection");

        let people_api = Client::from_url(runtime.default_people_rpc_url())
            .await
            .expect("expect valid RPC connection");

        Ok((relay_api, people_api))
    }
}

/// Background task that subscribes finalized block and sends response over channel.
pub fn subscribe_finalized_block(
    api: &OnlineClient<PolkadotConfig>,
    sub_id: SubscriptionId,
    tx: UnboundedSender<Output>,
) {
    let api = api.clone();

    spawn_local(async move {
        match api.blocks().subscribe_finalized().await {
            Ok(mut blocks_sub) => {
                while let Some(result) = blocks_sub.next().await {
                    match result {
                        Ok(block) => {
                            let _ = tx.send_now(Output::BlockNumber(sub_id, block.number().into()));
                        }
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}

/// Background task that fetches child bounties and sends response over channel.
pub fn fetch_child_bounties(
    api: &OnlineClient<PolkadotConfig>,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<Output>,
) {
    let api = api.clone();
    let tx = tx.clone();
    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => polkadot::fetch_child_bounties(&api, tx).await,
            SupportedRelayRuntime::Kusama => kusama::fetch_child_bounties(&api, tx).await,
            SupportedRelayRuntime::Paseo => paseo::fetch_child_bounties(&api, tx).await,
        };
        match response {
            Err(e) => {
                error!("error: {:?}", e);
            }
            _ => (),
        }
    });
}

/// Background task that fetches account balance and sends response over channel.
pub fn fetch_account_balance(
    api: &OnlineClient<PolkadotConfig>,
    account_id: AccountId32,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<Output>,
) {
    let api = api.clone();
    let tx = tx.clone();
    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot::fetch_account_balance(&api, account_id.clone()).await
            }
            SupportedRelayRuntime::Kusama => {
                kusama::fetch_account_balance(&api, account_id.clone()).await
            }
            SupportedRelayRuntime::Paseo => {
                paseo::fetch_account_balance(&api, account_id.clone()).await
            }
        };
        match response {
            Ok(balance) => {
                let _ = tx.send_now(Output::AccountBalance(account_id, balance));
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}

/// Background task that fetches account identity and sends response over channel.
pub fn fetch_account_identity(
    api: &OnlineClient<PolkadotConfig>,
    account_id: AccountId32,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<Output>,
) {
    let api = api.clone();
    let tx = tx.clone();
    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot_people::fetch_display_name(&api, &account_id, None).await
            }
            SupportedRelayRuntime::Kusama => {
                kusama_people::fetch_display_name(&api, &account_id, None).await
            }
            SupportedRelayRuntime::Paseo => {
                paseo_people::fetch_display_name(&api, &account_id, None).await
            }
        };
        match response {
            Ok(identity) => {
                let _ = tx.send_now(Output::AccountIdentity(account_id, identity));
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}

/// Background task that creates a payload and sends response over channel.
pub fn create_payload_tx(
    api: &OnlineClient<PolkadotConfig>,
    child_bounties_ids: ChildBountiesIds,
    signer_address: SignerAddress,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<Output>,
) {
    let api = api.clone();
    let tx = tx.clone();

    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot::create_payload_tx(
                    &api,
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Kusama => {
                kusama::create_payload_tx(&api, child_bounties_ids.clone(), signer_address.clone())
                    .await
            }
            SupportedRelayRuntime::Paseo => {
                paseo::create_payload_tx(&api, child_bounties_ids.clone(), signer_address.clone())
                    .await
            }
        };
        match response {
            Ok(payload) => {
                let _ = tx.send_now(Output::TxPayload(payload));
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}

/// Background task that signs and submits transaction with the signature provided and sends response over channel.
pub fn sign_and_submit_tx(
    api: &OnlineClient<PolkadotConfig>,
    child_bounties_ids: ChildBountiesIds,
    signer_address: SignerAddress,
    signature: Vec<u8>,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<Output>,
) {
    let api = api.clone();
    let tx = tx.clone();

    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot::sign_and_submit_tx(
                    &api,
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                    signature.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Kusama => {
                kusama::sign_and_submit_tx(
                    &api,
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                    signature.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Paseo => {
                paseo::sign_and_submit_tx(
                    &api,
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                    signature.clone(),
                )
                .await
            }
        };
        match response {
            Ok(result) => {
                let _ = tx.send_now(Output::TxCompleted(result));
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}
