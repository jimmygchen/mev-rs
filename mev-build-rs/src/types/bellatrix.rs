use crate::types::base::{BuilderBid, SignedBuilderBid};
use crate::{ExecutionPayload, SignedBlindedBeaconBlock};
pub use ethereum_consensus::bellatrix::mainnet::{
    ExecutionPayload as ExecutionPayloadBellatrix,
    ExecutionPayloadHeader as ExecutionPayloadHeaderBellatrix,
    SignedBlindedBeaconBlock as SignedBlindedBeaconBlockBellatrix,
};
use ethereum_consensus::primitives::{BlsPublicKey, BlsSignature};
use ssz_rs::prelude::*;
use ssz_rs::U256;

impl ExecutionPayload for ExecutionPayloadBellatrix {}
impl SignedBlindedBeaconBlock for SignedBlindedBeaconBlockBellatrix {}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuilderBidBellatrix {
    pub header: ExecutionPayloadHeaderBellatrix,
    pub value: U256,
    #[serde(rename = "pubkey")]
    pub public_key: BlsPublicKey,
}

impl BuilderBid<ExecutionPayloadBellatrix> for BuilderBidBellatrix {}

#[derive(Debug, Default, Clone, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedBuilderBidBellatrix {
    pub message: BuilderBidBellatrix,
    pub signature: BlsSignature,
}

impl SignedBuilderBid<ExecutionPayloadBellatrix> for SignedBuilderBidBellatrix {}
