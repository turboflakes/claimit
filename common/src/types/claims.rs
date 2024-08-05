use crate::types::child_bounties::Id;
use serde::{Deserialize, Serialize};
use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaimStatus {
    Initializing,
    Signing,
    Inprogress,
    Completed,
    Error,
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

    pub fn is_initializing(&self) -> bool {
        self.status == ClaimStatus::Initializing
    }

    pub fn is_signing(&self) -> bool {
        self.status == ClaimStatus::Signing
    }

    pub fn is_inprogress(&self) -> bool {
        self.status == ClaimStatus::Inprogress
    }

    pub fn is_error(&self) -> bool {
        self.status == ClaimStatus::Error
    }
}
