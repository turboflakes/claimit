use crate::runtimes::support::SupportedRelayRuntime;
use crate::types::{
    accounts::Balance,
    child_bounties::{ChildBounties, ChildBountiesIds},
    network::SubscriptionId,
};
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;

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
