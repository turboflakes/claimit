use crate::runtimes::{
    support::SupportedRelayRuntime,
    utils::{amount_human, compact},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr};
use subxt::config::substrate::AccountId32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: usize,
    /// ss58 formatted address as string.
    pub address: String,
    /// account identity retrieved from people chain
    pub identity: Option<String>,
    /// disable from being claimable
    pub disabled: bool,
    /// child bounty ids where the account is a beneficiary
    pub child_bounty_ids: BTreeSet<u32>,
    /// account transferable balance
    pub free_balance: u128,
}

impl Account {
    pub fn to_compact_string(&self) -> String {
        match AccountId32::from_str(&self.address) {
            Ok(account) => compact(&account),
            _ => String::new(),
        }
    }

    pub fn free_balance_human(&self, runtime: SupportedRelayRuntime) -> String {
        amount_human(self.free_balance, runtime.decimals().into())
    }
}
