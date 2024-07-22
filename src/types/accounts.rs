use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr};
use subxt::config::substrate::AccountId32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: usize,
    /// account name
    pub name: String,
    /// name of the browser extension
    pub source: String,
    /// the signature type, e.g. "sr25519" or "ed25519"
    pub r#type: String,
    /// ss58 formatted address as string. Can be converted into AccountId32 via it's FromStr implementation.
    pub address: String,
    /// account identity retrieved from people chain
    pub identity: Option<String>,
    /// disable from being claimable
    pub disabled: bool,
    /// child bounty ids where the account is a beneficiary 
    pub child_bounty_ids: BTreeSet<u32>,
}

impl Account {
    pub fn to_compact_string(&self) -> String {
        match AccountId32::from_str(&self.address) {
            Ok(account) => compact(&account),
            _ => String::new(),
        }
    }
}

pub fn compact(account: &AccountId32) -> String {
    let a = account.to_string();
    [&a[0..4], &a[a.len() - 4..a.len()]].join("...")
}
