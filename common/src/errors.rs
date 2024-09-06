use std::string::String;
use subxt::{
    error::{DispatchError, MetadataError, RpcError},
    lightclient::LightClientError,
};

use thiserror::Error;

/// Claimit specific error messages
#[derive(Error, Debug)]
pub enum ClaimitError {
    #[error("Subxt error: {0}")]
    SubxtError(#[from] subxt::Error),
    #[error("SubxtCore error: {0}")]
    SubxtCoreError(#[from] subxt::ext::subxt_core::Error),
    #[error("LightClient error: {0}")]
    LightClientError(#[from] LightClientError),
    #[error("Metadata error: {0}")]
    MetadataError(#[from] MetadataError),
    #[error("Dispatch error: {0}")]
    DispatchError(#[from] DispatchError),
    #[error("{0}")]
    RpcError(#[from] RpcError),
    #[error("Other error: {0}")]
    Other(String),
}

/// Convert &str to ClaimitError
impl From<&str> for ClaimitError {
    fn from(error: &str) -> Self {
        ClaimitError::Other(error.into())
    }
}
