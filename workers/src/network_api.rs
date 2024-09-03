use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::Balance,
    child_bounties::{ChildBounties, ChildBountiesIds, ChildBountyId},
    network::SubscriptionId,
};
use claimeer_kusama::kusama;
use claimeer_kusama_people::kusama_people;
use claimeer_polkadot::polkadot;
use claimeer_polkadot_people::polkadot_people;
use claimeer_rococo::rococo;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use log::error;
use log::info;
use serde::{Deserialize, Serialize};
use subxt::{
    backend::unstable::UnstableBackend, lightclient::LightClient, utils::AccountId32, OnlineClient,
    PolkadotConfig,
};
use yew::platform::{
    pinned::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    spawn_local,
};
use yew_agent::{prelude::reactor, reactor::ReactorScope};

pub type BlockNumber = u32;
///  SignerAddress must be ss58 formatted address as string
pub type SignerAddress = String;
/// UseLightClient instructs worker to start a light client connection to the network
pub type UseLightClient = bool;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Input {
    Start(SubscriptionId, SupportedRelayRuntime, UseLightClient),
    FetchChildBounties,
    FetchAccountBalance(AccountId32),
    FetchAccountIdentity(AccountId32),
    CreatePayloadTx(ChildBountiesIds, SignerAddress),
    SignAndSubmitTx(ChildBountiesIds, SignerAddress, Vec<u8>),
    Finish,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Output {
    Active(SubscriptionId),
    BlockNumber(SubscriptionId, BlockNumber),
    ChildBounties(ChildBounties),
    AccountBalance(AccountId32, Balance),
    AccountIdentity(AccountId32, Option<String>),
    TxPayload(String),
    TxCompleted(Vec<u32>),
    Err(SubscriptionId),
}

#[reactor(Worker)]
pub async fn worker(mut scope: ReactorScope<Input, Output>) {
    'outer: while let Some(input) = scope.next().await {
        if let Input::Start(sub_id, runtime, use_light_client) = input {
            // let (relay_api, people_api) = create_api_clients(use_light_client).await;
            let (relay_api, people_api) = if use_light_client {
                // Initiate light client (smoldot)
                let (lc, rpc) = LightClient::relay_chain(runtime.chain_specs())
                    .expect("expect valid smoldot connection");

                // NOTE: The latest RPC specs are implemented via UnstableBackend in Subxt which is the preferred way to connect to smoldot v0.18
                let (unstable_backend, mut driver) = UnstableBackend::builder().build(rpc);

                // Unstable backend needs manually driving at the moment see here:
                // https://github.com/paritytech/subxt/issues/1453#issuecomment-2011922808
                spawn_local(async move {
                    while let Some(val) = driver.next().await {
                        if let Err(e) = val {
                            // Something went wrong driving unstable backend.
                            error!("error driving unstable backend: {:?}", e);
                            break;
                        }
                    }
                });

                // Create client from unstable backend (ie using new RPCs).
                let relay_api =
                    OnlineClient::<PolkadotConfig>::from_backend(unstable_backend.into())
                        .await
                        .expect("expect valid RPC connection");

                // OnlineClient::<PolkadotConfig>::from_rpc_client(rpc.clone())
                //     .await
                //     .expect("expect valid RPC connection")

                let people_rpc = lc
                    .parachain(runtime.chain_specs_people())
                    .expect("expect valid smoldot connection");

                let people_api = OnlineClient::<PolkadotConfig>::from_rpc_client(people_rpc)
                    .await
                    .expect("expect valid RPC connection");

                (relay_api, people_api)
            } else {
                // Initiate RPC client from default RPCs provider
                let relay_api = OnlineClient::<PolkadotConfig>::from_url(runtime.default_rpc_url())
                    .await
                    .expect("expect valid RPC connection");

                let people_api =
                    OnlineClient::<PolkadotConfig>::from_url(runtime.default_people_rpc_url())
                        .await
                        .expect("expect valid RPC connection");

                (relay_api, people_api)
            };

            //
            let mut rx_subscription = subscribe_finalized_block(&relay_api.clone());

            //
            let (tx_child_bounties, mut rx_child_bounties) = unbounded::<ChildBounties>();
            let (tx_account_balance, mut rx_account_balance) =
                unbounded::<(AccountId32, Balance)>();
            let (tx_account_identity, mut rx_account_identity) =
                unbounded::<(AccountId32, Option<String>)>();
            let (tx_create_payload, mut rx_create_payload) = unbounded::<String>();
            let (tx_sign_and_submit, mut rx_sign_and_submit) = unbounded::<Vec<ChildBountyId>>();

            // Inform the reactor is active
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
                                break 'outer;
                            },
                            Some(Input::FetchChildBounties) => {
                                fetch_child_bounties(&relay_api.clone(), runtime.clone(), tx_child_bounties.clone());
                            }
                            Some(Input::FetchAccountBalance(account_id)) => {
                                fetch_account_balance(&relay_api.clone(), account_id.clone(), runtime.clone(), tx_account_balance.clone());
                            }
                            Some(Input::FetchAccountIdentity(account_id)) => {
                                fetch_account_identity(&people_api.clone(), account_id.clone(), runtime.clone(), tx_account_identity.clone());
                            }
                            Some(Input::CreatePayloadTx(child_bounty_ids, signer_address)) => {
                                create_payload_tx(&relay_api.clone(), child_bounty_ids.clone(), signer_address.clone(), runtime.clone(), tx_create_payload.clone());
                            }
                            Some(Input::SignAndSubmitTx(child_bounty_ids, signer_address, signature)) => {
                                sign_and_submit_tx(&relay_api.clone(), child_bounty_ids.clone(), signer_address.clone(), signature.clone(), runtime.clone(), tx_sign_and_submit.clone());
                            }
                            _ => ()
                        }
                    },
                    b = rx_subscription.next() => {
                        if let Some(block_number) = b {
                            if scope
                                .send(Output::BlockNumber(sub_id, block_number))
                                .await
                                .is_err()
                                {
                                    break 'outer;
                                }
                        }
                    },
                    c = rx_child_bounties.next() => {
                        if let Some(child_bounties) = c {
                            if scope
                                .send(Output::ChildBounties(child_bounties))
                                .await
                                .is_err()
                            {
                                break 'outer;
                            }
                        }
                    },
                    d = rx_account_balance.next() => {
                        if let Some((account_id, balance)) = d {
                            if scope
                                .send(Output::AccountBalance(account_id, balance))
                                .await
                                .is_err()
                            {
                                break 'outer;
                            }
                        }
                    },
                    e = rx_create_payload.next() => {
                        if let Some(payload) = e {
                            if scope
                                .send(Output::TxPayload(payload))
                                .await
                                .is_err()
                            {
                                break 'outer;
                            }
                        }
                    },
                    f = rx_sign_and_submit.next() => {
                        if let Some(child_bounties_claimed) = f {
                            if scope
                                .send(Output::TxCompleted(child_bounties_claimed))
                                .await
                                .is_err()
                            {
                                break 'outer;
                            }
                        }
                    },
                    g = rx_account_identity.next() => {
                        if let Some((account_id, identity)) = g {
                            if scope
                                .send(Output::AccountIdentity(account_id, identity))
                                .await
                                .is_err()
                            {
                                break 'outer;
                            }
                        }
                    },
                }
            }
        }
    }
}

/// Background task that subscribes finalized block and sends response over channel.
pub fn subscribe_finalized_block(api: &OnlineClient<PolkadotConfig>) -> UnboundedReceiver<u32> {
    let (tx, rx) = unbounded::<u32>();
    let api = api.clone();

    spawn_local(async move {
        match api.blocks().subscribe_finalized().await {
            Ok(mut blocks_sub) => {
                while let Some(result) = blocks_sub.next().await {
                    match result {
                        Ok(block) => {
                            let _ = tx.send_now(block.number().into());
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

    rx
}

/// Background task that fetches child bounties and sends response over channel.
pub fn fetch_child_bounties(
    api: &OnlineClient<PolkadotConfig>,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<ChildBounties>,
) {
    let api = api.clone();
    let tx = tx.clone();
    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => polkadot::fetch_child_bounties(&api.clone()).await,
            SupportedRelayRuntime::Kusama => kusama::fetch_child_bounties(&api.clone()).await,
            SupportedRelayRuntime::Rococo => rococo::fetch_child_bounties(&api.clone()).await,
        };
        match response {
            Ok(child_bounties) => {
                let _ = tx.send_now(child_bounties);
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}

/// Background task that fetches account balance and sends response over channel.
pub fn fetch_account_balance(
    api: &OnlineClient<PolkadotConfig>,
    account_id: AccountId32,
    runtime: SupportedRelayRuntime,
    tx: UnboundedSender<(AccountId32, Balance)>,
) {
    let api = api.clone();
    let tx = tx.clone();
    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot::fetch_account_balance(&api.clone(), account_id.clone()).await
            }
            SupportedRelayRuntime::Kusama => {
                kusama::fetch_account_balance(&api.clone(), account_id.clone()).await
            }
            SupportedRelayRuntime::Rococo => {
                rococo::fetch_account_balance(&api.clone(), account_id.clone()).await
            }
        };
        match response {
            Ok(balance) => {
                let _ = tx.send_now((account_id, balance));
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
    tx: UnboundedSender<(AccountId32, Option<String>)>,
) {
    let api = api.clone();
    let tx = tx.clone();
    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot_people::fetch_display_name(&api.clone(), &account_id, None).await
            }
            SupportedRelayRuntime::Kusama => {
                kusama_people::fetch_display_name(&api.clone(), &account_id, None).await
            }
            // SupportedRelayRuntime::Rococo => {
            //     rococo_people::fetch_display_name(&api.clone(), &account_id, None).await
            // }
            _ => todo!(),
        };
        match response {
            Ok(identity) => {
                let _ = tx.send_now((account_id, identity));
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
    tx: UnboundedSender<String>,
) {
    let api = api.clone();
    let tx = tx.clone();

    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot::create_payload_tx(
                    &api.clone(),
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Kusama => {
                kusama::create_payload_tx(
                    &api.clone(),
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Rococo => {
                rococo::create_payload_tx(
                    &api.clone(),
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                )
                .await
            }
        };
        match response {
            Ok(payload) => {
                let _ = tx.send_now(payload);
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
    tx: UnboundedSender<Vec<ChildBountyId>>,
) {
    let api = api.clone();
    let tx = tx.clone();

    spawn_local(async move {
        let response = match runtime {
            SupportedRelayRuntime::Polkadot => {
                polkadot::sign_and_submit_tx(
                    &api.clone(),
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                    signature.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Kusama => {
                kusama::sign_and_submit_tx(
                    &api.clone(),
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                    signature.clone(),
                )
                .await
            }
            SupportedRelayRuntime::Rococo => {
                rococo::sign_and_submit_tx(
                    &api.clone(),
                    child_bounties_ids.clone(),
                    signer_address.clone(),
                    signature.clone(),
                )
                .await
            }
        };
        match response {
            Ok(payload) => {
                let _ = tx.send_now(payload);
            }
            Err(e) => {
                error!("error: {:?}", e);
            }
        }
    });
}
