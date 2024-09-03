use serde::{Deserialize, Serialize};
use yew::AttrValue;

pub type ChainPrefix = u16;

pub const POLKADOT_SPEC: &str = include_str!("../../artifacts/chain_specs/polkadot.json");
pub const POLKADOT_PEOPLE_SPEC: &str =
    include_str!("../../artifacts/chain_specs/polkadot_people.json");
pub const KUSAMA_SPEC: &str = include_str!("../../artifacts/chain_specs/kusama.json");
pub const KUSAMA_PEOPLE_SPEC: &str = include_str!("../../artifacts/chain_specs/kusama_people.json");
pub const ROCOCO_SPEC: &str = include_str!("../../artifacts/chain_specs/rococo.json");
pub const ROCOCO_PEOPLE_SPEC: &str = include_str!("../../artifacts/chain_specs/rococo_people.json");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum SupportedRelayRuntime {
    Polkadot,
    Kusama,
    Rococo,
}

impl SupportedRelayRuntime {
    pub fn _chain_prefix(&self) -> ChainPrefix {
        match &self {
            Self::Polkadot => 0,
            Self::Kusama => 2,
            Self::Rococo => 42,
        }
    }

    pub fn default_rpc_url(&self) -> &'static str {
        match &self {
            Self::Polkadot => "wss://rpc.ibp.network:443/polkadot",
            Self::Kusama => "wss://rpc.ibp.network:443/kusama",
            Self::Rococo => "wss://rococo-rpc.polkadot.io:443",
        }
    }

    pub fn default_people_rpc_url(&self) -> &'static str {
        match &self {
            Self::Polkadot => "wss://sys.ibp.network:443/people-polkadot",
            Self::Kusama => "wss://sys.ibp.network:443/people-kusama",
            Self::Rococo => "wss://rococo-people-rpc.polkadot.io:443",
        }
    }

    pub fn chain_specs(&self) -> &str {
        match &self {
            Self::Polkadot => POLKADOT_SPEC,
            Self::Kusama => KUSAMA_SPEC,
            Self::Rococo => ROCOCO_SPEC,
        }
    }

    pub fn chain_specs_people(&self) -> &str {
        match &self {
            Self::Polkadot => POLKADOT_PEOPLE_SPEC,
            Self::Kusama => KUSAMA_PEOPLE_SPEC,
            Self::Rococo => ROCOCO_PEOPLE_SPEC,
        }
    }

    pub fn unit(&self) -> &'static str {
        match &self {
            Self::Polkadot => "DOT",
            Self::Kusama => "KSM",
            Self::Rococo => "ROC",
        }
    }

    pub fn decimals(&self) -> u16 {
        match &self {
            Self::Polkadot => 10,
            Self::Kusama => 12,
            Self::Rococo => 12,
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
            "Rococo" => Self::Rococo,
            "rococo" => Self::Rococo,
            "ROC" => Self::Rococo,
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
            "Rococo" => Self::Rococo,
            "rococo" => Self::Rococo,
            "ROC" => Self::Rococo,
            _ => unimplemented!("Chain prefix not supported"),
        }
    }
}

impl From<ChainPrefix> for SupportedRelayRuntime {
    fn from(v: ChainPrefix) -> Self {
        match v {
            0 => Self::Polkadot,
            2 => Self::Kusama,
            42 => Self::Rococo,
            _ => unimplemented!("Chain prefix not supported"),
        }
    }
}

impl std::fmt::Display for SupportedRelayRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Polkadot => write!(f, "Polkadot"),
            Self::Kusama => write!(f, "Kusama"),
            Self::Rococo => write!(f, "Rococo"),
        }
    }
}
