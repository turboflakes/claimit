use claimeer_common::errors::ClaimeerError;
use claimeer_common::runtimes::utils::get_child_bounty_id_from_storage_key;
use claimeer_common::runtimes::utils::str;
use claimeer_common::types::child_bounties::{ChildBountiesKeys, ChildBountyId};
use claimeer_common::types::extensions::ExtensionAccount;
use claimeer_common::types::{
    child_bounties::{ChildBounties, ChildBounty, Status},
    extensions::extension_signature_for_extrinsic,
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
use std::str::FromStr;
use subxt::{
    config::DefaultExtrinsicParamsBuilder as TxParams,
    ext::codec::Decode,
    tx::SubmittableExtrinsic,
    tx::TxStatus,
    utils::{AccountId32, MultiSignature},
    OnlineClient, PolkadotConfig,
};

#[subxt::subxt(
    runtime_metadata_path = "artifacts/metadata/kusama_metadata.scale",
    derive_for_all_types = "PartialEq, Clone"
)]

mod node_runtime {}
type Call = node_runtime::runtime_types::staging_kusama_runtime::RuntimeCall;
type ChildBountyCall = node_runtime::runtime_types::pallet_child_bounties::pallet::Call;

pub async fn fetch_child_bounties(
    api: &OnlineClient<PolkadotConfig>,
) -> Result<ChildBounties, subxt::Error> {
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

                let cb = ChildBounty {
                    id,
                    parent_id: storage.value.parent_bounty,
                    description,
                    value: storage.value.value,
                    status: Status::Pending,
                    beneficiary: beneficiary.clone(),
                    unlock_at,
                };
                out.insert(id, cb);
            }
            _ => continue,
        }
    }
    return Ok(out);
}

pub async fn create_and_sign_tx(
    api: &OnlineClient<PolkadotConfig>,
    signer: ExtensionAccount,
    child_bounties_keys: ChildBountiesKeys,
) -> Result<Vec<u8>, ClaimeerError> {
    let account_address = signer.address.clone();
    let account_id = AccountId32::from_str(&account_address).unwrap();

    // Fetch account nounce
    let account_nonce = api.tx().account_nonce(&account_id).await?;

    // Create a batch call with the child bounty claims extrinsics
    let mut calls_for_batch: Vec<Call> = vec![];
    for (parent_bounty_id, child_bounty_id) in child_bounties_keys.into_iter() {
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
    let call_data = api.tx().call_data(&batch_call)?;

    let Ok(signature) = extension_signature_for_extrinsic(
        &call_data,
        &api,
        account_nonce,
        signer.source.clone(),
        signer.address.clone(),
    )
    .await
    else {
        return Err(ClaimeerError::Other(
            "Signing via extension failed".to_string(),
        ));
    };

    let Ok(multi_signature) = MultiSignature::decode(&mut &signature[..]) else {
        return Err(ClaimeerError::Other("MultiSignature Decoding".to_string()));
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
    info!("dry_res: {:?}", dry_res);

    Ok(signed_extrinsic.into_encoded())
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
