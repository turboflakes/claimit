use claimeer_common::errors::ClaimeerError;
use claimeer_common::runtimes::utils::get_child_bounty_id_from_storage_key;
use claimeer_common::runtimes::utils::str;
use claimeer_common::types::child_bounties::ChildBountiesKeys;
use claimeer_common::types::extensions::ExtensionAccount;
use claimeer_common::types::{
    child_bounties::{ChildBounties, ChildBounty, Status},
    extensions::extension_signature_for_extrinsic,
};
use log::info;
use node_runtime::runtime_types::{
    bounded_collections::bounded_vec::BoundedVec, pallet_child_bounties::ChildBountyStatus,
};
use std::str::FromStr;
use subxt::{
    config::DefaultExtrinsicParamsBuilder as TxParams,
    ext::codec::Decode,
    tx::SubmittableExtrinsic,
    utils::{AccountId32, MultiSignature},
    OnlineClient, PolkadotConfig,
};

#[subxt::subxt(
    runtime_metadata_path = "artifacts/metadata/kusama_metadata_small.scale",
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
) -> Result<
    (
        MultiSignature,
        SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    ),
    ClaimeerError,
> {
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
    info!("___dry_res: {:?}", dry_res);
    // web_sys::console::log_1(&format!("Validation Result: {:?}", dry_res).into());

    // return the signature and signed extrinsic
    Ok((multi_signature, signed_extrinsic))
}
