use crate::types::child_bounties::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaimStatus {
    Initializing,
    Signing,
    //
    Inprocess,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimState {
    /// An aray of child bounty ids to claim
    pub child_bounty_ids: Vec<Id>,
    /// The status of the claim.
    pub status: ClaimStatus,
}

impl ClaimState {
    pub fn new(child_bounty_ids: Vec<Id>) -> Self {
        Self {
            child_bounty_ids,
            status: ClaimStatus::Initializing,
        }
    }
}
