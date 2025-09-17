use serde::{Deserialize, Serialize};
use yew::AttrValue;

pub type ChainPrefix = u16;

pub const POLKADOT_SPEC: &str = include_str!("../../artifacts/chain_specs/polkadot.json");
pub const POLKADOT_PEOPLE_SPEC: &str =
    include_str!("../../artifacts/chain_specs/polkadot_people.json");
pub const POLKADOT_ASSET_HUB_SPEC: &str =
    include_str!("../../artifacts/chain_specs/polkadot_asset_hub.json");
pub const KUSAMA_SPEC: &str = include_str!("../../artifacts/chain_specs/kusama.json");
pub const KUSAMA_PEOPLE_SPEC: &str = include_str!("../../artifacts/chain_specs/kusama_people.json");
pub const KUSAMA_ASSET_HUB_SPEC: &str =
    include_str!("../../artifacts/chain_specs/kusama_asset_hub.json");
pub const PASEO_SPEC: &str = include_str!("../../artifacts/chain_specs/paseo.json");
pub const PASEO_PEOPLE_SPEC: &str = include_str!("../../artifacts/chain_specs/paseo_people.json");
pub const PASEO_ASSET_HUB_SPEC: &str =
    include_str!("../../artifacts/chain_specs/paseo_asset_hub.json");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedRelayRuntime {
    Polkadot,
    Kusama,
    Paseo,
}

impl SupportedRelayRuntime {
    pub fn _chain_prefix(&self) -> ChainPrefix {
        match &self {
            Self::Polkadot => 0,
            Self::Kusama => 2,
            Self::Paseo => 0,
        }
    }

    pub fn default_rpc_url(&self) -> &'static str {
        match &self {
            Self::Polkadot => "wss://rpc.ibp.network:443/polkadot",
            Self::Kusama => "wss://rpc.ibp.network:443/kusama",
            Self::Paseo => "wss://rpc.ibp.network:443/paseo",
        }
    }

    pub fn default_people_rpc_url(&self) -> &'static str {
        match &self {
            Self::Polkadot => "wss://sys.ibp.network:443/people-polkadot",
            Self::Kusama => "wss://sys.ibp.network:443/people-kusama",
            Self::Paseo => "wss://sys.ibp.network:443/people-paseo",
        }
    }

    pub fn default_asset_hub_rpc_url(&self) -> &'static str {
        match &self {
            Self::Polkadot => "wss://sys.ibp.network:443/asset-hub-polkadot",
            Self::Kusama => "wss://sys.ibp.network:443/asset-hub-kusama",
            Self::Paseo => "wss://sys.ibp.network:443/asset-hub-paseo",
        }
    }

    pub fn chain_specs(&self) -> &str {
        match &self {
            Self::Polkadot => POLKADOT_SPEC,
            Self::Kusama => KUSAMA_SPEC,
            Self::Paseo => PASEO_SPEC,
        }
    }

    pub fn chain_specs_people(&self) -> &str {
        match &self {
            Self::Polkadot => POLKADOT_PEOPLE_SPEC,
            Self::Kusama => KUSAMA_PEOPLE_SPEC,
            Self::Paseo => PASEO_PEOPLE_SPEC,
        }
    }

    pub fn chain_specs_asset_hub(&self) -> &str {
        match &self {
            Self::Polkadot => POLKADOT_ASSET_HUB_SPEC,
            Self::Kusama => KUSAMA_ASSET_HUB_SPEC,
            Self::Paseo => PASEO_ASSET_HUB_SPEC,
        }
    }

    pub fn unit(&self) -> &'static str {
        match &self {
            Self::Polkadot => "DOT",
            Self::Kusama => "KSM",
            Self::Paseo => "PAS",
        }
    }

    pub fn decimals(&self) -> u16 {
        match &self {
            Self::Polkadot => 10,
            Self::Kusama => 12,
            Self::Paseo => 10,
        }
    }

    pub fn class(&self) -> String {
        self.to_string().to_lowercase()
    }
}

impl Default for SupportedRelayRuntime {
    fn default() -> Self {
        SupportedRelayRuntime::Polkadot
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
            "Paseo" => Self::Paseo,
            "paseo" => Self::Paseo,
            "PAS" => Self::Paseo,
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
            "Paseo" => Self::Paseo,
            "paseo" => Self::Paseo,
            "PAS" => Self::Paseo,
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
            Self::Paseo => write!(f, "Paseo"),
        }
    }
}
