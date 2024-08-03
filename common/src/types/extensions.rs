use crate::runtimes::utils::compact;
use anyhow::anyhow;
use js_sys::Promise;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::BTreeSet, str::FromStr};
use subxt::{
    config::substrate::AccountId32,
    ext::codec::{Compact, Encode},
    utils::Era,
    OnlineClient, PolkadotConfig,
};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExtensionStatus {
    /// An extension instance has been been created
    Initialized,
    /// The extension is being connected
    Connecting,
    /// The extension is available, connected and accounts enabled
    Connected,
    /// The signer account is available in the connected extension
    Ready,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionState {
    /// The signer account
    pub signer: Option<ExtensionAccount>,
    /// The status of the claim.
    pub status: ExtensionStatus,
}

impl ExtensionState {
    pub fn new(signer: Option<ExtensionAccount>) -> Self {
        Self {
            signer,
            status: ExtensionStatus::Initialized,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.status == ExtensionStatus::Connected
    }

    pub fn is_ready(&self) -> bool {
        self.status == ExtensionStatus::Ready
    }

    pub fn is_connected_or_ready(&self) -> bool {
        self.is_connected() || self.is_ready()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtensionAccount {
    /// account name
    pub name: String,
    /// name of the browser extension
    pub source: String,
    /// the signature type, e.g. "sr25519" or "ed25519"
    pub r#type: String,
    /// ss58 formatted address as string.
    pub address: String,
}

impl ExtensionAccount {
    pub fn to_compact_string(&self) -> String {
        match AccountId32::from_str(&self.address) {
            Ok(account) => compact(&account),
            _ => String::new(),
        }
    }

    pub fn is_pjs(&self) -> bool {
        self.source == "polkadot-js".to_string()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getAccounts)]
    pub fn js_get_accounts() -> Promise;
    #[wasm_bindgen(js_name = signPayload)]
    pub fn js_sign_payload(payload: String, source: String, address: String) -> Promise;
}

pub async fn get_accounts() -> Result<Vec<ExtensionAccount>, anyhow::Error> {
    let result = JsFuture::from(js_get_accounts())
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let accounts_str = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    let accounts: Vec<ExtensionAccount> = serde_json::from_str(&accounts_str)?;

    // TODO: Signing is currently only available for polkadot-js extension, further testing is needed for other wallets
    Ok(accounts
        .into_iter()
        .filter(|account| account.is_pjs())
        .collect())
}

fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex::encode(bytes.as_ref()))
}

fn encode_then_hex<E: Encode>(input: &E) -> String {
    format!("0x{}", hex::encode(input.encode()))
}

/// communicates with JavaScript to obtain a signature for the `partial_extrinsic` via a browser extension (e.g. polkadot-js or Talisman)
///
/// Some parameters are hard-coded here and not taken from the partial_extrinsic itself (mortality_checkpoint, era, tip).
pub async fn extension_signature_for_extrinsic(
    call_data: &[u8],
    api: &OnlineClient<PolkadotConfig>,
    account_nonce: u64,
    account_source: String,
    account_address: String,
) -> Result<Vec<u8>, anyhow::Error> {
    let genesis_hash = encode_then_hex(&api.genesis_hash());
    // These numbers aren't SCALE encoded; their bytes are just converted to hex:
    let spec_version = to_hex(&api.runtime_version().spec_version.to_be_bytes());
    let transaction_version = to_hex(&api.runtime_version().transaction_version.to_be_bytes());
    let nonce = to_hex(&account_nonce.to_be_bytes());
    // If you construct a mortal transaction, then this block hash needs to correspond
    // to the block number passed to `Era::mortal()`.
    let mortality_checkpoint = encode_then_hex(&api.genesis_hash());
    let era = encode_then_hex(&Era::Immortal);
    let method = to_hex(call_data);
    let signed_extensions: Vec<String> = api
        .metadata()
        .extrinsic()
        .signed_extensions()
        .iter()
        .map(|e| e.identifier().to_string())
        .collect();
    let tip = encode_then_hex(&Compact(0u128));

    let payload = json!({
        "specVersion": spec_version,
        "transactionVersion": transaction_version,
        "address": account_address,
        "blockHash": mortality_checkpoint,
        "blockNumber": "0x00000000",
        "era": era,
        "genesisHash": genesis_hash,
        "method": method,
        "nonce": nonce,
        "signedExtensions": signed_extensions,
        "tip": tip,
        "version": 4,
    });

    let payload = payload.to_string();
    let result = JsFuture::from(js_sign_payload(payload, account_source, account_address))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let signature = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    let signature = hex::decode(&signature[2..])?;
    Ok(signature)
}