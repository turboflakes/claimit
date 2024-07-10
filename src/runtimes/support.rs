use serde::{Deserialize, Serialize};
use yew::AttrValue;

pub type ChainPrefix = u16;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum SupportedRelayRuntime {
    Polkadot,
    Kusama,
}

impl SupportedRelayRuntime {
    pub fn _chain_prefix(&self) -> ChainPrefix {
        match &self {
            Self::Polkadot => 0,
            Self::Kusama => 2,
        }
    }

    pub fn default_rpc_url(&self) -> String {
        match &self {
            Self::Polkadot => "wss://rpc.turboflakes.io:443/polkadot".to_string(),
            Self::Kusama => "wss://rpc.turboflakes.io:443/kusama".to_string(),
        }
    }
}

impl From<AttrValue> for SupportedRelayRuntime {
    fn from(v: AttrValue) -> Self {
        match v.as_str() {
            "Polkadot" => Self::Polkadot,
            "polkadot" => Self::Polkadot,
            "DOT" => Self::Polkadot,
            "Kusama" => Self::Kusama,
            "kusama" => Self::Kusama,
            "KSM" => Self::Kusama,
            _ => unimplemented!("Chain prefix not supported"),
        }
    }
}

impl From<String> for SupportedRelayRuntime {
    fn from(v: String) -> Self {
        match v.as_str() {
            "Polkadot" => Self::Polkadot,
            "polkadot" => Self::Polkadot,
            "DOT" => Self::Polkadot,
            "Kusama" => Self::Kusama,
            "kusama" => Self::Kusama,
            "KSM" => Self::Kusama,
            _ => unimplemented!("Chain prefix not supported"),
        }
    }
}

impl From<ChainPrefix> for SupportedRelayRuntime {
    fn from(v: ChainPrefix) -> Self {
        match v {
            0 => Self::Polkadot,
            2 => Self::Kusama,
            _ => unimplemented!("Chain prefix not supported"),
        }
    }
}

impl std::fmt::Display for SupportedRelayRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Polkadot => write!(f, "Polkadot"),
            Self::Kusama => write!(f, "Kusama"),
        }
    }
}
