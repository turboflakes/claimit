use crate::runtimes::support::SupportedRelayRuntime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkStatus {
    Initializing,
    Switching,
    Active,
    Inactive,
}

impl std::fmt::Display for NetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Switching => write!(f, "Switching"),
            Self::Active => write!(f, "Active"),
            Self::Inactive => write!(f, "Inactive"),
        }
    }
}

/// NetworkState is a shared state between all components.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkState {
    /// The status of the network.
    pub status: NetworkStatus,
    /// A runtime supported by the App.
    pub runtime: SupportedRelayRuntime,
    /// Network finalized block.
    pub finalized_block_number: Option<u32>,
}

impl NetworkState {
    pub fn new(runtime: SupportedRelayRuntime) -> Self {
        Self {
            status: NetworkStatus::Initializing,
            runtime,
            finalized_block_number: None,
        }
    }

    pub fn _is_initializing(&self) -> bool {
        self.status == NetworkStatus::Initializing
    }

    pub fn _is_active(&self) -> bool {
        self.status == NetworkStatus::Active
    }

    pub fn _is_switching(&self) -> bool {
        self.status == NetworkStatus::Switching
    }

    pub fn _class(&self) -> String {
        self.runtime.to_string().to_lowercase()
    }
}
