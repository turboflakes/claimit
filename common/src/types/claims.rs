use crate::types::child_bounties::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaimStatus {
    Initializing,
    Signing,
    Submitting(Vec<u8>),
    Completed,
    Error,
}

impl std::fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Signing => write!(f, "Signing"),
            Self::Submitting(_) => write!(f, "Submitting"),
            Self::Completed => write!(f, "Completed"),
            Self::Error => write!(f, "Error"),
        }
    }
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

    pub fn is_signing_or_submitting(&self) -> bool {
        match self.status {
            ClaimStatus::Signing | ClaimStatus::Submitting(_) => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        self.status == ClaimStatus::Error
    }
}
