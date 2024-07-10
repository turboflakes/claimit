use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use subxt::utils::AccountId32;

pub type Id = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChildBounty {
    pub id: Id,
    pub parent_bounty: Id,
    pub value: u128,
    pub status: Status,
    pub beneficiary: AccountId32,
    pub unlock_at: u32,
}

pub type ChildBounties = BTreeMap<Id, ChildBounty>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Pending,
    Claimed,
}
