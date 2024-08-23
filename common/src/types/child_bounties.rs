use crate::runtimes::{support::SupportedRelayRuntime, utils::amount_human};
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;
use strum_macros::{Display, EnumIter};
use subxt::utils::AccountId32;
use yew::{html, html::IntoPropValue, Html};

// TODO deprecated Id
pub type Id = u32;
pub type ParentBountyId = u32;
pub type ChildBountyId = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChildBounty {
    pub id: ChildBountyId,
    pub parent_id: ParentBountyId,
    pub description: String,
    pub value: u128,
    pub status: Status,
    pub beneficiary: AccountId32,
    pub unlock_at: u32,
}

impl ChildBounty {
    pub fn key(&self) -> (ParentBountyId, ChildBountyId) {
        (self.parent_id, self.id)
    }

    pub fn value_human(&self, runtime: SupportedRelayRuntime) -> String {
        amount_human(self.value, runtime.decimals().into())
    }

    pub fn is_claimable(&self, block_number: u32) -> bool {
        self.unlock_at < block_number
    }

    pub fn unlock_duration(&self, block_number: u32) -> String {
        if !self.is_claimable(block_number) {
            let n = self.unlock_at - block_number;
            let d = Duration::new(n as u64 * 6, 0);
            format_duration(d).to_string()
        } else {
            "".into()
        }
    }
}

pub type ChildBounties = BTreeMap<Id, ChildBounty>;
pub type ChildBountiesIds = Vec<(ParentBountyId, ChildBountyId)>;

#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Pending,
    Claimed,
}

impl IntoPropValue<Html> for Status {
    fn into_prop_value(self) -> Html {
        html! {<>{self.to_string()}</>}
    }
}

#[derive(Clone, Debug, EnumIter, Serialize, Deserialize, PartialEq, Eq)]
pub enum Filter {
    All,
    Following(Vec<AccountId32>),
    Claimable(Vec<ChildBountyId>),
}

impl Filter {
    pub fn check(&self, child_bounty: &ChildBounty) -> bool {
        match self {
            Filter::All => true,
            Filter::Following(accounts) => accounts
                .iter()
                .any(|account| *account == child_bounty.beneficiary),
            Filter::Claimable(child_bounty_ids) => child_bounty_ids
                .iter()
                .any(|id: &u32| *id == child_bounty.id),
        }
    }

    pub fn is_claimable(&self) -> bool {
        match self {
            Filter::Claimable(_) => true,
            _ => false,
        }
    }

    pub fn is_following(&self) -> bool {
        match self {
            Filter::Following(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::Following(_) => write!(f, "Following"),
            Self::Claimable(_) => write!(f, "Claimable"),
        }
    }
}

impl IntoPropValue<Html> for Filter {
    fn into_prop_value(self) -> Html {
        html! {<>{self.to_string()}</>}
    }
}
