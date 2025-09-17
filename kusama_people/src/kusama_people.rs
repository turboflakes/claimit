use async_recursion::async_recursion;
use claimit_common::errors::ClaimitError;
use claimit_common::runtimes::utils::str;
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};

#[subxt::subxt(
    runtime_metadata_path = "artifacts/metadata/kusama_people_metadata_small.scale",
    derive_for_all_types = "PartialEq, Clone"
)]
mod node_runtime {}

/*
Recursive function that looks up the identity of a given ss58 address,
outputs a tuple with [primary_identity/sub_identity], primary identity and whether
an identity is present.
*/
#[async_recursion]
pub async fn fetch_display_name(
    api: &OnlineClient<PolkadotConfig>,
    address: &AccountId32,
    sub_account_name: Option<String>,
) -> Result<Option<String>, ClaimitError> {
    let identity_of_addr = node_runtime::storage()
        .identity()
        .identity_of(address.clone());
    match api
        .storage()
        .at_latest()
        .await?
        .fetch(&identity_of_addr)
        .await?
    {
        Some(identity) => {
            let parent = parse_identity_data(identity.info.display);
            let name = match sub_account_name {
                Some(child) => format!("{}/{}", &parent, child),
                None => parent.clone(),
            };
            Ok(Some(name))
        }
        None => {
            let super_of_addr = node_runtime::storage().identity().super_of(address.clone());
            if let Some((parent_account, data)) = api
                .storage()
                .at_latest()
                .await?
                .fetch(&super_of_addr)
                .await?
            {
                let sub_account_name = parse_identity_data(data);
                return fetch_display_name(
                    &api,
                    &parent_account,
                    Some(sub_account_name.to_string()),
                )
                .await;
            } else {
                Ok(None)
            }
        }
    }
}

fn parse_identity_data(data: node_runtime::runtime_types::pallet_identity::types::Data) -> String {
    match data {
        node_runtime::runtime_types::pallet_identity::types::Data::Raw0(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw1(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw2(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw3(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw4(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw5(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw6(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw7(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw8(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw9(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw10(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw11(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw12(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw13(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw14(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw15(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw16(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw17(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw18(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw19(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw20(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw21(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw22(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw23(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw24(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw25(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw26(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw27(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw28(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw29(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw30(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw31(bytes) => {
            str(bytes.to_vec())
        }
        node_runtime::runtime_types::pallet_identity::types::Data::Raw32(bytes) => {
            str(bytes.to_vec())
        }
        _ => format!("???"),
    }
}
