use crate::types::child_bounties::ChildBountiesIds;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaimStatus {
    /// Initialize claiming process
    Initializing,
    /// Prepare payload to be ready for signing
    Preparing,
    /// Sign payload via browser extension
    Signing(String),
    /// Submit signed payload
    Submitting(Vec<u8>),
    /// Complete claiming process
    Completed,
    Error(String),
}

impl std::fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Preparing => write!(f, "Preparing"),
            Self::Signing(_) => write!(f, "Signing"),
            Self::Submitting(_) => write!(f, "Submitting"),
            Self::Completed => write!(f, "Completed"),
            Self::Error(_) => write!(f, "Error"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimState {
    /// An aray of child bounty ids
    pub child_bounty_ids: ChildBountiesIds,
    /// The status of the claim.
    pub status: ClaimStatus,
}

impl ClaimState {
    pub fn new(child_bounty_ids: ChildBountiesIds) -> Self {
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
            ClaimStatus::Signing(_) | ClaimStatus::Submitting(_) => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self.status {
            ClaimStatus::Error(_) => true,
            _ => false,
        }
    }
}
