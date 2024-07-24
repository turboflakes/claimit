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

    pub fn default_rpc_url(&self) -> &'static str {
        match &self {
            Self::Polkadot => "wss://rpc.ibp.network:443/polkadot",
            Self::Kusama => "wss://rpc.ibp.network:443/kusama",
        }
    }

    pub fn unit(&self) -> &'static str {
        match &self {
            Self::Polkadot => "DOT",
            Self::Kusama => "KSM",
        }
    }

    pub fn decimals(&self) -> u16 {
        match &self {
            Self::Polkadot => 10,
            Self::Kusama => 12,
        }
    }

    pub fn class(&self) -> String {
        self.to_string().to_lowercase()
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
