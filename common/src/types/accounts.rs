use crate::runtimes::utils::compact;
use anyhow::anyhow;
use js_sys::Promise;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::BTreeSet, str::FromStr};
use subxt::utils::Era;
use subxt::{
    config::substrate::AccountId32,
    ext::codec::{Compact, Encode},
    OnlineClient, PolkadotConfig,
};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;

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
}

impl Account {
    pub fn to_compact_string(&self) -> String {
        match AccountId32::from_str(&self.address) {
            Ok(account) => compact(&account),
            _ => String::new(),
        }
    }
}
