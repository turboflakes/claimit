use crate::types::{accounts::ExtensionAccount, child_bounties::Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaimStatus {
    Initializing,
    Inprocess,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimState {
    /// The signer account
    pub signer: Option<ExtensionAccount>,
    /// An aray of child bounty ids to claim
    pub child_bounty_ids: Vec<Id>,
    /// The status of the claim.
    pub status: ClaimStatus,
}

impl ClaimState {
    pub fn new(child_bounty_ids: Vec<Id>, signer: Option<ExtensionAccount>) -> Self {
        Self {
            child_bounty_ids,
            signer,
            status: ClaimStatus::Initializing,
        }
    }
}
