pub use ethereum_consensus::builder::SignedValidatorRegistration;
use ethereum_consensus::primitives::{BlsPublicKey, Hash32, Slot};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use ssz_rs::prelude::SimpleSerialize;
use ssz_rs::Sized;
use ssz_rs::U256;
use std::fmt::{Debug, Display};

pub trait BuilderBid<T: ExecutionPayload> {}

pub trait SignedBuilderBid<T: ExecutionPayload> {}

pub trait ExecutionPayload: DeserializeOwned + Default + Serialize + DeserializeOwned {}

pub trait SignedBlindedBeaconBlock: Serialize {}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BidRequest {
    #[serde(with = "crate::serde::as_string")]
    pub slot: Slot,
    pub parent_hash: Hash32,
    pub public_key: BlsPublicKey,
}

#[derive(Debug, Default, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionPayloadWithValue<T: ExecutionPayload> {
    #[serde(with = "crate::serde::as_string")]
    pub payload: T,
    pub value: U256,
}
