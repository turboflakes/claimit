use claimit_common::errors::ClaimitError;
use claimit_common::runtimes::utils::get_child_bounty_id_from_storage_key;
use claimit_common::runtimes::utils::str;
use claimit_common::types::{
    accounts::Balance,
    child_bounties::{ChildBounties, ChildBountiesIds, ChildBounty, ChildBountyId, Status},
    extensions::create_payload_as_string,
    worker::Output,
};
use log::{error, info};
use node_runtime::{
    child_bounties::events::Claimed,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec, pallet_child_bounties::ChildBountyStatus,
    },
    system::events::ExtrinsicFailed,
    system::events::ExtrinsicSuccess,
    utility::events::BatchCompleted,
    utility::events::BatchCompletedWithErrors,
};
use std::collections::BTreeMap;
use std::str::FromStr;
use subxt::{
    config::DefaultExtrinsicParamsBuilder as TxParams,
    ext::codec::Decode,
    tx::{SubmittableTransaction, TxStatus},
    utils::{AccountId32, MultiSignature},
    OnlineClient, PolkadotConfig,
};
use yew::platform::pinned::mpsc::UnboundedSender;

#[subxt::subxt(
    runtime_metadata_path = "artifacts/metadata/polkadot_metadata_small.scale",
    derive_for_all_types = "PartialEq, Clone"
)]
mod node_runtime {}
type Call = node_runtime::runtime_types::polkadot_runtime::RuntimeCall;
type ChildBountyCall = node_runtime::runtime_types::pallet_child_bounties::pallet::Call;

pub async fn fetch_child_bounties(
    api: &OnlineClient<PolkadotConfig>,
    tx: UnboundedSender<Output>,
) -> Result<(), ClaimitError> {
    let mut out = ChildBounties::new();
    let mut temp = BTreeMap::new();

    // Fetch child bounties
    let address = node_runtime::storage()
        .child_bounties()
        .child_bounties_iter();

    let mut iter = api.storage().at_latest().await?.iter(address).await?;

    while let Some(Ok(storage)) = iter.next().await {
        match storage.value.status {
            ChildBountyStatus::PendingPayout { .. } => {
                temp.insert(
                    get_child_bounty_id_from_storage_key(storage.key_bytes),
                    storage.value,
                );
            }
            _ => continue,
        }
    }

    // Fetch all child bounties descriptions
    let address = node_runtime::storage()
        .child_bounties()
        .child_bounty_descriptions_v1_iter();

    let mut iter = api.storage().at_latest().await?.iter(address).await?;

    while let Some(Ok(storage)) = iter.next().await {
        let id = get_child_bounty_id_from_storage_key(storage.key_bytes);

        if let Some(cb_storage) = temp.get(&id) {
            match &cb_storage.status {
                ChildBountyStatus::PendingPayout {
                    curator: _,
                    beneficiary,
                    unlock_at,
                } => {
                    let BoundedVec(description) = storage.value;

                    let cb = ChildBounty {
                        id,
                        parent_id: cb_storage.parent_bounty,
                        description: str(description),
                        value: cb_storage.value,
                        status: Status::Pending,
                        beneficiary: beneficiary.clone(),
                        beneficiary_identity: None,
                        unlock_at: *unlock_at,
                    };
                    out.insert(id, cb);
                    if out.len() % 2 == 0 {
                        let _ = tx.send_now(Output::ChildBounties(out));
                        out = ChildBounties::new();
                    }
                }
                _ => continue,
            }
        }
    }

    // Send whatever is left or empty
    let _ = tx.send_now(Output::ChildBounties(out));
    //
    return Ok(());
}

pub async fn fetch_account_balance(
    api: &OnlineClient<PolkadotConfig>,
    account: AccountId32,
) -> Result<Balance, ClaimitError> {
    let address = node_runtime::storage().system().account(account);

    if let Some(result) = api.storage().at_latest().await?.fetch(&address).await? {
        return Ok(Balance {
            free: result.data.free,
            reserved: result.data.reserved,
        });
    }

    return Err(ClaimitError::Other(
        "An unexpected error occurred, balance couldn't be retrieved.".into(),
    ));
}

pub async fn create_payload_tx(
    api: &OnlineClient<PolkadotConfig>,
    child_bounties_ids: ChildBountiesIds,
    signer_address: String,
) -> Result<String, ClaimitError> {
    let account_id = AccountId32::from_str(&signer_address).unwrap();
    let account_nonce = api.tx().account_nonce(&account_id).await?;

    // Create a batch call with the child bounty claims extrinsics
    let mut calls_for_batch: Vec<Call> = vec![];
    for (parent_bounty_id, child_bounty_id) in child_bounties_ids.into_iter() {
        let call = Call::ChildBounties(ChildBountyCall::claim_child_bounty {
            parent_bounty_id,
            child_bounty_id,
        });
        calls_for_batch.push(call);
    }

    // Create a batch call TX payload
    let batch_call = node_runtime::tx()
        .utility()
        .force_batch(calls_for_batch.clone());

    // Get SCALE encoded data from TX payload
    let Ok(call_data) = api.tx().call_data(&batch_call) else {
        return Err(ClaimitError::Other("SCALE encoding failed".to_string()));
    };

    let Ok(payload) =
        create_payload_as_string(&api, &call_data, account_nonce, signer_address).await
    else {
        return Err(ClaimitError::Other("Payload creation failed".to_string()));
    };

    Ok(payload)
}

pub async fn sign_and_submit_tx(
    api: &OnlineClient<PolkadotConfig>,
    child_bounties_ids: ChildBountiesIds,
    signer_address: String,
    signature: Vec<u8>,
) -> Result<Vec<ChildBountyId>, ClaimitError> {
    let account_id = AccountId32::from_str(&signer_address).unwrap();
    let account_nonce = api.tx().account_nonce(&account_id).await?;

    // Create a batch call with the child bounty claims extrinsics
    let mut calls_for_batch: Vec<Call> = vec![];
    for (parent_bounty_id, child_bounty_id) in child_bounties_ids.into_iter() {
        let call = Call::ChildBounties(ChildBountyCall::claim_child_bounty {
            parent_bounty_id,
            child_bounty_id,
        });
        calls_for_batch.push(call);
    }

    // Create a batch call TX payload
    let batch_call = node_runtime::tx()
        .utility()
        .force_batch(calls_for_batch.clone());

    let Ok(multi_signature) = MultiSignature::decode(&mut &signature[..]) else {
        return Err(ClaimitError::Other(
            "MultiSignature decoding failed".to_string(),
        ));
    };

    let params = TxParams::new().nonce(account_nonce).build();

    let Ok(mut partial_signed) = api.tx().create_partial_offline(&batch_call, params) else {
        return Err(ClaimitError::Other(
            "PartialExtrinsic creation failed".to_string(),
        ));
    };

    // Apply the signature
    let signed_extrinsic =
        partial_signed.sign_with_account_and_signature(&account_id.into(), &multi_signature);

    // check the TX validity (to debug in the js console if the extrinsic would work)
    let dry_res = signed_extrinsic.validate().await;
    info!("__dry_res: {:?}", dry_res);

    // Submit and watch transaction
    submit_and_watch_tx(&api.clone(), signed_extrinsic.into_encoded()).await
}

pub async fn submit_and_watch_tx(
    api: &OnlineClient<PolkadotConfig>,
    tx_bytes: Vec<u8>,
) -> Result<Vec<ChildBountyId>, ClaimitError> {
    let mut out = Vec::new();

    let extrinsic = SubmittableTransaction::from_bytes(api.clone(), tx_bytes);

    let mut tx_progress = extrinsic.submit_and_watch().await?;

    while let Some(status) = tx_progress.next().await {
        match status? {
            TxStatus::InFinalizedBlock(in_block) => {
                // Get block number
                let block_number = if let Some(header) =
                    api.backend().block_header(in_block.block_hash()).await?
                {
                    header.number
                } else {
                    0
                };

                // Fetch events from block
                let tx_events = in_block.fetch_events().await?;

                // Iterate over events to retrieve child bounties claimed
                for event in tx_events.iter() {
                    let event = event?;
                    if let Some(ev) = event.as_event::<Claimed>()? {
                        out.push(ev.child_index);
                    } else if let Some(_ev) = event.as_event::<BatchCompleted>()? {
                        info!(
                            "Batch fully completed at block {} extrinsic {:?}",
                            block_number,
                            tx_events.extrinsic_hash()
                        );
                    } else if let Some(_ev) = event.as_event::<BatchCompletedWithErrors>()? {
                        info!(
                            "Batch completed with errors at block {} extrinsic {:?}",
                            block_number,
                            tx_events.extrinsic_hash()
                        );
                    } else if let Some(_ev) = event.as_event::<ExtrinsicSuccess>()? {
                        return Ok(out);
                    } else if let Some(_ev) = event.as_event::<ExtrinsicFailed>()? {
                        let message = format!(
                            "ExtrinsicFailed at block {} extrinsic {:?}",
                            block_number,
                            tx_events.extrinsic_hash()
                        );
                        error!("{message}");
                        return Err(ClaimitError::Other(message.into()));
                    }
                }

                return Err(ClaimitError::Other(
                    "An unexpected error occurred =/".into(),
                ));
            }
            TxStatus::Error { message } => {
                return Err(ClaimitError::Other(format!("TxStatus: {message:?}")))
            }
            TxStatus::Invalid { message } => {
                return Err(ClaimitError::Other(format!("TxStatus: {message:?}")))
            }
            TxStatus::Dropped { message } => {
                return Err(ClaimitError::Other(format!("TxStatus: {message:?}")))
            }
            _ => {}
        }
    }
    Err(ClaimitError::Other("TxStatus not available".into()))
}
