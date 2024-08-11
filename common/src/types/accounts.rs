use crate::runtimes::{
    support::SupportedRelayRuntime,
    utils::{amount_human, compact},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr};
use subxt::config::substrate::AccountId32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: u32,
    /// ss58 formatted address as string.
    pub address: String,
    /// account identity retrieved from people chain
    pub identity: Option<String>,
    /// disable from being claimable
    pub disabled: bool,
    /// child bounty ids where the account is a beneficiary
    pub child_bounty_ids: BTreeSet<u32>,
    /// account balance
    pub balance: Balance,
}

impl Account {
    pub fn to_compact_string(&self) -> String {
        match AccountId32::from_str(&self.address) {
            Ok(account) => compact(&account),
            _ => String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Balance {
    pub free: u128,
    pub reserved: u128,
}

impl Balance {
    pub fn new() -> Self {
        Self {
            free: 0,
            reserved: 0,
        }
    }

    pub fn total(&self) -> u128 {
        self.free + self.reserved
    }

    pub fn free_human(&self, runtime: SupportedRelayRuntime) -> String {
        amount_human(self.free, runtime.decimals().into())
    }

    pub fn reserved_human(&self, runtime: SupportedRelayRuntime) -> String {
        amount_human(self.reserved, runtime.decimals().into())
    }

    pub fn total_human(&self, runtime: SupportedRelayRuntime) -> String {
        amount_human(self.free + self.reserved, runtime.decimals().into())
    }
}
