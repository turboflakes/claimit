use claimeer_common::errors::ClaimeerError;
use claimeer_common::runtimes::utils::get_child_bounty_id_from_storage_key;
use claimeer_common::runtimes::utils::str;
use claimeer_common::types::{
    accounts::Balance,
    child_bounties::{ChildBounties, ChildBountiesIds, ChildBounty, ChildBountyId, Status},
    extensions::create_payload_as_string,
    worker::Output,
};
use claimeer_rococo_people::rococo_people::fetch_display_name;
use log::{error, info};
use node_runtime::{
    child_bounties::events::Claimed,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec, pallet_child_bounties::ChildBountyStatus,
    },
    system::events::ExtrinsicFailed,
    system::events::ExtrinsicSuccess,
    system::events::Remarked,
    utility::events::BatchCompleted,
    utility::events::BatchCompletedWithErrors,
};
use std::str::FromStr;
use subxt::{
    config::DefaultExtrinsicParamsBuilder as TxParams,
    ext::codec::Decode,
    tx::{SubmittableExtrinsic, TxStatus},
    utils::{AccountId32, MultiSignature},
    OnlineClient, PolkadotConfig,
};
use yew::platform::pinned::mpsc::UnboundedSender;

#[subxt::subxt(
    runtime_metadata_path = "artifacts/metadata/rococo_metadata.scale",
    derive_for_all_types = "PartialEq, Clone"
)]

mod node_runtime {}
type Call = node_runtime::runtime_types::rococo_runtime::RuntimeCall;
type ChildBountyCall = node_runtime::runtime_types::pallet_child_bounties::pallet::Call;
type SystemCall = node_runtime::runtime_types::frame_system::pallet::Call;

pub async fn fetch_child_bounties(
    api: &OnlineClient<PolkadotConfig>,
    people_api: &OnlineClient<PolkadotConfig>,
    tx: UnboundedSender<Output>,
) -> Result<(), ClaimeerError> {
    let mut out = ChildBounties::new();

    let address = node_runtime::storage()
        .child_bounties()
        .child_bounties_iter();

    let mut iter = api.storage().at_latest().await?.iter(address).await?;

    while let Some(Ok(storage)) = iter.next().await {
        match storage.value.status {
            ChildBountyStatus::PendingPayout {
                curator: _,
                beneficiary,
                unlock_at,
            } => {
                let id = get_child_bounty_id_from_storage_key(storage.key_bytes);

                // Fetch child bounty description
                let address = node_runtime::storage()
                    .child_bounties()
                    .child_bounty_descriptions(id);
                let description = if let Some(BoundedVec(data)) =
                    api.storage().at_latest().await?.fetch(&address).await?
                {
                    str(data)
                } else {
                    String::new()
                };

                // Fetch child bounty beneficiary identity
                let beneficiary_identity =
                    fetch_display_name(&people_api.clone(), &beneficiary.clone(), None).await?;

                let cb = ChildBounty {
                    id,
                    parent_id: storage.value.parent_bounty,
                    description,
                    value: storage.value.value,
                    status: Status::Pending,
                    beneficiary: beneficiary,
                    beneficiary_identity,
                    unlock_at,
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
    // Send whatever is left or empty
    let _ = tx.send_now(Output::ChildBounties(out));
    //
    return Ok(());
}

pub async fn fetch_account_balance(
    api: &OnlineClient<PolkadotConfig>,
    account: AccountId32,
) -> Result<Balance, ClaimeerError> {
    let address = node_runtime::storage().system().account(&account);

    if let Some(result) = api.storage().at_latest().await?.fetch(&address).await? {
        return Ok(Balance {
            free: result.data.free,
            reserved: result.data.reserved,
        });
    }

    return Err(ClaimeerError::Other(
        "An unexpected error occurred, balance couldn't be retrieved.".into(),
    ));
}

pub async fn create_payload_tx(
    api: &OnlineClient<PolkadotConfig>,
    child_bounties_ids: ChildBountiesIds,
    signer_address: String,
) -> Result<String, ClaimeerError> {
    let account_id = AccountId32::from_str(&signer_address).unwrap();
    let account_nonce = api.tx().account_nonce(&account_id).await?;

    // Create a batch call with the child bounty claims extrinsics
    let mut calls_for_batch: Vec<Call> = vec![];
    for (parent_bounty_id, child_bounty_id) in child_bounties_ids.into_iter() {
        let _call = Call::ChildBounties(ChildBountyCall::claim_child_bounty {
            parent_bounty_id,
            child_bounty_id,
        });
        let call = Call::System(SystemCall::remark_with_event {
            remark: b"test".to_vec(),
        });
        calls_for_batch.push(call);
    }

    // Create a batch call TX payload
    let batch_call = node_runtime::tx()
        .utility()
        .force_batch(calls_for_batch.clone());

    // Get SCALE encoded data from TX payload
    let Ok(call_data) = api.tx().call_data(&batch_call) else {
        return Err(ClaimeerError::Other("SCALE encoding failed".to_string()));
    };

    let Ok(payload) =
        create_payload_as_string(&api, &call_data, account_nonce, signer_address).await
    else {
        return Err(ClaimeerError::Other("Payload creation failed".to_string()));
    };

    Ok(payload)
}

pub async fn sign_and_submit_tx(
    api: &OnlineClient<PolkadotConfig>,
    child_bounties_ids: ChildBountiesIds,
    signer_address: String,
    signature: Vec<u8>,
) -> Result<Vec<ChildBountyId>, ClaimeerError> {
    let account_id = AccountId32::from_str(&signer_address).unwrap();
    let account_nonce = api.tx().account_nonce(&account_id).await?;

    // Create a batch call with the child bounty claims extrinsics
    let mut calls_for_batch: Vec<Call> = vec![];
    for (parent_bounty_id, child_bounty_id) in child_bounties_ids.into_iter() {
        let _call = Call::ChildBounties(ChildBountyCall::claim_child_bounty {
            parent_bounty_id,
            child_bounty_id,
        });
        // NOTE: To test on Rococo we create a remark rather than clearing the child_bounty!
        let call = Call::System(SystemCall::remark_with_event {
            remark: b"test".to_vec(),
        });

        calls_for_batch.push(call);
    }

    // Create a batch call TX payload
    let batch_call = node_runtime::tx()
        .utility()
        .force_batch(calls_for_batch.clone());

    let Ok(multi_signature) = MultiSignature::decode(&mut &signature[..]) else {
        return Err(ClaimeerError::Other(
            "MultiSignature decoding failed".to_string(),
        ));
    };

    let params = TxParams::new().nonce(account_nonce).build();

    let Ok(partial_signed) = api.tx().create_partial_signed_offline(&batch_call, params) else {
        return Err(ClaimeerError::Other(
            "PartialExtrinsic creation failed".to_string(),
        ));
    };

    // Apply the signature
    let signed_extrinsic =
        partial_signed.sign_with_address_and_signature(&account_id.into(), &multi_signature);

    // check the TX validity (to debug in the js console if the extrinsic would work)
    let dry_res = signed_extrinsic.validate().await;
    info!("__dry_res: {:?}", dry_res);

    // Submit and watch transaction
    submit_and_watch_tx(&api.clone(), signed_extrinsic.into_encoded()).await
}

pub async fn submit_and_watch_tx(
    api: &OnlineClient<PolkadotConfig>,
    tx_bytes: Vec<u8>,
) -> Result<Vec<ChildBountyId>, ClaimeerError> {
    let mut out = Vec::new();

    let extrinsic = SubmittableExtrinsic::from_bytes(api.clone(), tx_bytes);

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
                    } else if let Some(ev) = event.as_event::<Remarked>()? {
                        info!("Remarked: {}", ev.sender);
                        out.push(4);
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
                        return Err(ClaimeerError::Other(message.into()));
                    }
                }

                return Err(ClaimeerError::Other(
                    "An unexpected error occurred =/".into(),
                ));
            }
            TxStatus::Error { message } => {
                return Err(ClaimeerError::Other(format!("TxStatus: {message:?}")))
            }
            TxStatus::Invalid { message } => {
                return Err(ClaimeerError::Other(format!("TxStatus: {message:?}")))
            }
            TxStatus::Dropped { message } => {
                return Err(ClaimeerError::Other(format!("TxStatus: {message:?}")))
            }
            _ => {}
        }
    }
    Err(ClaimeerError::Other("TxStatus not available".into()))
}
