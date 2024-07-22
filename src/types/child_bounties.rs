use crate::runtimes::support::SupportedRelayRuntime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum_macros::{Display, EnumIter};
use subxt::utils::AccountId32;
use yew::{html, html::IntoPropValue, Html};

pub type Id = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChildBounty {
    pub id: Id,
    pub parent_id: Id,
    pub description: String,
    pub value: u128,
    pub status: Status,
    pub beneficiary: AccountId32,
    pub unlock_at: u32,
}

impl ChildBounty {
    pub fn value_human(&self, runtime: SupportedRelayRuntime) -> String {
        let base: u128 = 10;
        let n = self.value / base.pow(runtime.decimals().into()) as u128;
        let r = (self.value % base.pow(runtime.decimals().into()) as u128)
            / base.pow((runtime.decimals() - 2).into()) as u128;
        format!("{n}.{r}")
    }

    pub fn is_claimable(&self, block_numer: u32) -> bool {
        self.unlock_at < block_numer
    }
}

pub type ChildBounties = BTreeMap<Id, ChildBounty>;

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
    Claimable(Vec<Id>),
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
