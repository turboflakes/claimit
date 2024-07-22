use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr};
use subxt::config::substrate::AccountId32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: usize,
    pub ss58: String,
    pub identity: Option<String>,
    pub disabled: bool,
    pub child_bounty_ids: BTreeSet<u32>,
}

impl Account {
    pub fn to_compact_string(&self) -> String {
        match AccountId32::from_str(&self.ss58) {
            Ok(account) => compact(&account),
            _ => String::new(),
        }
    }
}

pub fn compact(account: &AccountId32) -> String {
    let a = account.to_string();
    [&a[0..4], &a[a.len() - 4..a.len()]].join("...")
}
