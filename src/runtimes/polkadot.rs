use crate::runtimes::utils::get_child_bounty_id_from_storage_key;
use crate::types::child_bounties::{ChildBounties, ChildBounty, Status};
use node_runtime::runtime_types::pallet_child_bounties::ChildBountyStatus;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(
    runtime_metadata_path = "artifacts/metadata/polkadot_metadata_small.scale",
    derive_for_all_types = "PartialEq, Clone"
)]
pub mod node_runtime {}

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
                let cb = ChildBounty {
                    id,
                    parent_bounty: storage.value.parent_bounty,
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
