use claimeer_common::runtimes::support::SupportedRelayRuntime;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub type SubscriptionId = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkStatus {
    Initializing,
    Active,
    Inactive,
}

impl std::fmt::Display for NetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Active => write!(f, "Active"),
            Self::Inactive => write!(f, "Inactive"),
        }
    }
}

/// NetworkState is a shared state between all components.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkState {
    /// An id to identify the subscription
    pub subscription_id: SubscriptionId,
    /// The status of the network.
    pub status: NetworkStatus,
    /// A runtime supported by the App.
    pub runtime: SupportedRelayRuntime,
    /// Network finalized block.
    pub finalized_block_number: Option<u32>,
    /// A counter to keep track of fetching queries.
    pub fetches_counter: u32,
}

impl NetworkState {
    pub fn new(runtime: SupportedRelayRuntime) -> Self {
        // Generate a unique subscription_id
        let mut rng = rand::thread_rng();
        let subscription_id = rng.gen::<SubscriptionId>();

        Self {
            subscription_id,
            status: NetworkStatus::Initializing,
            runtime,
            finalized_block_number: None,
            fetches_counter: 0,
        }
    }

    pub fn _is_initializing(&self) -> bool {
        self.status == NetworkStatus::Initializing
    }

    pub fn is_active(&self) -> bool {
        self.status == NetworkStatus::Active
    }

    pub fn is_fetching(&self) -> bool {
        self.fetches_counter > 0
    }

    pub fn is_busy(&self) -> bool {
        !self.is_active() || self.is_fetching()
    }

    pub fn _class(&self) -> String {
        self.runtime.to_string().to_lowercase()
    }
}
