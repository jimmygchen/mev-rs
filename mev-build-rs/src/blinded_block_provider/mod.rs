#[cfg(feature = "api")]
mod api;

#[cfg(feature = "api")]
pub use {api::client::Client, api::server::Server, beacon_api_client::Error as ClientError};

use crate::{
    builder::Error as BuilderError,
    types::{
        BidRequest, ExecutionPayload, SignedBlindedBeaconBlock, SignedBuilderBid,
        SignedValidatorRegistration,
    },
};
use async_trait::async_trait;
use beacon_api_client::ApiError;
use ethereum_consensus::state_transition::Error as ConsensusError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Consensus(#[from] ConsensusError),
    #[error("{0}")]
    Api(#[from] ApiError),
    #[error("{0}")]
    Builder(#[from] BuilderError),
    #[error("internal server error")]
    Internal(String),
    #[error("{0}")]
    Custom(String),
}

#[cfg(feature = "api")]
impl From<ClientError> for Error {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Api(err) => err.into(),
            err => Error::Internal(err.to_string()),
        }
    }
}

#[async_trait]
pub trait BlindedBlockProvider {
    async fn register_validators(
        &self,
        registrations: &mut [SignedValidatorRegistration],
    ) -> Result<(), Error>;

    async fn fetch_best_bid<P: ExecutionPayload, B: SignedBuilderBid<P>>(
        &self,
        bid_request: &BidRequest,
        consensus_version: Option<&str>,
    ) -> Result<B, Error>;

    async fn open_bid<P: ExecutionPayload, B: SignedBlindedBeaconBlock>(
        &self,
        signed_block: &mut B,
    ) -> Result<P, Error>;
}
